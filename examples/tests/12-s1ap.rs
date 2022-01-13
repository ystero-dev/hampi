#![allow(dead_code, unreachable_patterns, non_camel_case_types)]
use asn1_codecs_derive::AperCodec;
use bitvec::order::Msb0;
use bitvec::vec::BitVec;

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "0",
    sz_ub = "256"
)]
pub struct ActivatedCellsList(Vec<ActivatedCellsList_Item>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true)]
pub struct ActivatedCellsList_Item {
    pub cell_id: OCTET_STRING_2,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct Additional_GUTI {
    pub gummei: GUMMEI,
    pub m_tmsi: M_TMSI,
    #[asn(optional_idx = 0)]
    pub ie_extensions: Option<Additional_GUTIIE_Extensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "ENUMERATED", extensible = true, lb = "0", ub = "1")]
pub struct AdditionalCSFallbackIndicator(u8);
impl AdditionalCSFallbackIndicator {
    const NO_RESTRICTION: u8 = 0u8;
    const RESTRICTION: u8 = 1u8;
}

#[derive(Debug, AperCodec)]
#[asn(type = "BITSTRING", sz_extensible = false, sz_lb = "32", sz_ub = "32")]
pub struct AdditionalRRMPriorityIndex(BitVec<Msb0, u8>);

#[derive(Debug, AperCodec)]
#[asn(type = "ENUMERATED", extensible = true, lb = "0", ub = "1")]
pub struct AerialUEsubscriptionInformation(u8);
impl AerialUEsubscriptionInformation {
    const ALLOWED: u8 = 0u8;
    const NOT_ALLOWED: u8 = 1u8;
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct AllocationAndRetentionPriority {
    pub priority_level: PriorityLevel,
    pub pre_emption_capability: Pre_emptionCapability,
    pub pre_emption_vulnerability: Pre_emptionVulnerability,
    #[asn(optional_idx = 0)]
    pub ie_extensions: Option<AllocationAndRetentionPriorityIE_Extensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "CHOICE", lb = "0", ub = "2", extensible = true)]
pub enum AreaScopeOfMDT {
    #[asn(key = 0, extended = false)]
    CellBased(CellBasedMDT),
    #[asn(key = 1, extended = false)]
    TABased(TABasedMDT),
    #[asn(key = 2, extended = false)]
    PLMNWide(NULL_3),
    #[asn(key = 0, extended = true)]
    TAIBased(TAIBasedMDT),
}

#[derive(Debug, AperCodec)]
#[asn(type = "CHOICE", lb = "0", ub = "3", extensible = true)]
pub enum AreaScopeOfQMC {
    #[asn(key = 0, extended = false)]
    CellBased(CellBasedQMC),
    #[asn(key = 1, extended = false)]
    TABased(TABasedQMC),
    #[asn(key = 2, extended = false)]
    TAIBased(TAIBasedQMC),
    #[asn(key = 3, extended = false)]
    PLMNAreaBased(PLMNAreaBasedQMC),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct AssistanceDataForCECapableUEs {
    pub cell_identifier_and_ce_level_for_ce_capable_u_es: CellIdentifierAndCELevelForCECapableUEs,
    #[asn(optional_idx = 0)]
    pub ie_extensions: Option<AssistanceDataForCECapableUEsIE_Extensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 4)]
pub struct AssistanceDataForPaging {
    #[asn(optional_idx = 0)]
    pub assistance_data_for_recommended_cells: Option<AssistanceDataForRecommendedCells>,
    #[asn(optional_idx = 1)]
    pub assistance_data_for_ce_capable_u_es: Option<AssistanceDataForCECapableUEs>,
    #[asn(optional_idx = 2)]
    pub paging_attempt_information: Option<PagingAttemptInformation>,
    #[asn(optional_idx = 3)]
    pub ie_extensions: Option<AssistanceDataForPagingIE_Extensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct AssistanceDataForRecommendedCells {
    pub recommended_cells_for_paging: RecommendedCellsForPaging,
    #[asn(optional_idx = 0)]
    pub ie_extensions: Option<AssistanceDataForRecommendedCellsIE_Extensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "6")]
pub struct BPLMNs(Vec<PLMNidentity>);

#[derive(Debug, AperCodec)]
#[asn(type = "ENUMERATED", extensible = true, lb = "0", ub = "0")]
pub struct BearerType(u8);
impl BearerType {
    const NON_IP: u8 = 0u8;
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct Bearers_SubjectToEarlyStatusTransfer_Item {
    pub e_rab_id: E_RAB_ID,
    pub dlcount_pdcp_s_nlength: DLCOUNT_PDCP_SNlength,
    #[asn(optional_idx = 0)]
    pub ie_extensions: Option<Bearers_SubjectToEarlyStatusTransfer_ItemIE_Extensions>,
}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "256"
)]
pub struct Bearers_SubjectToEarlyStatusTransferList(
    Vec<Bearers_SubjectToEarlyStatusTransferList_Entry>,
);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 2)]
pub struct Bearers_SubjectToStatusTransfer_Item {
    pub e_rab_id: E_RAB_ID,
    pub ul_coun_tvalue: COUNTvalue,
    pub dl_coun_tvalue: COUNTvalue,
    #[asn(optional_idx = 0)]
    pub receive_statusof_ulpdcpsd_us: Option<ReceiveStatusofULPDCPSDUs>,
    #[asn(optional_idx = 1)]
    pub ie_extensions: Option<Bearers_SubjectToStatusTransfer_ItemIE_Extensions>,
}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "256"
)]
pub struct Bearers_SubjectToStatusTransferList(Vec<Bearers_SubjectToStatusTransferList_Entry>);

#[derive(Debug, AperCodec)]
#[asn(type = "INTEGER", lb = "0", ub = "10000000000")]
pub struct BitRate(u64);

#[derive(Debug, AperCodec)]
#[asn(type = "ENUMERATED", extensible = true, lb = "0", ub = "0")]
pub struct BluetoothMeasConfig(u8);
impl BluetoothMeasConfig {
    const SETUP: u8 = 0u8;
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "4")]
pub struct BluetoothMeasConfigNameList(Vec<BluetoothName>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 3)]
pub struct BluetoothMeasurementConfiguration {
    pub bluetooth_meas_config: BluetoothMeasConfig,
    #[asn(optional_idx = 0)]
    pub bluetooth_meas_config_name_list: Option<BluetoothMeasConfigNameList>,
    #[asn(optional_idx = 1)]
    pub bt_rssi: Option<ENUMERATED_4>,
    #[asn(optional_idx = 2)]
    pub ie_extensions: Option<BluetoothMeasurementConfigurationIE_Extensions>,
}

#[derive(Debug, AperCodec)]
#[asn(
    type = "OCTET-STRING",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "248"
)]
pub struct BluetoothName(Vec<u8>);

#[derive(Debug, AperCodec)]
#[asn(type = "CHOICE", lb = "0", ub = "2", extensible = true)]
pub enum BroadcastCancelledAreaList {
    #[asn(key = 0, extended = false)]
    CellID_Cancelled(CellID_Cancelled),
    #[asn(key = 1, extended = false)]
    TAI_Cancelled(TAI_Cancelled),
    #[asn(key = 2, extended = false)]
    EmergencyAreaID_Cancelled(EmergencyAreaID_Cancelled),
}

#[derive(Debug, AperCodec)]
#[asn(type = "CHOICE", lb = "0", ub = "2", extensible = true)]
pub enum BroadcastCompletedAreaList {
    #[asn(key = 0, extended = false)]
    CellID_Broadcast(CellID_Broadcast),
    #[asn(key = 1, extended = false)]
    TAI_Broadcast(TAI_Broadcast),
    #[asn(key = 2, extended = false)]
    EmergencyAreaID_Broadcast(EmergencyAreaID_Broadcast),
}

#[derive(Debug, AperCodec)]
#[asn(type = "ENUMERATED", extensible = true, lb = "0", ub = "1")]
pub struct CE_ModeBRestricted(u8);
impl CE_ModeBRestricted {
    const RESTRICTED: u8 = 0u8;
    const NOT_RESTRICTED: u8 = 1u8;
}

#[derive(Debug, AperCodec)]
#[asn(type = "ENUMERATED", extensible = true, lb = "0", ub = "0")]
pub struct CE_mode_B_SupportIndicator(u8);
impl CE_mode_B_SupportIndicator {
    const SUPPORTED: u8 = 0u8;
}

#[derive(Debug, AperCodec)]
#[asn(type = "OCTET-STRING")]
pub struct CELevel(Vec<u8>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 2)]
pub struct CGI {
    pub plm_nidentity: PLMNidentity,
    pub lac: LAC,
    pub ci: CI,
    #[asn(optional_idx = 0)]
    pub rac: Option<RAC>,
    #[asn(optional_idx = 1)]
    pub ie_extensions: Option<CGIIE_Extensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "OCTET-STRING", sz_extensible = false, sz_lb = "2", sz_ub = "2")]
pub struct CI(Vec<u8>);

#[derive(Debug, AperCodec)]
#[asn(type = "ENUMERATED", lb = "0", ub = "1")]
pub struct CNDomain(u8);
impl CNDomain {
    const PS: u8 = 0u8;
    const CS: u8 = 1u8;
}

#[derive(Debug, AperCodec)]
#[asn(type = "ENUMERATED", extensible = true, lb = "0", ub = "0")]
pub struct CNType(u8);
impl CNType {
    const FIVE_GC_FORBIDDEN: u8 = 0u8;
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "16")]
pub struct CNTypeRestrictions(Vec<CNTypeRestrictions_Item>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct CNTypeRestrictions_Item {
    pub plmn_identity: PLMNidentity,
    pub cn_type: CNType,
    #[asn(optional_idx = 0)]
    pub ie_extensions: Option<CNTypeRestrictions_ItemIE_Extensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct COUNTValueExtended {
    pub pdcp_sn_extended: PDCP_SNExtended,
    pub hfn_modified: HFNModified,
    #[asn(optional_idx = 0)]
    pub ie_extensions: Option<COUNTValueExtendedIE_Extensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct COUNTvalue {
    pub pdcp_sn: PDCP_SN,
    pub hfn: HFN,
    #[asn(optional_idx = 0)]
    pub ie_extensions: Option<COUNTvalueIE_Extensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct COUNTvaluePDCP_SNlength18 {
    pub pdcp_s_nlength18: PDCP_SNlength18,
    pub hf_nfor_pdcp_s_nlength18: HFNforPDCP_SNlength18,
    #[asn(optional_idx = 0)]
    pub ie_extensions: Option<COUNTvaluePDCP_SNlength18IE_Extensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "ENUMERATED", extensible = true, lb = "0", ub = "0")]
pub struct CSFallbackIndicator(u8);
impl CSFallbackIndicator {
    const CS_FALLBACK_REQUIRED: u8 = 0u8;
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
pub struct CSG_IdList(Vec<CSG_IdList_Item>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct CSG_IdList_Item {
    pub csg_id: CSG_Id,
    #[asn(optional_idx = 0)]
    pub ie_extensions: Option<CSG_IdList_ItemIE_Extensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 3)]
pub struct CSGMembershipInfo {
    pub csg_membership_status: CSGMembershipStatus,
    pub csg_id: CSG_Id,
    #[asn(optional_idx = 0)]
    pub cell_access_mode: Option<CellAccessMode>,
    #[asn(optional_idx = 1)]
    pub plm_nidentity: Option<PLMNidentity>,
    #[asn(optional_idx = 2)]
    pub ie_extensions: Option<CSGMembershipInfoIE_Extensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "ENUMERATED", lb = "0", ub = "1")]
pub struct CSGMembershipStatus(u8);
impl CSGMembershipStatus {
    const MEMBER: u8 = 0u8;
    const NOT_MEMBER: u8 = 1u8;
}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct CancelledCellinEAI(Vec<CancelledCellinEAI_Item>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct CancelledCellinEAI_Item {
    pub ecgi: EUTRAN_CGI,
    pub number_of_broadcasts: NumberOfBroadcasts,
    #[asn(optional_idx = 0)]
    pub ie_extensions: Option<CancelledCellinEAI_ItemIE_Extensions>,
}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct CancelledCellinTAI(Vec<CancelledCellinTAI_Item>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct CancelledCellinTAI_Item {
    pub ecgi: EUTRAN_CGI,
    pub number_of_broadcasts: NumberOfBroadcasts,
    #[asn(optional_idx = 0)]
    pub ie_extensions: Option<CancelledCellinTAI_ItemIE_Extensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "16")]
pub struct CandidateCellList(Vec<IRAT_Cell_ID>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true)]
pub struct CandidatePCI {
    pub pci: INTEGER_5,
    pub earfcn: OCTET_STRING_6,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "16")]
pub struct CandidatePCIList(Vec<CandidatePCI>);

#[derive(Debug, AperCodec)]
#[asn(type = "CHOICE", lb = "0", ub = "4", extensible = true)]
pub enum Cause {
    #[asn(key = 0, extended = false)]
    RadioNetwork(CauseRadioNetwork),
    #[asn(key = 1, extended = false)]
    Transport(CauseTransport),
    #[asn(key = 2, extended = false)]
    Nas(CauseNas),
    #[asn(key = 3, extended = false)]
    Protocol(CauseProtocol),
    #[asn(key = 4, extended = false)]
    Misc(CauseMisc),
}

#[derive(Debug, AperCodec)]
#[asn(type = "ENUMERATED", extensible = true, lb = "0", ub = "5")]
pub struct CauseMisc(u8);
impl CauseMisc {
    const CONTROL_PROCESSING_OVERLOAD: u8 = 0u8;
    const NOT_ENOUGH_USER_PLANE_PROCESSING_RESOURCES: u8 = 1u8;
    const HARDWARE_FAILURE: u8 = 2u8;
    const OM_INTERVENTION: u8 = 3u8;
    const UNSPECIFIED: u8 = 4u8;
    const UNKNOWN_PLMN: u8 = 5u8;
}

#[derive(Debug, AperCodec)]
#[asn(type = "ENUMERATED", extensible = true, lb = "0", ub = "3")]
pub struct CauseNas(u8);
impl CauseNas {
    const NORMAL_RELEASE: u8 = 0u8;
    const AUTHENTICATION_FAILURE: u8 = 1u8;
    const DETACH: u8 = 2u8;
    const UNSPECIFIED: u8 = 3u8;
}

#[derive(Debug, AperCodec)]
#[asn(type = "ENUMERATED", extensible = true, lb = "0", ub = "6")]
pub struct CauseProtocol(u8);
impl CauseProtocol {
    const TRANSFER_SYNTAX_ERROR: u8 = 0u8;
    const ABSTRACT_SYNTAX_ERROR_REJECT: u8 = 1u8;
    const ABSTRACT_SYNTAX_ERROR_IGNORE_AND_NOTIFY: u8 = 2u8;
    const MESSAGE_NOT_COMPATIBLE_WITH_RECEIVER_STATE: u8 = 3u8;
    const SEMANTIC_ERROR: u8 = 4u8;
    const ABSTRACT_SYNTAX_ERROR_FALSELY_CONSTRUCTED_MESSAGE: u8 = 5u8;
    const UNSPECIFIED: u8 = 6u8;
}

#[derive(Debug, AperCodec)]
#[asn(type = "ENUMERATED", extensible = true, lb = "0", ub = "35")]
pub struct CauseRadioNetwork(u8);
impl CauseRadioNetwork {
    const UNSPECIFIED: u8 = 0u8;
    const TX2RELOCOVERALL_EXPIRY: u8 = 1u8;
    const SUCCESSFUL_HANDOVER: u8 = 2u8;
    const RELEASE_DUE_TO_EUTRAN_GENERATED_REASON: u8 = 3u8;
    const HANDOVER_CANCELLED: u8 = 4u8;
    const PARTIAL_HANDOVER: u8 = 5u8;
    const HO_FAILURE_IN_TARGET_EPC_E_NB_OR_TARGET_SYSTEM: u8 = 6u8;
    const HO_TARGET_NOT_ALLOWED: u8 = 7u8;
    const T_S1RELOCOVERALL_EXPIRY: u8 = 8u8;
    const T_S1RELOCPREP_EXPIRY: u8 = 9u8;
    const CELL_NOT_AVAILABLE: u8 = 10u8;
    const UNKNOWN_TARGET_ID: u8 = 11u8;
    const NO_RADIO_RESOURCES_AVAILABLE_IN_TARGET_CELL: u8 = 12u8;
    const UNKNOWN_MME_UE_S1AP_ID: u8 = 13u8;
    const UNKNOWN_ENB_UE_S1AP_ID: u8 = 14u8;
    const UNKNOWN_PAIR_UE_S1AP_ID: u8 = 15u8;
    const HANDOVER_DESIRABLE_FOR_RADIO_REASON: u8 = 16u8;
    const TIME_CRITICAL_HANDOVER: u8 = 17u8;
    const RESOURCE_OPTIMISATION_HANDOVER: u8 = 18u8;
    const REDUCE_LOAD_IN_SERVING_CELL: u8 = 19u8;
    const USER_INACTIVITY: u8 = 20u8;
    const RADIO_CONNECTION_WITH_UE_LOST: u8 = 21u8;
    const LOAD_BALANCING_TAU_REQUIRED: u8 = 22u8;
    const CS_FALLBACK_TRIGGERED: u8 = 23u8;
    const UE_NOT_AVAILABLE_FOR_PS_SERVICE: u8 = 24u8;
    const RADIO_RESOURCES_NOT_AVAILABLE: u8 = 25u8;
    const FAILURE_IN_RADIO_INTERFACE_PROCEDURE: u8 = 26u8;
    const INVALID_QOS_COMBINATION: u8 = 27u8;
    const INTERRAT_REDIRECTION: u8 = 28u8;
    const INTERACTION_WITH_OTHER_PROCEDURE: u8 = 29u8;
    const UNKNOWN_E_RAB_ID: u8 = 30u8;
    const MULTIPLE_E_RAB_ID_INSTANCES: u8 = 31u8;
    const ENCRYPTION_AND_OR_INTEGRITY_PROTECTION_ALGORITHMS_NOT_SUPPORTED: u8 = 32u8;
    const S1_INTRA_SYSTEM_HANDOVER_TRIGGERED: u8 = 33u8;
    const S1_INTER_SYSTEM_HANDOVER_TRIGGERED: u8 = 34u8;
    const X2_HANDOVER_TRIGGERED: u8 = 35u8;
}

#[derive(Debug, AperCodec)]
#[asn(type = "ENUMERATED", extensible = true, lb = "0", ub = "1")]
pub struct CauseTransport(u8);
impl CauseTransport {
    const TRANSPORT_RESOURCE_UNAVAILABLE: u8 = 0u8;
    const UNSPECIFIED: u8 = 1u8;
}

#[derive(Debug, AperCodec)]
#[asn(type = "ENUMERATED", extensible = true, lb = "0", ub = "0")]
pub struct Cdma2000HORequiredIndication(u8);
impl Cdma2000HORequiredIndication {
    const TRUE: u8 = 0u8;
}

#[derive(Debug, AperCodec)]
#[asn(type = "ENUMERATED", extensible = true, lb = "0", ub = "1")]
pub struct Cdma2000HOStatus(u8);
impl Cdma2000HOStatus {
    const H_O_SUCCESS: u8 = 0u8;
    const H_O_FAILURE: u8 = 1u8;
}

#[derive(Debug, AperCodec)]
#[asn(type = "OCTET-STRING")]
pub struct Cdma2000OneXMEID(Vec<u8>);

#[derive(Debug, AperCodec)]
#[asn(type = "OCTET-STRING")]
pub struct Cdma2000OneXMSI(Vec<u8>);

#[derive(Debug, AperCodec)]
#[asn(type = "OCTET-STRING")]
pub struct Cdma2000OneXPilot(Vec<u8>);

#[derive(Debug, AperCodec)]
#[asn(type = "OCTET-STRING")]
pub struct Cdma2000OneXRAND(Vec<u8>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct Cdma2000OneXSRVCCInfo {
    pub cdma2000_one_xmeid: Cdma2000OneXMEID,
    pub cdma2000_one_xmsi: Cdma2000OneXMSI,
    pub cdma2000_one_x_pilot: Cdma2000OneXPilot,
    #[asn(optional_idx = 0)]
    pub ie_extensions: Option<Cdma2000OneXSRVCCInfoIE_Extensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "OCTET-STRING")]
pub struct Cdma2000PDU(Vec<u8>);

#[derive(Debug, AperCodec)]
#[asn(type = "ENUMERATED", extensible = true, lb = "0", ub = "1")]
pub struct Cdma2000RATType(u8);
impl Cdma2000RATType {
    const H_RPD: u8 = 0u8;
    const ONEX_RTT: u8 = 1u8;
}

#[derive(Debug, AperCodec)]
#[asn(type = "OCTET-STRING")]
pub struct Cdma2000SectorID(Vec<u8>);

#[derive(Debug, AperCodec)]
#[asn(type = "ENUMERATED", extensible = true, lb = "0", ub = "3")]
pub struct Cell_Size(u8);
impl Cell_Size {
    const VERYSMALL: u8 = 0u8;
    const SMALL: u8 = 1u8;
    const MEDIUM: u8 = 2u8;
    const LARGE: u8 = 3u8;
}

#[derive(Debug, AperCodec)]
#[asn(type = "ENUMERATED", extensible = true, lb = "0", ub = "0")]
pub struct CellAccessMode(u8);
impl CellAccessMode {
    const HYBRID: u8 = 0u8;
}

#[derive(Debug, AperCodec)]
#[asn(type = "ENUMERATED", extensible = true, lb = "0", ub = "2")]
pub struct CellActivationCause(u8);
impl CellActivationCause {
    const APPLICATION_CONTAINER_SYNTAX_ERROR: u8 = 0u8;
    const INCONSISTENT_REPORTING_CELL_IDENTIFIER: u8 = 1u8;
    const UNSPECIFIED: u8 = 2u8;
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct CellActivationRequest {
    pub cells_to_activate_list: CellsToActivateList,
    #[asn(optional_idx = 0)]
    pub minimum_activation_time: Option<INTEGER_7>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true)]
pub struct CellActivationResponse {
    pub activated_cells_list: ActivatedCellsList,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct CellBasedMDT {
    pub cell_id_listfor_mdt: CellIdListforMDT,
    #[asn(optional_idx = 0)]
    pub ie_extensions: Option<CellBasedMDTIE_Extensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct CellBasedQMC {
    pub cell_id_listfor_qmc: CellIdListforQMC,
    #[asn(optional_idx = 0)]
    pub ie_extensions: Option<CellBasedQMCIE_Extensions>,
}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct CellID_Broadcast(Vec<CellID_Broadcast_Item>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct CellID_Broadcast_Item {
    pub ecgi: EUTRAN_CGI,
    #[asn(optional_idx = 0)]
    pub ie_extensions: Option<CellID_Broadcast_ItemIE_Extensions>,
}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct CellID_Cancelled(Vec<CellID_Cancelled_Item>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct CellID_Cancelled_Item {
    pub ecgi: EUTRAN_CGI,
    pub number_of_broadcasts: NumberOfBroadcasts,
    #[asn(optional_idx = 0)]
    pub ie_extensions: Option<CellID_Cancelled_ItemIE_Extensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "32")]
pub struct CellIdListforMDT(Vec<EUTRAN_CGI>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "32")]
pub struct CellIdListforQMC(Vec<EUTRAN_CGI>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct CellIdentifierAndCELevelForCECapableUEs {
    pub global_cell_id: EUTRAN_CGI,
    pub ce_level: CELevel,
    #[asn(optional_idx = 0)]
    pub ie_extensions: Option<CellIdentifierAndCELevelForCECapableUEsIE_Extensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "BITSTRING", sz_extensible = false, sz_lb = "28", sz_ub = "28")]
pub struct CellIdentity(BitVec<Msb0, u8>);

#[derive(Debug, AperCodec)]
#[asn(type = "ENUMERATED", extensible = true, lb = "0", ub = "2")]
pub struct CellLoadReportingCause(u8);
impl CellLoadReportingCause {
    const APPLICATION_CONTAINER_SYNTAX_ERROR: u8 = 0u8;
    const INCONSISTENT_REPORTING_CELL_IDENTIFIER: u8 = 1u8;
    const UNSPECIFIED: u8 = 2u8;
}

#[derive(Debug, AperCodec)]
#[asn(type = "CHOICE", lb = "0", ub = "2", extensible = true)]
pub enum CellLoadReportingResponse {
    #[asn(key = 0, extended = false)]
    EUTRAN(EUTRANcellLoadReportingResponse),
    #[asn(key = 1, extended = false)]
    UTRAN(OCTET_STRING_8),
    #[asn(key = 2, extended = false)]
    GERAN(OCTET_STRING_9),
    #[asn(key = 0, extended = true)]
    EHRPD(EHRPDSectorLoadReportingResponse),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true)]
pub struct CellStateIndication {
    pub notification_cell_list: NotificationCellList,
}

#[derive(Debug, AperCodec)]
#[asn(type = "ENUMERATED", extensible = true, lb = "0", ub = "2")]
pub struct CellStateIndicationCause(u8);
impl CellStateIndicationCause {
    const APPLICATION_CONTAINER_SYNTAX_ERROR: u8 = 0u8;
    const INCONSISTENT_REPORTING_CELL_IDENTIFIER: u8 = 1u8;
    const UNSPECIFIED: u8 = 2u8;
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true)]
pub struct CellTrafficTrace {
    pub protocol_i_es: CellTrafficTraceProtocolIEs,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct CellType {
    pub cell_size: Cell_Size,
    #[asn(optional_idx = 0)]
    pub ie_extensions: Option<CellTypeIE_Extensions>,
}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "256"
)]
pub struct CellsToActivateList(Vec<CellsToActivateList_Item>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true)]
pub struct CellsToActivateList_Item {
    pub cell_id: OCTET_STRING_10,
}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct CompletedCellinEAI(Vec<CompletedCellinEAI_Item>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct CompletedCellinEAI_Item {
    pub ecgi: EUTRAN_CGI,
    #[asn(optional_idx = 0)]
    pub ie_extensions: Option<CompletedCellinEAI_ItemIE_Extensions>,
}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct CompletedCellinTAI(Vec<CompletedCellinTAI_Item>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct CompletedCellinTAI_Item {
    pub ecgi: EUTRAN_CGI,
    #[asn(optional_idx = 0)]
    pub ie_extensions: Option<CompletedCellinTAI_ItemIE_Extensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "OCTET-STRING")]
pub struct CompositeAvailableCapacityGroup(Vec<u8>);

#[derive(Debug, AperCodec)]
#[asn(type = "ENUMERATED", lb = "0", ub = "0")]
pub struct ConcurrentWarningMessageIndicator(u8);
impl ConcurrentWarningMessageIndicator {
    const TRUE: u8 = 0u8;
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct ConnectedengNBItem {
    pub en_g_nb_id: En_gNB_ID,
    pub supported_t_as: SupportedTAs,
    #[asn(optional_idx = 0)]
    pub ie_extensions: Option<ConnectedengNBItemIE_Extensions>,
}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "256"
)]
pub struct ConnectedengNBList(Vec<ConnectedengNBItem>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true)]
pub struct ConnectionEstablishmentIndication {
    pub protocol_i_es: ConnectionEstablishmentIndicationProtocolIEs,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct ContextatSource {
    pub source_ng_ran_node_id: Global_RAN_NODE_ID,
    pub ran_ue_ngap_id: RAN_UE_NGAP_ID,
    #[asn(optional_idx = 0)]
    pub ie_extensions: Option<ContextatSourceIE_Extensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "OCTET-STRING", sz_extensible = false, sz_lb = "4", sz_ub = "4")]
pub struct Correlation_ID(Vec<u8>);

#[derive(Debug, AperCodec)]
#[asn(type = "ENUMERATED", extensible = true, lb = "0", ub = "0")]
pub struct Coverage_Level(u8);
impl Coverage_Level {
    const EXTENDEDCOVERAGE: u8 = 0u8;
}

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
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct CriticalityDiagnostics_IE_Item {
    pub ie_criticality: Criticality,
    pub ie_id: ProtocolIE_ID,
    pub type_of_error: TypeOfError,
    #[asn(optional_idx = 0)]
    pub ie_extensions: Option<CriticalityDiagnostics_IE_ItemIE_Extensions>,
}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "256"
)]
pub struct CriticalityDiagnostics_IE_List(Vec<CriticalityDiagnostics_IE_Item>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct DAPSRequestInfo {
    pub daps_indicator: ENUMERATED_11,
    #[asn(optional_idx = 0)]
    pub ie_extensions: Option<DAPSRequestInfoIE_Extensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct DAPSResponseInfo {
    pub dapsresponseindicator: ENUMERATED_12,
    #[asn(optional_idx = 0)]
    pub ie_extensions: Option<DAPSResponseInfoIE_Extensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct DAPSResponseInfoItem {
    pub e_rab_id: E_RAB_ID,
    pub daps_response_info: DAPSResponseInfo,
    #[asn(optional_idx = 0)]
    pub ie_extensions: Option<DAPSResponseInfoItemIE_Extensions>,
}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "256"
)]
pub struct DAPSResponseInfoList(Vec<DAPSResponseInfoList_Entry>);

#[derive(Debug, AperCodec)]
#[asn(type = "INTEGER", lb = "0", ub = "65535")]
pub struct DCN_ID(u16);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct DL_CP_SecurityInformation {
    pub dl_nas_mac: DL_NAS_MAC,
    #[asn(optional_idx = 0)]
    pub ie_extensions: Option<DL_CP_SecurityInformationIE_Extensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "ENUMERATED", extensible = true, lb = "0", ub = "0")]
pub struct DL_Forwarding(u8);
impl DL_Forwarding {
    const D_L_FORWARDING_PROPOSED: u8 = 0u8;
}

#[derive(Debug, AperCodec)]
#[asn(type = "BITSTRING", sz_extensible = false, sz_lb = "16", sz_ub = "16")]
pub struct DL_NAS_MAC(BitVec<Msb0, u8>);

#[derive(Debug, AperCodec)]
#[asn(type = "CHOICE", lb = "0", ub = "2", extensible = true)]
pub enum DLCOUNT_PDCP_SNlength {
    #[asn(key = 0, extended = false)]
    DLCOUNTValuePDCP_SNlength12(COUNTvalue),
    #[asn(key = 1, extended = false)]
    DLCOUNTValuePDCP_SNlength15(COUNTValueExtended),
    #[asn(key = 2, extended = false)]
    DLCOUNTValuePDCP_SNlength18(COUNTvaluePDCP_SNlength18),
}

#[derive(Debug, AperCodec)]
#[asn(type = "ENUMERATED", extensible = true, lb = "0", ub = "0")]
pub struct DLNASPDUDeliveryAckRequest(u8);
impl DLNASPDUDeliveryAckRequest {
    const REQUESTED: u8 = 0u8;
}

#[derive(Debug, AperCodec)]
#[asn(type = "ENUMERATED", extensible = true, lb = "0", ub = "0")]
pub struct Data_Forwarding_Not_Possible(u8);
impl Data_Forwarding_Not_Possible {
    const DATA_FORWARDING_NOT_POSSIBLE: u8 = 0u8;
}

#[derive(Debug, AperCodec)]
#[asn(type = "BITSTRING", sz_extensible = false, sz_lb = "8", sz_ub = "8")]
pub struct DataCodingScheme(BitVec<Msb0, u8>);

#[derive(Debug, AperCodec)]
#[asn(type = "INTEGER", lb = "1", ub = "4095", extensible = true)]
pub struct DataSize(u16);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true)]
pub struct DeactivateTrace {
    pub protocol_i_es: DeactivateTraceProtocolIEs,
}

#[derive(Debug, AperCodec)]
#[asn(type = "ENUMERATED", extensible = true, lb = "0", ub = "0")]
pub struct Direct_Forwarding_Path_Availability(u8);
impl Direct_Forwarding_Path_Availability {
    const DIRECT_PATH_AVAILABLE: u8 = 0u8;
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true)]
pub struct DownlinkNASTransport {
    pub protocol_i_es: DownlinkNASTransportProtocolIEs,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true)]
pub struct DownlinkNonUEAssociatedLPPaTransport {
    pub protocol_i_es: DownlinkNonUEAssociatedLPPaTransportProtocolIEs,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true)]
pub struct DownlinkS1cdma2000tunnelling {
    pub protocol_i_es: DownlinkS1cdma2000tunnellingProtocolIEs,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true)]
pub struct DownlinkUEAssociatedLPPaTransport {
    pub protocol_i_es: DownlinkUEAssociatedLPPaTransportProtocolIEs,
}

#[derive(Debug, AperCodec)]
#[asn(type = "INTEGER", lb = "0", ub = "15", extensible = true)]
pub struct E_RAB_ID(u8);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 5)]
pub struct E_RABAdmittedItem {
    pub e_rab_id: E_RAB_ID,
    pub transport_layer_address: TransportLayerAddress,
    pub gtp_teid: GTP_TEID,
    #[asn(optional_idx = 0)]
    pub dl_transport_layer_address: Option<TransportLayerAddress>,
    #[asn(optional_idx = 1)]
    pub dl_g_tp_teid: Option<GTP_TEID>,
    #[asn(optional_idx = 2)]
    pub ul_transport_layer_address: Option<TransportLayerAddress>,
    #[asn(optional_idx = 3)]
    pub ul_gtp_teid: Option<GTP_TEID>,
    #[asn(optional_idx = 4)]
    pub ie_extensions: Option<E_RABAdmittedItemIE_Extensions>,
}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "256"
)]
pub struct E_RABAdmittedList(Vec<E_RABAdmittedList_Entry>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 5)]
pub struct E_RABDataForwardingItem {
    pub e_rab_id: E_RAB_ID,
    #[asn(optional_idx = 0)]
    pub dl_transport_layer_address: Option<TransportLayerAddress>,
    #[asn(optional_idx = 1)]
    pub dl_g_tp_teid: Option<GTP_TEID>,
    #[asn(optional_idx = 2)]
    pub ul_transport_layer_address: Option<TransportLayerAddress>,
    #[asn(optional_idx = 3)]
    pub ul_gtp_teid: Option<GTP_TEID>,
    #[asn(optional_idx = 4)]
    pub ie_extensions: Option<E_RABDataForwardingItemIE_Extensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct E_RABFailedToResumeItemResumeReq {
    pub e_rab_id: E_RAB_ID,
    pub cause: Cause,
    #[asn(optional_idx = 0)]
    pub ie_extensions: Option<E_RABFailedToResumeItemResumeReqIE_Extensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct E_RABFailedToResumeItemResumeRes {
    pub e_rab_id: E_RAB_ID,
    pub cause: Cause,
    #[asn(optional_idx = 0)]
    pub ie_extensions: Option<E_RABFailedToResumeItemResumeResIE_Extensions>,
}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "256"
)]
pub struct E_RABFailedToResumeListResumeReq(Vec<E_RABFailedToResumeListResumeReq_Entry>);

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "256"
)]
pub struct E_RABFailedToResumeListResumeRes(Vec<E_RABFailedToResumeListResumeRes_Entry>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct E_RABFailedToSetupItemHOReqAck {
    pub e_rab_id: E_RAB_ID,
    pub cause: Cause,
    #[asn(optional_idx = 0)]
    pub ie_extensions: Option<E_RABFailedToSetupItemHOReqAckIE_Extensions>,
}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "256"
)]
pub struct E_RABFailedtoSetupListHOReqAck(Vec<E_RABFailedtoSetupListHOReqAck_Entry>);

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "256"
)]
pub struct E_RABInformationList(Vec<E_RABInformationList_Entry>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 2)]
pub struct E_RABInformationListItem {
    pub e_rab_id: E_RAB_ID,
    #[asn(optional_idx = 0)]
    pub dl_forwarding: Option<DL_Forwarding>,
    #[asn(optional_idx = 1)]
    pub ie_extensions: Option<E_RABInformationListItemIE_Extensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct E_RABItem {
    pub e_rab_id: E_RAB_ID,
    pub cause: Cause,
    #[asn(optional_idx = 0)]
    pub ie_extensions: Option<E_RABItemIE_Extensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 2)]
pub struct E_RABLevelQoSParameters {
    pub qci: QCI,
    pub allocation_retention_priority: AllocationAndRetentionPriority,
    #[asn(optional_idx = 0)]
    pub gbr_qos_information: Option<GBR_QosInformation>,
    #[asn(optional_idx = 1)]
    pub ie_extensions: Option<E_RABLevelQoSParametersIE_Extensions>,
}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "256"
)]
pub struct E_RABList(Vec<E_RABList_Entry>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true)]
pub struct E_RABModificationConfirm {
    pub protocol_i_es: E_RABModificationConfirmProtocolIEs,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true)]
pub struct E_RABModificationIndication {
    pub protocol_i_es: E_RABModificationIndicationProtocolIEs,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct E_RABModifyItemBearerModConf {
    pub e_rab_id: E_RAB_ID,
    #[asn(optional_idx = 0)]
    pub ie_extensions: Option<E_RABModifyItemBearerModConfIE_Extensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct E_RABModifyItemBearerModRes {
    pub e_rab_id: E_RAB_ID,
    #[asn(optional_idx = 0)]
    pub ie_extensions: Option<E_RABModifyItemBearerModResIE_Extensions>,
}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "256"
)]
pub struct E_RABModifyListBearerModConf(Vec<E_RABModifyListBearerModConf_Entry>);

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "256"
)]
pub struct E_RABModifyListBearerModRes(Vec<E_RABModifyListBearerModRes_Entry>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true)]
pub struct E_RABModifyRequest {
    pub protocol_i_es: E_RABModifyRequestProtocolIEs,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true)]
pub struct E_RABModifyResponse {
    pub protocol_i_es: E_RABModifyResponseProtocolIEs,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct E_RABNotToBeModifiedItemBearerModInd {
    pub e_rab_id: E_RAB_ID,
    pub transport_layer_address: TransportLayerAddress,
    pub dl_gtp_teid: GTP_TEID,
    #[asn(optional_idx = 0)]
    pub ie_extensions: Option<E_RABNotToBeModifiedItemBearerModIndIE_Extensions>,
}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "256"
)]
pub struct E_RABNotToBeModifiedListBearerModInd(Vec<E_RABNotToBeModifiedListBearerModInd_Entry>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true)]
pub struct E_RABReleaseCommand {
    pub protocol_i_es: E_RABReleaseCommandProtocolIEs,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true)]
pub struct E_RABReleaseIndication {
    pub protocol_i_es: E_RABReleaseIndicationProtocolIEs,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct E_RABReleaseItemBearerRelComp {
    pub e_rab_id: E_RAB_ID,
    #[asn(optional_idx = 0)]
    pub ie_extensions: Option<E_RABReleaseItemBearerRelCompIE_Extensions>,
}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "256"
)]
pub struct E_RABReleaseListBearerRelComp(Vec<E_RABReleaseListBearerRelComp_Entry>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true)]
pub struct E_RABReleaseResponse {
    pub protocol_i_es: E_RABReleaseResponseProtocolIEs,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct E_RABSetupItemBearerSURes {
    pub e_rab_id: E_RAB_ID,
    pub transport_layer_address: TransportLayerAddress,
    pub gtp_teid: GTP_TEID,
    #[asn(optional_idx = 0)]
    pub ie_extensions: Option<E_RABSetupItemBearerSUResIE_Extensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct E_RABSetupItemCtxtSURes {
    pub e_rab_id: E_RAB_ID,
    pub transport_layer_address: TransportLayerAddress,
    pub gtp_teid: GTP_TEID,
    #[asn(optional_idx = 0)]
    pub ie_extensions: Option<E_RABSetupItemCtxtSUResIE_Extensions>,
}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "256"
)]
pub struct E_RABSetupListBearerSURes(Vec<E_RABSetupListBearerSURes_Entry>);

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "256"
)]
pub struct E_RABSetupListCtxtSURes(Vec<E_RABSetupListCtxtSURes_Entry>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true)]
pub struct E_RABSetupRequest {
    pub protocol_i_es: E_RABSetupRequestProtocolIEs,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true)]
pub struct E_RABSetupResponse {
    pub protocol_i_es: E_RABSetupResponseProtocolIEs,
}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "256"
)]
pub struct E_RABSubjecttoDataForwardingList(Vec<E_RABSubjecttoDataForwardingList_Entry>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct E_RABToBeModifiedItemBearerModInd {
    pub e_rab_id: E_RAB_ID,
    pub transport_layer_address: TransportLayerAddress,
    pub dl_gtp_teid: GTP_TEID,
    #[asn(optional_idx = 0)]
    pub ie_extensions: Option<E_RABToBeModifiedItemBearerModIndIE_Extensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct E_RABToBeModifiedItemBearerModReq {
    pub e_rab_id: E_RAB_ID,
    pub e_rab_level_qo_s_parameters: E_RABLevelQoSParameters,
    pub nas_pdu: NAS_PDU,
    #[asn(optional_idx = 0)]
    pub ie_extensions: Option<E_RABToBeModifiedItemBearerModReqIE_Extensions>,
}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "256"
)]
pub struct E_RABToBeModifiedListBearerModInd(Vec<E_RABToBeModifiedListBearerModInd_Entry>);

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "256"
)]
pub struct E_RABToBeModifiedListBearerModReq(Vec<E_RABToBeModifiedListBearerModReq_Entry>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct E_RABToBeSetupItemBearerSUReq {
    pub e_rab_id: E_RAB_ID,
    pub e_ra_blevel_qo_s_parameters: E_RABLevelQoSParameters,
    pub transport_layer_address: TransportLayerAddress,
    pub gtp_teid: GTP_TEID,
    pub nas_pdu: NAS_PDU,
    #[asn(optional_idx = 0)]
    pub ie_extensions: Option<E_RABToBeSetupItemBearerSUReqIE_Extensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 2)]
pub struct E_RABToBeSetupItemCtxtSUReq {
    pub e_rab_id: E_RAB_ID,
    pub e_ra_blevel_qo_s_parameters: E_RABLevelQoSParameters,
    pub transport_layer_address: TransportLayerAddress,
    pub gtp_teid: GTP_TEID,
    #[asn(optional_idx = 0)]
    pub nas_pdu: Option<NAS_PDU>,
    #[asn(optional_idx = 1)]
    pub ie_extensions: Option<E_RABToBeSetupItemCtxtSUReqIE_Extensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct E_RABToBeSetupItemHOReq {
    pub e_rab_id: E_RAB_ID,
    pub transport_layer_address: TransportLayerAddress,
    pub gtp_teid: GTP_TEID,
    pub e_ra_blevel_qos_parameters: E_RABLevelQoSParameters,
    #[asn(optional_idx = 0)]
    pub ie_extensions: Option<E_RABToBeSetupItemHOReqIE_Extensions>,
}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "256"
)]
pub struct E_RABToBeSetupListBearerSUReq(Vec<E_RABToBeSetupListBearerSUReq_Entry>);

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "256"
)]
pub struct E_RABToBeSetupListCtxtSUReq(Vec<E_RABToBeSetupListCtxtSUReq_Entry>);

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "256"
)]
pub struct E_RABToBeSetupListHOReq(Vec<E_RABToBeSetupListHOReq_Entry>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct E_RABToBeSwitchedDLItem {
    pub e_rab_id: E_RAB_ID,
    pub transport_layer_address: TransportLayerAddress,
    pub gtp_teid: GTP_TEID,
    #[asn(optional_idx = 0)]
    pub ie_extensions: Option<E_RABToBeSwitchedDLItemIE_Extensions>,
}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "256"
)]
pub struct E_RABToBeSwitchedDLList(Vec<E_RABToBeSwitchedDLList_Entry>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct E_RABToBeSwitchedULItem {
    pub e_rab_id: E_RAB_ID,
    pub transport_layer_address: TransportLayerAddress,
    pub gtp_teid: GTP_TEID,
    #[asn(optional_idx = 0)]
    pub ie_extensions: Option<E_RABToBeSwitchedULItemIE_Extensions>,
}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "256"
)]
pub struct E_RABToBeSwitchedULList(Vec<E_RABToBeSwitchedULList_Entry>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct E_RABUsageReportItem {
    pub start_timestamp: OCTET_STRING_13,
    pub end_timestamp: OCTET_STRING_14,
    pub usage_count_ul: INTEGER_15,
    pub usage_count_dl: INTEGER_16,
    #[asn(optional_idx = 0)]
    pub ie_extensions: Option<E_RABUsageReportItemIE_Extensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "2")]
pub struct E_RABUsageReportList(Vec<E_RABUsageReportList_Entry>);

#[derive(Debug, AperCodec)]
#[asn(type = "OCTET-STRING", sz_extensible = false, sz_lb = "8", sz_ub = "8")]
pub struct E_UTRAN_Trace_ID(Vec<u8>);

#[derive(Debug, AperCodec)]
#[asn(type = "INTEGER", lb = "0", ub = "262143", extensible = true)]
pub struct EARFCN(u32);

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "256"
)]
pub struct ECGI_List(Vec<EUTRAN_CGI>);

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct ECGIList(Vec<EUTRAN_CGI>);

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "256"
)]
pub struct ECGIListForRestart(Vec<EUTRAN_CGI>);

#[derive(Debug, AperCodec)]
#[asn(type = "ENUMERATED", extensible = true, lb = "0", ub = "0")]
pub struct EDT_Session(u8);
impl EDT_Session {
    const TRUE: u8 = 0u8;
}

#[derive(Debug, AperCodec)]
#[asn(
    type = "OCTET-STRING",
    sz_extensible = false,
    sz_lb = "16",
    sz_ub = "16"
)]
pub struct EHRPD_Sector_ID(Vec<u8>);

#[derive(Debug, AperCodec)]
#[asn(type = "INTEGER", lb = "0", ub = "100")]
pub struct EHRPDCapacityValue(u8);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true)]
pub struct EHRPDCompositeAvailableCapacity {
    pub ehrpd_sector_capacity_class_value: EHRPDSectorCapacityClassValue,
    pub ehrpd_capacity_value: EHRPDCapacityValue,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true)]
pub struct EHRPDMultiSectorLoadReportingResponseItem {
    pub ehrpd_sector_id: EHRPD_Sector_ID,
    pub ehrpd_sector_load_reporting_response: EHRPDSectorLoadReportingResponse,
}

#[derive(Debug, AperCodec)]
#[asn(type = "INTEGER", lb = "1", ub = "100", extensible = true)]
pub struct EHRPDSectorCapacityClassValue(u8);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true)]
pub struct EHRPDSectorLoadReportingResponse {
    pub dl_ehrpd_composite_available_capacity: EHRPDCompositeAvailableCapacity,
    pub ul_ehrpd_composite_available_capacity: EHRPDCompositeAvailableCapacity,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 2)]
pub struct EN_DCSONConfigurationTransfer {
    pub transfertype: EN_DCSONTransferType,
    pub son_information: SONInformation,
    #[asn(optional_idx = 0)]
    pub x2tnl_config_info: Option<X2TNLConfigurationInfo>,
    #[asn(optional_idx = 1)]
    pub ie_extensions: Option<EN_DCSONConfigurationTransferIE_Extensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "CHOICE", lb = "0", ub = "1", extensible = true)]
pub enum EN_DCSONTransferType {
    #[asn(key = 0, extended = false)]
    Request(EN_DCTransferTypeRequest),
    #[asn(key = 1, extended = false)]
    Reply(EN_DCTransferTypeReply),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct EN_DCSONeNBIdentification {
    pub globale_nbid: Global_ENB_ID,
    pub selected_tai: TAI,
    #[asn(optional_idx = 0)]
    pub ie_extensions: Option<EN_DCSONeNBIdentificationIE_Extensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct EN_DCSONengNBIdentification {
    pub globaleng_nbid: Global_en_gNB_ID,
    pub selected_tai: TAI,
    #[asn(optional_idx = 0)]
    pub ie_extensions: Option<EN_DCSONengNBIdentificationIE_Extensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct EN_DCTransferTypeReply {
    pub sourceeng_nb: EN_DCSONengNBIdentification,
    pub targete_nb: EN_DCSONeNBIdentification,
    #[asn(optional_idx = 0)]
    pub ie_extensions: Option<EN_DCTransferTypeReplyIE_Extensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 4)]
pub struct EN_DCTransferTypeRequest {
    pub sourcee_nb: EN_DCSONeNBIdentification,
    pub targeteng_nb: EN_DCSONengNBIdentification,
    #[asn(optional_idx = 0)]
    pub targete_nb: Option<EN_DCSONeNBIdentification>,
    #[asn(optional_idx = 1)]
    pub associated_tai: Option<TAI>,
    #[asn(optional_idx = 2)]
    pub broadcast5_gstai: Option<FiveGSTAI>,
    #[asn(optional_idx = 3)]
    pub ie_extensions: Option<EN_DCTransferTypeRequestIE_Extensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct ENB_EarlyStatusTransfer_TransparentContainer {
    pub bearers_subject_to_early_status_transfer_list: Bearers_SubjectToEarlyStatusTransferList,
    #[asn(optional_idx = 0)]
    pub ie_extensions: Option<ENB_EarlyStatusTransfer_TransparentContainerIE_Extensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "CHOICE", lb = "0", ub = "1", extensible = true)]
pub enum ENB_ID {
    #[asn(key = 0, extended = false)]
    MacroENB_ID(BIT_STRING_17),
    #[asn(key = 1, extended = false)]
    HomeENB_ID(BIT_STRING_18),
    #[asn(key = 0, extended = true)]
    Short_macroENB_ID(BIT_STRING_19),
    #[asn(key = 1, extended = true)]
    Long_macroENB_ID(BIT_STRING_20),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct ENB_StatusTransfer_TransparentContainer {
    pub bearers_subject_to_status_transfer_list: Bearers_SubjectToStatusTransferList,
    #[asn(optional_idx = 0)]
    pub ie_extensions: Option<ENB_StatusTransfer_TransparentContainerIE_Extensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "INTEGER", lb = "0", ub = "16777215")]
pub struct ENB_UE_S1AP_ID(u32);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true)]
pub struct ENBCPRelocationIndication {
    pub protocol_i_es: ENBCPRelocationIndicationProtocolIEs,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true)]
pub struct ENBConfigurationTransfer {
    pub protocol_i_es: ENBConfigurationTransferProtocolIEs,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true)]
pub struct ENBConfigurationUpdate {
    pub protocol_i_es: ENBConfigurationUpdateProtocolIEs,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true)]
pub struct ENBConfigurationUpdateAcknowledge {
    pub protocol_i_es: ENBConfigurationUpdateAcknowledgeProtocolIEs,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true)]
pub struct ENBConfigurationUpdateFailure {
    pub protocol_i_es: ENBConfigurationUpdateFailureProtocolIEs,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true)]
pub struct ENBDirectInformationTransfer {
    pub protocol_i_es: ENBDirectInformationTransferProtocolIEs,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true)]
pub struct ENBEarlyStatusTransfer {
    pub protocol_i_es: ENBEarlyStatusTransferProtocolIEs,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "2")]
pub struct ENBIndirectX2TransportLayerAddresses(Vec<TransportLayerAddress>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true)]
pub struct ENBStatusTransfer {
    pub protocol_i_es: ENBStatusTransferProtocolIEs,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 3)]
pub struct ENBX2ExtTLA {
    #[asn(optional_idx = 0)]
    pub i_psec_tla: Option<TransportLayerAddress>,
    #[asn(optional_idx = 1)]
    pub gtptl_aa: Option<ENBX2GTPTLAs>,
    #[asn(optional_idx = 2)]
    pub ie_extensions: Option<ENBX2ExtTLAIE_Extensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "16")]
pub struct ENBX2ExtTLAs(Vec<ENBX2ExtTLA>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "16")]
pub struct ENBX2GTPTLAs(Vec<TransportLayerAddress>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "2")]
pub struct ENBX2TLAs(Vec<TransportLayerAddress>);

#[derive(Debug, AperCodec)]
#[asn(
    type = "PrintableString",
    sz_extensible = true,
    sz_lb = "1",
    sz_ub = "150"
)]
pub struct ENBname(String);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "15")]
pub struct EPLMNs(Vec<PLMNidentity>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct EUTRAN_CGI {
    pub plm_nidentity: PLMNidentity,
    pub cell_id: CellIdentity,
    #[asn(optional_idx = 0)]
    pub ie_extensions: Option<EUTRAN_CGIIE_Extensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true)]
pub struct EUTRANResponse {
    pub cell_id: OCTET_STRING_21,
    pub eutra_ncell_load_reporting_response: EUTRANcellLoadReportingResponse,
}

#[derive(Debug, AperCodec)]
#[asn(type = "INTEGER", lb = "0", ub = "2047")]
pub struct EUTRANRoundTripDelayEstimationInfo(u16);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true)]
pub struct EUTRANcellLoadReportingResponse {
    pub composite_available_capacity_group: CompositeAvailableCapacityGroup,
}

#[derive(Debug, AperCodec)]
#[asn(type = "OCTET-STRING", sz_extensible = false, sz_lb = "3", sz_ub = "3")]
pub struct EmergencyAreaID(Vec<u8>);

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct EmergencyAreaID_Broadcast(Vec<EmergencyAreaID_Broadcast_Item>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct EmergencyAreaID_Broadcast_Item {
    pub emergency_area_id: EmergencyAreaID,
    pub completed_cellin_eai: CompletedCellinEAI,
    #[asn(optional_idx = 0)]
    pub ie_extensions: Option<EmergencyAreaID_Broadcast_ItemIE_Extensions>,
}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct EmergencyAreaID_Cancelled(Vec<EmergencyAreaID_Cancelled_Item>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct EmergencyAreaID_Cancelled_Item {
    pub emergency_area_id: EmergencyAreaID,
    pub cancelled_cellin_eai: CancelledCellinEAI,
    #[asn(optional_idx = 0)]
    pub ie_extensions: Option<EmergencyAreaID_Cancelled_ItemIE_Extensions>,
}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct EmergencyAreaIDList(Vec<EmergencyAreaID>);

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "256"
)]
pub struct EmergencyAreaIDListForRestart(Vec<EmergencyAreaID>);

#[derive(Debug, AperCodec)]
#[asn(type = "ENUMERATED", extensible = true, lb = "0", ub = "0")]
pub struct EmergencyIndicator(u8);
impl EmergencyIndicator {
    const TRUE: u8 = 0u8;
}

#[derive(Debug, AperCodec)]
#[asn(type = "BITSTRING", sz_extensible = true, sz_lb = "22", sz_ub = "32")]
pub struct En_gNB_ID(BitVec<Msb0, u8>);

#[derive(Debug, AperCodec)]
#[asn(type = "BITSTRING", sz_extensible = true, sz_lb = "16", sz_ub = "16")]
pub struct EncryptionAlgorithms(BitVec<Msb0, u8>);

#[derive(Debug, AperCodec)]
#[asn(type = "ENUMERATED", extensible = true, lb = "0", ub = "1")]
pub struct EndIndication(u8);
impl EndIndication {
    const NO_FURTHER_DATA: u8 = 0u8;
    const FURTHER_DATA_EXISTS: u8 = 1u8;
}

#[derive(Debug, AperCodec)]
#[asn(type = "ENUMERATED", extensible = true, lb = "0", ub = "0")]
pub struct EnhancedCoverageRestricted(u8);
impl EnhancedCoverageRestricted {
    const RESTRICTED: u8 = 0u8;
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true)]
pub struct ErrorIndication {
    pub protocol_i_es: ErrorIndicationProtocolIEs,
}

#[derive(Debug, AperCodec)]
#[asn(type = "ENUMERATED", extensible = true, lb = "0", ub = "0")]
pub struct Ethernet_Type(u8);
impl Ethernet_Type {
    const TRUE: u8 = 0u8;
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true)]
pub struct EventTriggeredCellLoadReportingRequest {
    pub number_of_measurement_reporting_levels: NumberOfMeasurementReportingLevels,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct EventTriggeredCellLoadReportingResponse {
    pub cell_load_reporting_response: CellLoadReportingResponse,
    #[asn(optional_idx = 0)]
    pub overload_flag: Option<OverloadFlag>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "ENUMERATED", extensible = true, lb = "0", ub = "2")]
pub struct EventType(u8);
impl EventType {
    const DIRECT: u8 = 0u8;
    const CHANGE_OF_SERVE_CELL: u8 = 1u8;
    const STOP_CHANGE_OF_SERVE_CELL: u8 = 2u8;
}

#[derive(Debug, AperCodec)]
#[asn(type = "INTEGER", lb = "1", ub = "30", extensible = true)]
pub struct ExpectedActivityPeriod(u8);

#[derive(Debug, AperCodec)]
#[asn(type = "ENUMERATED", extensible = true, lb = "0", ub = "6")]
pub struct ExpectedHOInterval(u8);
impl ExpectedHOInterval {
    const SEC15: u8 = 0u8;
    const SEC30: u8 = 1u8;
    const SEC60: u8 = 2u8;
    const SEC90: u8 = 3u8;
    const SEC120: u8 = 4u8;
    const SEC180: u8 = 5u8;
    const LONG_TIME: u8 = 6u8;
}

#[derive(Debug, AperCodec)]
#[asn(type = "INTEGER", lb = "1", ub = "30", extensible = true)]
pub struct ExpectedIdlePeriod(u8);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 4)]
pub struct ExpectedUEActivityBehaviour {
    #[asn(optional_idx = 0)]
    pub expected_activity_period: Option<ExpectedActivityPeriod>,
    #[asn(optional_idx = 1)]
    pub expected_idle_period: Option<ExpectedIdlePeriod>,
    #[asn(optional_idx = 2)]
    pub sourceof_ue_activity_behaviour_information: Option<SourceOfUEActivityBehaviourInformation>,
    #[asn(optional_idx = 3)]
    pub ie_extensions: Option<ExpectedUEActivityBehaviourIE_Extensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 3)]
pub struct ExpectedUEBehaviour {
    #[asn(optional_idx = 0)]
    pub expected_activity: Option<ExpectedUEActivityBehaviour>,
    #[asn(optional_idx = 1)]
    pub expected_ho_interval: Option<ExpectedHOInterval>,
    #[asn(optional_idx = 2)]
    pub ie_extensions: Option<ExpectedUEBehaviourIE_Extensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "BITSTRING", sz_extensible = false, sz_lb = "14", sz_ub = "14")]
pub struct Extended_UEIdentityIndexValue(BitVec<Msb0, u8>);

#[derive(Debug, AperCodec)]
#[asn(
    type = "INTEGER",
    lb = "10000000001",
    ub = "4000000000000",
    extensible = true
)]
pub struct ExtendedBitRate(u64);

#[derive(Debug, AperCodec)]
#[asn(type = "INTEGER", lb = "4096", ub = "65535")]
pub struct ExtendedRNC_ID(u16);

#[derive(Debug, AperCodec)]
#[asn(type = "INTEGER", lb = "4096", ub = "131071")]
pub struct ExtendedRepetitionPeriod(u32);

#[derive(Debug, AperCodec)]
#[asn(type = "CHOICE", lb = "0", ub = "0", extensible = true)]
pub enum FailureEventReport {
    #[asn(key = 0, extended = false)]
    TooEarlyInterRATHOReportFromEUTRAN(TooEarlyInterRATHOReportReportFromEUTRAN),
}

#[derive(Debug, AperCodec)]
#[asn(type = "ENUMERATED", extensible = true, lb = "0", ub = "2")]
pub struct FailureEventReportingCause(u8);
impl FailureEventReportingCause {
    const APPLICATION_CONTAINER_SYNTAX_ERROR: u8 = 0u8;
    const INCONSISTENT_REPORTING_CELL_IDENTIFIER: u8 = 1u8;
    const UNSPECIFIED: u8 = 2u8;
}

#[derive(Debug, AperCodec)]
#[asn(type = "OCTET-STRING", sz_extensible = false, sz_lb = "3", sz_ub = "3")]
pub struct FiveGSTAC(Vec<u8>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct FiveGSTAI {
    pub plm_nidentity: PLMNidentity,
    pub five_gstac: FiveGSTAC,
    #[asn(optional_idx = 0)]
    pub ie_extensions: Option<FiveGSTAIIE_Extensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "INTEGER", lb = "0", ub = "255", extensible = true)]
pub struct FiveQI(u8);

#[derive(Debug, AperCodec)]
#[asn(type = "ENUMERATED", extensible = true, lb = "0", ub = "3")]
pub struct ForbiddenInterRATs(u8);
impl ForbiddenInterRATs {
    const ALL: u8 = 0u8;
    const GERAN: u8 = 1u8;
    const UTRAN: u8 = 2u8;
    const CDMA2000: u8 = 3u8;
}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "4096"
)]
pub struct ForbiddenLACs(Vec<LAC>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "16")]
pub struct ForbiddenLAs(Vec<ForbiddenLAs_Item>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct ForbiddenLAs_Item {
    pub plmn_identity: PLMNidentity,
    pub forbidden_la_cs: ForbiddenLACs,
    #[asn(optional_idx = 0)]
    pub ie_extensions: Option<ForbiddenLAs_ItemIE_Extensions>,
}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "4096"
)]
pub struct ForbiddenTACs(Vec<TAC>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "16")]
pub struct ForbiddenTAs(Vec<ForbiddenTAs_Item>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct ForbiddenTAs_Item {
    pub plmn_identity: PLMNidentity,
    pub forbidden_ta_cs: ForbiddenTACs,
    #[asn(optional_idx = 0)]
    pub ie_extensions: Option<ForbiddenTAs_ItemIE_Extensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct GBR_QosInformation {
    pub e_rab_maximum_bitrate_dl: BitRate,
    pub e_rab_maximum_bitrate_ul: BitRate,
    pub e_rab_guaranteed_bitrate_dl: BitRate,
    pub e_rab_guaranteed_bitrate_ul: BitRate,
    #[asn(optional_idx = 0)]
    pub ie_extensions: Option<GBR_QosInformationIE_Extensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct GERAN_Cell_ID {
    pub lai: LAI,
    pub rac: RAC,
    pub ci: CI,
    #[asn(optional_idx = 0)]
    pub ie_extensions: Option<GERAN_Cell_IDIE_Extensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct GNB {
    pub global_g_nb_id: Global_GNB_ID,
    #[asn(optional_idx = 0)]
    pub ie_extensions: Option<GNBIE_Extensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "BITSTRING", sz_extensible = false, sz_lb = "22", sz_ub = "32")]
pub struct GNB_ID(BitVec<Msb0, u8>);

#[derive(Debug, AperCodec)]
#[asn(type = "CHOICE", lb = "0", ub = "0", extensible = true)]
pub enum GNB_Identity {
    #[asn(key = 0, extended = false)]
    GNB_ID(GNB_ID),
}

#[derive(Debug, AperCodec)]
#[asn(type = "OCTET-STRING", sz_extensible = false, sz_lb = "4", sz_ub = "4")]
pub struct GTP_TEID(Vec<u8>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct GUMMEI {
    pub plmn_identity: PLMNidentity,
    pub mme_group_id: MME_Group_ID,
    pub mme_code: MME_Code,
    #[asn(optional_idx = 0)]
    pub ie_extensions: Option<GUMMEIIE_Extensions>,
}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "256"
)]
pub struct GUMMEIList(Vec<GUMMEI>);

#[derive(Debug, AperCodec)]
#[asn(type = "ENUMERATED", extensible = true, lb = "0", ub = "1")]
pub struct GUMMEIType(u8);
impl GUMMEIType {
    const NATIVE: u8 = 0u8;
    const MAPPED: u8 = 1u8;
}

#[derive(Debug, AperCodec)]
#[asn(type = "ENUMERATED", extensible = true, lb = "0", ub = "0")]
pub struct GWContextReleaseIndication(u8);
impl GWContextReleaseIndication {
    const TRUE: u8 = 0u8;
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct Global_ENB_ID {
    pub plm_nidentity: PLMNidentity,
    pub enb_id: ENB_ID,
    #[asn(optional_idx = 0)]
    pub ie_extensions: Option<Global_ENB_IDIE_Extensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct Global_GNB_ID {
    pub plmn_identity: PLMNidentity,
    pub gnb_id: GNB_Identity,
    #[asn(optional_idx = 0)]
    pub ie_extensions: Option<Global_GNB_IDIE_Extensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "CHOICE", lb = "0", ub = "1", extensible = true)]
pub enum Global_RAN_NODE_ID {
    #[asn(key = 0, extended = false)]
    GNB(GNB),
    #[asn(key = 1, extended = false)]
    Ng_eNB(NG_eNB),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct Global_en_gNB_ID {
    pub plm_nidentity: PLMNidentity,
    pub en_g_nb_id: En_gNB_ID,
    #[asn(optional_idx = 0)]
    pub ie_extensions: Option<Global_en_gNB_IDIE_Extensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "INTEGER", lb = "0", ub = "1048575")]
pub struct HFN(u32);

#[derive(Debug, AperCodec)]
#[asn(type = "INTEGER", lb = "0", ub = "131071")]
pub struct HFNModified(u32);

#[derive(Debug, AperCodec)]
#[asn(type = "INTEGER", lb = "0", ub = "16383")]
pub struct HFNforPDCP_SNlength18(u16);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct HOReport {
    pub ho_type: HoType,
    pub ho_report_type: HoReportType,
    pub hosource_id: IRAT_Cell_ID,
    pub ho_target_id: IRAT_Cell_ID,
    pub candidate_cell_list: CandidateCellList,
    #[asn(optional_idx = 0)]
    pub candidate_pci_list: Option<CandidatePCIList>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "ENUMERATED", extensible = true, lb = "0", ub = "2")]
pub struct HOReportingCause(u8);
impl HOReportingCause {
    const APPLICATION_CONTAINER_SYNTAX_ERROR: u8 = 0u8;
    const INCONSISTENT_REPORTING_CELL_IDENTIFIER: u8 = 1u8;
    const UNSPECIFIED: u8 = 2u8;
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true)]
pub struct HandoverCancel {
    pub protocol_i_es: HandoverCancelProtocolIEs,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true)]
pub struct HandoverCancelAcknowledge {
    pub protocol_i_es: HandoverCancelAcknowledgeProtocolIEs,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true)]
pub struct HandoverCommand {
    pub protocol_i_es: HandoverCommandProtocolIEs,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true)]
pub struct HandoverFailure {
    pub protocol_i_es: HandoverFailureProtocolIEs,
}

#[derive(Debug, AperCodec)]
#[asn(type = "ENUMERATED", extensible = true, lb = "0", ub = "0")]
pub struct HandoverFlag(u8);
impl HandoverFlag {
    const HANDOVER_PREPARATION: u8 = 0u8;
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true)]
pub struct HandoverNotify {
    pub protocol_i_es: HandoverNotifyProtocolIEs,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true)]
pub struct HandoverPreparationFailure {
    pub protocol_i_es: HandoverPreparationFailureProtocolIEs,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true)]
pub struct HandoverRequest {
    pub protocol_i_es: HandoverRequestProtocolIEs,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true)]
pub struct HandoverRequestAcknowledge {
    pub protocol_i_es: HandoverRequestAcknowledgeProtocolIEs,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true)]
pub struct HandoverRequired {
    pub protocol_i_es: HandoverRequiredProtocolIEs,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 5)]
pub struct HandoverRestrictionList {
    pub serving_plmn: PLMNidentity,
    #[asn(optional_idx = 0)]
    pub equivalent_plm_ns: Option<EPLMNs>,
    #[asn(optional_idx = 1)]
    pub forbidden_t_as: Option<ForbiddenTAs>,
    #[asn(optional_idx = 2)]
    pub forbidden_l_as: Option<ForbiddenLAs>,
    #[asn(optional_idx = 3)]
    pub forbidden_inter_ra_ts: Option<ForbiddenInterRATs>,
    #[asn(optional_idx = 4)]
    pub ie_extensions: Option<HandoverRestrictionListIE_Extensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true)]
pub struct HandoverSuccess {
    pub protocol_i_es: HandoverSuccessProtocolIEs,
}

#[derive(Debug, AperCodec)]
#[asn(type = "ENUMERATED", extensible = true, lb = "0", ub = "4")]
pub struct HandoverType(u8);
impl HandoverType {
    const INTRALTE: u8 = 0u8;
    const LTETOUTRAN: u8 = 1u8;
    const LTETOGERAN: u8 = 2u8;
    const UTRANTOLTE: u8 = 3u8;
    const GERANTOLTE: u8 = 4u8;
}

#[derive(Debug, AperCodec)]
#[asn(type = "ENUMERATED", extensible = true, lb = "0", ub = "0")]
pub struct HoReportType(u8);
impl HoReportType {
    const UNNECESSARYHOTOANOTHERRAT: u8 = 0u8;
}

#[derive(Debug, AperCodec)]
#[asn(type = "ENUMERATED", extensible = true, lb = "0", ub = "1")]
pub struct HoType(u8);
impl HoType {
    const LTETOUTRAN: u8 = 0u8;
    const LTETOGERAN: u8 = 1u8;
}

#[derive(Debug, AperCodec)]
#[asn(type = "ENUMERATED", extensible = true, lb = "0", ub = "1")]
pub struct IAB_Authorized(u8);
impl IAB_Authorized {
    const AUTHORIZED: u8 = 0u8;
    const NOT_AUTHORIZED: u8 = 1u8;
}

#[derive(Debug, AperCodec)]
#[asn(type = "ENUMERATED", extensible = true, lb = "0", ub = "0")]
pub struct IAB_Node_Indication(u8);
impl IAB_Node_Indication {
    const TRUE: u8 = 0u8;
}

#[derive(Debug, AperCodec)]
#[asn(type = "ENUMERATED", extensible = true, lb = "0", ub = "0")]
pub struct IAB_Supported(u8);
impl IAB_Supported {
    const TRUE: u8 = 0u8;
}

#[derive(Debug, AperCodec)]
#[asn(type = "OCTET-STRING", sz_extensible = false, sz_lb = "3", sz_ub = "8")]
pub struct IMSI(Vec<u8>);

#[derive(Debug, AperCodec)]
#[asn(type = "ENUMERATED", extensible = true, lb = "0", ub = "0")]
pub struct IMSvoiceEPSfallbackfrom5G(u8);
impl IMSvoiceEPSfallbackfrom5G {
    const TRUE: u8 = 0u8;
}

#[derive(Debug, AperCodec)]
#[asn(type = "CHOICE", lb = "0", ub = "2", extensible = true)]
pub enum IRAT_Cell_ID {
    #[asn(key = 0, extended = false)]
    EUTRAN(OCTET_STRING_22),
    #[asn(key = 1, extended = false)]
    UTRAN(OCTET_STRING_23),
    #[asn(key = 2, extended = false)]
    GERAN(OCTET_STRING_24),
    #[asn(key = 0, extended = true)]
    EHRPD(EHRPD_Sector_ID),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 3)]
pub struct ImmediateMDT {
    pub measurements_to_activate: MeasurementsToActivate,
    pub m1reporting_trigger: M1ReportingTrigger,
    #[asn(optional_idx = 0)]
    pub m1thresholdevent_a2: Option<M1ThresholdEventA2>,
    #[asn(optional_idx = 1)]
    pub m1periodic_reporting: Option<M1PeriodicReporting>,
    #[asn(optional_idx = 2)]
    pub ie_extensions: Option<ImmediateMDTIE_Extensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct InformationOnRecommendedCellsAndENBsForPaging {
    pub recommended_cells_for_paging: RecommendedCellsForPaging,
    pub recommend_en_bs_for_paging: RecommendedENBsForPaging,
    #[asn(optional_idx = 0)]
    pub ie_extensions: Option<InformationOnRecommendedCellsAndENBsForPagingIE_Extensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true)]
pub struct InitialContextSetupFailure {
    pub protocol_i_es: InitialContextSetupFailureProtocolIEs,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true)]
pub struct InitialContextSetupRequest {
    pub protocol_i_es: InitialContextSetupRequestProtocolIEs,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true)]
pub struct InitialContextSetupResponse {
    pub protocol_i_es: InitialContextSetupResponseProtocolIEs,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true)]
pub struct InitialUEMessage {
    pub protocol_i_es: InitialUEMessageProtocolIEs,
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
#[asn(type = "BITSTRING", sz_extensible = true, sz_lb = "16", sz_ub = "16")]
pub struct IntegrityProtectionAlgorithms(BitVec<Msb0, u8>);

#[derive(Debug, AperCodec)]
#[asn(type = "INTEGER", lb = "1", ub = "16", extensible = true)]
pub struct IntendedNumberOfPagingAttempts(u8);

#[derive(Debug, AperCodec)]
#[asn(type = "CHOICE", lb = "0", ub = "0", extensible = true)]
pub enum Inter_SystemInformationTransferType {
    #[asn(key = 0, extended = false)]
    RIMTransfer(RIMTransfer),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false, optional_fields = 8)]
pub struct InterSystemMeasurementItem {
    pub freq_band_indicator_nr: INTEGER_25,
    pub ss_bfrequencies: INTEGER_26,
    pub subcarrier_spacing_ssb: ENUMERATED_27,
    #[asn(optional_idx = 0)]
    pub max_rs_index_cell_qual: Option<INTEGER_28>,
    #[asn(optional_idx = 1)]
    pub smtc: Option<OCTET_STRING_29>,
    #[asn(optional_idx = 2)]
    pub thresh_rs_index_r15: Option<OCTET_STRING_30>,
    #[asn(optional_idx = 3)]
    pub ssb_to_measure: Option<OCTET_STRING_31>,
    #[asn(optional_idx = 4)]
    pub ssrssi_measurement: Option<OCTET_STRING_32>,
    #[asn(optional_idx = 5)]
    pub quantity_config_nr_r15: Option<OCTET_STRING_33>,
    #[asn(optional_idx = 6)]
    pub black_cells_to_add_mod_list: Option<OCTET_STRING_34>,
    #[asn(optional_idx = 7)]
    pub ie_extensions: Option<InterSystemMeasurementItemIE_Extensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "64")]
pub struct InterSystemMeasurementList(Vec<InterSystemMeasurementItem>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 2)]
pub struct InterSystemMeasurementParameters {
    pub measurement_duration: INTEGER_35,
    #[asn(optional_idx = 0)]
    pub inter_system_measurement_list: Option<InterSystemMeasurementList>,
    #[asn(optional_idx = 1)]
    pub ie_extensions: Option<InterSystemMeasurementParametersIE_Extensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "BITSTRING", sz_extensible = false, sz_lb = "8", sz_ub = "8")]
pub struct InterfacesToTrace(BitVec<Msb0, u8>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 4)]
pub struct IntersystemMeasurementConfiguration {
    #[asn(optional_idx = 0)]
    pub rsrp: Option<INTEGER_36>,
    #[asn(optional_idx = 1)]
    pub rsrq: Option<INTEGER_37>,
    #[asn(optional_idx = 2)]
    pub sinr: Option<INTEGER_38>,
    pub inter_system_measurement_parameters: InterSystemMeasurementParameters,
    #[asn(optional_idx = 3)]
    pub ie_extensions: Option<IntersystemMeasurementConfigurationIE_Extensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "OCTET-STRING")]
pub struct IntersystemSONConfigurationTransfer(Vec<u8>);

#[derive(Debug, AperCodec)]
#[asn(type = "ENUMERATED", lb = "0", ub = "0")]
pub struct KillAllWarningMessages(u8);
impl KillAllWarningMessages {
    const TRUE: u8 = 0u8;
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true)]
pub struct KillRequest {
    pub protocol_i_es: KillRequestProtocolIEs,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true)]
pub struct KillResponse {
    pub protocol_i_es: KillResponseProtocolIEs,
}

#[derive(Debug, AperCodec)]
#[asn(type = "OCTET-STRING")]
pub struct L3_Information(Vec<u8>);

#[derive(Debug, AperCodec)]
#[asn(type = "OCTET-STRING", sz_extensible = false, sz_lb = "2", sz_ub = "2")]
pub struct LAC(Vec<u8>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct LAI {
    pub plm_nidentity: PLMNidentity,
    pub lac: LAC,
    #[asn(optional_idx = 0)]
    pub ie_extensions: Option<LAIIE_Extensions>,
}

#[derive(Debug, AperCodec)]
#[asn(
    type = "OCTET-STRING",
    sz_extensible = false,
    sz_lb = "32",
    sz_ub = "256"
)]
pub struct LHN_ID(Vec<u8>);

#[derive(Debug, AperCodec)]
#[asn(type = "OCTET-STRING")]
pub struct LPPa_PDU(Vec<u8>);

#[derive(Debug, AperCodec)]
#[asn(type = "ENUMERATED", extensible = true, lb = "0", ub = "0")]
pub struct LTE_M_Indication(u8);
impl LTE_M_Indication {
    const LTE_M: u8 = 0u8;
}

#[derive(Debug, AperCodec)]
#[asn(type = "CHOICE", lb = "0", ub = "2", extensible = true)]
pub enum LastVisitedCell_Item {
    #[asn(key = 0, extended = false)]
    E_UTRAN_Cell(LastVisitedEUTRANCellInformation),
    #[asn(key = 1, extended = false)]
    UTRAN_Cell(LastVisitedUTRANCellInformation),
    #[asn(key = 2, extended = false)]
    GERAN_Cell(LastVisitedGERANCellInformation),
    #[asn(key = 0, extended = true)]
    NG_RAN_Cell(LastVisitedNGRANCellInformation),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct LastVisitedEUTRANCellInformation {
    pub global_cell_id: EUTRAN_CGI,
    pub cell_type: CellType,
    pub time_ue_stayed_in_cell: Time_UE_StayedInCell,
    #[asn(optional_idx = 0)]
    pub ie_extensions: Option<LastVisitedEUTRANCellInformationIE_Extensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "CHOICE", lb = "0", ub = "0", extensible = true)]
pub enum LastVisitedGERANCellInformation {
    #[asn(key = 0, extended = false)]
    Undefined(NULL_39),
}

#[derive(Debug, AperCodec)]
#[asn(type = "OCTET-STRING")]
pub struct LastVisitedNGRANCellInformation(Vec<u8>);

#[derive(Debug, AperCodec)]
#[asn(type = "OCTET-STRING")]
pub struct LastVisitedUTRANCellInformation(Vec<u8>);

#[derive(Debug, AperCodec)]
#[asn(type = "ENUMERATED", extensible = true, lb = "0", ub = "2")]
pub struct Links_to_log(u8);
impl Links_to_log {
    const UPLINK: u8 = 0u8;
    const DOWNLINK: u8 = 1u8;
    const BOTH_UPLINK_AND_DOWNLINK: u8 = 2u8;
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct ListeningSubframePattern {
    pub pattern_period: ENUMERATED_40,
    pub pattern_offset: INTEGER_41,
    #[asn(optional_idx = 0)]
    pub ie_extensions: Option<ListeningSubframePatternIE_Extensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true)]
pub struct LocationReport {
    pub protocol_i_es: LocationReportProtocolIEs,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true)]
pub struct LocationReportingControl {
    pub protocol_i_es: LocationReportingControlProtocolIEs,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true)]
pub struct LocationReportingFailureIndication {
    pub protocol_i_es: LocationReportingFailureIndicationProtocolIEs,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 2)]
pub struct LoggedMBSFNMDT {
    pub logging_interval: LoggingInterval,
    pub logging_duration: LoggingDuration,
    #[asn(optional_idx = 0)]
    pub mbsfn_result_to_log: Option<MBSFN_ResultToLog>,
    #[asn(optional_idx = 1)]
    pub ie_extensions: Option<LoggedMBSFNMDTIE_Extensions>,
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
#[asn(type = "ENUMERATED", lb = "0", ub = "5")]
pub struct LoggingDuration(u8);
impl LoggingDuration {
    const M10: u8 = 0u8;
    const M20: u8 = 1u8;
    const M40: u8 = 2u8;
    const M60: u8 = 3u8;
    const M90: u8 = 4u8;
    const M120: u8 = 5u8;
}

#[derive(Debug, AperCodec)]
#[asn(type = "ENUMERATED", lb = "0", ub = "7")]
pub struct LoggingInterval(u8);
impl LoggingInterval {
    const MS128: u8 = 0u8;
    const MS256: u8 = 1u8;
    const MS512: u8 = 2u8;
    const MS1024: u8 = 3u8;
    const MS2048: u8 = 4u8;
    const MS3072: u8 = 5u8;
    const MS4096: u8 = 6u8;
    const MS6144: u8 = 7u8;
}

#[derive(Debug, AperCodec)]
#[asn(type = "OCTET-STRING", sz_extensible = false, sz_lb = "4", sz_ub = "4")]
pub struct M_TMSI(Vec<u8>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct M1PeriodicReporting {
    pub report_interval: ReportIntervalMDT,
    pub report_amount: ReportAmountMDT,
    #[asn(optional_idx = 0)]
    pub ie_extensions: Option<M1PeriodicReportingIE_Extensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "ENUMERATED", extensible = true, lb = "0", ub = "1")]
pub struct M1ReportingTrigger(u8);
impl M1ReportingTrigger {
    const PERIODIC: u8 = 0u8;
    const A2EVENTTRIGGERED: u8 = 1u8;
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct M1ThresholdEventA2 {
    pub measurement_threshold: MeasurementThresholdA2,
    #[asn(optional_idx = 0)]
    pub ie_extensions: Option<M1ThresholdEventA2IE_Extensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct M3Configuration {
    pub m3period: M3period,
    #[asn(optional_idx = 0)]
    pub ie_extensions: Option<M3ConfigurationIE_Extensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "ENUMERATED", extensible = true, lb = "0", ub = "2")]
pub struct M3period(u8);
impl M3period {
    const MS100: u8 = 0u8;
    const MS1000: u8 = 1u8;
    const MS10000: u8 = 2u8;
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct M4Configuration {
    pub m4period: M4period,
    pub m4_links_to_log: Links_to_log,
    #[asn(optional_idx = 0)]
    pub ie_extensions: Option<M4ConfigurationIE_Extensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "ENUMERATED", extensible = true, lb = "0", ub = "4")]
pub struct M4period(u8);
impl M4period {
    const MS1024: u8 = 0u8;
    const MS2048: u8 = 1u8;
    const MS5120: u8 = 2u8;
    const MS10240: u8 = 3u8;
    const MIN1: u8 = 4u8;
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct M5Configuration {
    pub m5period: M5period,
    pub m5_links_to_log: Links_to_log,
    #[asn(optional_idx = 0)]
    pub ie_extensions: Option<M5ConfigurationIE_Extensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "ENUMERATED", extensible = true, lb = "0", ub = "4")]
pub struct M5period(u8);
impl M5period {
    const MS1024: u8 = 0u8;
    const MS2048: u8 = 1u8;
    const MS5120: u8 = 2u8;
    const MS10240: u8 = 3u8;
    const MIN1: u8 = 4u8;
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 2)]
pub struct M6Configuration {
    pub m6report_interval: M6report_Interval,
    #[asn(optional_idx = 0)]
    pub m6delay_threshold: Option<M6delay_threshold>,
    pub m6_links_to_log: Links_to_log,
    #[asn(optional_idx = 1)]
    pub ie_extensions: Option<M6ConfigurationIE_Extensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "ENUMERATED", extensible = true, lb = "0", ub = "11")]
pub struct M6delay_threshold(u8);
impl M6delay_threshold {
    const MS30: u8 = 0u8;
    const MS40: u8 = 1u8;
    const MS50: u8 = 2u8;
    const MS60: u8 = 3u8;
    const MS70: u8 = 4u8;
    const MS80: u8 = 5u8;
    const MS90: u8 = 6u8;
    const MS100: u8 = 7u8;
    const MS150: u8 = 8u8;
    const MS300: u8 = 9u8;
    const MS500: u8 = 10u8;
    const MS750: u8 = 11u8;
}

#[derive(Debug, AperCodec)]
#[asn(type = "ENUMERATED", extensible = true, lb = "0", ub = "3")]
pub struct M6report_Interval(u8);
impl M6report_Interval {
    const MS1024: u8 = 0u8;
    const MS2048: u8 = 1u8;
    const MS5120: u8 = 2u8;
    const MS10240: u8 = 3u8;
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct M7Configuration {
    pub m7period: M7period,
    pub m7_links_to_log: Links_to_log,
    #[asn(optional_idx = 0)]
    pub ie_extensions: Option<M7ConfigurationIE_Extensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "INTEGER", lb = "1", ub = "60", extensible = true)]
pub struct M7period(u8);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "8")]
pub struct MBSFN_ResultToLog(Vec<MBSFN_ResultToLogInfo>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 2)]
pub struct MBSFN_ResultToLogInfo {
    #[asn(optional_idx = 0)]
    pub mbsfn_area_id: Option<INTEGER_42>,
    pub carrier_freq: EARFCN,
    #[asn(optional_idx = 1)]
    pub ie_extensions: Option<MBSFN_ResultToLogInfoIE_Extensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "ENUMERATED", extensible = true, lb = "0", ub = "2")]
pub struct MDT_Activation(u8);
impl MDT_Activation {
    const IMMEDIATE_MDT_ONLY: u8 = 0u8;
    const IMMEDIATE_MDT_AND_TRACE: u8 = 1u8;
    const LOGGED_MDT_ONLY: u8 = 2u8;
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct MDT_Configuration {
    pub mdt_activation: MDT_Activation,
    pub area_scope_of_mdt: AreaScopeOfMDT,
    pub mdt_mode: MDTMode,
    #[asn(optional_idx = 0)]
    pub ie_extensions: Option<MDT_ConfigurationIE_Extensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "OCTET-STRING")]
pub struct MDT_ConfigurationNR(Vec<u8>);

#[derive(Debug, AperCodec)]
#[asn(type = "BITSTRING", sz_extensible = false, sz_lb = "8", sz_ub = "8")]
pub struct MDT_Location_Info(BitVec<Msb0, u8>);

#[derive(Debug, AperCodec)]
#[asn(type = "CHOICE", lb = "0", ub = "1", extensible = true)]
pub enum MDTMode {
    #[asn(key = 0, extended = false)]
    ImmediateMDT(ImmediateMDT),
    #[asn(key = 1, extended = false)]
    LoggedMDT(LoggedMDT),
    #[asn(key = 0, extended = true)]
    MDTMode_Extension(MDTMode_Extension),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct MDTMode_Extension {
    #[asn(key_field = true)]
    pub id: ProtocolIE_ID,
    pub criticality: Criticality,
    pub value: MDTMode_ExtensionValue,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "16")]
pub struct MDTPLMNList(Vec<PLMNidentity>);

#[derive(Debug, AperCodec)]
#[asn(type = "OCTET-STRING", sz_extensible = false, sz_lb = "1", sz_ub = "1")]
pub struct MME_Code(Vec<u8>);

#[derive(Debug, AperCodec)]
#[asn(type = "OCTET-STRING", sz_extensible = false, sz_lb = "2", sz_ub = "2")]
pub struct MME_Group_ID(Vec<u8>);

#[derive(Debug, AperCodec)]
#[asn(type = "INTEGER", lb = "0", ub = "4294967295")]
pub struct MME_UE_S1AP_ID(u32);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true)]
pub struct MMECPRelocationIndication {
    pub protocol_i_es: MMECPRelocationIndicationProtocolIEs,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true)]
pub struct MMEConfigurationTransfer {
    pub protocol_i_es: MMEConfigurationTransferProtocolIEs,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true)]
pub struct MMEConfigurationUpdate {
    pub protocol_i_es: MMEConfigurationUpdateProtocolIEs,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true)]
pub struct MMEConfigurationUpdateAcknowledge {
    pub protocol_i_es: MMEConfigurationUpdateAcknowledgeProtocolIEs,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true)]
pub struct MMEConfigurationUpdateFailure {
    pub protocol_i_es: MMEConfigurationUpdateFailureProtocolIEs,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true)]
pub struct MMEDirectInformationTransfer {
    pub protocol_i_es: MMEDirectInformationTransferProtocolIEs,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true)]
pub struct MMEEarlyStatusTransfer {
    pub protocol_i_es: MMEEarlyStatusTransferProtocolIEs,
}

#[derive(Debug, AperCodec)]
#[asn(type = "CHOICE", lb = "0", ub = "1", extensible = true)]
pub enum MMEPagingTarget {
    #[asn(key = 0, extended = false)]
    Global_ENB_ID(Global_ENB_ID),
    #[asn(key = 1, extended = false)]
    TAI(TAI),
}

#[derive(Debug, AperCodec)]
#[asn(type = "ENUMERATED", extensible = true, lb = "0", ub = "0")]
pub struct MMERelaySupportIndicator(u8);
impl MMERelaySupportIndicator {
    const TRUE: u8 = 0u8;
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true)]
pub struct MMEStatusTransfer {
    pub protocol_i_es: MMEStatusTransferProtocolIEs,
}

#[derive(Debug, AperCodec)]
#[asn(
    type = "PrintableString",
    sz_extensible = true,
    sz_lb = "1",
    sz_ub = "150"
)]
pub struct MMEname(String);

#[derive(Debug, AperCodec)]
#[asn(type = "OCTET-STRING")]
pub struct MSClassmark2(Vec<u8>);

#[derive(Debug, AperCodec)]
#[asn(type = "OCTET-STRING")]
pub struct MSClassmark3(Vec<u8>);

#[derive(Debug, AperCodec)]
#[asn(type = "ENUMERATED", extensible = true, lb = "0", ub = "0")]
pub struct ManagementBasedMDTAllowed(u8);
impl ManagementBasedMDTAllowed {
    const ALLOWED: u8 = 0u8;
}

#[derive(Debug, AperCodec)]
#[asn(type = "BITSTRING", sz_extensible = false, sz_lb = "64", sz_ub = "64")]
pub struct Masked_IMEISV(BitVec<Msb0, u8>);

#[derive(Debug, AperCodec)]
#[asn(type = "CHOICE", lb = "0", ub = "1", extensible = true)]
pub enum MeasurementThresholdA2 {
    #[asn(key = 0, extended = false)]
    Threshold_RSRP(Threshold_RSRP),
    #[asn(key = 1, extended = false)]
    Threshold_RSRQ(Threshold_RSRQ),
}

#[derive(Debug, AperCodec)]
#[asn(type = "BITSTRING", sz_extensible = false, sz_lb = "8", sz_ub = "8")]
pub struct MeasurementsToActivate(BitVec<Msb0, u8>);

#[derive(Debug, AperCodec)]
#[asn(type = "BITSTRING", sz_extensible = false, sz_lb = "16", sz_ub = "16")]
pub struct MessageIdentifier(BitVec<Msb0, u8>);

#[derive(Debug, AperCodec)]
#[asn(type = "BITSTRING", sz_extensible = false, sz_lb = "32", sz_ub = "32")]
pub struct MobilityInformation(BitVec<Msb0, u8>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true)]
pub struct MultiCellLoadReportingRequest {
    pub requested_cell_list: RequestedCellList,
}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "128"
)]
pub struct MultiCellLoadReportingResponse(Vec<MultiCellLoadReportingResponse_Item>);

#[derive(Debug, AperCodec)]
#[asn(type = "CHOICE", lb = "0", ub = "2", extensible = true)]
pub enum MultiCellLoadReportingResponse_Item {
    #[asn(key = 0, extended = false)]
    EUTRANResponse(EUTRANResponse),
    #[asn(key = 1, extended = false)]
    UTRANResponse(OCTET_STRING_43),
    #[asn(key = 2, extended = false)]
    GERANResponse(OCTET_STRING_44),
    #[asn(key = 0, extended = true)]
    EHRPD(EHRPDMultiSectorLoadReportingResponseItem),
}

#[derive(Debug, AperCodec)]
#[asn(type = "ENUMERATED", extensible = true, lb = "0", ub = "1")]
pub struct MutingAvailabilityIndication(u8);
impl MutingAvailabilityIndication {
    const AVAILABLE: u8 = 0u8;
    const UNAVAILABLE: u8 = 1u8;
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 2)]
pub struct MutingPatternInformation {
    pub muting_pattern_period: ENUMERATED_45,
    #[asn(optional_idx = 0)]
    pub muting_pattern_offset: Option<INTEGER_46>,
    #[asn(optional_idx = 1)]
    pub ie_extensions: Option<MutingPatternInformationIE_Extensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "OCTET-STRING")]
pub struct NAS_PDU(Vec<u8>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true)]
pub struct NASDeliveryIndication {
    pub protocol_i_es: NASDeliveryIndicationProtocolIEs,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true)]
pub struct NASNonDeliveryIndication {
    pub protocol_i_es: NASNonDeliveryIndicationProtocolIEs,
}

#[derive(Debug, AperCodec)]
#[asn(type = "OCTET-STRING")]
pub struct NASSecurityParametersfromE_UTRAN(Vec<u8>);

#[derive(Debug, AperCodec)]
#[asn(type = "OCTET-STRING")]
pub struct NASSecurityParameterstoE_UTRAN(Vec<u8>);

#[derive(Debug, AperCodec)]
#[asn(type = "ENUMERATED", extensible = true, lb = "0", ub = "3")]
pub struct NB_IoT_DefaultPagingDRX(u8);
impl NB_IoT_DefaultPagingDRX {
    const V128: u8 = 0u8;
    const V256: u8 = 1u8;
    const V512: u8 = 2u8;
    const V1024: u8 = 3u8;
}

#[derive(Debug, AperCodec)]
#[asn(type = "ENUMERATED", extensible = true, lb = "0", ub = "13")]
pub struct NB_IoT_Paging_eDRX_Cycle(u8);
impl NB_IoT_Paging_eDRX_Cycle {
    const HF2: u8 = 0u8;
    const HF4: u8 = 1u8;
    const HF6: u8 = 2u8;
    const HF8: u8 = 3u8;
    const HF10: u8 = 4u8;
    const HF12: u8 = 5u8;
    const HF14: u8 = 6u8;
    const HF16: u8 = 7u8;
    const HF32: u8 = 8u8;
    const HF64: u8 = 9u8;
    const HF128: u8 = 10u8;
    const HF256: u8 = 11u8;
    const HF512: u8 = 12u8;
    const HF1024: u8 = 13u8;
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 2)]
pub struct NB_IoT_Paging_eDRXInformation {
    pub nb_io_t_paging_e_drx_cycle: NB_IoT_Paging_eDRX_Cycle,
    #[asn(optional_idx = 0)]
    pub nb_io_t_paging_time_window: Option<NB_IoT_PagingTimeWindow>,
    #[asn(optional_idx = 1)]
    pub ie_extensions: Option<NB_IoT_Paging_eDRXInformationIE_Extensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "ENUMERATED", extensible = true, lb = "0", ub = "5")]
pub struct NB_IoT_PagingDRX(u8);
impl NB_IoT_PagingDRX {
    const V32: u8 = 0u8;
    const V64: u8 = 1u8;
    const V128: u8 = 2u8;
    const V256: u8 = 3u8;
    const V512: u8 = 4u8;
    const V1024: u8 = 5u8;
}

#[derive(Debug, AperCodec)]
#[asn(type = "ENUMERATED", extensible = true, lb = "0", ub = "15")]
pub struct NB_IoT_PagingTimeWindow(u8);
impl NB_IoT_PagingTimeWindow {
    const S1: u8 = 0u8;
    const S2: u8 = 1u8;
    const S3: u8 = 2u8;
    const S4: u8 = 3u8;
    const S5: u8 = 4u8;
    const S6: u8 = 5u8;
    const S7: u8 = 6u8;
    const S8: u8 = 7u8;
    const S9: u8 = 8u8;
    const S10: u8 = 9u8;
    const S11: u8 = 10u8;
    const S12: u8 = 11u8;
    const S13: u8 = 12u8;
    const S14: u8 = 13u8;
    const S15: u8 = 14u8;
    const S16: u8 = 15u8;
}

#[derive(Debug, AperCodec)]
#[asn(type = "OCTET-STRING")]
pub struct NB_IoT_RLF_Report_Container(Vec<u8>);

#[derive(Debug, AperCodec)]
#[asn(type = "BITSTRING", sz_extensible = false, sz_lb = "12", sz_ub = "12")]
pub struct NB_IoT_UEIdentityIndexValue(BitVec<Msb0, u8>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct NG_eNB {
    pub global_ng_e_nb_id: Global_ENB_ID,
    #[asn(optional_idx = 0)]
    pub ie_extensions: Option<NG_eNBIE_Extensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct NR_CGI {
    pub plmn_identity: PLMNidentity,
    pub nr_cell_identity: NRCellIdentity,
    #[asn(optional_idx = 0)]
    pub ie_extensions: Option<NR_CGIIE_Extensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "BITSTRING", sz_extensible = false, sz_lb = "36", sz_ub = "36")]
pub struct NRCellIdentity(BitVec<Msb0, u8>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct NRUESecurityCapabilities {
    pub n_rencryption_algorithms: NRencryptionAlgorithms,
    pub n_rintegrity_protection_algorithms: NRintegrityProtectionAlgorithms,
    #[asn(optional_idx = 0)]
    pub ie_extensions: Option<NRUESecurityCapabilitiesIE_Extensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct NRUESidelinkAggregateMaximumBitrate {
    pub u_eaggregate_maximum_bit_rate: BitRate,
    #[asn(optional_idx = 0)]
    pub ie_extensions: Option<NRUESidelinkAggregateMaximumBitrateIE_Extensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 3)]
pub struct NRV2XServicesAuthorized {
    #[asn(optional_idx = 0)]
    pub vehicle_ue: Option<VehicleUE>,
    #[asn(optional_idx = 1)]
    pub pedestrian_ue: Option<PedestrianUE>,
    #[asn(optional_idx = 2)]
    pub ie_extensions: Option<NRV2XServicesAuthorizedIE_Extensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "BITSTRING", sz_extensible = true, sz_lb = "16", sz_ub = "16")]
pub struct NRencryptionAlgorithms(BitVec<Msb0, u8>);

#[derive(Debug, AperCodec)]
#[asn(type = "BITSTRING", sz_extensible = true, sz_lb = "16", sz_ub = "16")]
pub struct NRintegrityProtectionAlgorithms(BitVec<Msb0, u8>);

#[derive(Debug, AperCodec)]
#[asn(type = "ENUMERATED", extensible = true, lb = "0", ub = "0")]
pub struct NRrestrictionin5GS(u8);
impl NRrestrictionin5GS {
    const N_RRESTRICTEDIN5_GS: u8 = 0u8;
}

#[derive(Debug, AperCodec)]
#[asn(type = "ENUMERATED", extensible = true, lb = "0", ub = "0")]
pub struct NRrestrictioninEPSasSecondaryRAT(u8);
impl NRrestrictioninEPSasSecondaryRAT {
    const N_RRESTRICTEDIN_EP_SAS_SECONDARY_RAT: u8 = 0u8;
}

#[derive(Debug, AperCodec)]
#[asn(type = "ENUMERATED", extensible = true, lb = "0", ub = "1")]
pub struct NextPagingAreaScope(u8);
impl NextPagingAreaScope {
    const SAME: u8 = 0u8;
    const CHANGED: u8 = 1u8;
}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "256"
)]
pub struct NotificationCellList(Vec<NotificationCellList_Item>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true)]
pub struct NotificationCellList_Item {
    pub cell_id: OCTET_STRING_47,
    pub notify_flag: NotifyFlag,
}

#[derive(Debug, AperCodec)]
#[asn(type = "ENUMERATED", extensible = true, lb = "0", ub = "1")]
pub struct NotifyFlag(u8);
impl NotifyFlag {
    const ACTIVATED: u8 = 0u8;
    const DEACTIVATED: u8 = 1u8;
}

#[derive(Debug, AperCodec)]
#[asn(type = "ENUMERATED", extensible = true, lb = "0", ub = "0")]
pub struct NotifySourceeNB(u8);
impl NotifySourceeNB {
    const NOTIFY_SOURCE: u8 = 0u8;
}

#[derive(Debug, AperCodec)]
#[asn(type = "INTEGER", lb = "0", ub = "65535")]
pub struct NumberOfBroadcasts(u16);

#[derive(Debug, AperCodec)]
#[asn(type = "ENUMERATED", extensible = true, lb = "0", ub = "4")]
pub struct NumberOfMeasurementReportingLevels(u8);
impl NumberOfMeasurementReportingLevels {
    const RL2: u8 = 0u8;
    const RL3: u8 = 1u8;
    const RL4: u8 = 2u8;
    const RL5: u8 = 3u8;
    const RL10: u8 = 4u8;
}

#[derive(Debug, AperCodec)]
#[asn(type = "INTEGER", lb = "0", ub = "65535")]
pub struct NumberofBroadcastRequest(u16);

#[derive(Debug, AperCodec)]
#[asn(type = "OCTET-STRING")]
pub struct OldBSS_ToNewBSS_Information(Vec<u8>);

#[derive(Debug, AperCodec)]
#[asn(type = "ENUMERATED", extensible = true, lb = "0", ub = "2")]
pub struct OverloadAction(u8);
impl OverloadAction {
    const REJECT_NON_EMERGENCY_MO_DT: u8 = 0u8;
    const REJECT_RRC_CR_SIGNALLING: u8 = 1u8;
    const PERMIT_EMERGENCY_SESSIONS_AND_MOBILE_TERMINATED_SERVICES_ONLY: u8 = 2u8;
}

#[derive(Debug, AperCodec)]
#[asn(type = "ENUMERATED", extensible = true, lb = "0", ub = "0")]
pub struct OverloadFlag(u8);
impl OverloadFlag {
    const OVERLOAD: u8 = 0u8;
}

#[derive(Debug, AperCodec)]
#[asn(type = "CHOICE", lb = "0", ub = "0", extensible = true)]
pub enum OverloadResponse {
    #[asn(key = 0, extended = false)]
    OverloadAction(OverloadAction),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true)]
pub struct OverloadStart {
    pub protocol_i_es: OverloadStartProtocolIEs,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true)]
pub struct OverloadStop {
    pub protocol_i_es: OverloadStopProtocolIEs,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct PC5FlowBitRates {
    pub guaranteed_flow_bit_rate: BitRate,
    pub maximum_flow_bit_rate: BitRate,
    #[asn(optional_idx = 0)]
    pub ie_extensions: Option<PC5FlowBitRatesIE_Extensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 3)]
pub struct PC5QoSFlowItem {
    pub pqi: FiveQI,
    #[asn(optional_idx = 0)]
    pub pc5_flow_bit_rates: Option<PC5FlowBitRates>,
    #[asn(optional_idx = 1)]
    pub range: Option<Range>,
    #[asn(optional_idx = 2)]
    pub ie_extensions: Option<PC5QoSFlowItemIE_Extensions>,
}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "2048"
)]
pub struct PC5QoSFlowList(Vec<PC5QoSFlowItem>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 2)]
pub struct PC5QoSParameters {
    pub pc5_qo_s_flow_list: PC5QoSFlowList,
    #[asn(optional_idx = 0)]
    pub pc5_link_aggregated_bit_rates: Option<BitRate>,
    #[asn(optional_idx = 1)]
    pub ie_extensions: Option<PC5QoSParametersIE_Extensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "INTEGER", lb = "0", ub = "4095")]
pub struct PDCP_SN(u16);

#[derive(Debug, AperCodec)]
#[asn(type = "INTEGER", lb = "0", ub = "32767")]
pub struct PDCP_SNExtended(u16);

#[derive(Debug, AperCodec)]
#[asn(type = "INTEGER", lb = "0", ub = "262143")]
pub struct PDCP_SNlength18(u32);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct PLMNAreaBasedQMC {
    pub plmn_listfor_qmc: PLMNListforQMC,
    #[asn(optional_idx = 0)]
    pub ie_extensions: Option<PLMNAreaBasedQMCIE_Extensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "16")]
pub struct PLMNListforQMC(Vec<PLMNidentity>);

pub type PLMNidentity = TBCD_STRING;

#[derive(Debug, AperCodec)]
#[asn(type = "ENUMERATED", extensible = true, lb = "0", ub = "0")]
pub struct PS_ServiceNotAvailable(u8);
impl PS_ServiceNotAvailable {
    const PS_SERVICE_NOT_AVAILABLE: u8 = 0u8;
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct PSCellInformation {
    pub ncgi: NR_CGI,
    #[asn(optional_idx = 0)]
    pub ie_extensions: Option<PSCellInformationIE_Extensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true)]
pub struct PWSFailureIndication {
    pub protocol_i_es: PWSFailureIndicationProtocolIEs,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true)]
pub struct PWSRestartIndication {
    pub protocol_i_es: PWSRestartIndicationProtocolIEs,
}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "256"
)]
pub struct PWSfailedECGIList(Vec<EUTRAN_CGI>);

#[derive(Debug, AperCodec)]
#[asn(type = "INTEGER", lb = "0", ub = "1000")]
pub struct Packet_LossRate(u16);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true)]
pub struct Paging {
    pub protocol_i_es: PagingProtocolIEs,
}

#[derive(Debug, AperCodec)]
#[asn(type = "ENUMERATED", extensible = true, lb = "0", ub = "13")]
pub struct Paging_eDRX_Cycle(u8);
impl Paging_eDRX_Cycle {
    const HFHALF: u8 = 0u8;
    const HF1: u8 = 1u8;
    const HF2: u8 = 2u8;
    const HF4: u8 = 3u8;
    const HF6: u8 = 4u8;
    const HF8: u8 = 5u8;
    const HF10: u8 = 6u8;
    const HF12: u8 = 7u8;
    const HF14: u8 = 8u8;
    const HF16: u8 = 9u8;
    const HF32: u8 = 10u8;
    const HF64: u8 = 11u8;
    const HF128: u8 = 12u8;
    const HF256: u8 = 13u8;
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 2)]
pub struct Paging_eDRXInformation {
    pub paging_e_drx_cycle: Paging_eDRX_Cycle,
    #[asn(optional_idx = 0)]
    pub paging_time_window: Option<PagingTimeWindow>,
    #[asn(optional_idx = 1)]
    pub ie_extensions: Option<Paging_eDRXInformationIE_Extensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "INTEGER", lb = "1", ub = "16", extensible = true)]
pub struct PagingAttemptCount(u8);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 2)]
pub struct PagingAttemptInformation {
    pub paging_attempt_count: PagingAttemptCount,
    pub intended_number_of_paging_attempts: IntendedNumberOfPagingAttempts,
    #[asn(optional_idx = 0)]
    pub next_paging_area_scope: Option<NextPagingAreaScope>,
    #[asn(optional_idx = 1)]
    pub ie_extensions: Option<PagingAttemptInformationIE_Extensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "ENUMERATED", extensible = true, lb = "0", ub = "3")]
pub struct PagingDRX(u8);
impl PagingDRX {
    const V32: u8 = 0u8;
    const V64: u8 = 1u8;
    const V128: u8 = 2u8;
    const V256: u8 = 3u8;
}

#[derive(Debug, AperCodec)]
#[asn(type = "ENUMERATED", extensible = true, lb = "0", ub = "7")]
pub struct PagingPriority(u8);
impl PagingPriority {
    const PRIOLEVEL1: u8 = 0u8;
    const PRIOLEVEL2: u8 = 1u8;
    const PRIOLEVEL3: u8 = 2u8;
    const PRIOLEVEL4: u8 = 3u8;
    const PRIOLEVEL5: u8 = 4u8;
    const PRIOLEVEL6: u8 = 5u8;
    const PRIOLEVEL7: u8 = 6u8;
    const PRIOLEVEL8: u8 = 7u8;
}

#[derive(Debug, AperCodec)]
#[asn(type = "ENUMERATED", extensible = true, lb = "0", ub = "20")]
pub struct PagingProbabilityInformation(u8);
impl PagingProbabilityInformation {
    const P00: u8 = 0u8;
    const P05: u8 = 1u8;
    const P10: u8 = 2u8;
    const P15: u8 = 3u8;
    const P20: u8 = 4u8;
    const P25: u8 = 5u8;
    const P30: u8 = 6u8;
    const P35: u8 = 7u8;
    const P40: u8 = 8u8;
    const P45: u8 = 9u8;
    const P50: u8 = 10u8;
    const P55: u8 = 11u8;
    const P60: u8 = 12u8;
    const P65: u8 = 13u8;
    const P70: u8 = 14u8;
    const P75: u8 = 15u8;
    const P80: u8 = 16u8;
    const P85: u8 = 17u8;
    const P90: u8 = 18u8;
    const P95: u8 = 19u8;
    const P100: u8 = 20u8;
}

#[derive(Debug, AperCodec)]
#[asn(type = "ENUMERATED", extensible = true, lb = "0", ub = "15")]
pub struct PagingTimeWindow(u8);
impl PagingTimeWindow {
    const S1: u8 = 0u8;
    const S2: u8 = 1u8;
    const S3: u8 = 2u8;
    const S4: u8 = 3u8;
    const S5: u8 = 4u8;
    const S6: u8 = 5u8;
    const S7: u8 = 6u8;
    const S8: u8 = 7u8;
    const S9: u8 = 8u8;
    const S10: u8 = 9u8;
    const S11: u8 = 10u8;
    const S12: u8 = 11u8;
    const S13: u8 = 12u8;
    const S14: u8 = 13u8;
    const S15: u8 = 14u8;
    const S16: u8 = 15u8;
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true)]
pub struct PathSwitchRequest {
    pub protocol_i_es: PathSwitchRequestProtocolIEs,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true)]
pub struct PathSwitchRequestAcknowledge {
    pub protocol_i_es: PathSwitchRequestAcknowledgeProtocolIEs,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true)]
pub struct PathSwitchRequestFailure {
    pub protocol_i_es: PathSwitchRequestFailureProtocolIEs,
}

#[derive(Debug, AperCodec)]
#[asn(type = "ENUMERATED", extensible = true, lb = "0", ub = "1")]
pub struct PedestrianUE(u8);
impl PedestrianUE {
    const AUTHORIZED: u8 = 0u8;
    const NOT_AUTHORIZED: u8 = 1u8;
}

#[derive(Debug, AperCodec)]
#[asn(type = "ENUMERATED", extensible = true, lb = "0", ub = "0")]
pub struct PendingDataIndication(u8);
impl PendingDataIndication {
    const TRUE: u8 = 0u8;
}

#[derive(Debug, AperCodec)]
#[asn(type = "OCTET-STRING", sz_extensible = false, sz_lb = "2", sz_ub = "2")]
pub struct Port_Number(Vec<u8>);

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
#[asn(type = "INTEGER", lb = "0", ub = "15")]
pub struct PriorityLevel(u8);

#[derive(Debug, AperCodec)]
#[asn(type = "ENUMERATED", extensible = true, lb = "0", ub = "1")]
pub struct PrivacyIndicator(u8);
impl PrivacyIndicator {
    const IMMEDIATE_MDT: u8 = 0u8;
    const LOGGED_MDT: u8 = 1u8;
}

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
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 3)]
pub struct ProSeAuthorized {
    #[asn(optional_idx = 0)]
    pub pro_se_direct_discovery: Option<ProSeDirectDiscovery>,
    #[asn(optional_idx = 1)]
    pub pro_se_direct_communication: Option<ProSeDirectCommunication>,
    #[asn(optional_idx = 2)]
    pub ie_extensions: Option<ProSeAuthorizedIE_Extensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "ENUMERATED", extensible = true, lb = "0", ub = "1")]
pub struct ProSeDirectCommunication(u8);
impl ProSeDirectCommunication {
    const AUTHORIZED: u8 = 0u8;
    const NOT_AUTHORIZED: u8 = 1u8;
}

#[derive(Debug, AperCodec)]
#[asn(type = "ENUMERATED", extensible = true, lb = "0", ub = "1")]
pub struct ProSeDirectDiscovery(u8);
impl ProSeDirectDiscovery {
    const AUTHORIZED: u8 = 0u8;
    const NOT_AUTHORIZED: u8 = 1u8;
}

#[derive(Debug, AperCodec)]
#[asn(type = "ENUMERATED", extensible = true, lb = "0", ub = "1")]
pub struct ProSeUEtoNetworkRelaying(u8);
impl ProSeUEtoNetworkRelaying {
    const AUTHORIZED: u8 = 0u8;
    const NOT_AUTHORIZED: u8 = 1u8;
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
#[asn(type = "INTEGER", lb = "0", ub = "255")]
pub struct QCI(u8);

#[derive(Debug, AperCodec)]
#[asn(type = "OCTET-STRING", sz_extensible = false, sz_lb = "1", sz_ub = "1")]
pub struct RAC(Vec<u8>);

#[derive(Debug, AperCodec)]
#[asn(type = "INTEGER", lb = "0", ub = "4294967295")]
pub struct RAN_UE_NGAP_ID(u32);

#[derive(Debug, AperCodec)]
#[asn(type = "ENUMERATED", extensible = true, lb = "0", ub = "0")]
pub struct RAT_Type(u8);
impl RAT_Type {
    const NBIOT: u8 = 0u8;
}

#[derive(Debug, AperCodec)]
#[asn(type = "OCTET-STRING")]
pub struct RIMInformation(Vec<u8>);

#[derive(Debug, AperCodec)]
#[asn(type = "CHOICE", lb = "0", ub = "0", extensible = true)]
pub enum RIMRoutingAddress {
    #[asn(key = 0, extended = false)]
    GERAN_Cell_ID(GERAN_Cell_ID),
    #[asn(key = 0, extended = true)]
    TargetRNC_ID(TargetRNC_ID),
    #[asn(key = 1, extended = true)]
    EHRPD_Sector_ID(OCTET_STRING_50),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 2)]
pub struct RIMTransfer {
    pub rim_information: RIMInformation,
    #[asn(optional_idx = 0)]
    pub rim_routing_address: Option<RIMRoutingAddress>,
    #[asn(optional_idx = 1)]
    pub ie_extensions: Option<RIMTransferIE_Extensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 2)]
pub struct RLFReportInformation {
    pub ue_rlf_report_container: UE_RLF_Report_Container,
    #[asn(optional_idx = 0)]
    pub ue_rlf_report_container_for_extended_bands:
        Option<UE_RLF_Report_Container_for_extended_bands>,
    #[asn(optional_idx = 1)]
    pub ie_extensions: Option<RLFReportInformationIE_Extensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "INTEGER", lb = "0", ub = "4095")]
pub struct RNC_ID(u16);

#[derive(Debug, AperCodec)]
#[asn(type = "OCTET-STRING")]
pub struct RRC_Container(Vec<u8>);

#[derive(Debug, AperCodec)]
#[asn(type = "ENUMERATED", extensible = true, lb = "0", ub = "4")]
pub struct RRC_Establishment_Cause(u8);
impl RRC_Establishment_Cause {
    const EMERGENCY: u8 = 0u8;
    const HIGH_PRIORITY_ACCESS: u8 = 1u8;
    const MT_ACCESS: u8 = 2u8;
    const MO_SIGNALLING: u8 = 3u8;
    const MO_DATA: u8 = 4u8;
}

#[derive(Debug, AperCodec)]
#[asn(type = "ENUMERATED", extensible = true, lb = "0", ub = "8")]
pub struct Range(u8);
impl Range {
    const M50: u8 = 0u8;
    const M80: u8 = 1u8;
    const M180: u8 = 2u8;
    const M200: u8 = 3u8;
    const M350: u8 = 4u8;
    const M400: u8 = 5u8;
    const M500: u8 = 6u8;
    const M700: u8 = 7u8;
    const M1000: u8 = 8u8;
}

#[derive(Debug, AperCodec)]
#[asn(
    type = "BITSTRING",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "16384"
)]
pub struct ReceiveStatusOfULPDCPSDUsExtended(BitVec<Msb0, u8>);

#[derive(Debug, AperCodec)]
#[asn(
    type = "BITSTRING",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "131072"
)]
pub struct ReceiveStatusOfULPDCPSDUsPDCP_SNlength18(BitVec<Msb0, u8>);

#[derive(Debug, AperCodec)]
#[asn(
    type = "BITSTRING",
    sz_extensible = false,
    sz_lb = "4096",
    sz_ub = "4096"
)]
pub struct ReceiveStatusofULPDCPSDUs(BitVec<Msb0, u8>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 2)]
pub struct RecommendedCellItem {
    pub eutran_cgi: EUTRAN_CGI,
    #[asn(optional_idx = 0)]
    pub time_stayed_in_cell: Option<INTEGER_51>,
    #[asn(optional_idx = 1)]
    pub ie_extensions: Option<RecommendedCellItemIE_Extensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "16")]
pub struct RecommendedCellList(Vec<RecommendedCellList_Entry>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct RecommendedCellsForPaging {
    pub recommended_cell_list: RecommendedCellList,
    #[asn(optional_idx = 0)]
    pub ie_extensions: Option<RecommendedCellsForPagingIE_Extensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct RecommendedENBItem {
    pub mme_paging_target: MMEPagingTarget,
    #[asn(optional_idx = 0)]
    pub ie_extensions: Option<RecommendedENBItemIE_Extensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "16")]
pub struct RecommendedENBList(Vec<RecommendedENBList_Entry>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct RecommendedENBsForPaging {
    pub recommended_enb_list: RecommendedENBList,
    #[asn(optional_idx = 0)]
    pub ie_extensions: Option<RecommendedENBsForPagingIE_Extensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "INTEGER", lb = "0", ub = "255")]
pub struct RelativeMMECapacity(u8);

#[derive(Debug, AperCodec)]
#[asn(type = "ENUMERATED", extensible = true, lb = "0", ub = "0")]
pub struct RelayNode_Indicator(u8);
impl RelayNode_Indicator {
    const TRUE: u8 = 0u8;
}

#[derive(Debug, AperCodec)]
#[asn(type = "INTEGER", lb = "0", ub = "4095")]
pub struct RepetitionPeriod(u16);

#[derive(Debug, AperCodec)]
#[asn(type = "ENUMERATED", lb = "0", ub = "7")]
pub struct ReportAmountMDT(u8);
impl ReportAmountMDT {
    const R1: u8 = 0u8;
    const R2: u8 = 1u8;
    const R4: u8 = 2u8;
    const R8: u8 = 3u8;
    const R16: u8 = 4u8;
    const R32: u8 = 5u8;
    const R64: u8 = 6u8;
    const RINFINITY: u8 = 7u8;
}

#[derive(Debug, AperCodec)]
#[asn(type = "ENUMERATED", extensible = true, lb = "0", ub = "0")]
pub struct ReportArea(u8);
impl ReportArea {
    const ECGI: u8 = 0u8;
}

#[derive(Debug, AperCodec)]
#[asn(type = "ENUMERATED", lb = "0", ub = "12")]
pub struct ReportIntervalMDT(u8);
impl ReportIntervalMDT {
    const MS120: u8 = 0u8;
    const MS240: u8 = 1u8;
    const MS480: u8 = 2u8;
    const MS640: u8 = 3u8;
    const MS1024: u8 = 4u8;
    const MS2048: u8 = 5u8;
    const MS5120: u8 = 6u8;
    const MS10240: u8 = 7u8;
    const MIN1: u8 = 8u8;
    const MIN6: u8 = 9u8;
    const MIN12: u8 = 10u8;
    const MIN30: u8 = 11u8;
    const MIN60: u8 = 12u8;
}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "128"
)]
pub struct ReportingCellList(Vec<ReportingCellList_Item>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true)]
pub struct ReportingCellList_Item {
    pub cell_id: IRAT_Cell_ID,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct RequestType {
    pub event_type: EventType,
    pub report_area: ReportArea,
    #[asn(optional_idx = 0)]
    pub ie_extensions: Option<RequestTypeIE_Extensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "ENUMERATED", extensible = true, lb = "0", ub = "0")]
pub struct RequestTypeAdditionalInfo(u8);
impl RequestTypeAdditionalInfo {
    const INCLUDE_PS_CELL: u8 = 0u8;
}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "128"
)]
pub struct RequestedCellList(Vec<IRAT_Cell_ID>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true)]
pub struct RerouteNASRequest {
    pub protocol_i_es: RerouteNASRequestProtocolIEs,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true)]
pub struct Reset {
    pub protocol_i_es: ResetProtocolIEs,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true)]
pub struct ResetAcknowledge {
    pub protocol_i_es: ResetAcknowledgeProtocolIEs,
}

#[derive(Debug, AperCodec)]
#[asn(type = "ENUMERATED", extensible = true, lb = "0", ub = "0")]
pub struct ResetAll(u8);
impl ResetAll {
    const RESET_ALL: u8 = 0u8;
}

#[derive(Debug, AperCodec)]
#[asn(type = "CHOICE", lb = "0", ub = "1", extensible = true)]
pub enum ResetType {
    #[asn(key = 0, extended = false)]
    S1_Interface(ResetAll),
    #[asn(key = 1, extended = false)]
    PartOfS1_Interface(UE_associatedLogicalS1_ConnectionListRes),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true)]
pub struct RetrieveUEInformation {
    pub protocol_i_es: RetrieveUEInformationProtocolIEs,
}

#[derive(Debug, AperCodec)]
#[asn(type = "INTEGER", lb = "0", ub = "255")]
pub struct Routing_ID(u8);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct S_TMSI {
    pub mmec: MME_Code,
    pub m_tmsi: M_TMSI,
    #[asn(optional_idx = 0)]
    pub ie_extensions: Option<S_TMSIIE_Extensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "CHOICE", lb = "0", ub = "2", extensible = true)]
pub enum S1AP_PDU {
    #[asn(key = 0, extended = false)]
    InitiatingMessage(InitiatingMessage),
    #[asn(key = 1, extended = false)]
    SuccessfulOutcome(SuccessfulOutcome),
    #[asn(key = 2, extended = false)]
    UnsuccessfulOutcome(UnsuccessfulOutcome),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true)]
pub struct S1SetupFailure {
    pub protocol_i_es: S1SetupFailureProtocolIEs,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true)]
pub struct S1SetupRequest {
    pub protocol_i_es: S1SetupRequestProtocolIEs,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true)]
pub struct S1SetupResponse {
    pub protocol_i_es: S1SetupResponseProtocolIEs,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct SONConfigurationTransfer {
    pub targete_nb_id: TargeteNB_ID,
    pub sourcee_nb_id: SourceeNB_ID,
    pub son_information: SONInformation,
    #[asn(optional_idx = 0)]
    pub ie_extensions: Option<SONConfigurationTransferIE_Extensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "CHOICE", lb = "0", ub = "1", extensible = true)]
pub enum SONInformation {
    #[asn(key = 0, extended = false)]
    SONInformationRequest(SONInformationRequest),
    #[asn(key = 1, extended = false)]
    SONInformationReply(SONInformationReply),
    #[asn(key = 0, extended = true)]
    SONInformation_Extension(SONInformation_Extension),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct SONInformation_Extension {
    #[asn(key_field = true)]
    pub id: ProtocolIE_ID,
    pub criticality: Criticality,
    pub value: SONInformation_ExtensionValue,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 2)]
pub struct SONInformationReply {
    #[asn(optional_idx = 0)]
    pub x2tnl_configuration_info: Option<X2TNLConfigurationInfo>,
    #[asn(optional_idx = 1)]
    pub ie_extensions: Option<SONInformationReplyIE_Extensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "CHOICE", lb = "0", ub = "0", extensible = true)]
pub enum SONInformationReport {
    #[asn(key = 0, extended = false)]
    RLFReportInformation(RLFReportInformation),
}

#[derive(Debug, AperCodec)]
#[asn(type = "ENUMERATED", extensible = true, lb = "0", ub = "0")]
pub struct SONInformationRequest(u8);
impl SONInformationRequest {
    const X2_TNL_CONFIGURATION_INFO: u8 = 0u8;
}

#[derive(Debug, AperCodec)]
#[asn(type = "ENUMERATED", extensible = true, lb = "0", ub = "0")]
pub struct SONtransferApplicationIdentity(u8);
impl SONtransferApplicationIdentity {
    const CELL_LOAD_REPORTING: u8 = 0u8;
}

#[derive(Debug, AperCodec)]
#[asn(type = "CHOICE", lb = "0", ub = "0", extensible = true)]
pub enum SONtransferCause {
    #[asn(key = 0, extended = false)]
    CellLoadReporting(CellLoadReportingCause),
    #[asn(key = 0, extended = true)]
    MultiCellLoadReporting(CellLoadReportingCause),
    #[asn(key = 1, extended = true)]
    EventTriggeredCellLoadReporting(CellLoadReportingCause),
    #[asn(key = 2, extended = true)]
    HOReporting(HOReportingCause),
    #[asn(key = 3, extended = true)]
    EutranCellActivation(CellActivationCause),
    #[asn(key = 4, extended = true)]
    EnergySavingsIndication(CellStateIndicationCause),
    #[asn(key = 5, extended = true)]
    FailureEventReporting(FailureEventReportingCause),
}

#[derive(Debug, AperCodec)]
#[asn(type = "CHOICE", lb = "0", ub = "0", extensible = true)]
pub enum SONtransferRequestContainer {
    #[asn(key = 0, extended = false)]
    CellLoadReporting(NULL_52),
    #[asn(key = 0, extended = true)]
    MultiCellLoadReporting(MultiCellLoadReportingRequest),
    #[asn(key = 1, extended = true)]
    EventTriggeredCellLoadReporting(EventTriggeredCellLoadReportingRequest),
    #[asn(key = 2, extended = true)]
    HOReporting(HOReport),
    #[asn(key = 3, extended = true)]
    EutranCellActivation(CellActivationRequest),
    #[asn(key = 4, extended = true)]
    EnergySavingsIndication(CellStateIndication),
    #[asn(key = 5, extended = true)]
    FailureEventReporting(FailureEventReport),
}

#[derive(Debug, AperCodec)]
#[asn(type = "CHOICE", lb = "0", ub = "0", extensible = true)]
pub enum SONtransferResponseContainer {
    #[asn(key = 0, extended = false)]
    CellLoadReporting(CellLoadReportingResponse),
    #[asn(key = 0, extended = true)]
    MultiCellLoadReporting(MultiCellLoadReportingResponse),
    #[asn(key = 1, extended = true)]
    EventTriggeredCellLoadReporting(EventTriggeredCellLoadReportingResponse),
    #[asn(key = 2, extended = true)]
    HOReporting(NULL_53),
    #[asn(key = 3, extended = true)]
    EutranCellActivation(CellActivationResponse),
    #[asn(key = 4, extended = true)]
    EnergySavingsIndication(NULL_54),
    #[asn(key = 5, extended = true)]
    FailureEventReporting(NULL_55),
}

#[derive(Debug, AperCodec)]
#[asn(type = "ENUMERATED", extensible = true, lb = "0", ub = "1")]
pub struct SRVCCHOIndication(u8);
impl SRVCCHOIndication {
    const P_SAND_CS: u8 = 0u8;
    const C_SONLY: u8 = 1u8;
}

#[derive(Debug, AperCodec)]
#[asn(type = "ENUMERATED", extensible = true, lb = "0", ub = "0")]
pub struct SRVCCOperationNotPossible(u8);
impl SRVCCOperationNotPossible {
    const NOT_POSSIBLE: u8 = 0u8;
}

#[derive(Debug, AperCodec)]
#[asn(type = "ENUMERATED", extensible = true, lb = "0", ub = "0")]
pub struct SRVCCOperationPossible(u8);
impl SRVCCOperationPossible {
    const POSSIBLE: u8 = 0u8;
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 4)]
pub struct ScheduledCommunicationTime {
    #[asn(optional_idx = 0)]
    pub dayof_week: Option<BIT_STRING_56>,
    #[asn(optional_idx = 1)]
    pub timeof_day_start: Option<INTEGER_57>,
    #[asn(optional_idx = 2)]
    pub timeof_day_end: Option<INTEGER_58>,
    #[asn(optional_idx = 3)]
    pub ie_extensions: Option<ScheduledCommunicationTimeIE_Extensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true)]
pub struct SecondaryRATDataUsageReport {
    pub protocol_i_es: SecondaryRATDataUsageReportProtocolIEs,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct SecondaryRATDataUsageReportItem {
    pub e_rab_id: E_RAB_ID,
    pub secondary_rat_type: SecondaryRATType,
    pub e_rab_usage_report_list: E_RABUsageReportList,
    #[asn(optional_idx = 0)]
    pub ie_extensions: Option<SecondaryRATDataUsageReportItemIE_Extensions>,
}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "256"
)]
pub struct SecondaryRATDataUsageReportList(Vec<SecondaryRATDataUsageReportList_Entry>);

#[derive(Debug, AperCodec)]
#[asn(type = "ENUMERATED", extensible = true, lb = "0", ub = "0")]
pub struct SecondaryRATDataUsageRequest(u8);
impl SecondaryRATDataUsageRequest {
    const REQUESTED: u8 = 0u8;
}

#[derive(Debug, AperCodec)]
#[asn(type = "ENUMERATED", extensible = true, lb = "0", ub = "0")]
pub struct SecondaryRATType(u8);
impl SecondaryRATType {
    const N_R: u8 = 0u8;
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct SecurityContext {
    pub next_hop_chaining_count: INTEGER_59,
    pub next_hop_parameter: SecurityKey,
    #[asn(optional_idx = 0)]
    pub ie_extensions: Option<SecurityContextIE_Extensions>,
}

#[derive(Debug, AperCodec)]
#[asn(
    type = "BITSTRING",
    sz_extensible = false,
    sz_lb = "256",
    sz_ub = "256"
)]
pub struct SecurityKey(BitVec<Msb0, u8>);

#[derive(Debug, AperCodec)]
#[asn(type = "BITSTRING", sz_extensible = false, sz_lb = "16", sz_ub = "16")]
pub struct SerialNumber(BitVec<Msb0, u8>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "0", sz_ub = "32")]
pub struct ServedDCNs(Vec<ServedDCNsItem>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct ServedDCNsItem {
    pub dcn_id: DCN_ID,
    pub relative_dcn_capacity: RelativeMMECapacity,
    #[asn(optional_idx = 0)]
    pub ie_extensions: Option<ServedDCNsItemIE_Extensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "8")]
pub struct ServedGUMMEIs(Vec<ServedGUMMEIsItem>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct ServedGUMMEIsItem {
    pub served_plm_ns: ServedPLMNs,
    pub served_group_i_ds: ServedGroupIDs,
    pub served_mme_cs: ServedMMECs,
    #[asn(optional_idx = 0)]
    pub ie_extensions: Option<ServedGUMMEIsItemIE_Extensions>,
}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct ServedGroupIDs(Vec<MME_Group_ID>);

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "256"
)]
pub struct ServedMMECs(Vec<MME_Code>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "32")]
pub struct ServedPLMNs(Vec<PLMNidentity>);

#[derive(Debug, AperCodec)]
#[asn(type = "ENUMERATED", extensible = true, lb = "0", ub = "1")]
pub struct ServiceType(u8);
impl ServiceType {
    const Q_MC_FOR_STREAMING_SERVICE: u8 = 0u8;
    const Q_MC_FOR_MTSI_SERVICE: u8 = 1u8;
}

#[derive(Debug, AperCodec)]
#[asn(type = "OCTET-STRING")]
pub struct Source_ToTarget_TransparentContainer(Vec<u8>);

#[derive(Debug, AperCodec)]
#[asn(type = "OCTET-STRING")]
pub struct SourceBSS_ToTargetBSS_TransparentContainer(Vec<u8>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct SourceNgRanNode_ID {
    pub global_ran_node_id: Global_RAN_NODE_ID,
    pub selected_tai: FiveGSTAI,
    #[asn(optional_idx = 0)]
    pub ie_extensions: Option<SourceNgRanNode_IDIE_Extensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "OCTET-STRING")]
pub struct SourceNgRanNode_ToTargetNgRanNode_TransparentContainer(Vec<u8>);

#[derive(Debug, AperCodec)]
#[asn(type = "CHOICE", lb = "0", ub = "1", extensible = false)]
pub enum SourceNodeID {
    #[asn(key = 0, extended = false)]
    SourceNgRanNode_ID(SourceNgRanNode_ID),
    #[asn(key = 1, extended = false)]
    SourceNodeID_Extension(SourceNodeID_Extension),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct SourceNodeID_Extension {}

#[derive(Debug, AperCodec)]
#[asn(type = "ENUMERATED", extensible = true, lb = "0", ub = "1")]
pub struct SourceOfUEActivityBehaviourInformation(u8);
impl SourceOfUEActivityBehaviourInformation {
    const SUBSCRIPTION_INFORMATION: u8 = 0u8;
    const STATISTICS: u8 = 1u8;
}

#[derive(Debug, AperCodec)]
#[asn(type = "OCTET-STRING")]
pub struct SourceRNC_ToTargetRNC_TransparentContainer(Vec<u8>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false, optional_fields = 1)]
pub struct SourceeNB_ID {
    pub global_enb_id: Global_ENB_ID,
    pub selected_tai: TAI,
    #[asn(optional_idx = 0)]
    pub ie_extensions: Option<SourceeNB_IDIE_Extensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 3)]
pub struct SourceeNB_ToTargeteNB_TransparentContainer {
    pub rrc_container: RRC_Container,
    #[asn(optional_idx = 0)]
    pub e_rab_information_list: Option<E_RABInformationList>,
    pub target_cell_id: EUTRAN_CGI,
    #[asn(optional_idx = 1)]
    pub subscriber_profile_i_dfor_rfp: Option<SubscriberProfileIDforRFP>,
    pub ue_history_information: UE_HistoryInformation,
    #[asn(optional_idx = 2)]
    pub ie_extensions: Option<SourceeNB_ToTargeteNB_TransparentContainerIE_Extensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "INTEGER", lb = "0", ub = "3", extensible = true)]
pub struct StratumLevel(u8);

#[derive(Debug, AperCodec)]
#[asn(type = "INTEGER", lb = "1", ub = "256")]
pub struct SubscriberProfileIDforRFP(u16);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 7)]
pub struct Subscription_Based_UE_DifferentiationInfo {
    #[asn(optional_idx = 0)]
    pub periodic_communication_indicator: Option<ENUMERATED_60>,
    #[asn(optional_idx = 1)]
    pub periodic_time: Option<INTEGER_61>,
    #[asn(optional_idx = 2)]
    pub scheduled_communication_time: Option<ScheduledCommunicationTime>,
    #[asn(optional_idx = 3)]
    pub stationary_indication: Option<ENUMERATED_62>,
    #[asn(optional_idx = 4)]
    pub traffic_profile: Option<ENUMERATED_63>,
    #[asn(optional_idx = 5)]
    pub battery_indication: Option<ENUMERATED_64>,
    #[asn(optional_idx = 6)]
    pub ie_extensions: Option<Subscription_Based_UE_DifferentiationInfoIE_Extensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct SuccessfulOutcome {
    #[asn(key_field = true)]
    pub procedure_code: ProcedureCode,
    pub criticality: Criticality,
    pub value: SuccessfulOutcomeValue,
}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "256"
)]
pub struct SupportedTAs(Vec<SupportedTAs_Item>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct SupportedTAs_Item {
    pub tac: TAC,
    pub broadcast_plm_ns: BPLMNs,
    #[asn(optional_idx = 0)]
    pub ie_extensions: Option<SupportedTAs_ItemIE_Extensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 4)]
pub struct SynchronisationInformation {
    #[asn(optional_idx = 0)]
    pub source_stratum_level: Option<StratumLevel>,
    #[asn(optional_idx = 1)]
    pub listening_subframe_pattern: Option<ListeningSubframePattern>,
    #[asn(optional_idx = 2)]
    pub aggressore_cgi_list: Option<ECGI_List>,
    #[asn(optional_idx = 3)]
    pub ie_extensions: Option<SynchronisationInformationIE_Extensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "ENUMERATED", extensible = true, lb = "0", ub = "1")]
pub struct SynchronisationStatus(u8);
impl SynchronisationStatus {
    const SYNCHRONOUS: u8 = 0u8;
    const ASYNCHRONOUS: u8 = 1u8;
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct TABasedMDT {
    pub ta_listfor_mdt: TAListforMDT,
    #[asn(optional_idx = 0)]
    pub ie_extensions: Option<TABasedMDTIE_Extensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct TABasedQMC {
    pub ta_listfor_qmc: TAListforQMC,
    #[asn(optional_idx = 0)]
    pub ie_extensions: Option<TABasedQMCIE_Extensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "OCTET-STRING", sz_extensible = false, sz_lb = "2", sz_ub = "2")]
pub struct TAC(Vec<u8>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct TAI {
    pub plm_nidentity: PLMNidentity,
    pub tac: TAC,
    #[asn(optional_idx = 0)]
    pub ie_extensions: Option<TAIIE_Extensions>,
}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct TAI_Broadcast(Vec<TAI_Broadcast_Item>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct TAI_Broadcast_Item {
    pub tai: TAI,
    pub completed_cellin_tai: CompletedCellinTAI,
    #[asn(optional_idx = 0)]
    pub ie_extensions: Option<TAI_Broadcast_ItemIE_Extensions>,
}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct TAI_Cancelled(Vec<TAI_Cancelled_Item>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct TAI_Cancelled_Item {
    pub tai: TAI,
    pub cancelled_cellin_tai: CancelledCellinTAI,
    #[asn(optional_idx = 0)]
    pub ie_extensions: Option<TAI_Cancelled_ItemIE_Extensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct TAIBasedMDT {
    pub tai_listfor_mdt: TAIListforMDT,
    #[asn(optional_idx = 0)]
    pub ie_extensions: Option<TAIBasedMDTIE_Extensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct TAIBasedQMC {
    pub tai_listfor_qmc: TAIListforQMC,
    #[asn(optional_idx = 0)]
    pub ie_extensions: Option<TAIBasedQMCIE_Extensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct TAIItem {
    pub tai: TAI,
    #[asn(optional_idx = 0)]
    pub ie_extensions: Option<TAIItemIE_Extensions>,
}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "256"
)]
pub struct TAIList(Vec<TAIList_Entry>);

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "2048"
)]
pub struct TAIListForRestart(Vec<TAI>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "8")]
pub struct TAIListforMDT(Vec<TAI>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "8")]
pub struct TAIListforQMC(Vec<TAI>);

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct TAIListforWarning(Vec<TAI>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "8")]
pub struct TAListforMDT(Vec<TAC>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "8")]
pub struct TAListforQMC(Vec<TAC>);

#[derive(Debug, AperCodec)]
#[asn(type = "OCTET-STRING", sz_extensible = false, sz_lb = "3", sz_ub = "3")]
pub struct TBCD_STRING(Vec<u8>);

#[derive(Debug, AperCodec)]
#[asn(type = "OCTET-STRING")]
pub struct Target_ToSource_TransparentContainer(Vec<u8>);

#[derive(Debug, AperCodec)]
#[asn(type = "OCTET-STRING")]
pub struct TargetBSS_ToSourceBSS_TransparentContainer(Vec<u8>);

#[derive(Debug, AperCodec)]
#[asn(type = "CHOICE", lb = "0", ub = "2", extensible = true)]
pub enum TargetID {
    #[asn(key = 0, extended = false)]
    TargeteNB_ID(TargeteNB_ID),
    #[asn(key = 1, extended = false)]
    TargetRNC_ID(TargetRNC_ID),
    #[asn(key = 2, extended = false)]
    CGI(CGI),
    #[asn(key = 0, extended = true)]
    TargetgNgRanNode_ID(TargetNgRanNode_ID),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct TargetNgRanNode_ID {
    pub global_ran_node_id: Global_RAN_NODE_ID,
    pub selected_tai: FiveGSTAI,
    #[asn(optional_idx = 0)]
    pub ie_extensions: Option<TargetNgRanNode_IDIE_Extensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "OCTET-STRING")]
pub struct TargetNgRanNode_ToSourceNgRanNode_TransparentContainer(Vec<u8>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 3)]
pub struct TargetRNC_ID {
    pub lai: LAI,
    #[asn(optional_idx = 0)]
    pub rac: Option<RAC>,
    pub rnc_id: RNC_ID,
    #[asn(optional_idx = 1)]
    pub extended_rnc_id: Option<ExtendedRNC_ID>,
    #[asn(optional_idx = 2)]
    pub ie_extensions: Option<TargetRNC_IDIE_Extensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "OCTET-STRING")]
pub struct TargetRNC_ToSourceRNC_TransparentContainer(Vec<u8>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct TargeteNB_ID {
    pub global_enb_id: Global_ENB_ID,
    pub selected_tai: TAI,
    #[asn(optional_idx = 0)]
    pub ie_extensions: Option<TargeteNB_IDIE_Extensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct TargeteNB_ToSourceeNB_TransparentContainer {
    pub rrc_container: RRC_Container,
    #[asn(optional_idx = 0)]
    pub ie_extensions: Option<TargeteNB_ToSourceeNB_TransparentContainerIE_Extensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "INTEGER", lb = "0", ub = "97")]
pub struct Threshold_RSRP(u8);

#[derive(Debug, AperCodec)]
#[asn(type = "INTEGER", lb = "0", ub = "34")]
pub struct Threshold_RSRQ(u8);

#[derive(Debug, AperCodec)]
#[asn(type = "INTEGER", lb = "0", ub = "4095")]
pub struct Time_UE_StayedInCell(u16);

#[derive(Debug, AperCodec)]
#[asn(type = "INTEGER", lb = "0", ub = "40950")]
pub struct Time_UE_StayedInCell_EnhancedGranularity(u16);

#[derive(Debug, AperCodec)]
#[asn(type = "OCTET-STRING", sz_extensible = false, sz_lb = "4", sz_ub = "4")]
pub struct TimeSinceSecondaryNodeRelease(Vec<u8>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct TimeSynchronisationInfo {
    pub stratum_level: StratumLevel,
    pub synchronisation_status: SynchronisationStatus,
    #[asn(optional_idx = 0)]
    pub ie_extensions: Option<TimeSynchronisationInfoIE_Extensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "ENUMERATED", extensible = true, lb = "0", ub = "5")]
pub struct TimeToWait(u8);
impl TimeToWait {
    const V1S: u8 = 0u8;
    const V2S: u8 = 1u8;
    const V5S: u8 = 2u8;
    const V10S: u8 = 3u8;
    const V20S: u8 = 4u8;
    const V60S: u8 = 5u8;
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct TooEarlyInterRATHOReportReportFromEUTRAN {
    pub uerlf_report_container: OCTET_STRING_65,
    #[asn(optional_idx = 0)]
    pub mobility_information: Option<MobilityInformation>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct TraceActivation {
    pub e_utran_trace_id: E_UTRAN_Trace_ID,
    pub interfaces_to_trace: InterfacesToTrace,
    pub trace_depth: TraceDepth,
    pub trace_collection_entity_ip_address: TransportLayerAddress,
    #[asn(optional_idx = 0)]
    pub ie_extensions: Option<TraceActivationIE_Extensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "ENUMERATED", extensible = true, lb = "0", ub = "5")]
pub struct TraceDepth(u8);
impl TraceDepth {
    const MINIMUM: u8 = 0u8;
    const MEDIUM: u8 = 1u8;
    const MAXIMUM: u8 = 2u8;
    const MINIMUM_WITHOUT_VENDOR_SPECIFIC_EXTENSION: u8 = 3u8;
    const MEDIUM_WITHOUT_VENDOR_SPECIFIC_EXTENSION: u8 = 4u8;
    const MAXIMUM_WITHOUT_VENDOR_SPECIFIC_EXTENSION: u8 = 5u8;
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true)]
pub struct TraceFailureIndication {
    pub protocol_i_es: TraceFailureIndicationProtocolIEs,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true)]
pub struct TraceStart {
    pub protocol_i_es: TraceStartProtocolIEs,
}

#[derive(Debug, AperCodec)]
#[asn(type = "INTEGER", lb = "1", ub = "99")]
pub struct TrafficLoadReductionIndication(u8);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true)]
pub struct TransportInformation {
    pub transport_layer_address: TransportLayerAddress,
    pub ul_gtp_teid: GTP_TEID,
}

#[derive(Debug, AperCodec)]
#[asn(type = "BITSTRING", sz_extensible = true, sz_lb = "1", sz_ub = "160")]
pub struct TransportLayerAddress(BitVec<Msb0, u8>);

#[derive(Debug, AperCodec)]
#[asn(type = "ENUMERATED", lb = "0", ub = "2")]
pub struct TriggeringMessage(u8);
impl TriggeringMessage {
    const INITIATING_MESSAGE: u8 = 0u8;
    const SUCCESSFUL_OUTCOME: u8 = 1u8;
    const UNSUCCESSFULL_OUTCOME: u8 = 2u8;
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
#[asn(type = "BITSTRING", sz_extensible = false, sz_lb = "8", sz_ub = "8")]
pub struct UE_Application_Layer_Measurement_Capability(BitVec<Msb0, u8>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "16")]
pub struct UE_HistoryInformation(Vec<LastVisitedCell_Item>);

#[derive(Debug, AperCodec)]
#[asn(type = "OCTET-STRING")]
pub struct UE_HistoryInformationFromTheUE(Vec<u8>);

#[derive(Debug, AperCodec)]
#[asn(type = "OCTET-STRING")]
pub struct UE_RLF_Report_Container(Vec<u8>);

#[derive(Debug, AperCodec)]
#[asn(type = "OCTET-STRING")]
pub struct UE_RLF_Report_Container_for_extended_bands(Vec<u8>);

#[derive(Debug, AperCodec)]
#[asn(type = "ENUMERATED", extensible = true, lb = "0", ub = "0")]
pub struct UE_RetentionInformation(u8);
impl UE_RetentionInformation {
    const UES_RETAINED: u8 = 0u8;
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct UE_S1AP_ID_pair {
    pub mme_ue_s1ap_id: MME_UE_S1AP_ID,
    pub enb_ue_s1ap_id: ENB_UE_S1AP_ID,
    #[asn(optional_idx = 0)]
    pub ie_extensions: Option<UE_S1AP_ID_pairIE_Extensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "CHOICE", lb = "0", ub = "1", extensible = true)]
pub enum UE_S1AP_IDs {
    #[asn(key = 0, extended = false)]
    UE_S1AP_ID_pair(UE_S1AP_ID_pair),
    #[asn(key = 1, extended = false)]
    MME_UE_S1AP_ID(MME_UE_S1AP_ID),
}

#[derive(Debug, AperCodec)]
#[asn(type = "INTEGER", lb = "0", ub = "255")]
pub struct UE_Usage_Type(u8);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 3)]
pub struct UE_associatedLogicalS1_ConnectionItem {
    #[asn(optional_idx = 0)]
    pub mme_ue_s1ap_id: Option<MME_UE_S1AP_ID>,
    #[asn(optional_idx = 1)]
    pub enb_ue_s1ap_id: Option<ENB_UE_S1AP_ID>,
    #[asn(optional_idx = 2)]
    pub ie_extensions: Option<UE_associatedLogicalS1_ConnectionItemIE_Extensions>,
}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "256"
)]
pub struct UE_associatedLogicalS1_ConnectionListRes(
    Vec<UE_associatedLogicalS1_ConnectionListRes_Entry>,
);

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "256"
)]
pub struct UE_associatedLogicalS1_ConnectionListResAck(
    Vec<UE_associatedLogicalS1_ConnectionListResAck_Entry>,
);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct UEAggregateMaximumBitrate {
    pub u_eaggregate_maximum_bit_rate_dl: BitRate,
    pub u_eaggregate_maximum_bit_rate_ul: BitRate,
    #[asn(optional_idx = 0)]
    pub ie_extensions: Option<UEAggregateMaximumBitrateIE_Extensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct UEAppLayerMeasConfig {
    pub container_for_app_layer_meas_config: OCTET_STRING_66,
    pub area_scope_of_qmc: AreaScopeOfQMC,
    #[asn(optional_idx = 0)]
    pub ie_extensions: Option<UEAppLayerMeasConfigIE_Extensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true)]
pub struct UECapabilityInfoIndication {
    pub protocol_i_es: UECapabilityInfoIndicationProtocolIEs,
}

#[derive(Debug, AperCodec)]
#[asn(type = "ENUMERATED", extensible = true, lb = "0", ub = "0")]
pub struct UECapabilityInfoRequest(u8);
impl UECapabilityInfoRequest {
    const REQUESTED: u8 = 0u8;
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true)]
pub struct UEContextModificationConfirm {
    pub protocol_i_es: UEContextModificationConfirmProtocolIEs,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true)]
pub struct UEContextModificationFailure {
    pub protocol_i_es: UEContextModificationFailureProtocolIEs,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true)]
pub struct UEContextModificationIndication {
    pub protocol_i_es: UEContextModificationIndicationProtocolIEs,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true)]
pub struct UEContextModificationRequest {
    pub protocol_i_es: UEContextModificationRequestProtocolIEs,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true)]
pub struct UEContextModificationResponse {
    pub protocol_i_es: UEContextModificationResponseProtocolIEs,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true)]
pub struct UEContextReleaseCommand {
    pub protocol_i_es: UEContextReleaseCommandProtocolIEs,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true)]
pub struct UEContextReleaseComplete {
    pub protocol_i_es: UEContextReleaseCompleteProtocolIEs,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true)]
pub struct UEContextReleaseRequest {
    pub protocol_i_es: UEContextReleaseRequestProtocolIEs,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true)]
pub struct UEContextResumeFailure {
    pub protocol_i_es: UEContextResumeFailureProtocolIEs,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true)]
pub struct UEContextResumeRequest {
    pub protocol_i_es: UEContextResumeRequestProtocolIEs,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true)]
pub struct UEContextResumeResponse {
    pub protocol_i_es: UEContextResumeResponseProtocolIEs,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true)]
pub struct UEContextSuspendRequest {
    pub protocol_i_es: UEContextSuspendRequestProtocolIEs,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true)]
pub struct UEContextSuspendResponse {
    pub protocol_i_es: UEContextSuspendResponseProtocolIEs,
}

#[derive(Debug, AperCodec)]
#[asn(type = "BITSTRING", sz_extensible = false, sz_lb = "10", sz_ub = "10")]
pub struct UEIdentityIndexValue(BitVec<Msb0, u8>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true)]
pub struct UEInformationTransfer {
    pub protocol_i_es: UEInformationTransferProtocolIEs,
}

#[derive(Debug, AperCodec)]
#[asn(type = "CHOICE", lb = "0", ub = "1", extensible = true)]
pub enum UEPagingID {
    #[asn(key = 0, extended = false)]
    S_TMSI(S_TMSI),
    #[asn(key = 1, extended = false)]
    IMSI(IMSI),
}

#[derive(Debug, AperCodec)]
#[asn(type = "OCTET-STRING")]
pub struct UERadioCapability(Vec<u8>);

#[derive(Debug, AperCodec)]
#[asn(type = "OCTET-STRING")]
pub struct UERadioCapabilityForPaging(Vec<u8>);

#[derive(Debug, AperCodec)]
#[asn(type = "OCTET-STRING")]
pub struct UERadioCapabilityID(Vec<u8>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true)]
pub struct UERadioCapabilityIDMappingRequest {
    pub protocol_i_es: UERadioCapabilityIDMappingRequestProtocolIEs,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true)]
pub struct UERadioCapabilityIDMappingResponse {
    pub protocol_i_es: UERadioCapabilityIDMappingResponseProtocolIEs,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true)]
pub struct UERadioCapabilityMatchRequest {
    pub protocol_i_es: UERadioCapabilityMatchRequestProtocolIEs,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true)]
pub struct UERadioCapabilityMatchResponse {
    pub protocol_i_es: UERadioCapabilityMatchResponseProtocolIEs,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct UESecurityCapabilities {
    pub encryption_algorithms: EncryptionAlgorithms,
    pub integrity_protection_algorithms: IntegrityProtectionAlgorithms,
    #[asn(optional_idx = 0)]
    pub ie_extensions: Option<UESecurityCapabilitiesIE_Extensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct UESidelinkAggregateMaximumBitrate {
    pub ue_sidelink_aggregate_maximum_bit_rate: BitRate,
    #[asn(optional_idx = 0)]
    pub ie_extensions: Option<UESidelinkAggregateMaximumBitrateIE_Extensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "ENUMERATED", extensible = true, lb = "0", ub = "0")]
pub struct UEUserPlaneCIoTSupportIndicator(u8);
impl UEUserPlaneCIoTSupportIndicator {
    const SUPPORTED: u8 = 0u8;
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct UL_CP_SecurityInformation {
    pub ul_nas_mac: UL_NAS_MAC,
    pub ul_nas_count: UL_NAS_Count,
    #[asn(optional_idx = 0)]
    pub ie_extensions: Option<UL_CP_SecurityInformationIE_Extensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "BITSTRING", sz_extensible = false, sz_lb = "5", sz_ub = "5")]
pub struct UL_NAS_Count(BitVec<Msb0, u8>);

#[derive(Debug, AperCodec)]
#[asn(type = "BITSTRING", sz_extensible = false, sz_lb = "16", sz_ub = "16")]
pub struct UL_NAS_MAC(BitVec<Msb0, u8>);

#[derive(Debug, AperCodec)]
#[asn(type = "VisibleString")]
pub struct URI_Address(String);

#[derive(Debug, AperCodec)]
#[asn(type = "ENUMERATED", extensible = true, lb = "0", ub = "0")]
pub struct UnlicensedSpectrumRestriction(u8);
impl UnlicensedSpectrumRestriction {
    const UNLICENSED_RESTRICTED: u8 = 0u8;
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct UnsuccessfulOutcome {
    #[asn(key_field = true)]
    pub procedure_code: ProcedureCode,
    pub criticality: Criticality,
    pub value: UnsuccessfulOutcomeValue,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true)]
pub struct UplinkNASTransport {
    pub protocol_i_es: UplinkNASTransportProtocolIEs,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true)]
pub struct UplinkNonUEAssociatedLPPaTransport {
    pub protocol_i_es: UplinkNonUEAssociatedLPPaTransportProtocolIEs,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true)]
pub struct UplinkS1cdma2000tunnelling {
    pub protocol_i_es: UplinkS1cdma2000tunnellingProtocolIEs,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true)]
pub struct UplinkUEAssociatedLPPaTransport {
    pub protocol_i_es: UplinkUEAssociatedLPPaTransportProtocolIEs,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct UserLocationInformation {
    pub eutran_cgi: EUTRAN_CGI,
    pub tai: TAI,
    #[asn(optional_idx = 0)]
    pub ie_extensions: Option<UserLocationInformationIE_Extensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 3)]
pub struct V2XServicesAuthorized {
    #[asn(optional_idx = 0)]
    pub vehicle_ue: Option<VehicleUE>,
    #[asn(optional_idx = 1)]
    pub pedestrian_ue: Option<PedestrianUE>,
    #[asn(optional_idx = 2)]
    pub ie_extensions: Option<V2XServicesAuthorizedIE_Extensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "ENUMERATED", extensible = true, lb = "0", ub = "1")]
pub struct VehicleUE(u8);
impl VehicleUE {
    const AUTHORIZED: u8 = 0u8;
    const NOT_AUTHORIZED: u8 = 1u8;
}

#[derive(Debug, AperCodec)]
#[asn(type = "ENUMERATED", extensible = true, lb = "0", ub = "1")]
pub struct VoiceSupportMatchIndicator(u8);
impl VoiceSupportMatchIndicator {
    const SUPPORTED: u8 = 0u8;
    const NOT_SUPPORTED: u8 = 1u8;
}

#[derive(Debug, AperCodec)]
#[asn(type = "ENUMERATED", extensible = true, lb = "0", ub = "0")]
pub struct WLANMeasConfig(u8);
impl WLANMeasConfig {
    const SETUP: u8 = 0u8;
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "4")]
pub struct WLANMeasConfigNameList(Vec<WLANName>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 4)]
pub struct WLANMeasurementConfiguration {
    pub wlan_meas_config: WLANMeasConfig,
    #[asn(optional_idx = 0)]
    pub wlan_meas_config_name_list: Option<WLANMeasConfigNameList>,
    #[asn(optional_idx = 1)]
    pub wlan_rssi: Option<ENUMERATED_67>,
    #[asn(optional_idx = 2)]
    pub wlan_rtt: Option<ENUMERATED_68>,
    #[asn(optional_idx = 3)]
    pub ie_extensions: Option<WLANMeasurementConfigurationIE_Extensions>,
}

#[derive(Debug, AperCodec)]
#[asn(
    type = "OCTET-STRING",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "32"
)]
pub struct WLANName(Vec<u8>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct WUS_Assistance_Information {
    pub paging_probability_information: PagingProbabilityInformation,
    #[asn(optional_idx = 0)]
    pub ie_extensions: Option<WUS_Assistance_InformationIE_Extensions>,
}

#[derive(Debug, AperCodec)]
#[asn(
    type = "OCTET-STRING",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "1024"
)]
pub struct WarningAreaCoordinates(Vec<u8>);

#[derive(Debug, AperCodec)]
#[asn(type = "CHOICE", lb = "0", ub = "2", extensible = true)]
pub enum WarningAreaList {
    #[asn(key = 0, extended = false)]
    CellIDList(ECGIList),
    #[asn(key = 1, extended = false)]
    TrackingAreaListforWarning(TAIListforWarning),
    #[asn(key = 2, extended = false)]
    EmergencyAreaIDList(EmergencyAreaIDList),
}

#[derive(Debug, AperCodec)]
#[asn(
    type = "OCTET-STRING",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "9600"
)]
pub struct WarningMessageContents(Vec<u8>);

#[derive(Debug, AperCodec)]
#[asn(
    type = "OCTET-STRING",
    sz_extensible = false,
    sz_lb = "50",
    sz_ub = "50"
)]
pub struct WarningSecurityInfo(Vec<u8>);

#[derive(Debug, AperCodec)]
#[asn(type = "OCTET-STRING", sz_extensible = false, sz_lb = "2", sz_ub = "2")]
pub struct WarningType(Vec<u8>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true)]
pub struct WriteReplaceWarningRequest {
    pub protocol_i_es: WriteReplaceWarningRequestProtocolIEs,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true)]
pub struct WriteReplaceWarningResponse {
    pub protocol_i_es: WriteReplaceWarningResponseProtocolIEs,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct X2TNLConfigurationInfo {
    pub enbx2_transport_layer_addresses: ENBX2TLAs,
    #[asn(optional_idx = 0)]
    pub ie_extensions: Option<X2TNLConfigurationInfoIE_Extensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "OCTET-STRING")]
pub struct OCTET_STRING_2(Vec<u8>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct Additional_GUTIIE_Extensions_Entry {}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct Additional_GUTIIE_Extensions(Vec<Additional_GUTIIE_Extensions_Entry>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct AllocationAndRetentionPriorityIE_Extensions_Entry {}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct AllocationAndRetentionPriorityIE_Extensions(
    Vec<AllocationAndRetentionPriorityIE_Extensions_Entry>,
);

#[derive(Debug, AperCodec)]
#[asn(type = "NULL")]
pub struct NULL_3;

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct AssistanceDataForCECapableUEsIE_Extensions_Entry {}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct AssistanceDataForCECapableUEsIE_Extensions(
    Vec<AssistanceDataForCECapableUEsIE_Extensions_Entry>,
);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct AssistanceDataForPagingIE_Extensions_Entry {}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct AssistanceDataForPagingIE_Extensions(Vec<AssistanceDataForPagingIE_Extensions_Entry>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct AssistanceDataForRecommendedCellsIE_Extensions_Entry {}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct AssistanceDataForRecommendedCellsIE_Extensions(
    Vec<AssistanceDataForRecommendedCellsIE_Extensions_Entry>,
);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct Bearers_SubjectToEarlyStatusTransfer_ItemIE_Extensions_Entry {}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct Bearers_SubjectToEarlyStatusTransfer_ItemIE_Extensions(
    Vec<Bearers_SubjectToEarlyStatusTransfer_ItemIE_Extensions_Entry>,
);

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum Bearers_SubjectToEarlyStatusTransferList_EntryValue {
    #[asn(key = 322)]
    Id_Bearers_SubjectToEarlyStatusTransfer_Item(Bearers_SubjectToEarlyStatusTransfer_Item),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct Bearers_SubjectToEarlyStatusTransferList_Entry {
    #[asn(key_field = true)]
    pub id: ProtocolIE_ID,
    pub criticality: Criticality,
    pub value: Bearers_SubjectToEarlyStatusTransferList_EntryValue,
}

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum Bearers_SubjectToStatusTransfer_ItemIE_Extensions_EntryExtensionValue {
    #[asn(key = 180)]
    Id_DLCOUNTValueExtended(COUNTValueExtended),
    #[asn(key = 218)]
    Id_DLCOUNTValuePDCP_SNlength18(COUNTvaluePDCP_SNlength18),
    #[asn(key = 181)]
    Id_ReceiveStatusOfULPDCPSDUsExtended(ReceiveStatusOfULPDCPSDUsExtended),
    #[asn(key = 219)]
    Id_ReceiveStatusOfULPDCPSDUsPDCP_SNlength18(ReceiveStatusOfULPDCPSDUsPDCP_SNlength18),
    #[asn(key = 179)]
    Id_ULCOUNTValueExtended(COUNTValueExtended),
    #[asn(key = 217)]
    Id_ULCOUNTValuePDCP_SNlength18(COUNTvaluePDCP_SNlength18),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct Bearers_SubjectToStatusTransfer_ItemIE_Extensions_Entry {
    #[asn(key_field = true)]
    pub id: ProtocolExtensionID,
    pub criticality: Criticality,
    pub extension_value: Bearers_SubjectToStatusTransfer_ItemIE_Extensions_EntryExtensionValue,
}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct Bearers_SubjectToStatusTransfer_ItemIE_Extensions(
    Vec<Bearers_SubjectToStatusTransfer_ItemIE_Extensions_Entry>,
);

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum Bearers_SubjectToStatusTransferList_EntryValue {
    #[asn(key = 89)]
    Id_Bearers_SubjectToStatusTransfer_Item(Bearers_SubjectToStatusTransfer_Item),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct Bearers_SubjectToStatusTransferList_Entry {
    #[asn(key_field = true)]
    pub id: ProtocolIE_ID,
    pub criticality: Criticality,
    pub value: Bearers_SubjectToStatusTransferList_EntryValue,
}

#[derive(Debug, AperCodec)]
#[asn(type = "ENUMERATED", extensible = true, lb = "0", ub = "0")]
pub struct ENUMERATED_4(u8);
impl ENUMERATED_4 {
    const TRUE: u8 = 0u8;
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct BluetoothMeasurementConfigurationIE_Extensions_Entry {}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct BluetoothMeasurementConfigurationIE_Extensions(
    Vec<BluetoothMeasurementConfigurationIE_Extensions_Entry>,
);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct CGIIE_Extensions_Entry {}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct CGIIE_Extensions(Vec<CGIIE_Extensions_Entry>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct CNTypeRestrictions_ItemIE_Extensions_Entry {}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct CNTypeRestrictions_ItemIE_Extensions(Vec<CNTypeRestrictions_ItemIE_Extensions_Entry>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct COUNTValueExtendedIE_Extensions_Entry {}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct COUNTValueExtendedIE_Extensions(Vec<COUNTValueExtendedIE_Extensions_Entry>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct COUNTvalueIE_Extensions_Entry {}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct COUNTvalueIE_Extensions(Vec<COUNTvalueIE_Extensions_Entry>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct COUNTvaluePDCP_SNlength18IE_Extensions_Entry {}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct COUNTvaluePDCP_SNlength18IE_Extensions(
    Vec<COUNTvaluePDCP_SNlength18IE_Extensions_Entry>,
);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct CSG_IdList_ItemIE_Extensions_Entry {}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct CSG_IdList_ItemIE_Extensions(Vec<CSG_IdList_ItemIE_Extensions_Entry>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct CSGMembershipInfoIE_Extensions_Entry {}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct CSGMembershipInfoIE_Extensions(Vec<CSGMembershipInfoIE_Extensions_Entry>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct CancelledCellinEAI_ItemIE_Extensions_Entry {}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct CancelledCellinEAI_ItemIE_Extensions(Vec<CancelledCellinEAI_ItemIE_Extensions_Entry>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct CancelledCellinTAI_ItemIE_Extensions_Entry {}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct CancelledCellinTAI_ItemIE_Extensions(Vec<CancelledCellinTAI_ItemIE_Extensions_Entry>);

#[derive(Debug, AperCodec)]
#[asn(type = "INTEGER", lb = "0", ub = "503")]
pub struct INTEGER_5(u16);

#[derive(Debug, AperCodec)]
#[asn(type = "OCTET-STRING")]
pub struct OCTET_STRING_6(Vec<u8>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct Cdma2000OneXSRVCCInfoIE_Extensions_Entry {}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct Cdma2000OneXSRVCCInfoIE_Extensions(Vec<Cdma2000OneXSRVCCInfoIE_Extensions_Entry>);

#[derive(Debug, AperCodec)]
#[asn(type = "INTEGER", lb = "1", ub = "60")]
pub struct INTEGER_7(u8);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct CellBasedMDTIE_Extensions_Entry {}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct CellBasedMDTIE_Extensions(Vec<CellBasedMDTIE_Extensions_Entry>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct CellBasedQMCIE_Extensions_Entry {}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct CellBasedQMCIE_Extensions(Vec<CellBasedQMCIE_Extensions_Entry>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct CellID_Broadcast_ItemIE_Extensions_Entry {}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct CellID_Broadcast_ItemIE_Extensions(Vec<CellID_Broadcast_ItemIE_Extensions_Entry>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct CellID_Cancelled_ItemIE_Extensions_Entry {}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct CellID_Cancelled_ItemIE_Extensions(Vec<CellID_Cancelled_ItemIE_Extensions_Entry>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct CellIdentifierAndCELevelForCECapableUEsIE_Extensions_Entry {}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct CellIdentifierAndCELevelForCECapableUEsIE_Extensions(
    Vec<CellIdentifierAndCELevelForCECapableUEsIE_Extensions_Entry>,
);

#[derive(Debug, AperCodec)]
#[asn(type = "OCTET-STRING")]
pub struct OCTET_STRING_8(Vec<u8>);

#[derive(Debug, AperCodec)]
#[asn(type = "OCTET-STRING")]
pub struct OCTET_STRING_9(Vec<u8>);

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum CellTrafficTraceProtocolIEs_EntryValue {
    #[asn(key = 86)]
    Id_E_UTRAN_Trace_ID(E_UTRAN_Trace_ID),
    #[asn(key = 100)]
    Id_EUTRAN_CGI(EUTRAN_CGI),
    #[asn(key = 0)]
    Id_MME_UE_S1AP_ID(MME_UE_S1AP_ID),
    #[asn(key = 166)]
    Id_PrivacyIndicator(PrivacyIndicator),
    #[asn(key = 131)]
    Id_TraceCollectionEntityIPAddress(TransportLayerAddress),
    #[asn(key = 8)]
    Id_eNB_UE_S1AP_ID(ENB_UE_S1AP_ID),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct CellTrafficTraceProtocolIEs_Entry {
    #[asn(key_field = true)]
    pub id: ProtocolIE_ID,
    pub criticality: Criticality,
    pub value: CellTrafficTraceProtocolIEs_EntryValue,
}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "0",
    sz_ub = "65535"
)]
pub struct CellTrafficTraceProtocolIEs(Vec<CellTrafficTraceProtocolIEs_Entry>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct CellTypeIE_Extensions_Entry {}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct CellTypeIE_Extensions(Vec<CellTypeIE_Extensions_Entry>);

#[derive(Debug, AperCodec)]
#[asn(type = "OCTET-STRING")]
pub struct OCTET_STRING_10(Vec<u8>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct CompletedCellinEAI_ItemIE_Extensions_Entry {}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct CompletedCellinEAI_ItemIE_Extensions(Vec<CompletedCellinEAI_ItemIE_Extensions_Entry>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct CompletedCellinTAI_ItemIE_Extensions_Entry {}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct CompletedCellinTAI_ItemIE_Extensions(Vec<CompletedCellinTAI_ItemIE_Extensions_Entry>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct ConnectedengNBItemIE_Extensions_Entry {}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct ConnectedengNBItemIE_Extensions(Vec<ConnectedengNBItemIE_Extensions_Entry>);

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum ConnectionEstablishmentIndicationProtocolIEs_EntryValue {
    #[asn(key = 271)]
    Id_CE_ModeBRestricted(CE_ModeBRestricted),
    #[asn(key = 253)]
    Id_DL_CP_SecurityInformation(DL_CP_SecurityInformation),
    #[asn(key = 280)]
    Id_EndIndication(EndIndication),
    #[asn(key = 251)]
    Id_EnhancedCoverageRestricted(EnhancedCoverageRestricted),
    #[asn(key = 0)]
    Id_MME_UE_S1AP_ID(MME_UE_S1AP_ID),
    #[asn(key = 278)]
    Id_Subscription_Based_UE_DifferentiationInfo(Subscription_Based_UE_DifferentiationInfo),
    #[asn(key = 252)]
    Id_UE_Level_QoS_Parameters(E_RABLevelQoSParameters),
    #[asn(key = 74)]
    Id_UERadioCapability(UERadioCapability),
    #[asn(key = 314)]
    Id_UERadioCapabilityID(UERadioCapabilityID),
    #[asn(key = 8)]
    Id_eNB_UE_S1AP_ID(ENB_UE_S1AP_ID),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct ConnectionEstablishmentIndicationProtocolIEs_Entry {
    #[asn(key_field = true)]
    pub id: ProtocolIE_ID,
    pub criticality: Criticality,
    pub value: ConnectionEstablishmentIndicationProtocolIEs_EntryValue,
}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "0",
    sz_ub = "65535"
)]
pub struct ConnectionEstablishmentIndicationProtocolIEs(
    Vec<ConnectionEstablishmentIndicationProtocolIEs_Entry>,
);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct ContextatSourceIE_Extensions_Entry {}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct ContextatSourceIE_Extensions(Vec<ContextatSourceIE_Extensions_Entry>);

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
#[asn(type = "SEQUENCE", extensible = false)]
pub struct CriticalityDiagnostics_IE_ItemIE_Extensions_Entry {}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct CriticalityDiagnostics_IE_ItemIE_Extensions(
    Vec<CriticalityDiagnostics_IE_ItemIE_Extensions_Entry>,
);

#[derive(Debug, AperCodec)]
#[asn(type = "ENUMERATED", extensible = true, lb = "0", ub = "0")]
pub struct ENUMERATED_11(u8);
impl ENUMERATED_11 {
    const D_APS_HO_REQUIRED: u8 = 0u8;
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct DAPSRequestInfoIE_Extensions_Entry {}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct DAPSRequestInfoIE_Extensions(Vec<DAPSRequestInfoIE_Extensions_Entry>);

#[derive(Debug, AperCodec)]
#[asn(type = "ENUMERATED", extensible = true, lb = "0", ub = "1")]
pub struct ENUMERATED_12(u8);
impl ENUMERATED_12 {
    const D_APS_HO_ACCEPTED: u8 = 0u8;
    const D_APS_HO_NOT_ACCEPTED: u8 = 1u8;
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct DAPSResponseInfoIE_Extensions_Entry {}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct DAPSResponseInfoIE_Extensions(Vec<DAPSResponseInfoIE_Extensions_Entry>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct DAPSResponseInfoItemIE_Extensions_Entry {}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct DAPSResponseInfoItemIE_Extensions(Vec<DAPSResponseInfoItemIE_Extensions_Entry>);

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum DAPSResponseInfoList_EntryValue {
    #[asn(key = 319)]
    Id_DAPSResponseInfoItem(DAPSResponseInfoItem),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct DAPSResponseInfoList_Entry {
    #[asn(key_field = true)]
    pub id: ProtocolIE_ID,
    pub criticality: Criticality,
    pub value: DAPSResponseInfoList_EntryValue,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct DL_CP_SecurityInformationIE_Extensions_Entry {}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct DL_CP_SecurityInformationIE_Extensions(
    Vec<DL_CP_SecurityInformationIE_Extensions_Entry>,
);

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum DeactivateTraceProtocolIEs_EntryValue {
    #[asn(key = 86)]
    Id_E_UTRAN_Trace_ID(E_UTRAN_Trace_ID),
    #[asn(key = 0)]
    Id_MME_UE_S1AP_ID(MME_UE_S1AP_ID),
    #[asn(key = 8)]
    Id_eNB_UE_S1AP_ID(ENB_UE_S1AP_ID),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct DeactivateTraceProtocolIEs_Entry {
    #[asn(key_field = true)]
    pub id: ProtocolIE_ID,
    pub criticality: Criticality,
    pub value: DeactivateTraceProtocolIEs_EntryValue,
}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "0",
    sz_ub = "65535"
)]
pub struct DeactivateTraceProtocolIEs(Vec<DeactivateTraceProtocolIEs_Entry>);

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum DownlinkNASTransportProtocolIEs_EntryValue {
    #[asn(key = 299)]
    Id_AdditionalRRMPriorityIndex(AdditionalRRMPriorityIndex),
    #[asn(key = 271)]
    Id_CE_ModeBRestricted(CE_ModeBRestricted),
    #[asn(key = 249)]
    Id_DLNASPDUDeliveryAckRequest(DLNASPDUDeliveryAckRequest),
    #[asn(key = 280)]
    Id_EndIndication(EndIndication),
    #[asn(key = 251)]
    Id_EnhancedCoverageRestricted(EnhancedCoverageRestricted),
    #[asn(key = 41)]
    Id_HandoverRestrictionList(HandoverRestrictionList),
    #[asn(key = 0)]
    Id_MME_UE_S1AP_ID(MME_UE_S1AP_ID),
    #[asn(key = 26)]
    Id_NAS_PDU(NAS_PDU),
    #[asn(key = 269)]
    Id_NRUESecurityCapabilities(NRUESecurityCapabilities),
    #[asn(key = 283)]
    Id_PendingDataIndication(PendingDataIndication),
    #[asn(key = 124)]
    Id_SRVCCOperationPossible(SRVCCOperationPossible),
    #[asn(key = 106)]
    Id_SubscriberProfileIDforRFP(SubscriberProfileIDforRFP),
    #[asn(key = 278)]
    Id_Subscription_Based_UE_DifferentiationInfo(Subscription_Based_UE_DifferentiationInfo),
    #[asn(key = 275)]
    Id_UECapabilityInfoRequest(UECapabilityInfoRequest),
    #[asn(key = 74)]
    Id_UERadioCapability(UERadioCapability),
    #[asn(key = 314)]
    Id_UERadioCapabilityID(UERadioCapabilityID),
    #[asn(key = 8)]
    Id_eNB_UE_S1AP_ID(ENB_UE_S1AP_ID),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct DownlinkNASTransportProtocolIEs_Entry {
    #[asn(key_field = true)]
    pub id: ProtocolIE_ID,
    pub criticality: Criticality,
    pub value: DownlinkNASTransportProtocolIEs_EntryValue,
}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "0",
    sz_ub = "65535"
)]
pub struct DownlinkNASTransportProtocolIEs(Vec<DownlinkNASTransportProtocolIEs_Entry>);

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum DownlinkNonUEAssociatedLPPaTransportProtocolIEs_EntryValue {
    #[asn(key = 147)]
    Id_LPPa_PDU(LPPa_PDU),
    #[asn(key = 148)]
    Id_Routing_ID(Routing_ID),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct DownlinkNonUEAssociatedLPPaTransportProtocolIEs_Entry {
    #[asn(key_field = true)]
    pub id: ProtocolIE_ID,
    pub criticality: Criticality,
    pub value: DownlinkNonUEAssociatedLPPaTransportProtocolIEs_EntryValue,
}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "0",
    sz_ub = "65535"
)]
pub struct DownlinkNonUEAssociatedLPPaTransportProtocolIEs(
    Vec<DownlinkNonUEAssociatedLPPaTransportProtocolIEs_Entry>,
);

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum DownlinkS1cdma2000tunnellingProtocolIEs_EntryValue {
    #[asn(key = 12)]
    Id_E_RABSubjecttoDataForwardingList(E_RABSubjecttoDataForwardingList),
    #[asn(key = 0)]
    Id_MME_UE_S1AP_ID(MME_UE_S1AP_ID),
    #[asn(key = 83)]
    Id_cdma2000HOStatus(Cdma2000HOStatus),
    #[asn(key = 70)]
    Id_cdma2000PDU(Cdma2000PDU),
    #[asn(key = 71)]
    Id_cdma2000RATType(Cdma2000RATType),
    #[asn(key = 8)]
    Id_eNB_UE_S1AP_ID(ENB_UE_S1AP_ID),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct DownlinkS1cdma2000tunnellingProtocolIEs_Entry {
    #[asn(key_field = true)]
    pub id: ProtocolIE_ID,
    pub criticality: Criticality,
    pub value: DownlinkS1cdma2000tunnellingProtocolIEs_EntryValue,
}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "0",
    sz_ub = "65535"
)]
pub struct DownlinkS1cdma2000tunnellingProtocolIEs(
    Vec<DownlinkS1cdma2000tunnellingProtocolIEs_Entry>,
);

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum DownlinkUEAssociatedLPPaTransportProtocolIEs_EntryValue {
    #[asn(key = 147)]
    Id_LPPa_PDU(LPPa_PDU),
    #[asn(key = 0)]
    Id_MME_UE_S1AP_ID(MME_UE_S1AP_ID),
    #[asn(key = 148)]
    Id_Routing_ID(Routing_ID),
    #[asn(key = 8)]
    Id_eNB_UE_S1AP_ID(ENB_UE_S1AP_ID),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct DownlinkUEAssociatedLPPaTransportProtocolIEs_Entry {
    #[asn(key_field = true)]
    pub id: ProtocolIE_ID,
    pub criticality: Criticality,
    pub value: DownlinkUEAssociatedLPPaTransportProtocolIEs_EntryValue,
}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "0",
    sz_ub = "65535"
)]
pub struct DownlinkUEAssociatedLPPaTransportProtocolIEs(
    Vec<DownlinkUEAssociatedLPPaTransportProtocolIEs_Entry>,
);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct E_RABAdmittedItemIE_Extensions_Entry {}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct E_RABAdmittedItemIE_Extensions(Vec<E_RABAdmittedItemIE_Extensions_Entry>);

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum E_RABAdmittedList_EntryValue {
    #[asn(key = 20)]
    Id_E_RABAdmittedItem(E_RABAdmittedItem),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct E_RABAdmittedList_Entry {
    #[asn(key_field = true)]
    pub id: ProtocolIE_ID,
    pub criticality: Criticality,
    pub value: E_RABAdmittedList_EntryValue,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct E_RABDataForwardingItemIE_Extensions_Entry {}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct E_RABDataForwardingItemIE_Extensions(Vec<E_RABDataForwardingItemIE_Extensions_Entry>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct E_RABFailedToResumeItemResumeReqIE_Extensions_Entry {}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct E_RABFailedToResumeItemResumeReqIE_Extensions(
    Vec<E_RABFailedToResumeItemResumeReqIE_Extensions_Entry>,
);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct E_RABFailedToResumeItemResumeResIE_Extensions_Entry {}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct E_RABFailedToResumeItemResumeResIE_Extensions(
    Vec<E_RABFailedToResumeItemResumeResIE_Extensions_Entry>,
);

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum E_RABFailedToResumeListResumeReq_EntryValue {
    #[asn(key = 236)]
    Id_E_RABFailedToResumeItemResumeReq(E_RABFailedToResumeItemResumeReq),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct E_RABFailedToResumeListResumeReq_Entry {
    #[asn(key_field = true)]
    pub id: ProtocolIE_ID,
    pub criticality: Criticality,
    pub value: E_RABFailedToResumeListResumeReq_EntryValue,
}

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum E_RABFailedToResumeListResumeRes_EntryValue {
    #[asn(key = 238)]
    Id_E_RABFailedToResumeItemResumeRes(E_RABFailedToResumeItemResumeRes),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct E_RABFailedToResumeListResumeRes_Entry {
    #[asn(key_field = true)]
    pub id: ProtocolIE_ID,
    pub criticality: Criticality,
    pub value: E_RABFailedToResumeListResumeRes_EntryValue,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct E_RABFailedToSetupItemHOReqAckIE_Extensions_Entry {}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct E_RABFailedToSetupItemHOReqAckIE_Extensions(
    Vec<E_RABFailedToSetupItemHOReqAckIE_Extensions_Entry>,
);

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum E_RABFailedtoSetupListHOReqAck_EntryValue {
    #[asn(key = 21)]
    Id_E_RABFailedtoSetupItemHOReqAck(E_RABFailedToSetupItemHOReqAck),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct E_RABFailedtoSetupListHOReqAck_Entry {
    #[asn(key_field = true)]
    pub id: ProtocolIE_ID,
    pub criticality: Criticality,
    pub value: E_RABFailedtoSetupListHOReqAck_EntryValue,
}

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum E_RABInformationList_EntryValue {
    #[asn(key = 78)]
    Id_E_RABInformationListItem(E_RABInformationListItem),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct E_RABInformationList_Entry {
    #[asn(key_field = true)]
    pub id: ProtocolIE_ID,
    pub criticality: Criticality,
    pub value: E_RABInformationList_EntryValue,
}

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum E_RABInformationListItemIE_Extensions_EntryExtensionValue {
    #[asn(key = 317)]
    Id_DAPSRequestInfo(DAPSRequestInfo),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct E_RABInformationListItemIE_Extensions_Entry {
    #[asn(key_field = true)]
    pub id: ProtocolExtensionID,
    pub criticality: Criticality,
    pub extension_value: E_RABInformationListItemIE_Extensions_EntryExtensionValue,
}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct E_RABInformationListItemIE_Extensions(Vec<E_RABInformationListItemIE_Extensions_Entry>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct E_RABItemIE_Extensions_Entry {}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct E_RABItemIE_Extensions(Vec<E_RABItemIE_Extensions_Entry>);

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum E_RABLevelQoSParametersIE_Extensions_EntryExtensionValue {
    #[asn(key = 273)]
    Id_DownlinkPacketLossRate(Packet_LossRate),
    #[asn(key = 274)]
    Id_UplinkPacketLossRate(Packet_LossRate),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct E_RABLevelQoSParametersIE_Extensions_Entry {
    #[asn(key_field = true)]
    pub id: ProtocolExtensionID,
    pub criticality: Criticality,
    pub extension_value: E_RABLevelQoSParametersIE_Extensions_EntryExtensionValue,
}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct E_RABLevelQoSParametersIE_Extensions(Vec<E_RABLevelQoSParametersIE_Extensions_Entry>);

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum E_RABList_EntryValue {
    #[asn(key = 35)]
    Id_E_RABItem(E_RABItem),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct E_RABList_Entry {
    #[asn(key_field = true)]
    pub id: ProtocolIE_ID,
    pub criticality: Criticality,
    pub value: E_RABList_EntryValue,
}

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum E_RABModificationConfirmProtocolIEs_EntryValue {
    #[asn(key = 146)]
    Id_CSGMembershipStatus(CSGMembershipStatus),
    #[asn(key = 58)]
    Id_CriticalityDiagnostics(CriticalityDiagnostics),
    #[asn(key = 205)]
    Id_E_RABFailedToModifyListBearerModConf(E_RABList),
    #[asn(key = 203)]
    Id_E_RABModifyListBearerModConf(E_RABModifyListBearerModConf),
    #[asn(key = 210)]
    Id_E_RABToBeReleasedListBearerModConf(E_RABList),
    #[asn(key = 0)]
    Id_MME_UE_S1AP_ID(MME_UE_S1AP_ID),
    #[asn(key = 8)]
    Id_eNB_UE_S1AP_ID(ENB_UE_S1AP_ID),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct E_RABModificationConfirmProtocolIEs_Entry {
    #[asn(key_field = true)]
    pub id: ProtocolIE_ID,
    pub criticality: Criticality,
    pub value: E_RABModificationConfirmProtocolIEs_EntryValue,
}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "0",
    sz_ub = "65535"
)]
pub struct E_RABModificationConfirmProtocolIEs(Vec<E_RABModificationConfirmProtocolIEs_Entry>);

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum E_RABModificationIndicationProtocolIEs_EntryValue {
    #[asn(key = 226)]
    Id_CSGMembershipInfo(CSGMembershipInfo),
    #[asn(key = 201)]
    Id_E_RABNotToBeModifiedListBearerModInd(E_RABNotToBeModifiedListBearerModInd),
    #[asn(key = 199)]
    Id_E_RABToBeModifiedListBearerModInd(E_RABToBeModifiedListBearerModInd),
    #[asn(key = 0)]
    Id_MME_UE_S1AP_ID(MME_UE_S1AP_ID),
    #[asn(key = 264)]
    Id_SecondaryRATDataUsageReportList(SecondaryRATDataUsageReportList),
    #[asn(key = 176)]
    Id_Tunnel_Information_for_BBF(TunnelInformation),
    #[asn(key = 189)]
    Id_UserLocationInformation(UserLocationInformation),
    #[asn(key = 8)]
    Id_eNB_UE_S1AP_ID(ENB_UE_S1AP_ID),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct E_RABModificationIndicationProtocolIEs_Entry {
    #[asn(key_field = true)]
    pub id: ProtocolIE_ID,
    pub criticality: Criticality,
    pub value: E_RABModificationIndicationProtocolIEs_EntryValue,
}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "0",
    sz_ub = "65535"
)]
pub struct E_RABModificationIndicationProtocolIEs(
    Vec<E_RABModificationIndicationProtocolIEs_Entry>,
);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct E_RABModifyItemBearerModConfIE_Extensions_Entry {}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct E_RABModifyItemBearerModConfIE_Extensions(
    Vec<E_RABModifyItemBearerModConfIE_Extensions_Entry>,
);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct E_RABModifyItemBearerModResIE_Extensions_Entry {}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct E_RABModifyItemBearerModResIE_Extensions(
    Vec<E_RABModifyItemBearerModResIE_Extensions_Entry>,
);

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum E_RABModifyListBearerModConf_EntryValue {
    #[asn(key = 204)]
    Id_E_RABModifyItemBearerModConf(E_RABModifyItemBearerModConf),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct E_RABModifyListBearerModConf_Entry {
    #[asn(key_field = true)]
    pub id: ProtocolIE_ID,
    pub criticality: Criticality,
    pub value: E_RABModifyListBearerModConf_EntryValue,
}

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum E_RABModifyListBearerModRes_EntryValue {
    #[asn(key = 37)]
    Id_E_RABModifyItemBearerModRes(E_RABModifyItemBearerModRes),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct E_RABModifyListBearerModRes_Entry {
    #[asn(key_field = true)]
    pub id: ProtocolIE_ID,
    pub criticality: Criticality,
    pub value: E_RABModifyListBearerModRes_EntryValue,
}

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum E_RABModifyRequestProtocolIEs_EntryValue {
    #[asn(key = 30)]
    Id_E_RABToBeModifiedListBearerModReq(E_RABToBeModifiedListBearerModReq),
    #[asn(key = 0)]
    Id_MME_UE_S1AP_ID(MME_UE_S1AP_ID),
    #[asn(key = 268)]
    Id_SecondaryRATDataUsageRequest(SecondaryRATDataUsageRequest),
    #[asn(key = 8)]
    Id_eNB_UE_S1AP_ID(ENB_UE_S1AP_ID),
    #[asn(key = 66)]
    Id_uEaggregateMaximumBitrate(UEAggregateMaximumBitrate),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct E_RABModifyRequestProtocolIEs_Entry {
    #[asn(key_field = true)]
    pub id: ProtocolIE_ID,
    pub criticality: Criticality,
    pub value: E_RABModifyRequestProtocolIEs_EntryValue,
}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "0",
    sz_ub = "65535"
)]
pub struct E_RABModifyRequestProtocolIEs(Vec<E_RABModifyRequestProtocolIEs_Entry>);

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum E_RABModifyResponseProtocolIEs_EntryValue {
    #[asn(key = 58)]
    Id_CriticalityDiagnostics(CriticalityDiagnostics),
    #[asn(key = 32)]
    Id_E_RABFailedToModifyList(E_RABList),
    #[asn(key = 31)]
    Id_E_RABModifyListBearerModRes(E_RABModifyListBearerModRes),
    #[asn(key = 0)]
    Id_MME_UE_S1AP_ID(MME_UE_S1AP_ID),
    #[asn(key = 264)]
    Id_SecondaryRATDataUsageReportList(SecondaryRATDataUsageReportList),
    #[asn(key = 8)]
    Id_eNB_UE_S1AP_ID(ENB_UE_S1AP_ID),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct E_RABModifyResponseProtocolIEs_Entry {
    #[asn(key_field = true)]
    pub id: ProtocolIE_ID,
    pub criticality: Criticality,
    pub value: E_RABModifyResponseProtocolIEs_EntryValue,
}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "0",
    sz_ub = "65535"
)]
pub struct E_RABModifyResponseProtocolIEs(Vec<E_RABModifyResponseProtocolIEs_Entry>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct E_RABNotToBeModifiedItemBearerModIndIE_Extensions_Entry {}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct E_RABNotToBeModifiedItemBearerModIndIE_Extensions(
    Vec<E_RABNotToBeModifiedItemBearerModIndIE_Extensions_Entry>,
);

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum E_RABNotToBeModifiedListBearerModInd_EntryValue {
    #[asn(key = 202)]
    Id_E_RABNotToBeModifiedItemBearerModInd(E_RABNotToBeModifiedItemBearerModInd),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct E_RABNotToBeModifiedListBearerModInd_Entry {
    #[asn(key_field = true)]
    pub id: ProtocolIE_ID,
    pub criticality: Criticality,
    pub value: E_RABNotToBeModifiedListBearerModInd_EntryValue,
}

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum E_RABReleaseCommandProtocolIEs_EntryValue {
    #[asn(key = 33)]
    Id_E_RABToBeReleasedList(E_RABList),
    #[asn(key = 0)]
    Id_MME_UE_S1AP_ID(MME_UE_S1AP_ID),
    #[asn(key = 26)]
    Id_NAS_PDU(NAS_PDU),
    #[asn(key = 8)]
    Id_eNB_UE_S1AP_ID(ENB_UE_S1AP_ID),
    #[asn(key = 66)]
    Id_uEaggregateMaximumBitrate(UEAggregateMaximumBitrate),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct E_RABReleaseCommandProtocolIEs_Entry {
    #[asn(key_field = true)]
    pub id: ProtocolIE_ID,
    pub criticality: Criticality,
    pub value: E_RABReleaseCommandProtocolIEs_EntryValue,
}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "0",
    sz_ub = "65535"
)]
pub struct E_RABReleaseCommandProtocolIEs(Vec<E_RABReleaseCommandProtocolIEs_Entry>);

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum E_RABReleaseIndicationProtocolIEs_EntryValue {
    #[asn(key = 110)]
    Id_E_RABReleasedList(E_RABList),
    #[asn(key = 0)]
    Id_MME_UE_S1AP_ID(MME_UE_S1AP_ID),
    #[asn(key = 264)]
    Id_SecondaryRATDataUsageReportList(SecondaryRATDataUsageReportList),
    #[asn(key = 189)]
    Id_UserLocationInformation(UserLocationInformation),
    #[asn(key = 8)]
    Id_eNB_UE_S1AP_ID(ENB_UE_S1AP_ID),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct E_RABReleaseIndicationProtocolIEs_Entry {
    #[asn(key_field = true)]
    pub id: ProtocolIE_ID,
    pub criticality: Criticality,
    pub value: E_RABReleaseIndicationProtocolIEs_EntryValue,
}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "0",
    sz_ub = "65535"
)]
pub struct E_RABReleaseIndicationProtocolIEs(Vec<E_RABReleaseIndicationProtocolIEs_Entry>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct E_RABReleaseItemBearerRelCompIE_Extensions_Entry {}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct E_RABReleaseItemBearerRelCompIE_Extensions(
    Vec<E_RABReleaseItemBearerRelCompIE_Extensions_Entry>,
);

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum E_RABReleaseListBearerRelComp_EntryValue {
    #[asn(key = 15)]
    Id_E_RABReleaseItemBearerRelComp(E_RABReleaseItemBearerRelComp),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct E_RABReleaseListBearerRelComp_Entry {
    #[asn(key_field = true)]
    pub id: ProtocolIE_ID,
    pub criticality: Criticality,
    pub value: E_RABReleaseListBearerRelComp_EntryValue,
}

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum E_RABReleaseResponseProtocolIEs_EntryValue {
    #[asn(key = 58)]
    Id_CriticalityDiagnostics(CriticalityDiagnostics),
    #[asn(key = 34)]
    Id_E_RABFailedToReleaseList(E_RABList),
    #[asn(key = 69)]
    Id_E_RABReleaseListBearerRelComp(E_RABReleaseListBearerRelComp),
    #[asn(key = 0)]
    Id_MME_UE_S1AP_ID(MME_UE_S1AP_ID),
    #[asn(key = 264)]
    Id_SecondaryRATDataUsageReportList(SecondaryRATDataUsageReportList),
    #[asn(key = 189)]
    Id_UserLocationInformation(UserLocationInformation),
    #[asn(key = 8)]
    Id_eNB_UE_S1AP_ID(ENB_UE_S1AP_ID),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct E_RABReleaseResponseProtocolIEs_Entry {
    #[asn(key_field = true)]
    pub id: ProtocolIE_ID,
    pub criticality: Criticality,
    pub value: E_RABReleaseResponseProtocolIEs_EntryValue,
}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "0",
    sz_ub = "65535"
)]
pub struct E_RABReleaseResponseProtocolIEs(Vec<E_RABReleaseResponseProtocolIEs_Entry>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct E_RABSetupItemBearerSUResIE_Extensions_Entry {}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct E_RABSetupItemBearerSUResIE_Extensions(
    Vec<E_RABSetupItemBearerSUResIE_Extensions_Entry>,
);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct E_RABSetupItemCtxtSUResIE_Extensions_Entry {}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct E_RABSetupItemCtxtSUResIE_Extensions(Vec<E_RABSetupItemCtxtSUResIE_Extensions_Entry>);

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum E_RABSetupListBearerSURes_EntryValue {
    #[asn(key = 39)]
    Id_E_RABSetupItemBearerSURes(E_RABSetupItemBearerSURes),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct E_RABSetupListBearerSURes_Entry {
    #[asn(key_field = true)]
    pub id: ProtocolIE_ID,
    pub criticality: Criticality,
    pub value: E_RABSetupListBearerSURes_EntryValue,
}

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum E_RABSetupListCtxtSURes_EntryValue {
    #[asn(key = 50)]
    Id_E_RABSetupItemCtxtSURes(E_RABSetupItemCtxtSURes),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct E_RABSetupListCtxtSURes_Entry {
    #[asn(key_field = true)]
    pub id: ProtocolIE_ID,
    pub criticality: Criticality,
    pub value: E_RABSetupListCtxtSURes_EntryValue,
}

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum E_RABSetupRequestProtocolIEs_EntryValue {
    #[asn(key = 16)]
    Id_E_RABToBeSetupListBearerSUReq(E_RABToBeSetupListBearerSUReq),
    #[asn(key = 0)]
    Id_MME_UE_S1AP_ID(MME_UE_S1AP_ID),
    #[asn(key = 8)]
    Id_eNB_UE_S1AP_ID(ENB_UE_S1AP_ID),
    #[asn(key = 66)]
    Id_uEaggregateMaximumBitrate(UEAggregateMaximumBitrate),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct E_RABSetupRequestProtocolIEs_Entry {
    #[asn(key_field = true)]
    pub id: ProtocolIE_ID,
    pub criticality: Criticality,
    pub value: E_RABSetupRequestProtocolIEs_EntryValue,
}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "0",
    sz_ub = "65535"
)]
pub struct E_RABSetupRequestProtocolIEs(Vec<E_RABSetupRequestProtocolIEs_Entry>);

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum E_RABSetupResponseProtocolIEs_EntryValue {
    #[asn(key = 58)]
    Id_CriticalityDiagnostics(CriticalityDiagnostics),
    #[asn(key = 29)]
    Id_E_RABFailedToSetupListBearerSURes(E_RABList),
    #[asn(key = 28)]
    Id_E_RABSetupListBearerSURes(E_RABSetupListBearerSURes),
    #[asn(key = 0)]
    Id_MME_UE_S1AP_ID(MME_UE_S1AP_ID),
    #[asn(key = 8)]
    Id_eNB_UE_S1AP_ID(ENB_UE_S1AP_ID),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct E_RABSetupResponseProtocolIEs_Entry {
    #[asn(key_field = true)]
    pub id: ProtocolIE_ID,
    pub criticality: Criticality,
    pub value: E_RABSetupResponseProtocolIEs_EntryValue,
}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "0",
    sz_ub = "65535"
)]
pub struct E_RABSetupResponseProtocolIEs(Vec<E_RABSetupResponseProtocolIEs_Entry>);

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum E_RABSubjecttoDataForwardingList_EntryValue {
    #[asn(key = 14)]
    Id_E_RABDataForwardingItem(E_RABDataForwardingItem),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct E_RABSubjecttoDataForwardingList_Entry {
    #[asn(key_field = true)]
    pub id: ProtocolIE_ID,
    pub criticality: Criticality,
    pub value: E_RABSubjecttoDataForwardingList_EntryValue,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct E_RABToBeModifiedItemBearerModIndIE_Extensions_Entry {}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct E_RABToBeModifiedItemBearerModIndIE_Extensions(
    Vec<E_RABToBeModifiedItemBearerModIndIE_Extensions_Entry>,
);

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum E_RABToBeModifiedItemBearerModReqIE_Extensions_EntryExtensionValue {
    #[asn(key = 185)]
    Id_TransportInformation(TransportInformation),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct E_RABToBeModifiedItemBearerModReqIE_Extensions_Entry {
    #[asn(key_field = true)]
    pub id: ProtocolExtensionID,
    pub criticality: Criticality,
    pub extension_value: E_RABToBeModifiedItemBearerModReqIE_Extensions_EntryExtensionValue,
}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct E_RABToBeModifiedItemBearerModReqIE_Extensions(
    Vec<E_RABToBeModifiedItemBearerModReqIE_Extensions_Entry>,
);

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum E_RABToBeModifiedListBearerModInd_EntryValue {
    #[asn(key = 200)]
    Id_E_RABToBeModifiedItemBearerModInd(E_RABToBeModifiedItemBearerModInd),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct E_RABToBeModifiedListBearerModInd_Entry {
    #[asn(key_field = true)]
    pub id: ProtocolIE_ID,
    pub criticality: Criticality,
    pub value: E_RABToBeModifiedListBearerModInd_EntryValue,
}

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum E_RABToBeModifiedListBearerModReq_EntryValue {
    #[asn(key = 36)]
    Id_E_RABToBeModifiedItemBearerModReq(E_RABToBeModifiedItemBearerModReq),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct E_RABToBeModifiedListBearerModReq_Entry {
    #[asn(key_field = true)]
    pub id: ProtocolIE_ID,
    pub criticality: Criticality,
    pub value: E_RABToBeModifiedListBearerModReq_EntryValue,
}

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum E_RABToBeSetupItemBearerSUReqIE_Extensions_EntryExtensionValue {
    #[asn(key = 233)]
    Id_BearerType(BearerType),
    #[asn(key = 156)]
    Id_Correlation_ID(Correlation_ID),
    #[asn(key = 305)]
    Id_Ethernet_Type(Ethernet_Type),
    #[asn(key = 183)]
    Id_SIPTO_Correlation_ID(Correlation_ID),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct E_RABToBeSetupItemBearerSUReqIE_Extensions_Entry {
    #[asn(key_field = true)]
    pub id: ProtocolExtensionID,
    pub criticality: Criticality,
    pub extension_value: E_RABToBeSetupItemBearerSUReqIE_Extensions_EntryExtensionValue,
}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct E_RABToBeSetupItemBearerSUReqIE_Extensions(
    Vec<E_RABToBeSetupItemBearerSUReqIE_Extensions_Entry>,
);

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum E_RABToBeSetupItemCtxtSUReqIE_Extensions_EntryExtensionValue {
    #[asn(key = 233)]
    Id_BearerType(BearerType),
    #[asn(key = 156)]
    Id_Correlation_ID(Correlation_ID),
    #[asn(key = 305)]
    Id_Ethernet_Type(Ethernet_Type),
    #[asn(key = 183)]
    Id_SIPTO_Correlation_ID(Correlation_ID),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct E_RABToBeSetupItemCtxtSUReqIE_Extensions_Entry {
    #[asn(key_field = true)]
    pub id: ProtocolExtensionID,
    pub criticality: Criticality,
    pub extension_value: E_RABToBeSetupItemCtxtSUReqIE_Extensions_EntryExtensionValue,
}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct E_RABToBeSetupItemCtxtSUReqIE_Extensions(
    Vec<E_RABToBeSetupItemCtxtSUReqIE_Extensions_Entry>,
);

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum E_RABToBeSetupItemHOReqIE_Extensions_EntryExtensionValue {
    #[asn(key = 233)]
    Id_BearerType(BearerType),
    #[asn(key = 143)]
    Id_Data_Forwarding_Not_Possible(Data_Forwarding_Not_Possible),
    #[asn(key = 305)]
    Id_Ethernet_Type(Ethernet_Type),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct E_RABToBeSetupItemHOReqIE_Extensions_Entry {
    #[asn(key_field = true)]
    pub id: ProtocolExtensionID,
    pub criticality: Criticality,
    pub extension_value: E_RABToBeSetupItemHOReqIE_Extensions_EntryExtensionValue,
}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct E_RABToBeSetupItemHOReqIE_Extensions(Vec<E_RABToBeSetupItemHOReqIE_Extensions_Entry>);

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum E_RABToBeSetupListBearerSUReq_EntryValue {
    #[asn(key = 17)]
    Id_E_RABToBeSetupItemBearerSUReq(E_RABToBeSetupItemBearerSUReq),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct E_RABToBeSetupListBearerSUReq_Entry {
    #[asn(key_field = true)]
    pub id: ProtocolIE_ID,
    pub criticality: Criticality,
    pub value: E_RABToBeSetupListBearerSUReq_EntryValue,
}

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum E_RABToBeSetupListCtxtSUReq_EntryValue {
    #[asn(key = 52)]
    Id_E_RABToBeSetupItemCtxtSUReq(E_RABToBeSetupItemCtxtSUReq),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct E_RABToBeSetupListCtxtSUReq_Entry {
    #[asn(key_field = true)]
    pub id: ProtocolIE_ID,
    pub criticality: Criticality,
    pub value: E_RABToBeSetupListCtxtSUReq_EntryValue,
}

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum E_RABToBeSetupListHOReq_EntryValue {
    #[asn(key = 27)]
    Id_E_RABToBeSetupItemHOReq(E_RABToBeSetupItemHOReq),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct E_RABToBeSetupListHOReq_Entry {
    #[asn(key_field = true)]
    pub id: ProtocolIE_ID,
    pub criticality: Criticality,
    pub value: E_RABToBeSetupListHOReq_EntryValue,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct E_RABToBeSwitchedDLItemIE_Extensions_Entry {}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct E_RABToBeSwitchedDLItemIE_Extensions(Vec<E_RABToBeSwitchedDLItemIE_Extensions_Entry>);

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum E_RABToBeSwitchedDLList_EntryValue {
    #[asn(key = 23)]
    Id_E_RABToBeSwitchedDLItem(E_RABToBeSwitchedDLItem),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct E_RABToBeSwitchedDLList_Entry {
    #[asn(key_field = true)]
    pub id: ProtocolIE_ID,
    pub criticality: Criticality,
    pub value: E_RABToBeSwitchedDLList_EntryValue,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct E_RABToBeSwitchedULItemIE_Extensions_Entry {}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct E_RABToBeSwitchedULItemIE_Extensions(Vec<E_RABToBeSwitchedULItemIE_Extensions_Entry>);

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum E_RABToBeSwitchedULList_EntryValue {
    #[asn(key = 94)]
    Id_E_RABToBeSwitchedULItem(E_RABToBeSwitchedULItem),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct E_RABToBeSwitchedULList_Entry {
    #[asn(key_field = true)]
    pub id: ProtocolIE_ID,
    pub criticality: Criticality,
    pub value: E_RABToBeSwitchedULList_EntryValue,
}

#[derive(Debug, AperCodec)]
#[asn(type = "OCTET-STRING", sz_extensible = false, sz_lb = "4", sz_ub = "4")]
pub struct OCTET_STRING_13(Vec<u8>);

#[derive(Debug, AperCodec)]
#[asn(type = "OCTET-STRING", sz_extensible = false, sz_lb = "4", sz_ub = "4")]
pub struct OCTET_STRING_14(Vec<u8>);

#[derive(Debug, AperCodec)]
#[asn(type = "INTEGER", lb = "0", ub = "18446744073709551615")]
pub struct INTEGER_15(u64);

#[derive(Debug, AperCodec)]
#[asn(type = "INTEGER", lb = "0", ub = "18446744073709551615")]
pub struct INTEGER_16(u64);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct E_RABUsageReportItemIE_Extensions_Entry {}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct E_RABUsageReportItemIE_Extensions(Vec<E_RABUsageReportItemIE_Extensions_Entry>);

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum E_RABUsageReportList_EntryValue {
    #[asn(key = 267)]
    Id_E_RABUsageReportItem(E_RABUsageReportItem),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct E_RABUsageReportList_Entry {
    #[asn(key_field = true)]
    pub id: ProtocolIE_ID,
    pub criticality: Criticality,
    pub value: E_RABUsageReportList_EntryValue,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct EN_DCSONConfigurationTransferIE_Extensions_Entry {}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct EN_DCSONConfigurationTransferIE_Extensions(
    Vec<EN_DCSONConfigurationTransferIE_Extensions_Entry>,
);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct EN_DCSONeNBIdentificationIE_Extensions_Entry {}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct EN_DCSONeNBIdentificationIE_Extensions(
    Vec<EN_DCSONeNBIdentificationIE_Extensions_Entry>,
);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct EN_DCSONengNBIdentificationIE_Extensions_Entry {}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct EN_DCSONengNBIdentificationIE_Extensions(
    Vec<EN_DCSONengNBIdentificationIE_Extensions_Entry>,
);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct EN_DCTransferTypeReplyIE_Extensions_Entry {}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct EN_DCTransferTypeReplyIE_Extensions(Vec<EN_DCTransferTypeReplyIE_Extensions_Entry>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct EN_DCTransferTypeRequestIE_Extensions_Entry {}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct EN_DCTransferTypeRequestIE_Extensions(Vec<EN_DCTransferTypeRequestIE_Extensions_Entry>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct ENB_EarlyStatusTransfer_TransparentContainerIE_Extensions_Entry {}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct ENB_EarlyStatusTransfer_TransparentContainerIE_Extensions(
    Vec<ENB_EarlyStatusTransfer_TransparentContainerIE_Extensions_Entry>,
);

#[derive(Debug, AperCodec)]
#[asn(type = "BITSTRING", sz_extensible = false, sz_lb = "20", sz_ub = "20")]
pub struct BIT_STRING_17(BitVec<Msb0, u8>);

#[derive(Debug, AperCodec)]
#[asn(type = "BITSTRING", sz_extensible = false, sz_lb = "28", sz_ub = "28")]
pub struct BIT_STRING_18(BitVec<Msb0, u8>);

#[derive(Debug, AperCodec)]
#[asn(type = "BITSTRING", sz_extensible = false, sz_lb = "18", sz_ub = "18")]
pub struct BIT_STRING_19(BitVec<Msb0, u8>);

#[derive(Debug, AperCodec)]
#[asn(type = "BITSTRING", sz_extensible = false, sz_lb = "21", sz_ub = "21")]
pub struct BIT_STRING_20(BitVec<Msb0, u8>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct ENB_StatusTransfer_TransparentContainerIE_Extensions_Entry {}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct ENB_StatusTransfer_TransparentContainerIE_Extensions(
    Vec<ENB_StatusTransfer_TransparentContainerIE_Extensions_Entry>,
);

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum ENBCPRelocationIndicationProtocolIEs_EntryValue {
    #[asn(key = 100)]
    Id_EUTRAN_CGI(EUTRAN_CGI),
    #[asn(key = 96)]
    Id_S_TMSI(S_TMSI),
    #[asn(key = 67)]
    Id_TAI(TAI),
    #[asn(key = 254)]
    Id_UL_CP_SecurityInformation(UL_CP_SecurityInformation),
    #[asn(key = 8)]
    Id_eNB_UE_S1AP_ID(ENB_UE_S1AP_ID),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct ENBCPRelocationIndicationProtocolIEs_Entry {
    #[asn(key_field = true)]
    pub id: ProtocolIE_ID,
    pub criticality: Criticality,
    pub value: ENBCPRelocationIndicationProtocolIEs_EntryValue,
}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "0",
    sz_ub = "65535"
)]
pub struct ENBCPRelocationIndicationProtocolIEs(Vec<ENBCPRelocationIndicationProtocolIEs_Entry>);

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum ENBConfigurationTransferProtocolIEs_EntryValue {
    #[asn(key = 294)]
    Id_EN_DCSONConfigurationTransfer_ECT(EN_DCSONConfigurationTransfer),
    #[asn(key = 310)]
    Id_IntersystemSONConfigurationTransferECT(IntersystemSONConfigurationTransfer),
    #[asn(key = 129)]
    Id_SONConfigurationTransferECT(SONConfigurationTransfer),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct ENBConfigurationTransferProtocolIEs_Entry {
    #[asn(key_field = true)]
    pub id: ProtocolIE_ID,
    pub criticality: Criticality,
    pub value: ENBConfigurationTransferProtocolIEs_EntryValue,
}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "0",
    sz_ub = "65535"
)]
pub struct ENBConfigurationTransferProtocolIEs(Vec<ENBConfigurationTransferProtocolIEs_Entry>);

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum ENBConfigurationUpdateProtocolIEs_EntryValue {
    #[asn(key = 128)]
    Id_CSG_IdList(CSG_IdList),
    #[asn(key = 292)]
    Id_ConnectedengNBToAddList(ConnectedengNBList),
    #[asn(key = 293)]
    Id_ConnectedengNBToRemoveList(ConnectedengNBList),
    #[asn(key = 137)]
    Id_DefaultPagingDRX(PagingDRX),
    #[asn(key = 234)]
    Id_NB_IoT_DefaultPagingDRX(NB_IoT_DefaultPagingDRX),
    #[asn(key = 64)]
    Id_SupportedTAs(SupportedTAs),
    #[asn(key = 60)]
    Id_eNBname(ENBname),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct ENBConfigurationUpdateProtocolIEs_Entry {
    #[asn(key_field = true)]
    pub id: ProtocolIE_ID,
    pub criticality: Criticality,
    pub value: ENBConfigurationUpdateProtocolIEs_EntryValue,
}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "0",
    sz_ub = "65535"
)]
pub struct ENBConfigurationUpdateProtocolIEs(Vec<ENBConfigurationUpdateProtocolIEs_Entry>);

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum ENBConfigurationUpdateAcknowledgeProtocolIEs_EntryValue {
    #[asn(key = 58)]
    Id_CriticalityDiagnostics(CriticalityDiagnostics),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct ENBConfigurationUpdateAcknowledgeProtocolIEs_Entry {
    #[asn(key_field = true)]
    pub id: ProtocolIE_ID,
    pub criticality: Criticality,
    pub value: ENBConfigurationUpdateAcknowledgeProtocolIEs_EntryValue,
}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "0",
    sz_ub = "65535"
)]
pub struct ENBConfigurationUpdateAcknowledgeProtocolIEs(
    Vec<ENBConfigurationUpdateAcknowledgeProtocolIEs_Entry>,
);

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum ENBConfigurationUpdateFailureProtocolIEs_EntryValue {
    #[asn(key = 2)]
    Id_Cause(Cause),
    #[asn(key = 58)]
    Id_CriticalityDiagnostics(CriticalityDiagnostics),
    #[asn(key = 65)]
    Id_TimeToWait(TimeToWait),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct ENBConfigurationUpdateFailureProtocolIEs_Entry {
    #[asn(key_field = true)]
    pub id: ProtocolIE_ID,
    pub criticality: Criticality,
    pub value: ENBConfigurationUpdateFailureProtocolIEs_EntryValue,
}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "0",
    sz_ub = "65535"
)]
pub struct ENBConfigurationUpdateFailureProtocolIEs(
    Vec<ENBConfigurationUpdateFailureProtocolIEs_Entry>,
);

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum ENBDirectInformationTransferProtocolIEs_EntryValue {
    #[asn(key = 121)]
    Id_Inter_SystemInformationTransferTypeEDT(Inter_SystemInformationTransferType),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct ENBDirectInformationTransferProtocolIEs_Entry {
    #[asn(key_field = true)]
    pub id: ProtocolIE_ID,
    pub criticality: Criticality,
    pub value: ENBDirectInformationTransferProtocolIEs_EntryValue,
}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "0",
    sz_ub = "65535"
)]
pub struct ENBDirectInformationTransferProtocolIEs(
    Vec<ENBDirectInformationTransferProtocolIEs_Entry>,
);

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum ENBEarlyStatusTransferProtocolIEs_EntryValue {
    #[asn(key = 0)]
    Id_MME_UE_S1AP_ID(MME_UE_S1AP_ID),
    #[asn(key = 321)]
    Id_eNB_EarlyStatusTransfer_TransparentContainer(ENB_EarlyStatusTransfer_TransparentContainer),
    #[asn(key = 8)]
    Id_eNB_UE_S1AP_ID(ENB_UE_S1AP_ID),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct ENBEarlyStatusTransferProtocolIEs_Entry {
    #[asn(key_field = true)]
    pub id: ProtocolIE_ID,
    pub criticality: Criticality,
    pub value: ENBEarlyStatusTransferProtocolIEs_EntryValue,
}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "0",
    sz_ub = "65535"
)]
pub struct ENBEarlyStatusTransferProtocolIEs(Vec<ENBEarlyStatusTransferProtocolIEs_Entry>);

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum ENBStatusTransferProtocolIEs_EntryValue {
    #[asn(key = 0)]
    Id_MME_UE_S1AP_ID(MME_UE_S1AP_ID),
    #[asn(key = 90)]
    Id_eNB_StatusTransfer_TransparentContainer(ENB_StatusTransfer_TransparentContainer),
    #[asn(key = 8)]
    Id_eNB_UE_S1AP_ID(ENB_UE_S1AP_ID),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct ENBStatusTransferProtocolIEs_Entry {
    #[asn(key_field = true)]
    pub id: ProtocolIE_ID,
    pub criticality: Criticality,
    pub value: ENBStatusTransferProtocolIEs_EntryValue,
}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "0",
    sz_ub = "65535"
)]
pub struct ENBStatusTransferProtocolIEs(Vec<ENBStatusTransferProtocolIEs_Entry>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct ENBX2ExtTLAIE_Extensions_Entry {}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct ENBX2ExtTLAIE_Extensions(Vec<ENBX2ExtTLAIE_Extensions_Entry>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct EUTRAN_CGIIE_Extensions_Entry {}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct EUTRAN_CGIIE_Extensions(Vec<EUTRAN_CGIIE_Extensions_Entry>);

#[derive(Debug, AperCodec)]
#[asn(type = "OCTET-STRING")]
pub struct OCTET_STRING_21(Vec<u8>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct EmergencyAreaID_Broadcast_ItemIE_Extensions_Entry {}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct EmergencyAreaID_Broadcast_ItemIE_Extensions(
    Vec<EmergencyAreaID_Broadcast_ItemIE_Extensions_Entry>,
);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct EmergencyAreaID_Cancelled_ItemIE_Extensions_Entry {}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct EmergencyAreaID_Cancelled_ItemIE_Extensions(
    Vec<EmergencyAreaID_Cancelled_ItemIE_Extensions_Entry>,
);

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum ErrorIndicationProtocolIEs_EntryValue {
    #[asn(key = 2)]
    Id_Cause(Cause),
    #[asn(key = 58)]
    Id_CriticalityDiagnostics(CriticalityDiagnostics),
    #[asn(key = 0)]
    Id_MME_UE_S1AP_ID(MME_UE_S1AP_ID),
    #[asn(key = 96)]
    Id_S_TMSI(S_TMSI),
    #[asn(key = 8)]
    Id_eNB_UE_S1AP_ID(ENB_UE_S1AP_ID),
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
#[asn(type = "SEQUENCE", extensible = false)]
pub struct ExpectedUEActivityBehaviourIE_Extensions_Entry {}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct ExpectedUEActivityBehaviourIE_Extensions(
    Vec<ExpectedUEActivityBehaviourIE_Extensions_Entry>,
);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct ExpectedUEBehaviourIE_Extensions_Entry {}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct ExpectedUEBehaviourIE_Extensions(Vec<ExpectedUEBehaviourIE_Extensions_Entry>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct FiveGSTAIIE_Extensions_Entry {}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct FiveGSTAIIE_Extensions(Vec<FiveGSTAIIE_Extensions_Entry>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct ForbiddenLAs_ItemIE_Extensions_Entry {}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct ForbiddenLAs_ItemIE_Extensions(Vec<ForbiddenLAs_ItemIE_Extensions_Entry>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct ForbiddenTAs_ItemIE_Extensions_Entry {}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct ForbiddenTAs_ItemIE_Extensions(Vec<ForbiddenTAs_ItemIE_Extensions_Entry>);

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum GBR_QosInformationIE_Extensions_EntryExtensionValue {
    #[asn(key = 257)]
    Id_extended_e_RAB_GuaranteedBitrateDL(ExtendedBitRate),
    #[asn(key = 258)]
    Id_extended_e_RAB_GuaranteedBitrateUL(ExtendedBitRate),
    #[asn(key = 255)]
    Id_extended_e_RAB_MaximumBitrateDL(ExtendedBitRate),
    #[asn(key = 256)]
    Id_extended_e_RAB_MaximumBitrateUL(ExtendedBitRate),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct GBR_QosInformationIE_Extensions_Entry {
    #[asn(key_field = true)]
    pub id: ProtocolExtensionID,
    pub criticality: Criticality,
    pub extension_value: GBR_QosInformationIE_Extensions_EntryExtensionValue,
}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct GBR_QosInformationIE_Extensions(Vec<GBR_QosInformationIE_Extensions_Entry>);

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
pub struct GNBIE_Extensions_Entry {}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct GNBIE_Extensions(Vec<GNBIE_Extensions_Entry>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct GUMMEIIE_Extensions_Entry {}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct GUMMEIIE_Extensions(Vec<GUMMEIIE_Extensions_Entry>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct Global_ENB_IDIE_Extensions_Entry {}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct Global_ENB_IDIE_Extensions(Vec<Global_ENB_IDIE_Extensions_Entry>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct Global_GNB_IDIE_Extensions_Entry {}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct Global_GNB_IDIE_Extensions(Vec<Global_GNB_IDIE_Extensions_Entry>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct Global_en_gNB_IDIE_Extensions_Entry {}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct Global_en_gNB_IDIE_Extensions(Vec<Global_en_gNB_IDIE_Extensions_Entry>);

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum HandoverCancelProtocolIEs_EntryValue {
    #[asn(key = 2)]
    Id_Cause(Cause),
    #[asn(key = 0)]
    Id_MME_UE_S1AP_ID(MME_UE_S1AP_ID),
    #[asn(key = 8)]
    Id_eNB_UE_S1AP_ID(ENB_UE_S1AP_ID),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct HandoverCancelProtocolIEs_Entry {
    #[asn(key_field = true)]
    pub id: ProtocolIE_ID,
    pub criticality: Criticality,
    pub value: HandoverCancelProtocolIEs_EntryValue,
}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "0",
    sz_ub = "65535"
)]
pub struct HandoverCancelProtocolIEs(Vec<HandoverCancelProtocolIEs_Entry>);

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum HandoverCancelAcknowledgeProtocolIEs_EntryValue {
    #[asn(key = 58)]
    Id_CriticalityDiagnostics(CriticalityDiagnostics),
    #[asn(key = 0)]
    Id_MME_UE_S1AP_ID(MME_UE_S1AP_ID),
    #[asn(key = 8)]
    Id_eNB_UE_S1AP_ID(ENB_UE_S1AP_ID),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct HandoverCancelAcknowledgeProtocolIEs_Entry {
    #[asn(key_field = true)]
    pub id: ProtocolIE_ID,
    pub criticality: Criticality,
    pub value: HandoverCancelAcknowledgeProtocolIEs_EntryValue,
}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "0",
    sz_ub = "65535"
)]
pub struct HandoverCancelAcknowledgeProtocolIEs(Vec<HandoverCancelAcknowledgeProtocolIEs_Entry>);

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum HandoverCommandProtocolIEs_EntryValue {
    #[asn(key = 58)]
    Id_CriticalityDiagnostics(CriticalityDiagnostics),
    #[asn(key = 12)]
    Id_E_RABSubjecttoDataForwardingList(E_RABSubjecttoDataForwardingList),
    #[asn(key = 13)]
    Id_E_RABtoReleaseListHOCmd(E_RABList),
    #[asn(key = 1)]
    Id_HandoverType(HandoverType),
    #[asn(key = 0)]
    Id_MME_UE_S1AP_ID(MME_UE_S1AP_ID),
    #[asn(key = 135)]
    Id_NASSecurityParametersfromE_UTRAN(NASSecurityParametersfromE_UTRAN),
    #[asn(key = 123)]
    Id_Target_ToSource_TransparentContainer(Target_ToSource_TransparentContainer),
    #[asn(key = 139)]
    Id_Target_ToSource_TransparentContainer_Secondary(Target_ToSource_TransparentContainer),
    #[asn(key = 8)]
    Id_eNB_UE_S1AP_ID(ENB_UE_S1AP_ID),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct HandoverCommandProtocolIEs_Entry {
    #[asn(key_field = true)]
    pub id: ProtocolIE_ID,
    pub criticality: Criticality,
    pub value: HandoverCommandProtocolIEs_EntryValue,
}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "0",
    sz_ub = "65535"
)]
pub struct HandoverCommandProtocolIEs(Vec<HandoverCommandProtocolIEs_Entry>);

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum HandoverFailureProtocolIEs_EntryValue {
    #[asn(key = 2)]
    Id_Cause(Cause),
    #[asn(key = 58)]
    Id_CriticalityDiagnostics(CriticalityDiagnostics),
    #[asn(key = 0)]
    Id_MME_UE_S1AP_ID(MME_UE_S1AP_ID),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct HandoverFailureProtocolIEs_Entry {
    #[asn(key_field = true)]
    pub id: ProtocolIE_ID,
    pub criticality: Criticality,
    pub value: HandoverFailureProtocolIEs_EntryValue,
}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "0",
    sz_ub = "65535"
)]
pub struct HandoverFailureProtocolIEs(Vec<HandoverFailureProtocolIEs_Entry>);

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum HandoverNotifyProtocolIEs_EntryValue {
    #[asn(key = 100)]
    Id_EUTRAN_CGI(EUTRAN_CGI),
    #[asn(key = 186)]
    Id_LHN_ID(LHN_ID),
    #[asn(key = 0)]
    Id_MME_UE_S1AP_ID(MME_UE_S1AP_ID),
    #[asn(key = 320)]
    Id_NotifySourceeNB(NotifySourceeNB),
    #[asn(key = 288)]
    Id_PSCellInformation(PSCellInformation),
    #[asn(key = 67)]
    Id_TAI(TAI),
    #[asn(key = 176)]
    Id_Tunnel_Information_for_BBF(TunnelInformation),
    #[asn(key = 8)]
    Id_eNB_UE_S1AP_ID(ENB_UE_S1AP_ID),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct HandoverNotifyProtocolIEs_Entry {
    #[asn(key_field = true)]
    pub id: ProtocolIE_ID,
    pub criticality: Criticality,
    pub value: HandoverNotifyProtocolIEs_EntryValue,
}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "0",
    sz_ub = "65535"
)]
pub struct HandoverNotifyProtocolIEs(Vec<HandoverNotifyProtocolIEs_Entry>);

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum HandoverPreparationFailureProtocolIEs_EntryValue {
    #[asn(key = 2)]
    Id_Cause(Cause),
    #[asn(key = 58)]
    Id_CriticalityDiagnostics(CriticalityDiagnostics),
    #[asn(key = 0)]
    Id_MME_UE_S1AP_ID(MME_UE_S1AP_ID),
    #[asn(key = 8)]
    Id_eNB_UE_S1AP_ID(ENB_UE_S1AP_ID),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct HandoverPreparationFailureProtocolIEs_Entry {
    #[asn(key_field = true)]
    pub id: ProtocolIE_ID,
    pub criticality: Criticality,
    pub value: HandoverPreparationFailureProtocolIEs_EntryValue,
}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "0",
    sz_ub = "65535"
)]
pub struct HandoverPreparationFailureProtocolIEs(Vec<HandoverPreparationFailureProtocolIEs_Entry>);

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum HandoverRequestProtocolIEs_EntryValue {
    #[asn(key = 299)]
    Id_AdditionalRRMPriorityIndex(AdditionalRRMPriorityIndex),
    #[asn(key = 277)]
    Id_AerialUEsubscriptionInformation(AerialUEsubscriptionInformation),
    #[asn(key = 271)]
    Id_CE_ModeBRestricted(CE_ModeBRestricted),
    #[asn(key = 127)]
    Id_CSG_Id(CSG_Id),
    #[asn(key = 146)]
    Id_CSGMembershipStatus(CSGMembershipStatus),
    #[asn(key = 2)]
    Id_Cause(Cause),
    #[asn(key = 53)]
    Id_E_RABToBeSetupListHOReq(E_RABToBeSetupListHOReq),
    #[asn(key = 251)]
    Id_EnhancedCoverageRestricted(EnhancedCoverageRestricted),
    #[asn(key = 196)]
    Id_ExpectedUEBehaviour(ExpectedUEBehaviour),
    #[asn(key = 75)]
    Id_GUMMEI_ID(GUMMEI),
    #[asn(key = 41)]
    Id_HandoverRestrictionList(HandoverRestrictionList),
    #[asn(key = 1)]
    Id_HandoverType(HandoverType),
    #[asn(key = 301)]
    Id_IAB_Authorized(IAB_Authorized),
    #[asn(key = 0)]
    Id_MME_UE_S1AP_ID(MME_UE_S1AP_ID),
    #[asn(key = 158)]
    Id_MME_UE_S1AP_ID_2(MME_UE_S1AP_ID),
    #[asn(key = 165)]
    Id_ManagementBasedMDTAllowed(ManagementBasedMDTAllowed),
    #[asn(key = 177)]
    Id_ManagementBasedMDTPLMNList(MDTPLMNList),
    #[asn(key = 192)]
    Id_Masked_IMEISV(Masked_IMEISV),
    #[asn(key = 136)]
    Id_NASSecurityParameterstoE_UTRAN(NASSecurityParameterstoE_UTRAN),
    #[asn(key = 269)]
    Id_NRUESecurityCapabilities(NRUESecurityCapabilities),
    #[asn(key = 307)]
    Id_NRUESidelinkAggregateMaximumBitrate(NRUESidelinkAggregateMaximumBitrate),
    #[asn(key = 306)]
    Id_NRV2XServicesAuthorized(NRV2XServicesAuthorized),
    #[asn(key = 308)]
    Id_PC5QoSParameters(PC5QoSParameters),
    #[asn(key = 283)]
    Id_PendingDataIndication(PendingDataIndication),
    #[asn(key = 195)]
    Id_ProSeAuthorized(ProSeAuthorized),
    #[asn(key = 98)]
    Id_RequestType(RequestType),
    #[asn(key = 124)]
    Id_SRVCCOperationPossible(SRVCCOperationPossible),
    #[asn(key = 40)]
    Id_SecurityContext(SecurityContext),
    #[asn(key = 104)]
    Id_Source_ToTarget_TransparentContainer(Source_ToTarget_TransparentContainer),
    #[asn(key = 278)]
    Id_Subscription_Based_UE_DifferentiationInfo(Subscription_Based_UE_DifferentiationInfo),
    #[asn(key = 25)]
    Id_TraceActivation(TraceActivation),
    #[asn(key = 314)]
    Id_UERadioCapabilityID(UERadioCapabilityID),
    #[asn(key = 107)]
    Id_UESecurityCapabilities(UESecurityCapabilities),
    #[asn(key = 248)]
    Id_UESidelinkAggregateMaximumBitrate(UESidelinkAggregateMaximumBitrate),
    #[asn(key = 241)]
    Id_UEUserPlaneCIoTSupportIndicator(UEUserPlaneCIoTSupportIndicator),
    #[asn(key = 240)]
    Id_V2XServicesAuthorized(V2XServicesAuthorized),
    #[asn(key = 66)]
    Id_uEaggregateMaximumBitrate(UEAggregateMaximumBitrate),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct HandoverRequestProtocolIEs_Entry {
    #[asn(key_field = true)]
    pub id: ProtocolIE_ID,
    pub criticality: Criticality,
    pub value: HandoverRequestProtocolIEs_EntryValue,
}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "0",
    sz_ub = "65535"
)]
pub struct HandoverRequestProtocolIEs(Vec<HandoverRequestProtocolIEs_Entry>);

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum HandoverRequestAcknowledgeProtocolIEs_EntryValue {
    #[asn(key = 242)]
    Id_CE_mode_B_SupportIndicator(CE_mode_B_SupportIndicator),
    #[asn(key = 127)]
    Id_CSG_Id(CSG_Id),
    #[asn(key = 145)]
    Id_CellAccessMode(CellAccessMode),
    #[asn(key = 58)]
    Id_CriticalityDiagnostics(CriticalityDiagnostics),
    #[asn(key = 18)]
    Id_E_RABAdmittedList(E_RABAdmittedList),
    #[asn(key = 19)]
    Id_E_RABFailedToSetupListHOReqAck(E_RABFailedtoSetupListHOReqAck),
    #[asn(key = 0)]
    Id_MME_UE_S1AP_ID(MME_UE_S1AP_ID),
    #[asn(key = 123)]
    Id_Target_ToSource_TransparentContainer(Target_ToSource_TransparentContainer),
    #[asn(key = 8)]
    Id_eNB_UE_S1AP_ID(ENB_UE_S1AP_ID),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct HandoverRequestAcknowledgeProtocolIEs_Entry {
    #[asn(key_field = true)]
    pub id: ProtocolIE_ID,
    pub criticality: Criticality,
    pub value: HandoverRequestAcknowledgeProtocolIEs_EntryValue,
}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "0",
    sz_ub = "65535"
)]
pub struct HandoverRequestAcknowledgeProtocolIEs(Vec<HandoverRequestAcknowledgeProtocolIEs_Entry>);

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum HandoverRequiredProtocolIEs_EntryValue {
    #[asn(key = 127)]
    Id_CSG_Id(CSG_Id),
    #[asn(key = 2)]
    Id_Cause(Cause),
    #[asn(key = 145)]
    Id_CellAccessMode(CellAccessMode),
    #[asn(key = 79)]
    Id_Direct_Forwarding_Path_Availability(Direct_Forwarding_Path_Availability),
    #[asn(key = 1)]
    Id_HandoverType(HandoverType),
    #[asn(key = 0)]
    Id_MME_UE_S1AP_ID(MME_UE_S1AP_ID),
    #[asn(key = 132)]
    Id_MSClassmark2(MSClassmark2),
    #[asn(key = 133)]
    Id_MSClassmark3(MSClassmark3),
    #[asn(key = 150)]
    Id_PS_ServiceNotAvailable(PS_ServiceNotAvailable),
    #[asn(key = 125)]
    Id_SRVCCHOIndication(SRVCCHOIndication),
    #[asn(key = 104)]
    Id_Source_ToTarget_TransparentContainer(Source_ToTarget_TransparentContainer),
    #[asn(key = 138)]
    Id_Source_ToTarget_TransparentContainer_Secondary(Source_ToTarget_TransparentContainer),
    #[asn(key = 4)]
    Id_TargetID(TargetID),
    #[asn(key = 8)]
    Id_eNB_UE_S1AP_ID(ENB_UE_S1AP_ID),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct HandoverRequiredProtocolIEs_Entry {
    #[asn(key_field = true)]
    pub id: ProtocolIE_ID,
    pub criticality: Criticality,
    pub value: HandoverRequiredProtocolIEs_EntryValue,
}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "0",
    sz_ub = "65535"
)]
pub struct HandoverRequiredProtocolIEs(Vec<HandoverRequiredProtocolIEs_Entry>);

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum HandoverRestrictionListIE_Extensions_EntryExtensionValue {
    #[asn(key = 282)]
    Id_CNTypeRestrictions(CNTypeRestrictions),
    #[asn(key = 290)]
    Id_LastNG_RANPLMNIdentity(PLMNidentity),
    #[asn(key = 287)]
    Id_NRrestrictionin5GS(NRrestrictionin5GS),
    #[asn(key = 261)]
    Id_NRrestrictioninEPSasSecondaryRAT(NRrestrictioninEPSasSecondaryRAT),
    #[asn(key = 270)]
    Id_UnlicensedSpectrumRestriction(UnlicensedSpectrumRestriction),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct HandoverRestrictionListIE_Extensions_Entry {
    #[asn(key_field = true)]
    pub id: ProtocolExtensionID,
    pub criticality: Criticality,
    pub extension_value: HandoverRestrictionListIE_Extensions_EntryExtensionValue,
}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct HandoverRestrictionListIE_Extensions(Vec<HandoverRestrictionListIE_Extensions_Entry>);

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum HandoverSuccessProtocolIEs_EntryValue {
    #[asn(key = 0)]
    Id_MME_UE_S1AP_ID(MME_UE_S1AP_ID),
    #[asn(key = 8)]
    Id_eNB_UE_S1AP_ID(ENB_UE_S1AP_ID),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct HandoverSuccessProtocolIEs_Entry {
    #[asn(key_field = true)]
    pub id: ProtocolIE_ID,
    pub criticality: Criticality,
    pub value: HandoverSuccessProtocolIEs_EntryValue,
}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "0",
    sz_ub = "65535"
)]
pub struct HandoverSuccessProtocolIEs(Vec<HandoverSuccessProtocolIEs_Entry>);

#[derive(Debug, AperCodec)]
#[asn(type = "OCTET-STRING")]
pub struct OCTET_STRING_22(Vec<u8>);

#[derive(Debug, AperCodec)]
#[asn(type = "OCTET-STRING")]
pub struct OCTET_STRING_23(Vec<u8>);

#[derive(Debug, AperCodec)]
#[asn(type = "OCTET-STRING")]
pub struct OCTET_STRING_24(Vec<u8>);

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum ImmediateMDTIE_Extensions_EntryExtensionValue {
    #[asn(key = 284)]
    Id_BluetoothMeasurementConfiguration(BluetoothMeasurementConfiguration),
    #[asn(key = 171)]
    Id_M3Configuration(M3Configuration),
    #[asn(key = 172)]
    Id_M4Configuration(M4Configuration),
    #[asn(key = 173)]
    Id_M5Configuration(M5Configuration),
    #[asn(key = 220)]
    Id_M6Configuration(M6Configuration),
    #[asn(key = 221)]
    Id_M7Configuration(M7Configuration),
    #[asn(key = 174)]
    Id_MDT_Location_Info(MDT_Location_Info),
    #[asn(key = 285)]
    Id_WLANMeasurementConfiguration(WLANMeasurementConfiguration),
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
#[asn(type = "SEQUENCE", extensible = false)]
pub struct InformationOnRecommendedCellsAndENBsForPagingIE_Extensions_Entry {}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct InformationOnRecommendedCellsAndENBsForPagingIE_Extensions(
    Vec<InformationOnRecommendedCellsAndENBsForPagingIE_Extensions_Entry>,
);

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum InitialContextSetupFailureProtocolIEs_EntryValue {
    #[asn(key = 2)]
    Id_Cause(Cause),
    #[asn(key = 58)]
    Id_CriticalityDiagnostics(CriticalityDiagnostics),
    #[asn(key = 0)]
    Id_MME_UE_S1AP_ID(MME_UE_S1AP_ID),
    #[asn(key = 8)]
    Id_eNB_UE_S1AP_ID(ENB_UE_S1AP_ID),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct InitialContextSetupFailureProtocolIEs_Entry {
    #[asn(key_field = true)]
    pub id: ProtocolIE_ID,
    pub criticality: Criticality,
    pub value: InitialContextSetupFailureProtocolIEs_EntryValue,
}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "0",
    sz_ub = "65535"
)]
pub struct InitialContextSetupFailureProtocolIEs(Vec<InitialContextSetupFailureProtocolIEs_Entry>);

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum InitialContextSetupRequestProtocolIEs_EntryValue {
    #[asn(key = 187)]
    Id_AdditionalCSFallbackIndicator(AdditionalCSFallbackIndicator),
    #[asn(key = 299)]
    Id_AdditionalRRMPriorityIndex(AdditionalRRMPriorityIndex),
    #[asn(key = 277)]
    Id_AerialUEsubscriptionInformation(AerialUEsubscriptionInformation),
    #[asn(key = 271)]
    Id_CE_ModeBRestricted(CE_ModeBRestricted),
    #[asn(key = 108)]
    Id_CSFallbackIndicator(CSFallbackIndicator),
    #[asn(key = 146)]
    Id_CSGMembershipStatus(CSGMembershipStatus),
    #[asn(key = 24)]
    Id_E_RABToBeSetupListCtxtSUReq(E_RABToBeSetupListCtxtSUReq),
    #[asn(key = 251)]
    Id_EnhancedCoverageRestricted(EnhancedCoverageRestricted),
    #[asn(key = 196)]
    Id_ExpectedUEBehaviour(ExpectedUEBehaviour),
    #[asn(key = 75)]
    Id_GUMMEI_ID(GUMMEI),
    #[asn(key = 41)]
    Id_HandoverRestrictionList(HandoverRestrictionList),
    #[asn(key = 301)]
    Id_IAB_Authorized(IAB_Authorized),
    #[asn(key = 0)]
    Id_MME_UE_S1AP_ID(MME_UE_S1AP_ID),
    #[asn(key = 158)]
    Id_MME_UE_S1AP_ID_2(MME_UE_S1AP_ID),
    #[asn(key = 165)]
    Id_ManagementBasedMDTAllowed(ManagementBasedMDTAllowed),
    #[asn(key = 177)]
    Id_ManagementBasedMDTPLMNList(MDTPLMNList),
    #[asn(key = 192)]
    Id_Masked_IMEISV(Masked_IMEISV),
    #[asn(key = 269)]
    Id_NRUESecurityCapabilities(NRUESecurityCapabilities),
    #[asn(key = 307)]
    Id_NRUESidelinkAggregateMaximumBitrate(NRUESidelinkAggregateMaximumBitrate),
    #[asn(key = 306)]
    Id_NRV2XServicesAuthorized(NRV2XServicesAuthorized),
    #[asn(key = 308)]
    Id_PC5QoSParameters(PC5QoSParameters),
    #[asn(key = 283)]
    Id_PendingDataIndication(PendingDataIndication),
    #[asn(key = 195)]
    Id_ProSeAuthorized(ProSeAuthorized),
    #[asn(key = 159)]
    Id_RegisteredLAI(LAI),
    #[asn(key = 124)]
    Id_SRVCCOperationPossible(SRVCCOperationPossible),
    #[asn(key = 73)]
    Id_SecurityKey(SecurityKey),
    #[asn(key = 106)]
    Id_SubscriberProfileIDforRFP(SubscriberProfileIDforRFP),
    #[asn(key = 278)]
    Id_Subscription_Based_UE_DifferentiationInfo(Subscription_Based_UE_DifferentiationInfo),
    #[asn(key = 25)]
    Id_TraceActivation(TraceActivation),
    #[asn(key = 74)]
    Id_UERadioCapability(UERadioCapability),
    #[asn(key = 314)]
    Id_UERadioCapabilityID(UERadioCapabilityID),
    #[asn(key = 107)]
    Id_UESecurityCapabilities(UESecurityCapabilities),
    #[asn(key = 248)]
    Id_UESidelinkAggregateMaximumBitrate(UESidelinkAggregateMaximumBitrate),
    #[asn(key = 241)]
    Id_UEUserPlaneCIoTSupportIndicator(UEUserPlaneCIoTSupportIndicator),
    #[asn(key = 240)]
    Id_V2XServicesAuthorized(V2XServicesAuthorized),
    #[asn(key = 8)]
    Id_eNB_UE_S1AP_ID(ENB_UE_S1AP_ID),
    #[asn(key = 66)]
    Id_uEaggregateMaximumBitrate(UEAggregateMaximumBitrate),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct InitialContextSetupRequestProtocolIEs_Entry {
    #[asn(key_field = true)]
    pub id: ProtocolIE_ID,
    pub criticality: Criticality,
    pub value: InitialContextSetupRequestProtocolIEs_EntryValue,
}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "0",
    sz_ub = "65535"
)]
pub struct InitialContextSetupRequestProtocolIEs(Vec<InitialContextSetupRequestProtocolIEs_Entry>);

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum InitialContextSetupResponseProtocolIEs_EntryValue {
    #[asn(key = 58)]
    Id_CriticalityDiagnostics(CriticalityDiagnostics),
    #[asn(key = 48)]
    Id_E_RABFailedToSetupListCtxtSURes(E_RABList),
    #[asn(key = 51)]
    Id_E_RABSetupListCtxtSURes(E_RABSetupListCtxtSURes),
    #[asn(key = 0)]
    Id_MME_UE_S1AP_ID(MME_UE_S1AP_ID),
    #[asn(key = 8)]
    Id_eNB_UE_S1AP_ID(ENB_UE_S1AP_ID),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct InitialContextSetupResponseProtocolIEs_Entry {
    #[asn(key_field = true)]
    pub id: ProtocolIE_ID,
    pub criticality: Criticality,
    pub value: InitialContextSetupResponseProtocolIEs_EntryValue,
}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "0",
    sz_ub = "65535"
)]
pub struct InitialContextSetupResponseProtocolIEs(
    Vec<InitialContextSetupResponseProtocolIEs_Entry>,
);

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum InitialUEMessageProtocolIEs_EntryValue {
    #[asn(key = 242)]
    Id_CE_mode_B_SupportIndicator(CE_mode_B_SupportIndicator),
    #[asn(key = 127)]
    Id_CSG_Id(CSG_Id),
    #[asn(key = 145)]
    Id_CellAccessMode(CellAccessMode),
    #[asn(key = 250)]
    Id_Coverage_Level(Coverage_Level),
    #[asn(key = 246)]
    Id_DCN_ID(DCN_ID),
    #[asn(key = 281)]
    Id_EDT_Session(EDT_Session),
    #[asn(key = 100)]
    Id_EUTRAN_CGI(EUTRAN_CGI),
    #[asn(key = 75)]
    Id_GUMMEI_ID(GUMMEI),
    #[asn(key = 170)]
    Id_GUMMEIType(GUMMEIType),
    #[asn(key = 155)]
    Id_GW_TransportLayerAddress(TransportLayerAddress),
    #[asn(key = 302)]
    Id_IAB_Node_Indication(IAB_Node_Indication),
    #[asn(key = 186)]
    Id_LHN_ID(LHN_ID),
    #[asn(key = 223)]
    Id_MME_Group_ID(MME_Group_ID),
    #[asn(key = 26)]
    Id_NAS_PDU(NAS_PDU),
    #[asn(key = 134)]
    Id_RRC_Establishment_Cause(RRC_Establishment_Cause),
    #[asn(key = 160)]
    Id_RelayNode_Indicator(RelayNode_Indicator),
    #[asn(key = 96)]
    Id_S_TMSI(S_TMSI),
    #[asn(key = 184)]
    Id_SIPTO_L_GW_TransportLayerAddress(TransportLayerAddress),
    #[asn(key = 67)]
    Id_TAI(TAI),
    #[asn(key = 176)]
    Id_Tunnel_Information_for_BBF(TunnelInformation),
    #[asn(key = 263)]
    Id_UE_Application_Layer_Measurement_Capability(UE_Application_Layer_Measurement_Capability),
    #[asn(key = 230)]
    Id_UE_Usage_Type(UE_Usage_Type),
    #[asn(key = 8)]
    Id_eNB_UE_S1AP_ID(ENB_UE_S1AP_ID),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct InitialUEMessageProtocolIEs_Entry {
    #[asn(key_field = true)]
    pub id: ProtocolIE_ID,
    pub criticality: Criticality,
    pub value: InitialUEMessageProtocolIEs_EntryValue,
}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "0",
    sz_ub = "65535"
)]
pub struct InitialUEMessageProtocolIEs(Vec<InitialUEMessageProtocolIEs_Entry>);

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum InitiatingMessageValue {
    #[asn(key = 42)]
    Id_CellTrafficTrace(CellTrafficTrace),
    #[asn(key = 54)]
    Id_ConnectionEstablishmentIndication(ConnectionEstablishmentIndication),
    #[asn(key = 26)]
    Id_DeactivateTrace(DeactivateTrace),
    #[asn(key = 19)]
    Id_DownlinkS1cdma2000tunnelling(DownlinkS1cdma2000tunnelling),
    #[asn(key = 50)]
    Id_E_RABModificationIndication(E_RABModificationIndication),
    #[asn(key = 6)]
    Id_E_RABModify(E_RABModifyRequest),
    #[asn(key = 7)]
    Id_E_RABRelease(E_RABReleaseCommand),
    #[asn(key = 8)]
    Id_E_RABReleaseIndication(E_RABReleaseIndication),
    #[asn(key = 5)]
    Id_E_RABSetup(E_RABSetupRequest),
    #[asn(key = 29)]
    Id_ENBConfigurationUpdate(ENBConfigurationUpdate),
    #[asn(key = 15)]
    Id_ErrorIndication(ErrorIndication),
    #[asn(key = 4)]
    Id_HandoverCancel(HandoverCancel),
    #[asn(key = 2)]
    Id_HandoverNotification(HandoverNotify),
    #[asn(key = 0)]
    Id_HandoverPreparation(HandoverRequired),
    #[asn(key = 1)]
    Id_HandoverResourceAllocation(HandoverRequest),
    #[asn(key = 64)]
    Id_HandoverSuccess(HandoverSuccess),
    #[asn(key = 9)]
    Id_InitialContextSetup(InitialContextSetupRequest),
    #[asn(key = 43)]
    Id_Kill(KillRequest),
    #[asn(key = 33)]
    Id_LocationReport(LocationReport),
    #[asn(key = 31)]
    Id_LocationReportingControl(LocationReportingControl),
    #[asn(key = 32)]
    Id_LocationReportingFailureIndication(LocationReportingFailureIndication),
    #[asn(key = 61)]
    Id_MMECPRelocationIndication(MMECPRelocationIndication),
    #[asn(key = 41)]
    Id_MMEConfigurationTransfer(MMEConfigurationTransfer),
    #[asn(key = 30)]
    Id_MMEConfigurationUpdate(MMEConfigurationUpdate),
    #[asn(key = 38)]
    Id_MMEDirectInformationTransfer(MMEDirectInformationTransfer),
    #[asn(key = 66)]
    Id_MMEEarlyStatusTransfer(MMEEarlyStatusTransfer),
    #[asn(key = 25)]
    Id_MMEStatusTransfer(MMEStatusTransfer),
    #[asn(key = 57)]
    Id_NASDeliveryIndication(NASDeliveryIndication),
    #[asn(key = 16)]
    Id_NASNonDeliveryIndication(NASNonDeliveryIndication),
    #[asn(key = 34)]
    Id_OverloadStart(OverloadStart),
    #[asn(key = 35)]
    Id_OverloadStop(OverloadStop),
    #[asn(key = 51)]
    Id_PWSFailureIndication(PWSFailureIndication),
    #[asn(key = 49)]
    Id_PWSRestartIndication(PWSRestartIndication),
    #[asn(key = 10)]
    Id_Paging(Paging),
    #[asn(key = 3)]
    Id_PathSwitchRequest(PathSwitchRequest),
    #[asn(key = 39)]
    Id_PrivateMessage(PrivateMessage),
    #[asn(key = 52)]
    Id_RerouteNASRequest(RerouteNASRequest),
    #[asn(key = 14)]
    Id_Reset(Reset),
    #[asn(key = 58)]
    Id_RetrieveUEInformation(RetrieveUEInformation),
    #[asn(key = 17)]
    Id_S1Setup(S1SetupRequest),
    #[asn(key = 62)]
    Id_SecondaryRATDataUsageReport(SecondaryRATDataUsageReport),
    #[asn(key = 28)]
    Id_TraceFailureIndication(TraceFailureIndication),
    #[asn(key = 27)]
    Id_TraceStart(TraceStart),
    #[asn(key = 22)]
    Id_UECapabilityInfoIndication(UECapabilityInfoIndication),
    #[asn(key = 21)]
    Id_UEContextModification(UEContextModificationRequest),
    #[asn(key = 53)]
    Id_UEContextModificationIndication(UEContextModificationIndication),
    #[asn(key = 23)]
    Id_UEContextRelease(UEContextReleaseCommand),
    #[asn(key = 18)]
    Id_UEContextReleaseRequest(UEContextReleaseRequest),
    #[asn(key = 56)]
    Id_UEContextResume(UEContextResumeRequest),
    #[asn(key = 55)]
    Id_UEContextSuspend(UEContextSuspendRequest),
    #[asn(key = 59)]
    Id_UEInformationTransfer(UEInformationTransfer),
    #[asn(key = 63)]
    Id_UERadioCapabilityIDMapping(UERadioCapabilityIDMappingRequest),
    #[asn(key = 48)]
    Id_UERadioCapabilityMatch(UERadioCapabilityMatchRequest),
    #[asn(key = 20)]
    Id_UplinkS1cdma2000tunnelling(UplinkS1cdma2000tunnelling),
    #[asn(key = 36)]
    Id_WriteReplaceWarning(WriteReplaceWarningRequest),
    #[asn(key = 11)]
    Id_downlinkNASTransport(DownlinkNASTransport),
    #[asn(key = 46)]
    Id_downlinkNonUEAssociatedLPPaTransport(DownlinkNonUEAssociatedLPPaTransport),
    #[asn(key = 44)]
    Id_downlinkUEAssociatedLPPaTransport(DownlinkUEAssociatedLPPaTransport),
    #[asn(key = 60)]
    Id_eNBCPRelocationIndication(ENBCPRelocationIndication),
    #[asn(key = 40)]
    Id_eNBConfigurationTransfer(ENBConfigurationTransfer),
    #[asn(key = 37)]
    Id_eNBDirectInformationTransfer(ENBDirectInformationTransfer),
    #[asn(key = 65)]
    Id_eNBEarlyStatusTransfer(ENBEarlyStatusTransfer),
    #[asn(key = 24)]
    Id_eNBStatusTransfer(ENBStatusTransfer),
    #[asn(key = 12)]
    Id_initialUEMessage(InitialUEMessage),
    #[asn(key = 13)]
    Id_uplinkNASTransport(UplinkNASTransport),
    #[asn(key = 47)]
    Id_uplinkNonUEAssociatedLPPaTransport(UplinkNonUEAssociatedLPPaTransport),
    #[asn(key = 45)]
    Id_uplinkUEAssociatedLPPaTransport(UplinkUEAssociatedLPPaTransport),
}

#[derive(Debug, AperCodec)]
#[asn(type = "INTEGER", lb = "1", ub = "1024")]
pub struct INTEGER_25(u16);

#[derive(Debug, AperCodec)]
#[asn(type = "INTEGER", lb = "0", ub = "32")]
pub struct INTEGER_26(u8);

#[derive(Debug, AperCodec)]
#[asn(type = "ENUMERATED", extensible = true, lb = "0", ub = "4")]
pub struct ENUMERATED_27(u8);
impl ENUMERATED_27 {
    const K_HZ15: u8 = 0u8;
    const K_HZ30: u8 = 1u8;
    const K_HZ60: u8 = 2u8;
    const K_HZ120: u8 = 3u8;
    const K_HZ240: u8 = 4u8;
}

#[derive(Debug, AperCodec)]
#[asn(type = "INTEGER", lb = "1", ub = "16")]
pub struct INTEGER_28(u8);

#[derive(Debug, AperCodec)]
#[asn(type = "OCTET-STRING")]
pub struct OCTET_STRING_29(Vec<u8>);

#[derive(Debug, AperCodec)]
#[asn(type = "OCTET-STRING")]
pub struct OCTET_STRING_30(Vec<u8>);

#[derive(Debug, AperCodec)]
#[asn(type = "OCTET-STRING")]
pub struct OCTET_STRING_31(Vec<u8>);

#[derive(Debug, AperCodec)]
#[asn(type = "OCTET-STRING")]
pub struct OCTET_STRING_32(Vec<u8>);

#[derive(Debug, AperCodec)]
#[asn(type = "OCTET-STRING")]
pub struct OCTET_STRING_33(Vec<u8>);

#[derive(Debug, AperCodec)]
#[asn(type = "OCTET-STRING")]
pub struct OCTET_STRING_34(Vec<u8>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct InterSystemMeasurementItemIE_Extensions_Entry {}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct InterSystemMeasurementItemIE_Extensions(
    Vec<InterSystemMeasurementItemIE_Extensions_Entry>,
);

#[derive(Debug, AperCodec)]
#[asn(type = "INTEGER", lb = "1", ub = "100")]
pub struct INTEGER_35(u8);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct InterSystemMeasurementParametersIE_Extensions_Entry {}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct InterSystemMeasurementParametersIE_Extensions(
    Vec<InterSystemMeasurementParametersIE_Extensions_Entry>,
);

#[derive(Debug, AperCodec)]
#[asn(type = "INTEGER", lb = "0", ub = "127")]
pub struct INTEGER_36(u8);

#[derive(Debug, AperCodec)]
#[asn(type = "INTEGER", lb = "0", ub = "127")]
pub struct INTEGER_37(u8);

#[derive(Debug, AperCodec)]
#[asn(type = "INTEGER", lb = "0", ub = "127")]
pub struct INTEGER_38(u8);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct IntersystemMeasurementConfigurationIE_Extensions_Entry {}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct IntersystemMeasurementConfigurationIE_Extensions(
    Vec<IntersystemMeasurementConfigurationIE_Extensions_Entry>,
);

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum KillRequestProtocolIEs_EntryValue {
    #[asn(key = 191)]
    Id_KillAllWarningMessages(KillAllWarningMessages),
    #[asn(key = 111)]
    Id_MessageIdentifier(MessageIdentifier),
    #[asn(key = 112)]
    Id_SerialNumber(SerialNumber),
    #[asn(key = 113)]
    Id_WarningAreaList(WarningAreaList),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct KillRequestProtocolIEs_Entry {
    #[asn(key_field = true)]
    pub id: ProtocolIE_ID,
    pub criticality: Criticality,
    pub value: KillRequestProtocolIEs_EntryValue,
}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "0",
    sz_ub = "65535"
)]
pub struct KillRequestProtocolIEs(Vec<KillRequestProtocolIEs_Entry>);

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum KillResponseProtocolIEs_EntryValue {
    #[asn(key = 141)]
    Id_BroadcastCancelledAreaList(BroadcastCancelledAreaList),
    #[asn(key = 58)]
    Id_CriticalityDiagnostics(CriticalityDiagnostics),
    #[asn(key = 111)]
    Id_MessageIdentifier(MessageIdentifier),
    #[asn(key = 112)]
    Id_SerialNumber(SerialNumber),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct KillResponseProtocolIEs_Entry {
    #[asn(key_field = true)]
    pub id: ProtocolIE_ID,
    pub criticality: Criticality,
    pub value: KillResponseProtocolIEs_EntryValue,
}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "0",
    sz_ub = "65535"
)]
pub struct KillResponseProtocolIEs(Vec<KillResponseProtocolIEs_Entry>);

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
#[asn(type = "OPEN")]
pub enum LastVisitedEUTRANCellInformationIE_Extensions_EntryExtensionValue {
    #[asn(key = 168)]
    Id_HO_Cause(Cause),
    #[asn(key = 167)]
    Id_Time_UE_StayedInCell_EnhancedGranularity(Time_UE_StayedInCell_EnhancedGranularity),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct LastVisitedEUTRANCellInformationIE_Extensions_Entry {
    #[asn(key_field = true)]
    pub id: ProtocolExtensionID,
    pub criticality: Criticality,
    pub extension_value: LastVisitedEUTRANCellInformationIE_Extensions_EntryExtensionValue,
}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct LastVisitedEUTRANCellInformationIE_Extensions(
    Vec<LastVisitedEUTRANCellInformationIE_Extensions_Entry>,
);

#[derive(Debug, AperCodec)]
#[asn(type = "NULL")]
pub struct NULL_39;

#[derive(Debug, AperCodec)]
#[asn(type = "ENUMERATED", extensible = true, lb = "0", ub = "3")]
pub struct ENUMERATED_40(u8);
impl ENUMERATED_40 {
    const MS1280: u8 = 0u8;
    const MS2560: u8 = 1u8;
    const MS5120: u8 = 2u8;
    const MS10240: u8 = 3u8;
}

#[derive(Debug, AperCodec)]
#[asn(type = "INTEGER", lb = "0", ub = "10239", extensible = true)]
pub struct INTEGER_41(u16);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct ListeningSubframePatternIE_Extensions_Entry {}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct ListeningSubframePatternIE_Extensions(Vec<ListeningSubframePatternIE_Extensions_Entry>);

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum LocationReportProtocolIEs_EntryValue {
    #[asn(key = 100)]
    Id_EUTRAN_CGI(EUTRAN_CGI),
    #[asn(key = 0)]
    Id_MME_UE_S1AP_ID(MME_UE_S1AP_ID),
    #[asn(key = 288)]
    Id_PSCellInformation(PSCellInformation),
    #[asn(key = 98)]
    Id_RequestType(RequestType),
    #[asn(key = 67)]
    Id_TAI(TAI),
    #[asn(key = 8)]
    Id_eNB_UE_S1AP_ID(ENB_UE_S1AP_ID),
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
pub enum LocationReportingControlProtocolIEs_EntryValue {
    #[asn(key = 0)]
    Id_MME_UE_S1AP_ID(MME_UE_S1AP_ID),
    #[asn(key = 98)]
    Id_RequestType(RequestType),
    #[asn(key = 8)]
    Id_eNB_UE_S1AP_ID(ENB_UE_S1AP_ID),
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
pub enum LocationReportingFailureIndicationProtocolIEs_EntryValue {
    #[asn(key = 2)]
    Id_Cause(Cause),
    #[asn(key = 0)]
    Id_MME_UE_S1AP_ID(MME_UE_S1AP_ID),
    #[asn(key = 8)]
    Id_eNB_UE_S1AP_ID(ENB_UE_S1AP_ID),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct LocationReportingFailureIndicationProtocolIEs_Entry {
    #[asn(key_field = true)]
    pub id: ProtocolIE_ID,
    pub criticality: Criticality,
    pub value: LocationReportingFailureIndicationProtocolIEs_EntryValue,
}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "0",
    sz_ub = "65535"
)]
pub struct LocationReportingFailureIndicationProtocolIEs(
    Vec<LocationReportingFailureIndicationProtocolIEs_Entry>,
);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct LoggedMBSFNMDTIE_Extensions_Entry {}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct LoggedMBSFNMDTIE_Extensions(Vec<LoggedMBSFNMDTIE_Extensions_Entry>);

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum LoggedMDTIE_Extensions_EntryExtensionValue {
    #[asn(key = 284)]
    Id_BluetoothMeasurementConfiguration(BluetoothMeasurementConfiguration),
    #[asn(key = 285)]
    Id_WLANMeasurementConfiguration(WLANMeasurementConfiguration),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct LoggedMDTIE_Extensions_Entry {
    #[asn(key_field = true)]
    pub id: ProtocolExtensionID,
    pub criticality: Criticality,
    pub extension_value: LoggedMDTIE_Extensions_EntryExtensionValue,
}

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
pub struct M1PeriodicReportingIE_Extensions_Entry {}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct M1PeriodicReportingIE_Extensions(Vec<M1PeriodicReportingIE_Extensions_Entry>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct M1ThresholdEventA2IE_Extensions_Entry {}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct M1ThresholdEventA2IE_Extensions(Vec<M1ThresholdEventA2IE_Extensions_Entry>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct M3ConfigurationIE_Extensions_Entry {}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct M3ConfigurationIE_Extensions(Vec<M3ConfigurationIE_Extensions_Entry>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct M4ConfigurationIE_Extensions_Entry {}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct M4ConfigurationIE_Extensions(Vec<M4ConfigurationIE_Extensions_Entry>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct M5ConfigurationIE_Extensions_Entry {}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct M5ConfigurationIE_Extensions(Vec<M5ConfigurationIE_Extensions_Entry>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct M6ConfigurationIE_Extensions_Entry {}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct M6ConfigurationIE_Extensions(Vec<M6ConfigurationIE_Extensions_Entry>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct M7ConfigurationIE_Extensions_Entry {}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct M7ConfigurationIE_Extensions(Vec<M7ConfigurationIE_Extensions_Entry>);

#[derive(Debug, AperCodec)]
#[asn(type = "INTEGER", lb = "0", ub = "255")]
pub struct INTEGER_42(u8);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct MBSFN_ResultToLogInfoIE_Extensions_Entry {}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct MBSFN_ResultToLogInfoIE_Extensions(Vec<MBSFN_ResultToLogInfoIE_Extensions_Entry>);

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum MDT_ConfigurationIE_Extensions_EntryExtensionValue {
    #[asn(key = 178)]
    Id_SignallingBasedMDTPLMNList(MDTPLMNList),
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
#[asn(type = "OPEN")]
pub enum MDTMode_ExtensionValue {
    #[asn(key = 197)]
    Id_LoggedMBSFNMDT(LoggedMBSFNMDT),
}

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum MMECPRelocationIndicationProtocolIEs_EntryValue {
    #[asn(key = 0)]
    Id_MME_UE_S1AP_ID(MME_UE_S1AP_ID),
    #[asn(key = 8)]
    Id_eNB_UE_S1AP_ID(ENB_UE_S1AP_ID),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct MMECPRelocationIndicationProtocolIEs_Entry {
    #[asn(key_field = true)]
    pub id: ProtocolIE_ID,
    pub criticality: Criticality,
    pub value: MMECPRelocationIndicationProtocolIEs_EntryValue,
}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "0",
    sz_ub = "65535"
)]
pub struct MMECPRelocationIndicationProtocolIEs(Vec<MMECPRelocationIndicationProtocolIEs_Entry>);

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum MMEConfigurationTransferProtocolIEs_EntryValue {
    #[asn(key = 295)]
    Id_EN_DCSONConfigurationTransfer_MCT(EN_DCSONConfigurationTransfer),
    #[asn(key = 309)]
    Id_IntersystemSONConfigurationTransferMCT(IntersystemSONConfigurationTransfer),
    #[asn(key = 130)]
    Id_SONConfigurationTransferMCT(SONConfigurationTransfer),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct MMEConfigurationTransferProtocolIEs_Entry {
    #[asn(key_field = true)]
    pub id: ProtocolIE_ID,
    pub criticality: Criticality,
    pub value: MMEConfigurationTransferProtocolIEs_EntryValue,
}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "0",
    sz_ub = "65535"
)]
pub struct MMEConfigurationTransferProtocolIEs(Vec<MMEConfigurationTransferProtocolIEs_Entry>);

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum MMEConfigurationUpdateProtocolIEs_EntryValue {
    #[asn(key = 61)]
    Id_MMEname(MMEname),
    #[asn(key = 87)]
    Id_RelativeMMECapacity(RelativeMMECapacity),
    #[asn(key = 247)]
    Id_ServedDCNs(ServedDCNs),
    #[asn(key = 105)]
    Id_ServedGUMMEIs(ServedGUMMEIs),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct MMEConfigurationUpdateProtocolIEs_Entry {
    #[asn(key_field = true)]
    pub id: ProtocolIE_ID,
    pub criticality: Criticality,
    pub value: MMEConfigurationUpdateProtocolIEs_EntryValue,
}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "0",
    sz_ub = "65535"
)]
pub struct MMEConfigurationUpdateProtocolIEs(Vec<MMEConfigurationUpdateProtocolIEs_Entry>);

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum MMEConfigurationUpdateAcknowledgeProtocolIEs_EntryValue {
    #[asn(key = 58)]
    Id_CriticalityDiagnostics(CriticalityDiagnostics),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct MMEConfigurationUpdateAcknowledgeProtocolIEs_Entry {
    #[asn(key_field = true)]
    pub id: ProtocolIE_ID,
    pub criticality: Criticality,
    pub value: MMEConfigurationUpdateAcknowledgeProtocolIEs_EntryValue,
}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "0",
    sz_ub = "65535"
)]
pub struct MMEConfigurationUpdateAcknowledgeProtocolIEs(
    Vec<MMEConfigurationUpdateAcknowledgeProtocolIEs_Entry>,
);

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum MMEConfigurationUpdateFailureProtocolIEs_EntryValue {
    #[asn(key = 2)]
    Id_Cause(Cause),
    #[asn(key = 58)]
    Id_CriticalityDiagnostics(CriticalityDiagnostics),
    #[asn(key = 65)]
    Id_TimeToWait(TimeToWait),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct MMEConfigurationUpdateFailureProtocolIEs_Entry {
    #[asn(key_field = true)]
    pub id: ProtocolIE_ID,
    pub criticality: Criticality,
    pub value: MMEConfigurationUpdateFailureProtocolIEs_EntryValue,
}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "0",
    sz_ub = "65535"
)]
pub struct MMEConfigurationUpdateFailureProtocolIEs(
    Vec<MMEConfigurationUpdateFailureProtocolIEs_Entry>,
);

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum MMEDirectInformationTransferProtocolIEs_EntryValue {
    #[asn(key = 122)]
    Id_Inter_SystemInformationTransferTypeMDT(Inter_SystemInformationTransferType),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct MMEDirectInformationTransferProtocolIEs_Entry {
    #[asn(key_field = true)]
    pub id: ProtocolIE_ID,
    pub criticality: Criticality,
    pub value: MMEDirectInformationTransferProtocolIEs_EntryValue,
}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "0",
    sz_ub = "65535"
)]
pub struct MMEDirectInformationTransferProtocolIEs(
    Vec<MMEDirectInformationTransferProtocolIEs_Entry>,
);

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum MMEEarlyStatusTransferProtocolIEs_EntryValue {
    #[asn(key = 0)]
    Id_MME_UE_S1AP_ID(MME_UE_S1AP_ID),
    #[asn(key = 321)]
    Id_eNB_EarlyStatusTransfer_TransparentContainer(ENB_EarlyStatusTransfer_TransparentContainer),
    #[asn(key = 8)]
    Id_eNB_UE_S1AP_ID(ENB_UE_S1AP_ID),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct MMEEarlyStatusTransferProtocolIEs_Entry {
    #[asn(key_field = true)]
    pub id: ProtocolIE_ID,
    pub criticality: Criticality,
    pub value: MMEEarlyStatusTransferProtocolIEs_EntryValue,
}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "0",
    sz_ub = "65535"
)]
pub struct MMEEarlyStatusTransferProtocolIEs(Vec<MMEEarlyStatusTransferProtocolIEs_Entry>);

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum MMEStatusTransferProtocolIEs_EntryValue {
    #[asn(key = 0)]
    Id_MME_UE_S1AP_ID(MME_UE_S1AP_ID),
    #[asn(key = 90)]
    Id_eNB_StatusTransfer_TransparentContainer(ENB_StatusTransfer_TransparentContainer),
    #[asn(key = 8)]
    Id_eNB_UE_S1AP_ID(ENB_UE_S1AP_ID),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct MMEStatusTransferProtocolIEs_Entry {
    #[asn(key_field = true)]
    pub id: ProtocolIE_ID,
    pub criticality: Criticality,
    pub value: MMEStatusTransferProtocolIEs_EntryValue,
}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "0",
    sz_ub = "65535"
)]
pub struct MMEStatusTransferProtocolIEs(Vec<MMEStatusTransferProtocolIEs_Entry>);

#[derive(Debug, AperCodec)]
#[asn(type = "OCTET-STRING")]
pub struct OCTET_STRING_43(Vec<u8>);

#[derive(Debug, AperCodec)]
#[asn(type = "OCTET-STRING")]
pub struct OCTET_STRING_44(Vec<u8>);

#[derive(Debug, AperCodec)]
#[asn(type = "ENUMERATED", extensible = true, lb = "0", ub = "4")]
pub struct ENUMERATED_45(u8);
impl ENUMERATED_45 {
    const MS0: u8 = 0u8;
    const MS1280: u8 = 1u8;
    const MS2560: u8 = 2u8;
    const MS5120: u8 = 3u8;
    const MS10240: u8 = 4u8;
}

#[derive(Debug, AperCodec)]
#[asn(type = "INTEGER", lb = "0", ub = "10239", extensible = true)]
pub struct INTEGER_46(u16);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct MutingPatternInformationIE_Extensions_Entry {}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct MutingPatternInformationIE_Extensions(Vec<MutingPatternInformationIE_Extensions_Entry>);

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum NASDeliveryIndicationProtocolIEs_EntryValue {
    #[asn(key = 0)]
    Id_MME_UE_S1AP_ID(MME_UE_S1AP_ID),
    #[asn(key = 8)]
    Id_eNB_UE_S1AP_ID(ENB_UE_S1AP_ID),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct NASDeliveryIndicationProtocolIEs_Entry {
    #[asn(key_field = true)]
    pub id: ProtocolIE_ID,
    pub criticality: Criticality,
    pub value: NASDeliveryIndicationProtocolIEs_EntryValue,
}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "0",
    sz_ub = "65535"
)]
pub struct NASDeliveryIndicationProtocolIEs(Vec<NASDeliveryIndicationProtocolIEs_Entry>);

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum NASNonDeliveryIndicationProtocolIEs_EntryValue {
    #[asn(key = 2)]
    Id_Cause(Cause),
    #[asn(key = 0)]
    Id_MME_UE_S1AP_ID(MME_UE_S1AP_ID),
    #[asn(key = 26)]
    Id_NAS_PDU(NAS_PDU),
    #[asn(key = 8)]
    Id_eNB_UE_S1AP_ID(ENB_UE_S1AP_ID),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct NASNonDeliveryIndicationProtocolIEs_Entry {
    #[asn(key_field = true)]
    pub id: ProtocolIE_ID,
    pub criticality: Criticality,
    pub value: NASNonDeliveryIndicationProtocolIEs_EntryValue,
}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "0",
    sz_ub = "65535"
)]
pub struct NASNonDeliveryIndicationProtocolIEs(Vec<NASNonDeliveryIndicationProtocolIEs_Entry>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct NB_IoT_Paging_eDRXInformationIE_Extensions_Entry {}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct NB_IoT_Paging_eDRXInformationIE_Extensions(
    Vec<NB_IoT_Paging_eDRXInformationIE_Extensions_Entry>,
);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct NG_eNBIE_Extensions_Entry {}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct NG_eNBIE_Extensions(Vec<NG_eNBIE_Extensions_Entry>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct NR_CGIIE_Extensions_Entry {}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct NR_CGIIE_Extensions(Vec<NR_CGIIE_Extensions_Entry>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct NRUESecurityCapabilitiesIE_Extensions_Entry {}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct NRUESecurityCapabilitiesIE_Extensions(Vec<NRUESecurityCapabilitiesIE_Extensions_Entry>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct NRUESidelinkAggregateMaximumBitrateIE_Extensions_Entry {}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct NRUESidelinkAggregateMaximumBitrateIE_Extensions(
    Vec<NRUESidelinkAggregateMaximumBitrateIE_Extensions_Entry>,
);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct NRV2XServicesAuthorizedIE_Extensions_Entry {}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct NRV2XServicesAuthorizedIE_Extensions(Vec<NRV2XServicesAuthorizedIE_Extensions_Entry>);

#[derive(Debug, AperCodec)]
#[asn(type = "OCTET-STRING")]
pub struct OCTET_STRING_47(Vec<u8>);

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum OverloadStartProtocolIEs_EntryValue {
    #[asn(key = 154)]
    Id_GUMMEIList(GUMMEIList),
    #[asn(key = 101)]
    Id_OverloadResponse(OverloadResponse),
    #[asn(key = 161)]
    Id_TrafficLoadReductionIndication(TrafficLoadReductionIndication),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct OverloadStartProtocolIEs_Entry {
    #[asn(key_field = true)]
    pub id: ProtocolIE_ID,
    pub criticality: Criticality,
    pub value: OverloadStartProtocolIEs_EntryValue,
}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "0",
    sz_ub = "65535"
)]
pub struct OverloadStartProtocolIEs(Vec<OverloadStartProtocolIEs_Entry>);

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum OverloadStopProtocolIEs_EntryValue {
    #[asn(key = 154)]
    Id_GUMMEIList(GUMMEIList),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct OverloadStopProtocolIEs_Entry {
    #[asn(key_field = true)]
    pub id: ProtocolIE_ID,
    pub criticality: Criticality,
    pub value: OverloadStopProtocolIEs_EntryValue,
}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "0",
    sz_ub = "65535"
)]
pub struct OverloadStopProtocolIEs(Vec<OverloadStopProtocolIEs_Entry>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct PC5FlowBitRatesIE_Extensions_Entry {}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct PC5FlowBitRatesIE_Extensions(Vec<PC5FlowBitRatesIE_Extensions_Entry>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct PC5QoSFlowItemIE_Extensions_Entry {}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct PC5QoSFlowItemIE_Extensions(Vec<PC5QoSFlowItemIE_Extensions_Entry>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct PC5QoSParametersIE_Extensions_Entry {}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct PC5QoSParametersIE_Extensions(Vec<PC5QoSParametersIE_Extensions_Entry>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct PLMNAreaBasedQMCIE_Extensions_Entry {}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct PLMNAreaBasedQMCIE_Extensions(Vec<PLMNAreaBasedQMCIE_Extensions_Entry>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct PSCellInformationIE_Extensions_Entry {}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct PSCellInformationIE_Extensions(Vec<PSCellInformationIE_Extensions_Entry>);

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum PWSFailureIndicationProtocolIEs_EntryValue {
    #[asn(key = 59)]
    Id_Global_ENB_ID(Global_ENB_ID),
    #[asn(key = 222)]
    Id_PWSfailedECGIList(PWSfailedECGIList),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct PWSFailureIndicationProtocolIEs_Entry {
    #[asn(key_field = true)]
    pub id: ProtocolIE_ID,
    pub criticality: Criticality,
    pub value: PWSFailureIndicationProtocolIEs_EntryValue,
}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "0",
    sz_ub = "65535"
)]
pub struct PWSFailureIndicationProtocolIEs(Vec<PWSFailureIndicationProtocolIEs_Entry>);

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum PWSRestartIndicationProtocolIEs_EntryValue {
    #[asn(key = 182)]
    Id_ECGIListForRestart(ECGIListForRestart),
    #[asn(key = 190)]
    Id_EmergencyAreaIDListForRestart(EmergencyAreaIDListForRestart),
    #[asn(key = 59)]
    Id_Global_ENB_ID(Global_ENB_ID),
    #[asn(key = 188)]
    Id_TAIListForRestart(TAIListForRestart),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct PWSRestartIndicationProtocolIEs_Entry {
    #[asn(key_field = true)]
    pub id: ProtocolIE_ID,
    pub criticality: Criticality,
    pub value: PWSRestartIndicationProtocolIEs_EntryValue,
}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "0",
    sz_ub = "65535"
)]
pub struct PWSRestartIndicationProtocolIEs(Vec<PWSRestartIndicationProtocolIEs_Entry>);

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum PagingProtocolIEs_EntryValue {
    #[asn(key = 211)]
    Id_AssistanceDataForPaging(AssistanceDataForPaging),
    #[asn(key = 271)]
    Id_CE_ModeBRestricted(CE_ModeBRestricted),
    #[asn(key = 109)]
    Id_CNDomain(CNDomain),
    #[asn(key = 128)]
    Id_CSG_IdList(CSG_IdList),
    #[asn(key = 304)]
    Id_DataSize(DataSize),
    #[asn(key = 251)]
    Id_EnhancedCoverageRestricted(EnhancedCoverageRestricted),
    #[asn(key = 239)]
    Id_NB_IoT_Paging_eDRXInformation(NB_IoT_Paging_eDRXInformation),
    #[asn(key = 324)]
    Id_NB_IoT_PagingDRX(NB_IoT_PagingDRX),
    #[asn(key = 244)]
    Id_NB_IoT_UEIdentityIndexValue(NB_IoT_UEIdentityIndexValue),
    #[asn(key = 227)]
    Id_Paging_eDRXInformation(Paging_eDRXInformation),
    #[asn(key = 151)]
    Id_PagingPriority(PagingPriority),
    #[asn(key = 46)]
    Id_TAIList(TAIList),
    #[asn(key = 80)]
    Id_UEIdentityIndexValue(UEIdentityIndexValue),
    #[asn(key = 43)]
    Id_UEPagingID(UEPagingID),
    #[asn(key = 198)]
    Id_UERadioCapabilityForPaging(UERadioCapabilityForPaging),
    #[asn(key = 323)]
    Id_WUS_Assistance_Information(WUS_Assistance_Information),
    #[asn(key = 231)]
    Id_extended_UEIdentityIndexValue(Extended_UEIdentityIndexValue),
    #[asn(key = 44)]
    Id_pagingDRX(PagingDRX),
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
#[asn(type = "SEQUENCE", extensible = false)]
pub struct Paging_eDRXInformationIE_Extensions_Entry {}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct Paging_eDRXInformationIE_Extensions(Vec<Paging_eDRXInformationIE_Extensions_Entry>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct PagingAttemptInformationIE_Extensions_Entry {}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct PagingAttemptInformationIE_Extensions(Vec<PagingAttemptInformationIE_Extensions_Entry>);

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum PathSwitchRequestProtocolIEs_EntryValue {
    #[asn(key = 127)]
    Id_CSG_Id(CSG_Id),
    #[asn(key = 146)]
    Id_CSGMembershipStatus(CSGMembershipStatus),
    #[asn(key = 145)]
    Id_CellAccessMode(CellAccessMode),
    #[asn(key = 22)]
    Id_E_RABToBeSwitchedDLList(E_RABToBeSwitchedDLList),
    #[asn(key = 100)]
    Id_EUTRAN_CGI(EUTRAN_CGI),
    #[asn(key = 186)]
    Id_LHN_ID(LHN_ID),
    #[asn(key = 269)]
    Id_NRUESecurityCapabilities(NRUESecurityCapabilities),
    #[asn(key = 288)]
    Id_PSCellInformation(PSCellInformation),
    #[asn(key = 245)]
    Id_RRC_Resume_Cause(RRC_Establishment_Cause),
    #[asn(key = 157)]
    Id_SourceMME_GUMMEI(GUMMEI),
    #[asn(key = 88)]
    Id_SourceMME_UE_S1AP_ID(MME_UE_S1AP_ID),
    #[asn(key = 67)]
    Id_TAI(TAI),
    #[asn(key = 176)]
    Id_Tunnel_Information_for_BBF(TunnelInformation),
    #[asn(key = 107)]
    Id_UESecurityCapabilities(UESecurityCapabilities),
    #[asn(key = 8)]
    Id_eNB_UE_S1AP_ID(ENB_UE_S1AP_ID),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct PathSwitchRequestProtocolIEs_Entry {
    #[asn(key_field = true)]
    pub id: ProtocolIE_ID,
    pub criticality: Criticality,
    pub value: PathSwitchRequestProtocolIEs_EntryValue,
}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "0",
    sz_ub = "65535"
)]
pub struct PathSwitchRequestProtocolIEs(Vec<PathSwitchRequestProtocolIEs_Entry>);

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum PathSwitchRequestAcknowledgeProtocolIEs_EntryValue {
    #[asn(key = 299)]
    Id_AdditionalRRMPriorityIndex(AdditionalRRMPriorityIndex),
    #[asn(key = 277)]
    Id_AerialUEsubscriptionInformation(AerialUEsubscriptionInformation),
    #[asn(key = 271)]
    Id_CE_ModeBRestricted(CE_ModeBRestricted),
    #[asn(key = 146)]
    Id_CSGMembershipStatus(CSGMembershipStatus),
    #[asn(key = 58)]
    Id_CriticalityDiagnostics(CriticalityDiagnostics),
    #[asn(key = 33)]
    Id_E_RABToBeReleasedList(E_RABList),
    #[asn(key = 95)]
    Id_E_RABToBeSwitchedULList(E_RABToBeSwitchedULList),
    #[asn(key = 251)]
    Id_EnhancedCoverageRestricted(EnhancedCoverageRestricted),
    #[asn(key = 41)]
    Id_HandoverRestrictionList(HandoverRestrictionList),
    #[asn(key = 0)]
    Id_MME_UE_S1AP_ID(MME_UE_S1AP_ID),
    #[asn(key = 158)]
    Id_MME_UE_S1AP_ID_2(MME_UE_S1AP_ID),
    #[asn(key = 269)]
    Id_NRUESecurityCapabilities(NRUESecurityCapabilities),
    #[asn(key = 307)]
    Id_NRUESidelinkAggregateMaximumBitrate(NRUESidelinkAggregateMaximumBitrate),
    #[asn(key = 306)]
    Id_NRV2XServicesAuthorized(NRV2XServicesAuthorized),
    #[asn(key = 308)]
    Id_PC5QoSParameters(PC5QoSParameters),
    #[asn(key = 283)]
    Id_PendingDataIndication(PendingDataIndication),
    #[asn(key = 195)]
    Id_ProSeAuthorized(ProSeAuthorized),
    #[asn(key = 40)]
    Id_SecurityContext(SecurityContext),
    #[asn(key = 278)]
    Id_Subscription_Based_UE_DifferentiationInfo(Subscription_Based_UE_DifferentiationInfo),
    #[asn(key = 314)]
    Id_UERadioCapabilityID(UERadioCapabilityID),
    #[asn(key = 248)]
    Id_UESidelinkAggregateMaximumBitrate(UESidelinkAggregateMaximumBitrate),
    #[asn(key = 241)]
    Id_UEUserPlaneCIoTSupportIndicator(UEUserPlaneCIoTSupportIndicator),
    #[asn(key = 240)]
    Id_V2XServicesAuthorized(V2XServicesAuthorized),
    #[asn(key = 8)]
    Id_eNB_UE_S1AP_ID(ENB_UE_S1AP_ID),
    #[asn(key = 66)]
    Id_uEaggregateMaximumBitrate(UEAggregateMaximumBitrate),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct PathSwitchRequestAcknowledgeProtocolIEs_Entry {
    #[asn(key_field = true)]
    pub id: ProtocolIE_ID,
    pub criticality: Criticality,
    pub value: PathSwitchRequestAcknowledgeProtocolIEs_EntryValue,
}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "0",
    sz_ub = "65535"
)]
pub struct PathSwitchRequestAcknowledgeProtocolIEs(
    Vec<PathSwitchRequestAcknowledgeProtocolIEs_Entry>,
);

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum PathSwitchRequestFailureProtocolIEs_EntryValue {
    #[asn(key = 2)]
    Id_Cause(Cause),
    #[asn(key = 58)]
    Id_CriticalityDiagnostics(CriticalityDiagnostics),
    #[asn(key = 0)]
    Id_MME_UE_S1AP_ID(MME_UE_S1AP_ID),
    #[asn(key = 8)]
    Id_eNB_UE_S1AP_ID(ENB_UE_S1AP_ID),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct PathSwitchRequestFailureProtocolIEs_Entry {
    #[asn(key_field = true)]
    pub id: ProtocolIE_ID,
    pub criticality: Criticality,
    pub value: PathSwitchRequestFailureProtocolIEs_EntryValue,
}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "0",
    sz_ub = "65535"
)]
pub struct PathSwitchRequestFailureProtocolIEs(Vec<PathSwitchRequestFailureProtocolIEs_Entry>);

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
pub enum ProSeAuthorizedIE_Extensions_EntryExtensionValue {
    #[asn(key = 216)]
    Id_ProSeUEtoNetworkRelaying(ProSeUEtoNetworkRelaying),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct ProSeAuthorizedIE_Extensions_Entry {
    #[asn(key_field = true)]
    pub id: ProtocolExtensionID,
    pub criticality: Criticality,
    pub extension_value: ProSeAuthorizedIE_Extensions_EntryExtensionValue,
}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct ProSeAuthorizedIE_Extensions(Vec<ProSeAuthorizedIE_Extensions_Entry>);

#[derive(Debug, AperCodec)]
#[asn(
    type = "OCTET-STRING",
    sz_extensible = false,
    sz_lb = "16",
    sz_ub = "16"
)]
pub struct OCTET_STRING_50(Vec<u8>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct RIMTransferIE_Extensions_Entry {}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct RIMTransferIE_Extensions(Vec<RIMTransferIE_Extensions_Entry>);

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum RLFReportInformationIE_Extensions_EntryExtensionValue {
    #[asn(key = 313)]
    Id_NB_IoT_RLF_Report_Container(NB_IoT_RLF_Report_Container),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct RLFReportInformationIE_Extensions_Entry {
    #[asn(key_field = true)]
    pub id: ProtocolExtensionID,
    pub criticality: Criticality,
    pub extension_value: RLFReportInformationIE_Extensions_EntryExtensionValue,
}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct RLFReportInformationIE_Extensions(Vec<RLFReportInformationIE_Extensions_Entry>);

#[derive(Debug, AperCodec)]
#[asn(type = "INTEGER", lb = "0", ub = "4095")]
pub struct INTEGER_51(u16);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct RecommendedCellItemIE_Extensions_Entry {}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct RecommendedCellItemIE_Extensions(Vec<RecommendedCellItemIE_Extensions_Entry>);

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum RecommendedCellList_EntryValue {
    #[asn(key = 214)]
    Id_RecommendedCellItem(RecommendedCellItem),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct RecommendedCellList_Entry {
    #[asn(key_field = true)]
    pub id: ProtocolIE_ID,
    pub criticality: Criticality,
    pub value: RecommendedCellList_EntryValue,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct RecommendedCellsForPagingIE_Extensions_Entry {}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct RecommendedCellsForPagingIE_Extensions(
    Vec<RecommendedCellsForPagingIE_Extensions_Entry>,
);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct RecommendedENBItemIE_Extensions_Entry {}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct RecommendedENBItemIE_Extensions(Vec<RecommendedENBItemIE_Extensions_Entry>);

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum RecommendedENBList_EntryValue {
    #[asn(key = 215)]
    Id_RecommendedENBItem(RecommendedENBItem),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct RecommendedENBList_Entry {
    #[asn(key_field = true)]
    pub id: ProtocolIE_ID,
    pub criticality: Criticality,
    pub value: RecommendedENBList_EntryValue,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct RecommendedENBsForPagingIE_Extensions_Entry {}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct RecommendedENBsForPagingIE_Extensions(Vec<RecommendedENBsForPagingIE_Extensions_Entry>);

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum RequestTypeIE_Extensions_EntryExtensionValue {
    #[asn(key = 298)]
    Id_RequestTypeAdditionalInfo(RequestTypeAdditionalInfo),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct RequestTypeIE_Extensions_Entry {
    #[asn(key_field = true)]
    pub id: ProtocolExtensionID,
    pub criticality: Criticality,
    pub extension_value: RequestTypeIE_Extensions_EntryExtensionValue,
}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct RequestTypeIE_Extensions(Vec<RequestTypeIE_Extensions_Entry>);

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum RerouteNASRequestProtocolIEs_EntryValue {
    #[asn(key = 224)]
    Id_Additional_GUTI(Additional_GUTI),
    #[asn(key = 223)]
    Id_MME_Group_ID(MME_Group_ID),
    #[asn(key = 0)]
    Id_MME_UE_S1AP_ID(MME_UE_S1AP_ID),
    #[asn(key = 230)]
    Id_UE_Usage_Type(UE_Usage_Type),
    #[asn(key = 8)]
    Id_eNB_UE_S1AP_ID(ENB_UE_S1AP_ID),
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
#[asn(type = "OPEN")]
pub enum ResetProtocolIEs_EntryValue {
    #[asn(key = 2)]
    Id_Cause(Cause),
    #[asn(key = 92)]
    Id_ResetType(ResetType),
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
pub enum ResetAcknowledgeProtocolIEs_EntryValue {
    #[asn(key = 58)]
    Id_CriticalityDiagnostics(CriticalityDiagnostics),
    #[asn(key = 93)]
    Id_UE_associatedLogicalS1_ConnectionListResAck(UE_associatedLogicalS1_ConnectionListResAck),
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
pub enum RetrieveUEInformationProtocolIEs_EntryValue {
    #[asn(key = 96)]
    Id_S_TMSI(S_TMSI),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct RetrieveUEInformationProtocolIEs_Entry {
    #[asn(key_field = true)]
    pub id: ProtocolIE_ID,
    pub criticality: Criticality,
    pub value: RetrieveUEInformationProtocolIEs_EntryValue,
}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "0",
    sz_ub = "65535"
)]
pub struct RetrieveUEInformationProtocolIEs(Vec<RetrieveUEInformationProtocolIEs_Entry>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct S_TMSIIE_Extensions_Entry {}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct S_TMSIIE_Extensions(Vec<S_TMSIIE_Extensions_Entry>);

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum S1SetupFailureProtocolIEs_EntryValue {
    #[asn(key = 2)]
    Id_Cause(Cause),
    #[asn(key = 58)]
    Id_CriticalityDiagnostics(CriticalityDiagnostics),
    #[asn(key = 65)]
    Id_TimeToWait(TimeToWait),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct S1SetupFailureProtocolIEs_Entry {
    #[asn(key_field = true)]
    pub id: ProtocolIE_ID,
    pub criticality: Criticality,
    pub value: S1SetupFailureProtocolIEs_EntryValue,
}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "0",
    sz_ub = "65535"
)]
pub struct S1SetupFailureProtocolIEs(Vec<S1SetupFailureProtocolIEs_Entry>);

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum S1SetupRequestProtocolIEs_EntryValue {
    #[asn(key = 128)]
    Id_CSG_IdList(CSG_IdList),
    #[asn(key = 291)]
    Id_ConnectedengNBList(ConnectedengNBList),
    #[asn(key = 137)]
    Id_DefaultPagingDRX(PagingDRX),
    #[asn(key = 59)]
    Id_Global_ENB_ID(Global_ENB_ID),
    #[asn(key = 234)]
    Id_NB_IoT_DefaultPagingDRX(NB_IoT_DefaultPagingDRX),
    #[asn(key = 64)]
    Id_SupportedTAs(SupportedTAs),
    #[asn(key = 228)]
    Id_UE_RetentionInformation(UE_RetentionInformation),
    #[asn(key = 60)]
    Id_eNBname(ENBname),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct S1SetupRequestProtocolIEs_Entry {
    #[asn(key_field = true)]
    pub id: ProtocolIE_ID,
    pub criticality: Criticality,
    pub value: S1SetupRequestProtocolIEs_EntryValue,
}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "0",
    sz_ub = "65535"
)]
pub struct S1SetupRequestProtocolIEs(Vec<S1SetupRequestProtocolIEs_Entry>);

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum S1SetupResponseProtocolIEs_EntryValue {
    #[asn(key = 58)]
    Id_CriticalityDiagnostics(CriticalityDiagnostics),
    #[asn(key = 303)]
    Id_IAB_Supported(IAB_Supported),
    #[asn(key = 163)]
    Id_MMERelaySupportIndicator(MMERelaySupportIndicator),
    #[asn(key = 61)]
    Id_MMEname(MMEname),
    #[asn(key = 87)]
    Id_RelativeMMECapacity(RelativeMMECapacity),
    #[asn(key = 247)]
    Id_ServedDCNs(ServedDCNs),
    #[asn(key = 105)]
    Id_ServedGUMMEIs(ServedGUMMEIs),
    #[asn(key = 228)]
    Id_UE_RetentionInformation(UE_RetentionInformation),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct S1SetupResponseProtocolIEs_Entry {
    #[asn(key_field = true)]
    pub id: ProtocolIE_ID,
    pub criticality: Criticality,
    pub value: S1SetupResponseProtocolIEs_EntryValue,
}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "0",
    sz_ub = "65535"
)]
pub struct S1SetupResponseProtocolIEs(Vec<S1SetupResponseProtocolIEs_Entry>);

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum SONConfigurationTransferIE_Extensions_EntryExtensionValue {
    #[asn(key = 209)]
    Id_Synchronisation_Information(SynchronisationInformation),
    #[asn(key = 152)]
    Id_x2TNLConfigurationInfo(X2TNLConfigurationInfo),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct SONConfigurationTransferIE_Extensions_Entry {
    #[asn(key_field = true)]
    pub id: ProtocolExtensionID,
    pub criticality: Criticality,
    pub extension_value: SONConfigurationTransferIE_Extensions_EntryExtensionValue,
}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct SONConfigurationTransferIE_Extensions(Vec<SONConfigurationTransferIE_Extensions_Entry>);

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum SONInformation_ExtensionValue {
    #[asn(key = 206)]
    Id_SON_Information_Report(SONInformationReport),
}

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum SONInformationReplyIE_Extensions_EntryExtensionValue {
    #[asn(key = 208)]
    Id_Muting_Pattern_Information(MutingPatternInformation),
    #[asn(key = 149)]
    Id_Time_Synchronisation_Info(TimeSynchronisationInfo),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct SONInformationReplyIE_Extensions_Entry {
    #[asn(key_field = true)]
    pub id: ProtocolExtensionID,
    pub criticality: Criticality,
    pub extension_value: SONInformationReplyIE_Extensions_EntryExtensionValue,
}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct SONInformationReplyIE_Extensions(Vec<SONInformationReplyIE_Extensions_Entry>);

#[derive(Debug, AperCodec)]
#[asn(type = "NULL")]
pub struct NULL_52;

#[derive(Debug, AperCodec)]
#[asn(type = "NULL")]
pub struct NULL_53;

#[derive(Debug, AperCodec)]
#[asn(type = "NULL")]
pub struct NULL_54;

#[derive(Debug, AperCodec)]
#[asn(type = "NULL")]
pub struct NULL_55;

#[derive(Debug, AperCodec)]
#[asn(type = "BITSTRING", sz_extensible = false, sz_lb = "7", sz_ub = "7")]
pub struct BIT_STRING_56(BitVec<Msb0, u8>);

#[derive(Debug, AperCodec)]
#[asn(type = "INTEGER", lb = "0", ub = "86399", extensible = true)]
pub struct INTEGER_57(u32);

#[derive(Debug, AperCodec)]
#[asn(type = "INTEGER", lb = "0", ub = "86399", extensible = true)]
pub struct INTEGER_58(u32);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct ScheduledCommunicationTimeIE_Extensions_Entry {}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct ScheduledCommunicationTimeIE_Extensions(
    Vec<ScheduledCommunicationTimeIE_Extensions_Entry>,
);

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum SecondaryRATDataUsageReportProtocolIEs_EntryValue {
    #[asn(key = 266)]
    Id_HandoverFlag(HandoverFlag),
    #[asn(key = 0)]
    Id_MME_UE_S1AP_ID(MME_UE_S1AP_ID),
    #[asn(key = 264)]
    Id_SecondaryRATDataUsageReportList(SecondaryRATDataUsageReportList),
    #[asn(key = 297)]
    Id_TimeSinceSecondaryNodeRelease(TimeSinceSecondaryNodeRelease),
    #[asn(key = 189)]
    Id_UserLocationInformation(UserLocationInformation),
    #[asn(key = 8)]
    Id_eNB_UE_S1AP_ID(ENB_UE_S1AP_ID),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct SecondaryRATDataUsageReportProtocolIEs_Entry {
    #[asn(key_field = true)]
    pub id: ProtocolIE_ID,
    pub criticality: Criticality,
    pub value: SecondaryRATDataUsageReportProtocolIEs_EntryValue,
}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "0",
    sz_ub = "65535"
)]
pub struct SecondaryRATDataUsageReportProtocolIEs(
    Vec<SecondaryRATDataUsageReportProtocolIEs_Entry>,
);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct SecondaryRATDataUsageReportItemIE_Extensions_Entry {}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct SecondaryRATDataUsageReportItemIE_Extensions(
    Vec<SecondaryRATDataUsageReportItemIE_Extensions_Entry>,
);

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum SecondaryRATDataUsageReportList_EntryValue {
    #[asn(key = 265)]
    Id_SecondaryRATDataUsageReportItem(SecondaryRATDataUsageReportItem),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct SecondaryRATDataUsageReportList_Entry {
    #[asn(key_field = true)]
    pub id: ProtocolIE_ID,
    pub criticality: Criticality,
    pub value: SecondaryRATDataUsageReportList_EntryValue,
}

#[derive(Debug, AperCodec)]
#[asn(type = "INTEGER", lb = "0", ub = "7")]
pub struct INTEGER_59(u8);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct SecurityContextIE_Extensions_Entry {}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct SecurityContextIE_Extensions(Vec<SecurityContextIE_Extensions_Entry>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct ServedDCNsItemIE_Extensions_Entry {}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct ServedDCNsItemIE_Extensions(Vec<ServedDCNsItemIE_Extensions_Entry>);

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum ServedGUMMEIsItemIE_Extensions_EntryExtensionValue {
    #[asn(key = 170)]
    Id_GUMMEIType(GUMMEIType),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct ServedGUMMEIsItemIE_Extensions_Entry {
    #[asn(key_field = true)]
    pub id: ProtocolExtensionID,
    pub criticality: Criticality,
    pub extension_value: ServedGUMMEIsItemIE_Extensions_EntryExtensionValue,
}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct ServedGUMMEIsItemIE_Extensions(Vec<ServedGUMMEIsItemIE_Extensions_Entry>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct SourceNgRanNode_IDIE_Extensions_Entry {}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct SourceNgRanNode_IDIE_Extensions(Vec<SourceNgRanNode_IDIE_Extensions_Entry>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct SourceeNB_IDIE_Extensions_Entry {}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct SourceeNB_IDIE_Extensions(Vec<SourceeNB_IDIE_Extensions_Entry>);

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum SourceeNB_ToTargeteNB_TransparentContainerIE_Extensions_EntryExtensionValue {
    #[asn(key = 299)]
    Id_AdditionalRRMPriorityIndex(AdditionalRRMPriorityIndex),
    #[asn(key = 300)]
    Id_ContextatSource(ContextatSource),
    #[asn(key = 326)]
    Id_EmergencyIndicator(EmergencyIndicator),
    #[asn(key = 296)]
    Id_IMSvoiceEPSfallbackfrom5G(IMSvoiceEPSfallbackfrom5G),
    #[asn(key = 311)]
    Id_IntersystemMeasurementConfiguration(IntersystemMeasurementConfiguration),
    #[asn(key = 175)]
    Id_MobilityInformation(MobilityInformation),
    #[asn(key = 312)]
    Id_SourceNodeID(SourceNodeID),
    #[asn(key = 194)]
    Id_uE_HistoryInformationFromTheUE(UE_HistoryInformationFromTheUE),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct SourceeNB_ToTargeteNB_TransparentContainerIE_Extensions_Entry {
    #[asn(key_field = true)]
    pub id: ProtocolExtensionID,
    pub criticality: Criticality,
    pub extension_value:
        SourceeNB_ToTargeteNB_TransparentContainerIE_Extensions_EntryExtensionValue,
}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct SourceeNB_ToTargeteNB_TransparentContainerIE_Extensions(
    Vec<SourceeNB_ToTargeteNB_TransparentContainerIE_Extensions_Entry>,
);

#[derive(Debug, AperCodec)]
#[asn(type = "ENUMERATED", extensible = true, lb = "0", ub = "1")]
pub struct ENUMERATED_60(u8);
impl ENUMERATED_60 {
    const PERIODICALLY: u8 = 0u8;
    const ONDEMAND: u8 = 1u8;
}

#[derive(Debug, AperCodec)]
#[asn(type = "INTEGER", lb = "1", ub = "3600", extensible = true)]
pub struct INTEGER_61(u16);

#[derive(Debug, AperCodec)]
#[asn(type = "ENUMERATED", extensible = true, lb = "0", ub = "1")]
pub struct ENUMERATED_62(u8);
impl ENUMERATED_62 {
    const STATIONARY: u8 = 0u8;
    const MOBILE: u8 = 1u8;
}

#[derive(Debug, AperCodec)]
#[asn(type = "ENUMERATED", extensible = true, lb = "0", ub = "2")]
pub struct ENUMERATED_63(u8);
impl ENUMERATED_63 {
    const SINGLE_PACKET: u8 = 0u8;
    const DUAL_PACKETS: u8 = 1u8;
    const MULTIPLE_PACKETS: u8 = 2u8;
}

#[derive(Debug, AperCodec)]
#[asn(type = "ENUMERATED", extensible = true, lb = "0", ub = "2")]
pub struct ENUMERATED_64(u8);
impl ENUMERATED_64 {
    const BATTERY_POWERED: u8 = 0u8;
    const BATTERY_POWERED_NOT_RECHARGEABLE_OR_REPLACEABLE: u8 = 1u8;
    const NOT_BATTERY_POWERED: u8 = 2u8;
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct Subscription_Based_UE_DifferentiationInfoIE_Extensions_Entry {}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct Subscription_Based_UE_DifferentiationInfoIE_Extensions(
    Vec<Subscription_Based_UE_DifferentiationInfoIE_Extensions_Entry>,
);

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum SuccessfulOutcomeValue {
    #[asn(key = 50)]
    Id_E_RABModificationIndication(E_RABModificationConfirm),
    #[asn(key = 6)]
    Id_E_RABModify(E_RABModifyResponse),
    #[asn(key = 7)]
    Id_E_RABRelease(E_RABReleaseResponse),
    #[asn(key = 5)]
    Id_E_RABSetup(E_RABSetupResponse),
    #[asn(key = 29)]
    Id_ENBConfigurationUpdate(ENBConfigurationUpdateAcknowledge),
    #[asn(key = 4)]
    Id_HandoverCancel(HandoverCancelAcknowledge),
    #[asn(key = 0)]
    Id_HandoverPreparation(HandoverCommand),
    #[asn(key = 1)]
    Id_HandoverResourceAllocation(HandoverRequestAcknowledge),
    #[asn(key = 9)]
    Id_InitialContextSetup(InitialContextSetupResponse),
    #[asn(key = 43)]
    Id_Kill(KillResponse),
    #[asn(key = 30)]
    Id_MMEConfigurationUpdate(MMEConfigurationUpdateAcknowledge),
    #[asn(key = 3)]
    Id_PathSwitchRequest(PathSwitchRequestAcknowledge),
    #[asn(key = 14)]
    Id_Reset(ResetAcknowledge),
    #[asn(key = 17)]
    Id_S1Setup(S1SetupResponse),
    #[asn(key = 21)]
    Id_UEContextModification(UEContextModificationResponse),
    #[asn(key = 53)]
    Id_UEContextModificationIndication(UEContextModificationConfirm),
    #[asn(key = 23)]
    Id_UEContextRelease(UEContextReleaseComplete),
    #[asn(key = 56)]
    Id_UEContextResume(UEContextResumeResponse),
    #[asn(key = 55)]
    Id_UEContextSuspend(UEContextSuspendResponse),
    #[asn(key = 63)]
    Id_UERadioCapabilityIDMapping(UERadioCapabilityIDMappingResponse),
    #[asn(key = 48)]
    Id_UERadioCapabilityMatch(UERadioCapabilityMatchResponse),
    #[asn(key = 36)]
    Id_WriteReplaceWarning(WriteReplaceWarningResponse),
}

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum SupportedTAs_ItemIE_Extensions_EntryExtensionValue {
    #[asn(key = 232)]
    Id_RAT_Type(RAT_Type),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct SupportedTAs_ItemIE_Extensions_Entry {
    #[asn(key_field = true)]
    pub id: ProtocolExtensionID,
    pub criticality: Criticality,
    pub extension_value: SupportedTAs_ItemIE_Extensions_EntryExtensionValue,
}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct SupportedTAs_ItemIE_Extensions(Vec<SupportedTAs_ItemIE_Extensions_Entry>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct SynchronisationInformationIE_Extensions_Entry {}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct SynchronisationInformationIE_Extensions(
    Vec<SynchronisationInformationIE_Extensions_Entry>,
);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct TABasedMDTIE_Extensions_Entry {}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct TABasedMDTIE_Extensions(Vec<TABasedMDTIE_Extensions_Entry>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct TABasedQMCIE_Extensions_Entry {}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct TABasedQMCIE_Extensions(Vec<TABasedQMCIE_Extensions_Entry>);

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
#[asn(type = "SEQUENCE", extensible = false)]
pub struct TAI_Broadcast_ItemIE_Extensions_Entry {}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct TAI_Broadcast_ItemIE_Extensions(Vec<TAI_Broadcast_ItemIE_Extensions_Entry>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct TAI_Cancelled_ItemIE_Extensions_Entry {}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct TAI_Cancelled_ItemIE_Extensions(Vec<TAI_Cancelled_ItemIE_Extensions_Entry>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct TAIBasedMDTIE_Extensions_Entry {}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct TAIBasedMDTIE_Extensions(Vec<TAIBasedMDTIE_Extensions_Entry>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct TAIBasedQMCIE_Extensions_Entry {}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct TAIBasedQMCIE_Extensions(Vec<TAIBasedQMCIE_Extensions_Entry>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct TAIItemIE_Extensions_Entry {}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct TAIItemIE_Extensions(Vec<TAIItemIE_Extensions_Entry>);

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum TAIList_EntryValue {
    #[asn(key = 47)]
    Id_TAIItem(TAIItem),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct TAIList_Entry {
    #[asn(key_field = true)]
    pub id: ProtocolIE_ID,
    pub criticality: Criticality,
    pub value: TAIList_EntryValue,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct TargetNgRanNode_IDIE_Extensions_Entry {}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct TargetNgRanNode_IDIE_Extensions(Vec<TargetNgRanNode_IDIE_Extensions_Entry>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct TargetRNC_IDIE_Extensions_Entry {}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct TargetRNC_IDIE_Extensions(Vec<TargetRNC_IDIE_Extensions_Entry>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct TargeteNB_IDIE_Extensions_Entry {}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct TargeteNB_IDIE_Extensions(Vec<TargeteNB_IDIE_Extensions_Entry>);

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum TargeteNB_ToSourceeNB_TransparentContainerIE_Extensions_EntryExtensionValue {
    #[asn(key = 318)]
    Id_DAPSResponseInfoList(DAPSResponseInfoList),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct TargeteNB_ToSourceeNB_TransparentContainerIE_Extensions_Entry {
    #[asn(key_field = true)]
    pub id: ProtocolExtensionID,
    pub criticality: Criticality,
    pub extension_value:
        TargeteNB_ToSourceeNB_TransparentContainerIE_Extensions_EntryExtensionValue,
}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct TargeteNB_ToSourceeNB_TransparentContainerIE_Extensions(
    Vec<TargeteNB_ToSourceeNB_TransparentContainerIE_Extensions_Entry>,
);

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum TimeSynchronisationInfoIE_Extensions_EntryExtensionValue {
    #[asn(key = 207)]
    Id_Muting_Availability_Indication(MutingAvailabilityIndication),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct TimeSynchronisationInfoIE_Extensions_Entry {
    #[asn(key_field = true)]
    pub id: ProtocolExtensionID,
    pub criticality: Criticality,
    pub extension_value: TimeSynchronisationInfoIE_Extensions_EntryExtensionValue,
}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct TimeSynchronisationInfoIE_Extensions(Vec<TimeSynchronisationInfoIE_Extensions_Entry>);

#[derive(Debug, AperCodec)]
#[asn(type = "OCTET-STRING")]
pub struct OCTET_STRING_65(Vec<u8>);

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum TraceActivationIE_Extensions_EntryExtensionValue {
    #[asn(key = 162)]
    Id_MDTConfiguration(MDT_Configuration),
    #[asn(key = 316)]
    Id_MDTConfigurationNR(MDT_ConfigurationNR),
    #[asn(key = 325)]
    Id_TraceCollectionEntityURI(URI_Address),
    #[asn(key = 262)]
    Id_UEAppLayerMeasConfig(UEAppLayerMeasConfig),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct TraceActivationIE_Extensions_Entry {
    #[asn(key_field = true)]
    pub id: ProtocolExtensionID,
    pub criticality: Criticality,
    pub extension_value: TraceActivationIE_Extensions_EntryExtensionValue,
}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct TraceActivationIE_Extensions(Vec<TraceActivationIE_Extensions_Entry>);

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum TraceFailureIndicationProtocolIEs_EntryValue {
    #[asn(key = 2)]
    Id_Cause(Cause),
    #[asn(key = 86)]
    Id_E_UTRAN_Trace_ID(E_UTRAN_Trace_ID),
    #[asn(key = 0)]
    Id_MME_UE_S1AP_ID(MME_UE_S1AP_ID),
    #[asn(key = 8)]
    Id_eNB_UE_S1AP_ID(ENB_UE_S1AP_ID),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct TraceFailureIndicationProtocolIEs_Entry {
    #[asn(key_field = true)]
    pub id: ProtocolIE_ID,
    pub criticality: Criticality,
    pub value: TraceFailureIndicationProtocolIEs_EntryValue,
}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "0",
    sz_ub = "65535"
)]
pub struct TraceFailureIndicationProtocolIEs(Vec<TraceFailureIndicationProtocolIEs_Entry>);

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum TraceStartProtocolIEs_EntryValue {
    #[asn(key = 0)]
    Id_MME_UE_S1AP_ID(MME_UE_S1AP_ID),
    #[asn(key = 25)]
    Id_TraceActivation(TraceActivation),
    #[asn(key = 8)]
    Id_eNB_UE_S1AP_ID(ENB_UE_S1AP_ID),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct TraceStartProtocolIEs_Entry {
    #[asn(key_field = true)]
    pub id: ProtocolIE_ID,
    pub criticality: Criticality,
    pub value: TraceStartProtocolIEs_EntryValue,
}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "0",
    sz_ub = "65535"
)]
pub struct TraceStartProtocolIEs(Vec<TraceStartProtocolIEs_Entry>);

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
#[asn(type = "SEQUENCE", extensible = false)]
pub struct UE_S1AP_ID_pairIE_Extensions_Entry {}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct UE_S1AP_ID_pairIE_Extensions(Vec<UE_S1AP_ID_pairIE_Extensions_Entry>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct UE_associatedLogicalS1_ConnectionItemIE_Extensions_Entry {}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct UE_associatedLogicalS1_ConnectionItemIE_Extensions(
    Vec<UE_associatedLogicalS1_ConnectionItemIE_Extensions_Entry>,
);

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum UE_associatedLogicalS1_ConnectionListRes_EntryValue {
    #[asn(key = 91)]
    Id_UE_associatedLogicalS1_ConnectionItem(UE_associatedLogicalS1_ConnectionItem),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct UE_associatedLogicalS1_ConnectionListRes_Entry {
    #[asn(key_field = true)]
    pub id: ProtocolIE_ID,
    pub criticality: Criticality,
    pub value: UE_associatedLogicalS1_ConnectionListRes_EntryValue,
}

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum UE_associatedLogicalS1_ConnectionListResAck_EntryValue {
    #[asn(key = 91)]
    Id_UE_associatedLogicalS1_ConnectionItem(UE_associatedLogicalS1_ConnectionItem),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct UE_associatedLogicalS1_ConnectionListResAck_Entry {
    #[asn(key_field = true)]
    pub id: ProtocolIE_ID,
    pub criticality: Criticality,
    pub value: UE_associatedLogicalS1_ConnectionListResAck_EntryValue,
}

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum UEAggregateMaximumBitrateIE_Extensions_EntryExtensionValue {
    #[asn(key = 259)]
    Id_extended_uEaggregateMaximumBitRateDL(ExtendedBitRate),
    #[asn(key = 260)]
    Id_extended_uEaggregateMaximumBitRateUL(ExtendedBitRate),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct UEAggregateMaximumBitrateIE_Extensions_Entry {
    #[asn(key_field = true)]
    pub id: ProtocolExtensionID,
    pub criticality: Criticality,
    pub extension_value: UEAggregateMaximumBitrateIE_Extensions_EntryExtensionValue,
}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct UEAggregateMaximumBitrateIE_Extensions(
    Vec<UEAggregateMaximumBitrateIE_Extensions_Entry>,
);

#[derive(Debug, AperCodec)]
#[asn(
    type = "OCTET-STRING",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "1000"
)]
pub struct OCTET_STRING_66(Vec<u8>);

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum UEAppLayerMeasConfigIE_Extensions_EntryExtensionValue {
    #[asn(key = 276)]
    Id_serviceType(ServiceType),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct UEAppLayerMeasConfigIE_Extensions_Entry {
    #[asn(key_field = true)]
    pub id: ProtocolExtensionID,
    pub criticality: Criticality,
    pub extension_value: UEAppLayerMeasConfigIE_Extensions_EntryExtensionValue,
}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct UEAppLayerMeasConfigIE_Extensions(Vec<UEAppLayerMeasConfigIE_Extensions_Entry>);

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum UECapabilityInfoIndicationProtocolIEs_EntryValue {
    #[asn(key = 272)]
    Id_LTE_M_Indication(LTE_M_Indication),
    #[asn(key = 0)]
    Id_MME_UE_S1AP_ID(MME_UE_S1AP_ID),
    #[asn(key = 263)]
    Id_UE_Application_Layer_Measurement_Capability(UE_Application_Layer_Measurement_Capability),
    #[asn(key = 74)]
    Id_UERadioCapability(UERadioCapability),
    #[asn(key = 315)]
    Id_UERadioCapability_NR_Format(UERadioCapability),
    #[asn(key = 198)]
    Id_UERadioCapabilityForPaging(UERadioCapabilityForPaging),
    #[asn(key = 8)]
    Id_eNB_UE_S1AP_ID(ENB_UE_S1AP_ID),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct UECapabilityInfoIndicationProtocolIEs_Entry {
    #[asn(key_field = true)]
    pub id: ProtocolIE_ID,
    pub criticality: Criticality,
    pub value: UECapabilityInfoIndicationProtocolIEs_EntryValue,
}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "0",
    sz_ub = "65535"
)]
pub struct UECapabilityInfoIndicationProtocolIEs(Vec<UECapabilityInfoIndicationProtocolIEs_Entry>);

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum UEContextModificationConfirmProtocolIEs_EntryValue {
    #[asn(key = 146)]
    Id_CSGMembershipStatus(CSGMembershipStatus),
    #[asn(key = 58)]
    Id_CriticalityDiagnostics(CriticalityDiagnostics),
    #[asn(key = 0)]
    Id_MME_UE_S1AP_ID(MME_UE_S1AP_ID),
    #[asn(key = 8)]
    Id_eNB_UE_S1AP_ID(ENB_UE_S1AP_ID),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct UEContextModificationConfirmProtocolIEs_Entry {
    #[asn(key_field = true)]
    pub id: ProtocolIE_ID,
    pub criticality: Criticality,
    pub value: UEContextModificationConfirmProtocolIEs_EntryValue,
}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "0",
    sz_ub = "65535"
)]
pub struct UEContextModificationConfirmProtocolIEs(
    Vec<UEContextModificationConfirmProtocolIEs_Entry>,
);

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum UEContextModificationFailureProtocolIEs_EntryValue {
    #[asn(key = 2)]
    Id_Cause(Cause),
    #[asn(key = 58)]
    Id_CriticalityDiagnostics(CriticalityDiagnostics),
    #[asn(key = 0)]
    Id_MME_UE_S1AP_ID(MME_UE_S1AP_ID),
    #[asn(key = 8)]
    Id_eNB_UE_S1AP_ID(ENB_UE_S1AP_ID),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct UEContextModificationFailureProtocolIEs_Entry {
    #[asn(key_field = true)]
    pub id: ProtocolIE_ID,
    pub criticality: Criticality,
    pub value: UEContextModificationFailureProtocolIEs_EntryValue,
}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "0",
    sz_ub = "65535"
)]
pub struct UEContextModificationFailureProtocolIEs(
    Vec<UEContextModificationFailureProtocolIEs_Entry>,
);

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum UEContextModificationIndicationProtocolIEs_EntryValue {
    #[asn(key = 226)]
    Id_CSGMembershipInfo(CSGMembershipInfo),
    #[asn(key = 0)]
    Id_MME_UE_S1AP_ID(MME_UE_S1AP_ID),
    #[asn(key = 8)]
    Id_eNB_UE_S1AP_ID(ENB_UE_S1AP_ID),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct UEContextModificationIndicationProtocolIEs_Entry {
    #[asn(key_field = true)]
    pub id: ProtocolIE_ID,
    pub criticality: Criticality,
    pub value: UEContextModificationIndicationProtocolIEs_EntryValue,
}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "0",
    sz_ub = "65535"
)]
pub struct UEContextModificationIndicationProtocolIEs(
    Vec<UEContextModificationIndicationProtocolIEs_Entry>,
);

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum UEContextModificationRequestProtocolIEs_EntryValue {
    #[asn(key = 187)]
    Id_AdditionalCSFallbackIndicator(AdditionalCSFallbackIndicator),
    #[asn(key = 299)]
    Id_AdditionalRRMPriorityIndex(AdditionalRRMPriorityIndex),
    #[asn(key = 277)]
    Id_AerialUEsubscriptionInformation(AerialUEsubscriptionInformation),
    #[asn(key = 108)]
    Id_CSFallbackIndicator(CSFallbackIndicator),
    #[asn(key = 146)]
    Id_CSGMembershipStatus(CSGMembershipStatus),
    #[asn(key = 301)]
    Id_IAB_Authorized(IAB_Authorized),
    #[asn(key = 0)]
    Id_MME_UE_S1AP_ID(MME_UE_S1AP_ID),
    #[asn(key = 269)]
    Id_NRUESecurityCapabilities(NRUESecurityCapabilities),
    #[asn(key = 307)]
    Id_NRUESidelinkAggregateMaximumBitrate(NRUESidelinkAggregateMaximumBitrate),
    #[asn(key = 306)]
    Id_NRV2XServicesAuthorized(NRV2XServicesAuthorized),
    #[asn(key = 308)]
    Id_PC5QoSParameters(PC5QoSParameters),
    #[asn(key = 195)]
    Id_ProSeAuthorized(ProSeAuthorized),
    #[asn(key = 159)]
    Id_RegisteredLAI(LAI),
    #[asn(key = 243)]
    Id_SRVCCOperationNotPossible(SRVCCOperationNotPossible),
    #[asn(key = 124)]
    Id_SRVCCOperationPossible(SRVCCOperationPossible),
    #[asn(key = 73)]
    Id_SecurityKey(SecurityKey),
    #[asn(key = 106)]
    Id_SubscriberProfileIDforRFP(SubscriberProfileIDforRFP),
    #[asn(key = 314)]
    Id_UERadioCapabilityID(UERadioCapabilityID),
    #[asn(key = 107)]
    Id_UESecurityCapabilities(UESecurityCapabilities),
    #[asn(key = 248)]
    Id_UESidelinkAggregateMaximumBitrate(UESidelinkAggregateMaximumBitrate),
    #[asn(key = 240)]
    Id_V2XServicesAuthorized(V2XServicesAuthorized),
    #[asn(key = 8)]
    Id_eNB_UE_S1AP_ID(ENB_UE_S1AP_ID),
    #[asn(key = 66)]
    Id_uEaggregateMaximumBitrate(UEAggregateMaximumBitrate),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct UEContextModificationRequestProtocolIEs_Entry {
    #[asn(key_field = true)]
    pub id: ProtocolIE_ID,
    pub criticality: Criticality,
    pub value: UEContextModificationRequestProtocolIEs_EntryValue,
}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "0",
    sz_ub = "65535"
)]
pub struct UEContextModificationRequestProtocolIEs(
    Vec<UEContextModificationRequestProtocolIEs_Entry>,
);

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum UEContextModificationResponseProtocolIEs_EntryValue {
    #[asn(key = 58)]
    Id_CriticalityDiagnostics(CriticalityDiagnostics),
    #[asn(key = 0)]
    Id_MME_UE_S1AP_ID(MME_UE_S1AP_ID),
    #[asn(key = 8)]
    Id_eNB_UE_S1AP_ID(ENB_UE_S1AP_ID),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct UEContextModificationResponseProtocolIEs_Entry {
    #[asn(key_field = true)]
    pub id: ProtocolIE_ID,
    pub criticality: Criticality,
    pub value: UEContextModificationResponseProtocolIEs_EntryValue,
}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "0",
    sz_ub = "65535"
)]
pub struct UEContextModificationResponseProtocolIEs(
    Vec<UEContextModificationResponseProtocolIEs_Entry>,
);

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum UEContextReleaseCommandProtocolIEs_EntryValue {
    #[asn(key = 2)]
    Id_Cause(Cause),
    #[asn(key = 99)]
    Id_UE_S1AP_IDs(UE_S1AP_IDs),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct UEContextReleaseCommandProtocolIEs_Entry {
    #[asn(key_field = true)]
    pub id: ProtocolIE_ID,
    pub criticality: Criticality,
    pub value: UEContextReleaseCommandProtocolIEs_EntryValue,
}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "0",
    sz_ub = "65535"
)]
pub struct UEContextReleaseCommandProtocolIEs(Vec<UEContextReleaseCommandProtocolIEs_Entry>);

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum UEContextReleaseCompleteProtocolIEs_EntryValue {
    #[asn(key = 212)]
    Id_CellIdentifierAndCELevelForCECapableUEs(CellIdentifierAndCELevelForCECapableUEs),
    #[asn(key = 58)]
    Id_CriticalityDiagnostics(CriticalityDiagnostics),
    #[asn(key = 213)]
    Id_InformationOnRecommendedCellsAndENBsForPaging(InformationOnRecommendedCellsAndENBsForPaging),
    #[asn(key = 0)]
    Id_MME_UE_S1AP_ID(MME_UE_S1AP_ID),
    #[asn(key = 264)]
    Id_SecondaryRATDataUsageReportList(SecondaryRATDataUsageReportList),
    #[asn(key = 297)]
    Id_TimeSinceSecondaryNodeRelease(TimeSinceSecondaryNodeRelease),
    #[asn(key = 189)]
    Id_UserLocationInformation(UserLocationInformation),
    #[asn(key = 8)]
    Id_eNB_UE_S1AP_ID(ENB_UE_S1AP_ID),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct UEContextReleaseCompleteProtocolIEs_Entry {
    #[asn(key_field = true)]
    pub id: ProtocolIE_ID,
    pub criticality: Criticality,
    pub value: UEContextReleaseCompleteProtocolIEs_EntryValue,
}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "0",
    sz_ub = "65535"
)]
pub struct UEContextReleaseCompleteProtocolIEs(Vec<UEContextReleaseCompleteProtocolIEs_Entry>);

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum UEContextReleaseRequestProtocolIEs_EntryValue {
    #[asn(key = 2)]
    Id_Cause(Cause),
    #[asn(key = 164)]
    Id_GWContextReleaseIndication(GWContextReleaseIndication),
    #[asn(key = 0)]
    Id_MME_UE_S1AP_ID(MME_UE_S1AP_ID),
    #[asn(key = 264)]
    Id_SecondaryRATDataUsageReportList(SecondaryRATDataUsageReportList),
    #[asn(key = 8)]
    Id_eNB_UE_S1AP_ID(ENB_UE_S1AP_ID),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct UEContextReleaseRequestProtocolIEs_Entry {
    #[asn(key_field = true)]
    pub id: ProtocolIE_ID,
    pub criticality: Criticality,
    pub value: UEContextReleaseRequestProtocolIEs_EntryValue,
}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "0",
    sz_ub = "65535"
)]
pub struct UEContextReleaseRequestProtocolIEs(Vec<UEContextReleaseRequestProtocolIEs_Entry>);

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum UEContextResumeFailureProtocolIEs_EntryValue {
    #[asn(key = 2)]
    Id_Cause(Cause),
    #[asn(key = 58)]
    Id_CriticalityDiagnostics(CriticalityDiagnostics),
    #[asn(key = 0)]
    Id_MME_UE_S1AP_ID(MME_UE_S1AP_ID),
    #[asn(key = 8)]
    Id_eNB_UE_S1AP_ID(ENB_UE_S1AP_ID),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct UEContextResumeFailureProtocolIEs_Entry {
    #[asn(key_field = true)]
    pub id: ProtocolIE_ID,
    pub criticality: Criticality,
    pub value: UEContextResumeFailureProtocolIEs_EntryValue,
}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "0",
    sz_ub = "65535"
)]
pub struct UEContextResumeFailureProtocolIEs(Vec<UEContextResumeFailureProtocolIEs_Entry>);

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum UEContextResumeRequestProtocolIEs_EntryValue {
    #[asn(key = 235)]
    Id_E_RABFailedToResumeListResumeReq(E_RABFailedToResumeListResumeReq),
    #[asn(key = 0)]
    Id_MME_UE_S1AP_ID(MME_UE_S1AP_ID),
    #[asn(key = 245)]
    Id_RRC_Resume_Cause(RRC_Establishment_Cause),
    #[asn(key = 8)]
    Id_eNB_UE_S1AP_ID(ENB_UE_S1AP_ID),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct UEContextResumeRequestProtocolIEs_Entry {
    #[asn(key_field = true)]
    pub id: ProtocolIE_ID,
    pub criticality: Criticality,
    pub value: UEContextResumeRequestProtocolIEs_EntryValue,
}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "0",
    sz_ub = "65535"
)]
pub struct UEContextResumeRequestProtocolIEs(Vec<UEContextResumeRequestProtocolIEs_Entry>);

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum UEContextResumeResponseProtocolIEs_EntryValue {
    #[asn(key = 58)]
    Id_CriticalityDiagnostics(CriticalityDiagnostics),
    #[asn(key = 237)]
    Id_E_RABFailedToResumeListResumeRes(E_RABFailedToResumeListResumeRes),
    #[asn(key = 0)]
    Id_MME_UE_S1AP_ID(MME_UE_S1AP_ID),
    #[asn(key = 283)]
    Id_PendingDataIndication(PendingDataIndication),
    #[asn(key = 40)]
    Id_SecurityContext(SecurityContext),
    #[asn(key = 8)]
    Id_eNB_UE_S1AP_ID(ENB_UE_S1AP_ID),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct UEContextResumeResponseProtocolIEs_Entry {
    #[asn(key_field = true)]
    pub id: ProtocolIE_ID,
    pub criticality: Criticality,
    pub value: UEContextResumeResponseProtocolIEs_EntryValue,
}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "0",
    sz_ub = "65535"
)]
pub struct UEContextResumeResponseProtocolIEs(Vec<UEContextResumeResponseProtocolIEs_Entry>);

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum UEContextSuspendRequestProtocolIEs_EntryValue {
    #[asn(key = 212)]
    Id_CellIdentifierAndCELevelForCECapableUEs(CellIdentifierAndCELevelForCECapableUEs),
    #[asn(key = 213)]
    Id_InformationOnRecommendedCellsAndENBsForPaging(InformationOnRecommendedCellsAndENBsForPaging),
    #[asn(key = 0)]
    Id_MME_UE_S1AP_ID(MME_UE_S1AP_ID),
    #[asn(key = 264)]
    Id_SecondaryRATDataUsageReportList(SecondaryRATDataUsageReportList),
    #[asn(key = 297)]
    Id_TimeSinceSecondaryNodeRelease(TimeSinceSecondaryNodeRelease),
    #[asn(key = 189)]
    Id_UserLocationInformation(UserLocationInformation),
    #[asn(key = 8)]
    Id_eNB_UE_S1AP_ID(ENB_UE_S1AP_ID),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct UEContextSuspendRequestProtocolIEs_Entry {
    #[asn(key_field = true)]
    pub id: ProtocolIE_ID,
    pub criticality: Criticality,
    pub value: UEContextSuspendRequestProtocolIEs_EntryValue,
}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "0",
    sz_ub = "65535"
)]
pub struct UEContextSuspendRequestProtocolIEs(Vec<UEContextSuspendRequestProtocolIEs_Entry>);

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum UEContextSuspendResponseProtocolIEs_EntryValue {
    #[asn(key = 58)]
    Id_CriticalityDiagnostics(CriticalityDiagnostics),
    #[asn(key = 0)]
    Id_MME_UE_S1AP_ID(MME_UE_S1AP_ID),
    #[asn(key = 40)]
    Id_SecurityContext(SecurityContext),
    #[asn(key = 8)]
    Id_eNB_UE_S1AP_ID(ENB_UE_S1AP_ID),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct UEContextSuspendResponseProtocolIEs_Entry {
    #[asn(key_field = true)]
    pub id: ProtocolIE_ID,
    pub criticality: Criticality,
    pub value: UEContextSuspendResponseProtocolIEs_EntryValue,
}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "0",
    sz_ub = "65535"
)]
pub struct UEContextSuspendResponseProtocolIEs(Vec<UEContextSuspendResponseProtocolIEs_Entry>);

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum UEInformationTransferProtocolIEs_EntryValue {
    #[asn(key = 283)]
    Id_PendingDataIndication(PendingDataIndication),
    #[asn(key = 96)]
    Id_S_TMSI(S_TMSI),
    #[asn(key = 278)]
    Id_Subscription_Based_UE_DifferentiationInfo(Subscription_Based_UE_DifferentiationInfo),
    #[asn(key = 252)]
    Id_UE_Level_QoS_Parameters(E_RABLevelQoSParameters),
    #[asn(key = 74)]
    Id_UERadioCapability(UERadioCapability),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct UEInformationTransferProtocolIEs_Entry {
    #[asn(key_field = true)]
    pub id: ProtocolIE_ID,
    pub criticality: Criticality,
    pub value: UEInformationTransferProtocolIEs_EntryValue,
}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "0",
    sz_ub = "65535"
)]
pub struct UEInformationTransferProtocolIEs(Vec<UEInformationTransferProtocolIEs_Entry>);

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum UERadioCapabilityIDMappingRequestProtocolIEs_EntryValue {
    #[asn(key = 314)]
    Id_UERadioCapabilityID(UERadioCapabilityID),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct UERadioCapabilityIDMappingRequestProtocolIEs_Entry {
    #[asn(key_field = true)]
    pub id: ProtocolIE_ID,
    pub criticality: Criticality,
    pub value: UERadioCapabilityIDMappingRequestProtocolIEs_EntryValue,
}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "0",
    sz_ub = "65535"
)]
pub struct UERadioCapabilityIDMappingRequestProtocolIEs(
    Vec<UERadioCapabilityIDMappingRequestProtocolIEs_Entry>,
);

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum UERadioCapabilityIDMappingResponseProtocolIEs_EntryValue {
    #[asn(key = 58)]
    Id_CriticalityDiagnostics(CriticalityDiagnostics),
    #[asn(key = 74)]
    Id_UERadioCapability(UERadioCapability),
    #[asn(key = 314)]
    Id_UERadioCapabilityID(UERadioCapabilityID),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct UERadioCapabilityIDMappingResponseProtocolIEs_Entry {
    #[asn(key_field = true)]
    pub id: ProtocolIE_ID,
    pub criticality: Criticality,
    pub value: UERadioCapabilityIDMappingResponseProtocolIEs_EntryValue,
}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "0",
    sz_ub = "65535"
)]
pub struct UERadioCapabilityIDMappingResponseProtocolIEs(
    Vec<UERadioCapabilityIDMappingResponseProtocolIEs_Entry>,
);

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum UERadioCapabilityMatchRequestProtocolIEs_EntryValue {
    #[asn(key = 0)]
    Id_MME_UE_S1AP_ID(MME_UE_S1AP_ID),
    #[asn(key = 74)]
    Id_UERadioCapability(UERadioCapability),
    #[asn(key = 314)]
    Id_UERadioCapabilityID(UERadioCapabilityID),
    #[asn(key = 8)]
    Id_eNB_UE_S1AP_ID(ENB_UE_S1AP_ID),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct UERadioCapabilityMatchRequestProtocolIEs_Entry {
    #[asn(key_field = true)]
    pub id: ProtocolIE_ID,
    pub criticality: Criticality,
    pub value: UERadioCapabilityMatchRequestProtocolIEs_EntryValue,
}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "0",
    sz_ub = "65535"
)]
pub struct UERadioCapabilityMatchRequestProtocolIEs(
    Vec<UERadioCapabilityMatchRequestProtocolIEs_Entry>,
);

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum UERadioCapabilityMatchResponseProtocolIEs_EntryValue {
    #[asn(key = 58)]
    Id_CriticalityDiagnostics(CriticalityDiagnostics),
    #[asn(key = 0)]
    Id_MME_UE_S1AP_ID(MME_UE_S1AP_ID),
    #[asn(key = 169)]
    Id_VoiceSupportMatchIndicator(VoiceSupportMatchIndicator),
    #[asn(key = 8)]
    Id_eNB_UE_S1AP_ID(ENB_UE_S1AP_ID),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct UERadioCapabilityMatchResponseProtocolIEs_Entry {
    #[asn(key_field = true)]
    pub id: ProtocolIE_ID,
    pub criticality: Criticality,
    pub value: UERadioCapabilityMatchResponseProtocolIEs_EntryValue,
}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "0",
    sz_ub = "65535"
)]
pub struct UERadioCapabilityMatchResponseProtocolIEs(
    Vec<UERadioCapabilityMatchResponseProtocolIEs_Entry>,
);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct UESecurityCapabilitiesIE_Extensions_Entry {}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct UESecurityCapabilitiesIE_Extensions(Vec<UESecurityCapabilitiesIE_Extensions_Entry>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct UESidelinkAggregateMaximumBitrateIE_Extensions_Entry {}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct UESidelinkAggregateMaximumBitrateIE_Extensions(
    Vec<UESidelinkAggregateMaximumBitrateIE_Extensions_Entry>,
);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct UL_CP_SecurityInformationIE_Extensions_Entry {}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct UL_CP_SecurityInformationIE_Extensions(
    Vec<UL_CP_SecurityInformationIE_Extensions_Entry>,
);

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum UnsuccessfulOutcomeValue {
    #[asn(key = 29)]
    Id_ENBConfigurationUpdate(ENBConfigurationUpdateFailure),
    #[asn(key = 0)]
    Id_HandoverPreparation(HandoverPreparationFailure),
    #[asn(key = 1)]
    Id_HandoverResourceAllocation(HandoverFailure),
    #[asn(key = 9)]
    Id_InitialContextSetup(InitialContextSetupFailure),
    #[asn(key = 30)]
    Id_MMEConfigurationUpdate(MMEConfigurationUpdateFailure),
    #[asn(key = 3)]
    Id_PathSwitchRequest(PathSwitchRequestFailure),
    #[asn(key = 17)]
    Id_S1Setup(S1SetupFailure),
    #[asn(key = 21)]
    Id_UEContextModification(UEContextModificationFailure),
    #[asn(key = 56)]
    Id_UEContextResume(UEContextResumeFailure),
}

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum UplinkNASTransportProtocolIEs_EntryValue {
    #[asn(key = 100)]
    Id_EUTRAN_CGI(EUTRAN_CGI),
    #[asn(key = 155)]
    Id_GW_TransportLayerAddress(TransportLayerAddress),
    #[asn(key = 186)]
    Id_LHN_ID(LHN_ID),
    #[asn(key = 0)]
    Id_MME_UE_S1AP_ID(MME_UE_S1AP_ID),
    #[asn(key = 26)]
    Id_NAS_PDU(NAS_PDU),
    #[asn(key = 288)]
    Id_PSCellInformation(PSCellInformation),
    #[asn(key = 184)]
    Id_SIPTO_L_GW_TransportLayerAddress(TransportLayerAddress),
    #[asn(key = 67)]
    Id_TAI(TAI),
    #[asn(key = 8)]
    Id_eNB_UE_S1AP_ID(ENB_UE_S1AP_ID),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct UplinkNASTransportProtocolIEs_Entry {
    #[asn(key_field = true)]
    pub id: ProtocolIE_ID,
    pub criticality: Criticality,
    pub value: UplinkNASTransportProtocolIEs_EntryValue,
}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "0",
    sz_ub = "65535"
)]
pub struct UplinkNASTransportProtocolIEs(Vec<UplinkNASTransportProtocolIEs_Entry>);

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum UplinkNonUEAssociatedLPPaTransportProtocolIEs_EntryValue {
    #[asn(key = 147)]
    Id_LPPa_PDU(LPPa_PDU),
    #[asn(key = 148)]
    Id_Routing_ID(Routing_ID),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct UplinkNonUEAssociatedLPPaTransportProtocolIEs_Entry {
    #[asn(key_field = true)]
    pub id: ProtocolIE_ID,
    pub criticality: Criticality,
    pub value: UplinkNonUEAssociatedLPPaTransportProtocolIEs_EntryValue,
}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "0",
    sz_ub = "65535"
)]
pub struct UplinkNonUEAssociatedLPPaTransportProtocolIEs(
    Vec<UplinkNonUEAssociatedLPPaTransportProtocolIEs_Entry>,
);

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum UplinkS1cdma2000tunnellingProtocolIEs_EntryValue {
    #[asn(key = 140)]
    Id_EUTRANRoundTripDelayEstimationInfo(EUTRANRoundTripDelayEstimationInfo),
    #[asn(key = 0)]
    Id_MME_UE_S1AP_ID(MME_UE_S1AP_ID),
    #[asn(key = 84)]
    Id_cdma2000HORequiredIndication(Cdma2000HORequiredIndication),
    #[asn(key = 97)]
    Id_cdma2000OneXRAND(Cdma2000OneXRAND),
    #[asn(key = 102)]
    Id_cdma2000OneXSRVCCInfo(Cdma2000OneXSRVCCInfo),
    #[asn(key = 70)]
    Id_cdma2000PDU(Cdma2000PDU),
    #[asn(key = 71)]
    Id_cdma2000RATType(Cdma2000RATType),
    #[asn(key = 72)]
    Id_cdma2000SectorID(Cdma2000SectorID),
    #[asn(key = 8)]
    Id_eNB_UE_S1AP_ID(ENB_UE_S1AP_ID),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct UplinkS1cdma2000tunnellingProtocolIEs_Entry {
    #[asn(key_field = true)]
    pub id: ProtocolIE_ID,
    pub criticality: Criticality,
    pub value: UplinkS1cdma2000tunnellingProtocolIEs_EntryValue,
}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "0",
    sz_ub = "65535"
)]
pub struct UplinkS1cdma2000tunnellingProtocolIEs(Vec<UplinkS1cdma2000tunnellingProtocolIEs_Entry>);

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum UplinkUEAssociatedLPPaTransportProtocolIEs_EntryValue {
    #[asn(key = 147)]
    Id_LPPa_PDU(LPPa_PDU),
    #[asn(key = 0)]
    Id_MME_UE_S1AP_ID(MME_UE_S1AP_ID),
    #[asn(key = 148)]
    Id_Routing_ID(Routing_ID),
    #[asn(key = 8)]
    Id_eNB_UE_S1AP_ID(ENB_UE_S1AP_ID),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct UplinkUEAssociatedLPPaTransportProtocolIEs_Entry {
    #[asn(key_field = true)]
    pub id: ProtocolIE_ID,
    pub criticality: Criticality,
    pub value: UplinkUEAssociatedLPPaTransportProtocolIEs_EntryValue,
}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "0",
    sz_ub = "65535"
)]
pub struct UplinkUEAssociatedLPPaTransportProtocolIEs(
    Vec<UplinkUEAssociatedLPPaTransportProtocolIEs_Entry>,
);

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum UserLocationInformationIE_Extensions_EntryExtensionValue {
    #[asn(key = 288)]
    Id_PSCellInformation(PSCellInformation),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct UserLocationInformationIE_Extensions_Entry {
    #[asn(key_field = true)]
    pub id: ProtocolExtensionID,
    pub criticality: Criticality,
    pub extension_value: UserLocationInformationIE_Extensions_EntryExtensionValue,
}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct UserLocationInformationIE_Extensions(Vec<UserLocationInformationIE_Extensions_Entry>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct V2XServicesAuthorizedIE_Extensions_Entry {}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct V2XServicesAuthorizedIE_Extensions(Vec<V2XServicesAuthorizedIE_Extensions_Entry>);

#[derive(Debug, AperCodec)]
#[asn(type = "ENUMERATED", extensible = true, lb = "0", ub = "0")]
pub struct ENUMERATED_67(u8);
impl ENUMERATED_67 {
    const TRUE: u8 = 0u8;
}

#[derive(Debug, AperCodec)]
#[asn(type = "ENUMERATED", extensible = true, lb = "0", ub = "0")]
pub struct ENUMERATED_68(u8);
impl ENUMERATED_68 {
    const TRUE: u8 = 0u8;
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct WLANMeasurementConfigurationIE_Extensions_Entry {}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct WLANMeasurementConfigurationIE_Extensions(
    Vec<WLANMeasurementConfigurationIE_Extensions_Entry>,
);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct WUS_Assistance_InformationIE_Extensions_Entry {}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct WUS_Assistance_InformationIE_Extensions(
    Vec<WUS_Assistance_InformationIE_Extensions_Entry>,
);

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum WriteReplaceWarningRequestProtocolIEs_EntryValue {
    #[asn(key = 142)]
    Id_ConcurrentWarningMessageIndicator(ConcurrentWarningMessageIndicator),
    #[asn(key = 118)]
    Id_DataCodingScheme(DataCodingScheme),
    #[asn(key = 144)]
    Id_ExtendedRepetitionPeriod(ExtendedRepetitionPeriod),
    #[asn(key = 111)]
    Id_MessageIdentifier(MessageIdentifier),
    #[asn(key = 115)]
    Id_NumberofBroadcastRequest(NumberofBroadcastRequest),
    #[asn(key = 114)]
    Id_RepetitionPeriod(RepetitionPeriod),
    #[asn(key = 112)]
    Id_SerialNumber(SerialNumber),
    #[asn(key = 286)]
    Id_WarningAreaCoordinates(WarningAreaCoordinates),
    #[asn(key = 113)]
    Id_WarningAreaList(WarningAreaList),
    #[asn(key = 119)]
    Id_WarningMessageContents(WarningMessageContents),
    #[asn(key = 117)]
    Id_WarningSecurityInfo(WarningSecurityInfo),
    #[asn(key = 116)]
    Id_WarningType(WarningType),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct WriteReplaceWarningRequestProtocolIEs_Entry {
    #[asn(key_field = true)]
    pub id: ProtocolIE_ID,
    pub criticality: Criticality,
    pub value: WriteReplaceWarningRequestProtocolIEs_EntryValue,
}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "0",
    sz_ub = "65535"
)]
pub struct WriteReplaceWarningRequestProtocolIEs(Vec<WriteReplaceWarningRequestProtocolIEs_Entry>);

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum WriteReplaceWarningResponseProtocolIEs_EntryValue {
    #[asn(key = 120)]
    Id_BroadcastCompletedAreaList(BroadcastCompletedAreaList),
    #[asn(key = 58)]
    Id_CriticalityDiagnostics(CriticalityDiagnostics),
    #[asn(key = 111)]
    Id_MessageIdentifier(MessageIdentifier),
    #[asn(key = 112)]
    Id_SerialNumber(SerialNumber),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct WriteReplaceWarningResponseProtocolIEs_Entry {
    #[asn(key_field = true)]
    pub id: ProtocolIE_ID,
    pub criticality: Criticality,
    pub value: WriteReplaceWarningResponseProtocolIEs_EntryValue,
}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "0",
    sz_ub = "65535"
)]
pub struct WriteReplaceWarningResponseProtocolIEs(
    Vec<WriteReplaceWarningResponseProtocolIEs_Entry>,
);

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum X2TNLConfigurationInfoIE_Extensions_EntryExtensionValue {
    #[asn(key = 193)]
    Id_eNBIndirectX2TransportLayerAddresses(ENBIndirectX2TransportLayerAddresses),
    #[asn(key = 153)]
    Id_eNBX2ExtendedTransportLayerAddresses(ENBX2ExtTLAs),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct X2TNLConfigurationInfoIE_Extensions_Entry {
    #[asn(key_field = true)]
    pub id: ProtocolExtensionID,
    pub criticality: Criticality,
    pub extension_value: X2TNLConfigurationInfoIE_Extensions_EntryExtensionValue,
}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct X2TNLConfigurationInfoIE_Extensions(Vec<X2TNLConfigurationInfoIE_Extensions_Entry>);

fn main() {
    eprintln!("S1AP");
}
