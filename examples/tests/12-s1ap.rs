#![allow(non_camel_case_types, dead_code, unreachable_patterns)]

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
    pub g_ummei: GUMMEI,
    pub m_tmsi: M_TMSI,
    #[asn(optional_idx = 0)]
    pub i_e_extensions: Option<Additional_GUTIiE_Extensions>,
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
    pub i_e_extensions: Option<AllocationAndRetentionPriorityiE_Extensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "CHOICE", lb = "0", ub = "2", extensible = true)]
pub enum AreaScopeOfMDT {
    #[asn(key = 0, extended = false)]
    cellBased(CellBasedMDT),
    #[asn(key = 1, extended = false)]
    tABased(TABasedMDT),
    #[asn(key = 2, extended = false)]
    pLMNWide(NULL_3),
    #[asn(key = 0, extended = true)]
    tAIBased(TAIBasedMDT),
}

#[derive(Debug, AperCodec)]
#[asn(type = "CHOICE", lb = "0", ub = "3", extensible = true)]
pub enum AreaScopeOfQMC {
    #[asn(key = 0, extended = false)]
    cellBased(CellBasedQMC),
    #[asn(key = 1, extended = false)]
    tABased(TABasedQMC),
    #[asn(key = 2, extended = false)]
    tAIBased(TAIBasedQMC),
    #[asn(key = 3, extended = false)]
    pLMNAreaBased(PLMNAreaBasedQMC),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct AssistanceDataForCECapableUEs {
    pub cell_identifier_and_ce_level_for_ce_capable_u_es: CellIdentifierAndCELevelForCECapableUEs,
    #[asn(optional_idx = 0)]
    pub i_e_extensions: Option<AssistanceDataForCECapableUEsiE_Extensions>,
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
    pub i_e_extensions: Option<AssistanceDataForPagingiE_Extensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct AssistanceDataForRecommendedCells {
    pub recommended_cells_for_paging: RecommendedCellsForPaging,
    #[asn(optional_idx = 0)]
    pub i_e_extensions: Option<AssistanceDataForRecommendedCellsiE_Extensions>,
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
    pub d_lcount_pdcp_s_nlength: DLCOUNT_PDCP_SNlength,
    #[asn(optional_idx = 0)]
    pub i_e_extensions: Option<Bearers_SubjectToEarlyStatusTransfer_ItemiE_Extensions>,
}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "256"
)]
pub struct Bearers_SubjectToEarlyStatusTransferList(
    Vec<Bearers_SubjectToEarlyStatusTransferList_Item>,
);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 2)]
pub struct Bearers_SubjectToStatusTransfer_Item {
    pub e_rab_id: E_RAB_ID,
    pub u_l_coun_tvalue: COUNTvalue,
    pub d_l_coun_tvalue: COUNTvalue,
    #[asn(optional_idx = 0)]
    pub receive_statusof_ulpdcpsd_us: Option<ReceiveStatusofULPDCPSDUs>,
    #[asn(optional_idx = 1)]
    pub i_e_extensions: Option<Bearers_SubjectToStatusTransfer_ItemiE_Extensions>,
}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "256"
)]
pub struct Bearers_SubjectToStatusTransferList(Vec<Bearers_SubjectToStatusTransferList_Item>);

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
    pub i_e_extensions: Option<BluetoothMeasurementConfigurationiE_Extensions>,
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
    cellID_Cancelled(CellID_Cancelled),
    #[asn(key = 1, extended = false)]
    tAI_Cancelled(TAI_Cancelled),
    #[asn(key = 2, extended = false)]
    emergencyAreaID_Cancelled(EmergencyAreaID_Cancelled),
}

#[derive(Debug, AperCodec)]
#[asn(type = "CHOICE", lb = "0", ub = "2", extensible = true)]
pub enum BroadcastCompletedAreaList {
    #[asn(key = 0, extended = false)]
    cellID_Broadcast(CellID_Broadcast),
    #[asn(key = 1, extended = false)]
    tAI_Broadcast(TAI_Broadcast),
    #[asn(key = 2, extended = false)]
    emergencyAreaID_Broadcast(EmergencyAreaID_Broadcast),
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
    pub p_lm_nidentity: PLMNidentity,
    pub l_ac: LAC,
    pub c_i: CI,
    #[asn(optional_idx = 0)]
    pub r_ac: Option<RAC>,
    #[asn(optional_idx = 1)]
    pub i_e_extensions: Option<CGIiE_Extensions>,
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
    pub p_lmn_identity: PLMNidentity,
    pub c_n_type: CNType,
    #[asn(optional_idx = 0)]
    pub i_e_extensions: Option<CNTypeRestrictions_ItemiE_Extensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct COUNTValueExtended {
    pub p_dcp_sn_extended: PDCP_SNExtended,
    pub h_fn_modified: HFNModified,
    #[asn(optional_idx = 0)]
    pub i_e_extensions: Option<COUNTValueExtendediE_Extensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct COUNTvalue {
    pub p_dcp_sn: PDCP_SN,
    pub h_fn: HFN,
    #[asn(optional_idx = 0)]
    pub i_e_extensions: Option<COUNTvalueiE_Extensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct COUNTvaluePDCP_SNlength18 {
    pub p_dcp_s_nlength18: PDCP_SNlength18,
    pub h_f_nfor_pdcp_s_nlength18: HFNforPDCP_SNlength18,
    #[asn(optional_idx = 0)]
    pub i_e_extensions: Option<COUNTvaluePDCP_SNlength18iE_Extensions>,
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
    pub c_sg_id: CSG_Id,
    #[asn(optional_idx = 0)]
    pub i_e_extensions: Option<CSG_IdList_ItemiE_Extensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 3)]
pub struct CSGMembershipInfo {
    pub c_sg_membership_status: CSGMembershipStatus,
    pub c_sg_id: CSG_Id,
    #[asn(optional_idx = 0)]
    pub cell_access_mode: Option<CellAccessMode>,
    #[asn(optional_idx = 1)]
    pub p_lm_nidentity: Option<PLMNidentity>,
    #[asn(optional_idx = 2)]
    pub i_e_extensions: Option<CSGMembershipInfoiE_Extensions>,
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
    pub e_cgi: EUTRAN_CGI,
    pub number_of_broadcasts: NumberOfBroadcasts,
    #[asn(optional_idx = 0)]
    pub i_e_extensions: Option<CancelledCellinEAI_ItemiE_Extensions>,
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
    pub e_cgi: EUTRAN_CGI,
    pub number_of_broadcasts: NumberOfBroadcasts,
    #[asn(optional_idx = 0)]
    pub i_e_extensions: Option<CancelledCellinTAI_ItemiE_Extensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "16")]
pub struct CandidateCellList(Vec<IRAT_Cell_ID>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true)]
pub struct CandidatePCI {
    pub p_ci: INTEGER_5,
    pub e_arfcn: OCTET_STRING_6,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "16")]
pub struct CandidatePCIList(Vec<CandidatePCI>);

#[derive(Debug, AperCodec)]
#[asn(type = "CHOICE", lb = "0", ub = "4", extensible = true)]
pub enum Cause {
    #[asn(key = 0, extended = false)]
    radioNetwork(CauseRadioNetwork),
    #[asn(key = 1, extended = false)]
    transport(CauseTransport),
    #[asn(key = 2, extended = false)]
    nas(CauseNas),
    #[asn(key = 3, extended = false)]
    protocol(CauseProtocol),
    #[asn(key = 4, extended = false)]
    misc(CauseMisc),
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
    pub i_e_extensions: Option<Cdma2000OneXSRVCCInfoiE_Extensions>,
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
    pub i_e_extensions: Option<CellBasedMDTiE_Extensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct CellBasedQMC {
    pub cell_id_listfor_qmc: CellIdListforQMC,
    #[asn(optional_idx = 0)]
    pub i_e_extensions: Option<CellBasedQMCiE_Extensions>,
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
    pub e_cgi: EUTRAN_CGI,
    #[asn(optional_idx = 0)]
    pub i_e_extensions: Option<CellID_Broadcast_ItemiE_Extensions>,
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
    pub e_cgi: EUTRAN_CGI,
    pub number_of_broadcasts: NumberOfBroadcasts,
    #[asn(optional_idx = 0)]
    pub i_e_extensions: Option<CellID_Cancelled_ItemiE_Extensions>,
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
    pub c_e_level: CELevel,
    #[asn(optional_idx = 0)]
    pub i_e_extensions: Option<CellIdentifierAndCELevelForCECapableUEsiE_Extensions>,
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
    eUTRAN(EUTRANcellLoadReportingResponse),
    #[asn(key = 1, extended = false)]
    uTRAN(OCTET_STRING_8),
    #[asn(key = 2, extended = false)]
    gERAN(OCTET_STRING_9),
    #[asn(key = 0, extended = true)]
    eHRPD(EHRPDSectorLoadReportingResponse),
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
    pub protocol_i_es: CellTrafficTraceprotocolIEs,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct CellType {
    pub cell_size: Cell_Size,
    #[asn(optional_idx = 0)]
    pub i_e_extensions: Option<CellTypeiE_Extensions>,
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
    pub e_cgi: EUTRAN_CGI,
    #[asn(optional_idx = 0)]
    pub i_e_extensions: Option<CompletedCellinEAI_ItemiE_Extensions>,
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
    pub e_cgi: EUTRAN_CGI,
    #[asn(optional_idx = 0)]
    pub i_e_extensions: Option<CompletedCellinTAI_ItemiE_Extensions>,
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
    pub i_e_extensions: Option<ConnectedengNBItemiE_Extensions>,
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
    pub protocol_i_es: ConnectionEstablishmentIndicationprotocolIEs,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct ContextatSource {
    pub source_ng_ran_node_id: Global_RAN_NODE_ID,
    pub r_an_ue_ngap_id: RAN_UE_NGAP_ID,
    #[asn(optional_idx = 0)]
    pub i_e_extensions: Option<ContextatSourceiE_Extensions>,
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
    pub i_e_extensions: Option<CriticalityDiagnosticsiE_Extensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct CriticalityDiagnostics_IE_Item {
    pub i_e_criticality: Criticality,
    pub i_e_id: ProtocolIE_ID,
    pub type_of_error: TypeOfError,
    #[asn(optional_idx = 0)]
    pub i_e_extensions: Option<CriticalityDiagnostics_IE_ItemiE_Extensions>,
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
    pub d_aps_indicator: ENUMERATED_11,
    #[asn(optional_idx = 0)]
    pub i_e_extensions: Option<DAPSRequestInfoiE_Extensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct DAPSResponseInfo {
    pub dapsresponseindicator: ENUMERATED_12,
    #[asn(optional_idx = 0)]
    pub i_e_extensions: Option<DAPSResponseInfoiE_Extensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct DAPSResponseInfoItem {
    pub e_rab_id: E_RAB_ID,
    pub d_aps_response_info: DAPSResponseInfo,
    #[asn(optional_idx = 0)]
    pub i_e_extensions: Option<DAPSResponseInfoItemiE_Extensions>,
}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "256"
)]
pub struct DAPSResponseInfoList(Vec<DAPSResponseInfoList_Item>);

#[derive(Debug, AperCodec)]
#[asn(type = "INTEGER", lb = "0", ub = "65535")]
pub struct DCN_ID(u16);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct DL_CP_SecurityInformation {
    pub dl_nas_mac: DL_NAS_MAC,
    #[asn(optional_idx = 0)]
    pub i_e_extensions: Option<DL_CP_SecurityInformationiE_Extensions>,
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
    dLCOUNTValuePDCP_SNlength12(COUNTvalue),
    #[asn(key = 1, extended = false)]
    dLCOUNTValuePDCP_SNlength15(COUNTValueExtended),
    #[asn(key = 2, extended = false)]
    dLCOUNTValuePDCP_SNlength18(COUNTvaluePDCP_SNlength18),
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
    pub protocol_i_es: DeactivateTraceprotocolIEs,
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
    pub protocol_i_es: DownlinkNASTransportprotocolIEs,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true)]
pub struct DownlinkNonUEAssociatedLPPaTransport {
    pub protocol_i_es: DownlinkNonUEAssociatedLPPaTransportprotocolIEs,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true)]
pub struct DownlinkS1cdma2000tunnelling {
    pub protocol_i_es: DownlinkS1cdma2000tunnellingprotocolIEs,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true)]
pub struct DownlinkUEAssociatedLPPaTransport {
    pub protocol_i_es: DownlinkUEAssociatedLPPaTransportprotocolIEs,
}

#[derive(Debug, AperCodec)]
#[asn(type = "INTEGER", lb = "0", ub = "15", extensible = true)]
pub struct E_RAB_ID(u8);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 5)]
pub struct E_RABAdmittedItem {
    pub e_rab_id: E_RAB_ID,
    pub transport_layer_address: TransportLayerAddress,
    pub g_tp_teid: GTP_TEID,
    #[asn(optional_idx = 0)]
    pub d_l_transport_layer_address: Option<TransportLayerAddress>,
    #[asn(optional_idx = 1)]
    pub d_l_g_tp_teid: Option<GTP_TEID>,
    #[asn(optional_idx = 2)]
    pub u_l_transport_layer_address: Option<TransportLayerAddress>,
    #[asn(optional_idx = 3)]
    pub u_l_gtp_teid: Option<GTP_TEID>,
    #[asn(optional_idx = 4)]
    pub i_e_extensions: Option<E_RABAdmittedItemiE_Extensions>,
}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "256"
)]
pub struct E_RABAdmittedList(Vec<E_RABAdmittedList_Item>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 5)]
pub struct E_RABDataForwardingItem {
    pub e_rab_id: E_RAB_ID,
    #[asn(optional_idx = 0)]
    pub d_l_transport_layer_address: Option<TransportLayerAddress>,
    #[asn(optional_idx = 1)]
    pub d_l_g_tp_teid: Option<GTP_TEID>,
    #[asn(optional_idx = 2)]
    pub u_l_transport_layer_address: Option<TransportLayerAddress>,
    #[asn(optional_idx = 3)]
    pub u_l_gtp_teid: Option<GTP_TEID>,
    #[asn(optional_idx = 4)]
    pub i_e_extensions: Option<E_RABDataForwardingItemiE_Extensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct E_RABFailedToResumeItemResumeReq {
    pub e_rab_id: E_RAB_ID,
    pub cause: Cause,
    #[asn(optional_idx = 0)]
    pub i_e_extensions: Option<E_RABFailedToResumeItemResumeReqiE_Extensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct E_RABFailedToResumeItemResumeRes {
    pub e_rab_id: E_RAB_ID,
    pub cause: Cause,
    #[asn(optional_idx = 0)]
    pub i_e_extensions: Option<E_RABFailedToResumeItemResumeResiE_Extensions>,
}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "256"
)]
pub struct E_RABFailedToResumeListResumeReq(Vec<E_RABFailedToResumeListResumeReq_Item>);

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "256"
)]
pub struct E_RABFailedToResumeListResumeRes(Vec<E_RABFailedToResumeListResumeRes_Item>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct E_RABFailedToSetupItemHOReqAck {
    pub e_rab_id: E_RAB_ID,
    pub cause: Cause,
    #[asn(optional_idx = 0)]
    pub i_e_extensions: Option<E_RABFailedToSetupItemHOReqAckiE_Extensions>,
}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "256"
)]
pub struct E_RABFailedtoSetupListHOReqAck(Vec<E_RABFailedtoSetupListHOReqAck_Item>);

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "256"
)]
pub struct E_RABInformationList(Vec<E_RABInformationList_Item>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 2)]
pub struct E_RABInformationListItem {
    pub e_rab_id: E_RAB_ID,
    #[asn(optional_idx = 0)]
    pub d_l_forwarding: Option<DL_Forwarding>,
    #[asn(optional_idx = 1)]
    pub i_e_extensions: Option<E_RABInformationListItemiE_Extensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct E_RABItem {
    pub e_rab_id: E_RAB_ID,
    pub cause: Cause,
    #[asn(optional_idx = 0)]
    pub i_e_extensions: Option<E_RABItemiE_Extensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 2)]
pub struct E_RABLevelQoSParameters {
    pub q_ci: QCI,
    pub allocation_retention_priority: AllocationAndRetentionPriority,
    #[asn(optional_idx = 0)]
    pub gbr_qos_information: Option<GBR_QosInformation>,
    #[asn(optional_idx = 1)]
    pub i_e_extensions: Option<E_RABLevelQoSParametersiE_Extensions>,
}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "256"
)]
pub struct E_RABList(Vec<E_RABList_Item>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true)]
pub struct E_RABModificationConfirm {
    pub protocol_i_es: E_RABModificationConfirmprotocolIEs,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true)]
pub struct E_RABModificationIndication {
    pub protocol_i_es: E_RABModificationIndicationprotocolIEs,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct E_RABModifyItemBearerModConf {
    pub e_rab_id: E_RAB_ID,
    #[asn(optional_idx = 0)]
    pub i_e_extensions: Option<E_RABModifyItemBearerModConfiE_Extensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct E_RABModifyItemBearerModRes {
    pub e_rab_id: E_RAB_ID,
    #[asn(optional_idx = 0)]
    pub i_e_extensions: Option<E_RABModifyItemBearerModResiE_Extensions>,
}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "256"
)]
pub struct E_RABModifyListBearerModConf(Vec<E_RABModifyListBearerModConf_Item>);

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "256"
)]
pub struct E_RABModifyListBearerModRes(Vec<E_RABModifyListBearerModRes_Item>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true)]
pub struct E_RABModifyRequest {
    pub protocol_i_es: E_RABModifyRequestprotocolIEs,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true)]
pub struct E_RABModifyResponse {
    pub protocol_i_es: E_RABModifyResponseprotocolIEs,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct E_RABNotToBeModifiedItemBearerModInd {
    pub e_rab_id: E_RAB_ID,
    pub transport_layer_address: TransportLayerAddress,
    pub d_l_gtp_teid: GTP_TEID,
    #[asn(optional_idx = 0)]
    pub i_e_extensions: Option<E_RABNotToBeModifiedItemBearerModIndiE_Extensions>,
}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "256"
)]
pub struct E_RABNotToBeModifiedListBearerModInd(Vec<E_RABNotToBeModifiedListBearerModInd_Item>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true)]
pub struct E_RABReleaseCommand {
    pub protocol_i_es: E_RABReleaseCommandprotocolIEs,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true)]
pub struct E_RABReleaseIndication {
    pub protocol_i_es: E_RABReleaseIndicationprotocolIEs,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct E_RABReleaseItemBearerRelComp {
    pub e_rab_id: E_RAB_ID,
    #[asn(optional_idx = 0)]
    pub i_e_extensions: Option<E_RABReleaseItemBearerRelCompiE_Extensions>,
}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "256"
)]
pub struct E_RABReleaseListBearerRelComp(Vec<E_RABReleaseListBearerRelComp_Item>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true)]
pub struct E_RABReleaseResponse {
    pub protocol_i_es: E_RABReleaseResponseprotocolIEs,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct E_RABSetupItemBearerSURes {
    pub e_rab_id: E_RAB_ID,
    pub transport_layer_address: TransportLayerAddress,
    pub g_tp_teid: GTP_TEID,
    #[asn(optional_idx = 0)]
    pub i_e_extensions: Option<E_RABSetupItemBearerSUResiE_Extensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct E_RABSetupItemCtxtSURes {
    pub e_rab_id: E_RAB_ID,
    pub transport_layer_address: TransportLayerAddress,
    pub g_tp_teid: GTP_TEID,
    #[asn(optional_idx = 0)]
    pub i_e_extensions: Option<E_RABSetupItemCtxtSUResiE_Extensions>,
}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "256"
)]
pub struct E_RABSetupListBearerSURes(Vec<E_RABSetupListBearerSURes_Item>);

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "256"
)]
pub struct E_RABSetupListCtxtSURes(Vec<E_RABSetupListCtxtSURes_Item>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true)]
pub struct E_RABSetupRequest {
    pub protocol_i_es: E_RABSetupRequestprotocolIEs,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true)]
pub struct E_RABSetupResponse {
    pub protocol_i_es: E_RABSetupResponseprotocolIEs,
}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "256"
)]
pub struct E_RABSubjecttoDataForwardingList(Vec<E_RABSubjecttoDataForwardingList_Item>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct E_RABToBeModifiedItemBearerModInd {
    pub e_rab_id: E_RAB_ID,
    pub transport_layer_address: TransportLayerAddress,
    pub d_l_gtp_teid: GTP_TEID,
    #[asn(optional_idx = 0)]
    pub i_e_extensions: Option<E_RABToBeModifiedItemBearerModIndiE_Extensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct E_RABToBeModifiedItemBearerModReq {
    pub e_rab_id: E_RAB_ID,
    pub e_rab_level_qo_s_parameters: E_RABLevelQoSParameters,
    pub n_as_pdu: NAS_PDU,
    #[asn(optional_idx = 0)]
    pub i_e_extensions: Option<E_RABToBeModifiedItemBearerModReqiE_Extensions>,
}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "256"
)]
pub struct E_RABToBeModifiedListBearerModInd(Vec<E_RABToBeModifiedListBearerModInd_Item>);

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "256"
)]
pub struct E_RABToBeModifiedListBearerModReq(Vec<E_RABToBeModifiedListBearerModReq_Item>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct E_RABToBeSetupItemBearerSUReq {
    pub e_rab_id: E_RAB_ID,
    pub e_ra_blevel_qo_s_parameters: E_RABLevelQoSParameters,
    pub transport_layer_address: TransportLayerAddress,
    pub g_tp_teid: GTP_TEID,
    pub n_as_pdu: NAS_PDU,
    #[asn(optional_idx = 0)]
    pub i_e_extensions: Option<E_RABToBeSetupItemBearerSUReqiE_Extensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 2)]
pub struct E_RABToBeSetupItemCtxtSUReq {
    pub e_rab_id: E_RAB_ID,
    pub e_ra_blevel_qo_s_parameters: E_RABLevelQoSParameters,
    pub transport_layer_address: TransportLayerAddress,
    pub g_tp_teid: GTP_TEID,
    #[asn(optional_idx = 0)]
    pub n_as_pdu: Option<NAS_PDU>,
    #[asn(optional_idx = 1)]
    pub i_e_extensions: Option<E_RABToBeSetupItemCtxtSUReqiE_Extensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct E_RABToBeSetupItemHOReq {
    pub e_rab_id: E_RAB_ID,
    pub transport_layer_address: TransportLayerAddress,
    pub g_tp_teid: GTP_TEID,
    pub e_ra_blevel_qos_parameters: E_RABLevelQoSParameters,
    #[asn(optional_idx = 0)]
    pub i_e_extensions: Option<E_RABToBeSetupItemHOReqiE_Extensions>,
}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "256"
)]
pub struct E_RABToBeSetupListBearerSUReq(Vec<E_RABToBeSetupListBearerSUReq_Item>);

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "256"
)]
pub struct E_RABToBeSetupListCtxtSUReq(Vec<E_RABToBeSetupListCtxtSUReq_Item>);

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "256"
)]
pub struct E_RABToBeSetupListHOReq(Vec<E_RABToBeSetupListHOReq_Item>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct E_RABToBeSwitchedDLItem {
    pub e_rab_id: E_RAB_ID,
    pub transport_layer_address: TransportLayerAddress,
    pub g_tp_teid: GTP_TEID,
    #[asn(optional_idx = 0)]
    pub i_e_extensions: Option<E_RABToBeSwitchedDLItemiE_Extensions>,
}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "256"
)]
pub struct E_RABToBeSwitchedDLList(Vec<E_RABToBeSwitchedDLList_Item>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct E_RABToBeSwitchedULItem {
    pub e_rab_id: E_RAB_ID,
    pub transport_layer_address: TransportLayerAddress,
    pub g_tp_teid: GTP_TEID,
    #[asn(optional_idx = 0)]
    pub i_e_extensions: Option<E_RABToBeSwitchedULItemiE_Extensions>,
}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "256"
)]
pub struct E_RABToBeSwitchedULList(Vec<E_RABToBeSwitchedULList_Item>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct E_RABUsageReportItem {
    pub start_timestamp: OCTET_STRING_13,
    pub end_timestamp: OCTET_STRING_14,
    pub usage_count_ul: INTEGER_15,
    pub usage_count_dl: INTEGER_16,
    #[asn(optional_idx = 0)]
    pub i_e_extensions: Option<E_RABUsageReportItemiE_Extensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "2")]
pub struct E_RABUsageReportList(Vec<E_RABUsageReportList_Item>);

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
    pub e_hrpd_sector_capacity_class_value: EHRPDSectorCapacityClassValue,
    pub e_hrpd_capacity_value: EHRPDCapacityValue,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true)]
pub struct EHRPDMultiSectorLoadReportingResponseItem {
    pub e_hrpd_sector_id: EHRPD_Sector_ID,
    pub e_hrpd_sector_load_reporting_response: EHRPDSectorLoadReportingResponse,
}

#[derive(Debug, AperCodec)]
#[asn(type = "INTEGER", lb = "1", ub = "100", extensible = true)]
pub struct EHRPDSectorCapacityClassValue(u8);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true)]
pub struct EHRPDSectorLoadReportingResponse {
    pub d_l_ehrpd_composite_available_capacity: EHRPDCompositeAvailableCapacity,
    pub u_l_ehrpd_composite_available_capacity: EHRPDCompositeAvailableCapacity,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 2)]
pub struct EN_DCSONConfigurationTransfer {
    pub transfertype: EN_DCSONTransferType,
    pub s_on_information: SONInformation,
    #[asn(optional_idx = 0)]
    pub x2_tnl_config_info: Option<X2TNLConfigurationInfo>,
    #[asn(optional_idx = 1)]
    pub i_e_extensions: Option<EN_DCSONConfigurationTransferiE_Extensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "CHOICE", lb = "0", ub = "1", extensible = true)]
pub enum EN_DCSONTransferType {
    #[asn(key = 0, extended = false)]
    request(EN_DCTransferTypeRequest),
    #[asn(key = 1, extended = false)]
    reply(EN_DCTransferTypeReply),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct EN_DCSONeNBIdentification {
    pub globale_nbid: Global_ENB_ID,
    pub selected_tai: TAI,
    #[asn(optional_idx = 0)]
    pub i_e_extensions: Option<EN_DCSONeNBIdentificationiE_Extensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct EN_DCSONengNBIdentification {
    pub globaleng_nbid: Global_en_gNB_ID,
    pub selected_tai: TAI,
    #[asn(optional_idx = 0)]
    pub i_e_extensions: Option<EN_DCSONengNBIdentificationiE_Extensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct EN_DCTransferTypeReply {
    pub sourceeng_nb: EN_DCSONengNBIdentification,
    pub targete_nb: EN_DCSONeNBIdentification,
    #[asn(optional_idx = 0)]
    pub i_e_extensions: Option<EN_DCTransferTypeReplyiE_Extensions>,
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
    pub i_e_extensions: Option<EN_DCTransferTypeRequestiE_Extensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct ENB_EarlyStatusTransfer_TransparentContainer {
    pub bearers_subject_to_early_status_transfer_list: Bearers_SubjectToEarlyStatusTransferList,
    #[asn(optional_idx = 0)]
    pub i_e_extensions: Option<ENB_EarlyStatusTransfer_TransparentContaineriE_Extensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "CHOICE", lb = "0", ub = "1", extensible = true)]
pub enum ENB_ID {
    #[asn(key = 0, extended = false)]
    macroENB_ID(BIT_STRING_17),
    #[asn(key = 1, extended = false)]
    homeENB_ID(BIT_STRING_18),
    #[asn(key = 0, extended = true)]
    short_macroENB_ID(BIT_STRING_19),
    #[asn(key = 1, extended = true)]
    long_macroENB_ID(BIT_STRING_20),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct ENB_StatusTransfer_TransparentContainer {
    pub bearers_subject_to_status_transfer_list: Bearers_SubjectToStatusTransferList,
    #[asn(optional_idx = 0)]
    pub i_e_extensions: Option<ENB_StatusTransfer_TransparentContaineriE_Extensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "INTEGER", lb = "0", ub = "16777215")]
pub struct ENB_UE_S1AP_ID(u32);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true)]
pub struct ENBCPRelocationIndication {
    pub protocol_i_es: ENBCPRelocationIndicationprotocolIEs,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true)]
pub struct ENBConfigurationTransfer {
    pub protocol_i_es: ENBConfigurationTransferprotocolIEs,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true)]
pub struct ENBConfigurationUpdate {
    pub protocol_i_es: ENBConfigurationUpdateprotocolIEs,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true)]
pub struct ENBConfigurationUpdateAcknowledge {
    pub protocol_i_es: ENBConfigurationUpdateAcknowledgeprotocolIEs,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true)]
pub struct ENBConfigurationUpdateFailure {
    pub protocol_i_es: ENBConfigurationUpdateFailureprotocolIEs,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true)]
pub struct ENBDirectInformationTransfer {
    pub protocol_i_es: ENBDirectInformationTransferprotocolIEs,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true)]
pub struct ENBEarlyStatusTransfer {
    pub protocol_i_es: ENBEarlyStatusTransferprotocolIEs,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "2")]
pub struct ENBIndirectX2TransportLayerAddresses(Vec<TransportLayerAddress>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true)]
pub struct ENBStatusTransfer {
    pub protocol_i_es: ENBStatusTransferprotocolIEs,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 3)]
pub struct ENBX2ExtTLA {
    #[asn(optional_idx = 0)]
    pub i_psec_tla: Option<TransportLayerAddress>,
    #[asn(optional_idx = 1)]
    pub g_tptl_aa: Option<ENBX2GTPTLAs>,
    #[asn(optional_idx = 2)]
    pub i_e_extensions: Option<ENBX2ExtTLAiE_Extensions>,
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
    pub p_lm_nidentity: PLMNidentity,
    pub cell_id: CellIdentity,
    #[asn(optional_idx = 0)]
    pub i_e_extensions: Option<EUTRAN_CGIiE_Extensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true)]
pub struct EUTRANResponse {
    pub cell_id: OCTET_STRING_21,
    pub e_utra_ncell_load_reporting_response: EUTRANcellLoadReportingResponse,
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
    pub i_e_extensions: Option<EmergencyAreaID_Broadcast_ItemiE_Extensions>,
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
    pub i_e_extensions: Option<EmergencyAreaID_Cancelled_ItemiE_Extensions>,
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
    pub protocol_i_es: ErrorIndicationprotocolIEs,
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
    pub i_e_extensions: Option<ExpectedUEActivityBehaviouriE_Extensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 3)]
pub struct ExpectedUEBehaviour {
    #[asn(optional_idx = 0)]
    pub expected_activity: Option<ExpectedUEActivityBehaviour>,
    #[asn(optional_idx = 1)]
    pub expected_ho_interval: Option<ExpectedHOInterval>,
    #[asn(optional_idx = 2)]
    pub i_e_extensions: Option<ExpectedUEBehaviouriE_Extensions>,
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
    tooEarlyInterRATHOReportFromEUTRAN(TooEarlyInterRATHOReportReportFromEUTRAN),
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
    pub p_lm_nidentity: PLMNidentity,
    pub five_gstac: FiveGSTAC,
    #[asn(optional_idx = 0)]
    pub i_e_extensions: Option<FiveGSTAIiE_Extensions>,
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
    pub p_lmn_identity: PLMNidentity,
    pub forbidden_la_cs: ForbiddenLACs,
    #[asn(optional_idx = 0)]
    pub i_e_extensions: Option<ForbiddenLAs_ItemiE_Extensions>,
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
    pub p_lmn_identity: PLMNidentity,
    pub forbidden_ta_cs: ForbiddenTACs,
    #[asn(optional_idx = 0)]
    pub i_e_extensions: Option<ForbiddenTAs_ItemiE_Extensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct GBR_QosInformation {
    pub e_rab_maximum_bitrate_dl: BitRate,
    pub e_rab_maximum_bitrate_ul: BitRate,
    pub e_rab_guaranteed_bitrate_dl: BitRate,
    pub e_rab_guaranteed_bitrate_ul: BitRate,
    #[asn(optional_idx = 0)]
    pub i_e_extensions: Option<GBR_QosInformationiE_Extensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct GERAN_Cell_ID {
    pub l_ai: LAI,
    pub r_ac: RAC,
    pub c_i: CI,
    #[asn(optional_idx = 0)]
    pub i_e_extensions: Option<GERAN_Cell_IDiE_Extensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct GNB {
    pub global_g_nb_id: Global_GNB_ID,
    #[asn(optional_idx = 0)]
    pub i_e_extensions: Option<GNBiE_Extensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "BITSTRING", sz_extensible = false, sz_lb = "22", sz_ub = "32")]
pub struct GNB_ID(BitVec<Msb0, u8>);

#[derive(Debug, AperCodec)]
#[asn(type = "CHOICE", lb = "0", ub = "0", extensible = true)]
pub enum GNB_Identity {
    #[asn(key = 0, extended = false)]
    gNB_ID(GNB_ID),
}

#[derive(Debug, AperCodec)]
#[asn(type = "OCTET-STRING", sz_extensible = false, sz_lb = "4", sz_ub = "4")]
pub struct GTP_TEID(Vec<u8>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct GUMMEI {
    pub p_lmn_identity: PLMNidentity,
    pub m_me_group_id: MME_Group_ID,
    pub m_me_code: MME_Code,
    #[asn(optional_idx = 0)]
    pub i_e_extensions: Option<GUMMEIiE_Extensions>,
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
    pub p_lm_nidentity: PLMNidentity,
    pub e_nb_id: ENB_ID,
    #[asn(optional_idx = 0)]
    pub i_e_extensions: Option<Global_ENB_IDiE_Extensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct Global_GNB_ID {
    pub p_lmn_identity: PLMNidentity,
    pub g_nb_id: GNB_Identity,
    #[asn(optional_idx = 0)]
    pub i_e_extensions: Option<Global_GNB_IDiE_Extensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "CHOICE", lb = "0", ub = "1", extensible = true)]
pub enum Global_RAN_NODE_ID {
    #[asn(key = 0, extended = false)]
    gNB(GNB),
    #[asn(key = 1, extended = false)]
    ng_eNB(NG_eNB),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct Global_en_gNB_ID {
    pub p_lm_nidentity: PLMNidentity,
    pub en_g_nb_id: En_gNB_ID,
    #[asn(optional_idx = 0)]
    pub i_e_extensions: Option<Global_en_gNB_IDiE_Extensions>,
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
    pub protocol_i_es: HandoverCancelprotocolIEs,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true)]
pub struct HandoverCancelAcknowledge {
    pub protocol_i_es: HandoverCancelAcknowledgeprotocolIEs,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true)]
pub struct HandoverCommand {
    pub protocol_i_es: HandoverCommandprotocolIEs,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true)]
pub struct HandoverFailure {
    pub protocol_i_es: HandoverFailureprotocolIEs,
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
    pub protocol_i_es: HandoverNotifyprotocolIEs,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true)]
pub struct HandoverPreparationFailure {
    pub protocol_i_es: HandoverPreparationFailureprotocolIEs,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true)]
pub struct HandoverRequest {
    pub protocol_i_es: HandoverRequestprotocolIEs,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true)]
pub struct HandoverRequestAcknowledge {
    pub protocol_i_es: HandoverRequestAcknowledgeprotocolIEs,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true)]
pub struct HandoverRequired {
    pub protocol_i_es: HandoverRequiredprotocolIEs,
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
    pub i_e_extensions: Option<HandoverRestrictionListiE_Extensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true)]
pub struct HandoverSuccess {
    pub protocol_i_es: HandoverSuccessprotocolIEs,
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
    eUTRAN(OCTET_STRING_22),
    #[asn(key = 1, extended = false)]
    uTRAN(OCTET_STRING_23),
    #[asn(key = 2, extended = false)]
    gERAN(OCTET_STRING_24),
    #[asn(key = 0, extended = true)]
    eHRPD(EHRPD_Sector_ID),
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
    pub i_e_extensions: Option<ImmediateMDTiE_Extensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct InformationOnRecommendedCellsAndENBsForPaging {
    pub recommended_cells_for_paging: RecommendedCellsForPaging,
    pub recommend_en_bs_for_paging: RecommendedENBsForPaging,
    #[asn(optional_idx = 0)]
    pub i_e_extensions: Option<InformationOnRecommendedCellsAndENBsForPagingiE_Extensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true)]
pub struct InitialContextSetupFailure {
    pub protocol_i_es: InitialContextSetupFailureprotocolIEs,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true)]
pub struct InitialContextSetupRequest {
    pub protocol_i_es: InitialContextSetupRequestprotocolIEs,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true)]
pub struct InitialContextSetupResponse {
    pub protocol_i_es: InitialContextSetupResponseprotocolIEs,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true)]
pub struct InitialUEMessage {
    pub protocol_i_es: InitialUEMessageprotocolIEs,
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
#[asn(type = "BITSTRING", sz_extensible = true, sz_lb = "16", sz_ub = "16")]
pub struct IntegrityProtectionAlgorithms(BitVec<Msb0, u8>);

#[derive(Debug, AperCodec)]
#[asn(type = "INTEGER", lb = "1", ub = "16", extensible = true)]
pub struct IntendedNumberOfPagingAttempts(u8);

#[derive(Debug, AperCodec)]
#[asn(type = "CHOICE", lb = "0", ub = "0", extensible = true)]
pub enum Inter_SystemInformationTransferType {
    #[asn(key = 0, extended = false)]
    rIMTransfer(RIMTransfer),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false, optional_fields = 8)]
pub struct InterSystemMeasurementItem {
    pub freq_band_indicator_nr: INTEGER_25,
    pub s_s_bfrequencies: INTEGER_26,
    pub subcarrier_spacing_ssb: ENUMERATED_27,
    #[asn(optional_idx = 0)]
    pub max_rs_index_cell_qual: Option<INTEGER_28>,
    #[asn(optional_idx = 1)]
    pub s_mtc: Option<OCTET_STRING_29>,
    #[asn(optional_idx = 2)]
    pub thresh_rs_index_r15: Option<OCTET_STRING_30>,
    #[asn(optional_idx = 3)]
    pub s_sb_to_measure: Option<OCTET_STRING_31>,
    #[asn(optional_idx = 4)]
    pub s_srssi_measurement: Option<OCTET_STRING_32>,
    #[asn(optional_idx = 5)]
    pub quantity_config_nr_r15: Option<OCTET_STRING_33>,
    #[asn(optional_idx = 6)]
    pub black_cells_to_add_mod_list: Option<OCTET_STRING_34>,
    #[asn(optional_idx = 7)]
    pub i_e_extensions: Option<InterSystemMeasurementItemiE_Extensions>,
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
    pub i_e_extensions: Option<InterSystemMeasurementParametersiE_Extensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "BITSTRING", sz_extensible = false, sz_lb = "8", sz_ub = "8")]
pub struct InterfacesToTrace(BitVec<Msb0, u8>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 4)]
pub struct IntersystemMeasurementConfiguration {
    #[asn(optional_idx = 0)]
    pub r_srp: Option<INTEGER_36>,
    #[asn(optional_idx = 1)]
    pub r_srq: Option<INTEGER_37>,
    #[asn(optional_idx = 2)]
    pub s_inr: Option<INTEGER_38>,
    pub inter_system_measurement_parameters: InterSystemMeasurementParameters,
    #[asn(optional_idx = 3)]
    pub i_e_extensions: Option<IntersystemMeasurementConfigurationiE_Extensions>,
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
    pub protocol_i_es: KillRequestprotocolIEs,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true)]
pub struct KillResponse {
    pub protocol_i_es: KillResponseprotocolIEs,
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
    pub p_lm_nidentity: PLMNidentity,
    pub l_ac: LAC,
    #[asn(optional_idx = 0)]
    pub i_e_extensions: Option<LAIiE_Extensions>,
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
    e_UTRAN_Cell(LastVisitedEUTRANCellInformation),
    #[asn(key = 1, extended = false)]
    uTRAN_Cell(LastVisitedUTRANCellInformation),
    #[asn(key = 2, extended = false)]
    gERAN_Cell(LastVisitedGERANCellInformation),
    #[asn(key = 0, extended = true)]
    nG_RAN_Cell(LastVisitedNGRANCellInformation),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct LastVisitedEUTRANCellInformation {
    pub global_cell_id: EUTRAN_CGI,
    pub cell_type: CellType,
    pub time_ue_stayed_in_cell: Time_UE_StayedInCell,
    #[asn(optional_idx = 0)]
    pub i_e_extensions: Option<LastVisitedEUTRANCellInformationiE_Extensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "CHOICE", lb = "0", ub = "0", extensible = true)]
pub enum LastVisitedGERANCellInformation {
    #[asn(key = 0, extended = false)]
    undefined(NULL_39),
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
    pub i_e_extensions: Option<ListeningSubframePatterniE_Extensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true)]
pub struct LocationReport {
    pub protocol_i_es: LocationReportprotocolIEs,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true)]
pub struct LocationReportingControl {
    pub protocol_i_es: LocationReportingControlprotocolIEs,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true)]
pub struct LocationReportingFailureIndication {
    pub protocol_i_es: LocationReportingFailureIndicationprotocolIEs,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 2)]
pub struct LoggedMBSFNMDT {
    pub logging_interval: LoggingInterval,
    pub logging_duration: LoggingDuration,
    #[asn(optional_idx = 0)]
    pub m_bsfn_result_to_log: Option<MBSFN_ResultToLog>,
    #[asn(optional_idx = 1)]
    pub i_e_extensions: Option<LoggedMBSFNMDTiE_Extensions>,
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
    pub i_e_extensions: Option<M1PeriodicReportingiE_Extensions>,
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
    pub i_e_extensions: Option<M1ThresholdEventA2iE_Extensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct M3Configuration {
    pub m3period: M3period,
    #[asn(optional_idx = 0)]
    pub i_e_extensions: Option<M3ConfigurationiE_Extensions>,
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
    pub i_e_extensions: Option<M4ConfigurationiE_Extensions>,
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
    pub i_e_extensions: Option<M5ConfigurationiE_Extensions>,
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
    pub i_e_extensions: Option<M6ConfigurationiE_Extensions>,
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
    pub i_e_extensions: Option<M7ConfigurationiE_Extensions>,
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
    pub m_bsfn_area_id: Option<INTEGER_42>,
    pub carrier_freq: EARFCN,
    #[asn(optional_idx = 1)]
    pub i_e_extensions: Option<MBSFN_ResultToLogInfoiE_Extensions>,
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
    pub m_dt_mode: MDTMode,
    #[asn(optional_idx = 0)]
    pub i_e_extensions: Option<MDT_ConfigurationiE_Extensions>,
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
    immediateMDT(ImmediateMDT),
    #[asn(key = 1, extended = false)]
    loggedMDT(LoggedMDT),
    #[asn(key = 0, extended = true)]
    mDTMode_Extension(MDTMode_Extension),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct MDTMode_Extension {
    #[asn(key_field = true)]
    pub id: ProtocolIE_ID,
    pub criticality: Criticality,
    pub value: MDTMode_Extensionvalue,
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
    pub protocol_i_es: MMECPRelocationIndicationprotocolIEs,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true)]
pub struct MMEConfigurationTransfer {
    pub protocol_i_es: MMEConfigurationTransferprotocolIEs,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true)]
pub struct MMEConfigurationUpdate {
    pub protocol_i_es: MMEConfigurationUpdateprotocolIEs,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true)]
pub struct MMEConfigurationUpdateAcknowledge {
    pub protocol_i_es: MMEConfigurationUpdateAcknowledgeprotocolIEs,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true)]
pub struct MMEConfigurationUpdateFailure {
    pub protocol_i_es: MMEConfigurationUpdateFailureprotocolIEs,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true)]
pub struct MMEDirectInformationTransfer {
    pub protocol_i_es: MMEDirectInformationTransferprotocolIEs,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true)]
pub struct MMEEarlyStatusTransfer {
    pub protocol_i_es: MMEEarlyStatusTransferprotocolIEs,
}

#[derive(Debug, AperCodec)]
#[asn(type = "CHOICE", lb = "0", ub = "1", extensible = true)]
pub enum MMEPagingTarget {
    #[asn(key = 0, extended = false)]
    global_ENB_ID(Global_ENB_ID),
    #[asn(key = 1, extended = false)]
    tAI(TAI),
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
    pub protocol_i_es: MMEStatusTransferprotocolIEs,
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
    threshold_RSRP(Threshold_RSRP),
    #[asn(key = 1, extended = false)]
    threshold_RSRQ(Threshold_RSRQ),
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
    eUTRANResponse(EUTRANResponse),
    #[asn(key = 1, extended = false)]
    uTRANResponse(OCTET_STRING_43),
    #[asn(key = 2, extended = false)]
    gERANResponse(OCTET_STRING_44),
    #[asn(key = 0, extended = true)]
    eHRPD(EHRPDMultiSectorLoadReportingResponseItem),
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
    pub i_e_extensions: Option<MutingPatternInformationiE_Extensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "OCTET-STRING")]
pub struct NAS_PDU(Vec<u8>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true)]
pub struct NASDeliveryIndication {
    pub protocol_i_es: NASDeliveryIndicationprotocolIEs,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true)]
pub struct NASNonDeliveryIndication {
    pub protocol_i_es: NASNonDeliveryIndicationprotocolIEs,
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
    pub n_b_io_t_paging_e_drx_cycle: NB_IoT_Paging_eDRX_Cycle,
    #[asn(optional_idx = 0)]
    pub n_b_io_t_paging_time_window: Option<NB_IoT_PagingTimeWindow>,
    #[asn(optional_idx = 1)]
    pub i_e_extensions: Option<NB_IoT_Paging_eDRXInformationiE_Extensions>,
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
    pub i_e_extensions: Option<NG_eNBiE_Extensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct NR_CGI {
    pub p_lmn_identity: PLMNidentity,
    pub n_r_cell_identity: NRCellIdentity,
    #[asn(optional_idx = 0)]
    pub i_e_extensions: Option<NR_CGIiE_Extensions>,
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
    pub i_e_extensions: Option<NRUESecurityCapabilitiesiE_Extensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct NRUESidelinkAggregateMaximumBitrate {
    pub u_eaggregate_maximum_bit_rate: BitRate,
    #[asn(optional_idx = 0)]
    pub i_e_extensions: Option<NRUESidelinkAggregateMaximumBitrateiE_Extensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 3)]
pub struct NRV2XServicesAuthorized {
    #[asn(optional_idx = 0)]
    pub vehicle_ue: Option<VehicleUE>,
    #[asn(optional_idx = 1)]
    pub pedestrian_ue: Option<PedestrianUE>,
    #[asn(optional_idx = 2)]
    pub i_e_extensions: Option<NRV2XServicesAuthorizediE_Extensions>,
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
    overloadAction(OverloadAction),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true)]
pub struct OverloadStart {
    pub protocol_i_es: OverloadStartprotocolIEs,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true)]
pub struct OverloadStop {
    pub protocol_i_es: OverloadStopprotocolIEs,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct PC5FlowBitRates {
    pub guaranteed_flow_bit_rate: BitRate,
    pub maximum_flow_bit_rate: BitRate,
    #[asn(optional_idx = 0)]
    pub i_e_extensions: Option<PC5FlowBitRatesiE_Extensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 3)]
pub struct PC5QoSFlowItem {
    pub p_qi: FiveQI,
    #[asn(optional_idx = 0)]
    pub pc5_flow_bit_rates: Option<PC5FlowBitRates>,
    #[asn(optional_idx = 1)]
    pub range: Option<Range>,
    #[asn(optional_idx = 2)]
    pub i_e_extensions: Option<PC5QoSFlowItemiE_Extensions>,
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
    pub i_e_extensions: Option<PC5QoSParametersiE_Extensions>,
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
    pub i_e_extensions: Option<PLMNAreaBasedQMCiE_Extensions>,
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
    pub n_cgi: NR_CGI,
    #[asn(optional_idx = 0)]
    pub i_e_extensions: Option<PSCellInformationiE_Extensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true)]
pub struct PWSFailureIndication {
    pub protocol_i_es: PWSFailureIndicationprotocolIEs,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true)]
pub struct PWSRestartIndication {
    pub protocol_i_es: PWSRestartIndicationprotocolIEs,
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
    pub protocol_i_es: PagingprotocolIEs,
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
    pub i_e_extensions: Option<Paging_eDRXInformationiE_Extensions>,
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
    pub i_e_extensions: Option<PagingAttemptInformationiE_Extensions>,
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
    pub protocol_i_es: PathSwitchRequestprotocolIEs,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true)]
pub struct PathSwitchRequestAcknowledge {
    pub protocol_i_es: PathSwitchRequestAcknowledgeprotocolIEs,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true)]
pub struct PathSwitchRequestFailure {
    pub protocol_i_es: PathSwitchRequestFailureprotocolIEs,
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
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 3)]
pub struct ProSeAuthorized {
    #[asn(optional_idx = 0)]
    pub pro_se_direct_discovery: Option<ProSeDirectDiscovery>,
    #[asn(optional_idx = 1)]
    pub pro_se_direct_communication: Option<ProSeDirectCommunication>,
    #[asn(optional_idx = 2)]
    pub i_e_extensions: Option<ProSeAuthorizediE_Extensions>,
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
    gERAN_Cell_ID(GERAN_Cell_ID),
    #[asn(key = 0, extended = true)]
    targetRNC_ID(TargetRNC_ID),
    #[asn(key = 1, extended = true)]
    eHRPD_Sector_ID(OCTET_STRING_50),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 2)]
pub struct RIMTransfer {
    pub r_im_information: RIMInformation,
    #[asn(optional_idx = 0)]
    pub r_im_routing_address: Option<RIMRoutingAddress>,
    #[asn(optional_idx = 1)]
    pub i_e_extensions: Option<RIMTransferiE_Extensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 2)]
pub struct RLFReportInformation {
    pub u_e_rlf_report_container: UE_RLF_Report_Container,
    #[asn(optional_idx = 0)]
    pub u_e_rlf_report_container_for_extended_bands:
        Option<UE_RLF_Report_Container_for_extended_bands>,
    #[asn(optional_idx = 1)]
    pub i_e_extensions: Option<RLFReportInformationiE_Extensions>,
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
    pub e_utran_cgi: EUTRAN_CGI,
    #[asn(optional_idx = 0)]
    pub time_stayed_in_cell: Option<INTEGER_51>,
    #[asn(optional_idx = 1)]
    pub i_e_extensions: Option<RecommendedCellItemiE_Extensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "16")]
pub struct RecommendedCellList(Vec<RecommendedCellList_Item>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct RecommendedCellsForPaging {
    pub recommended_cell_list: RecommendedCellList,
    #[asn(optional_idx = 0)]
    pub i_e_extensions: Option<RecommendedCellsForPagingiE_Extensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct RecommendedENBItem {
    pub m_me_paging_target: MMEPagingTarget,
    #[asn(optional_idx = 0)]
    pub i_e_extensions: Option<RecommendedENBItemiE_Extensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "16")]
pub struct RecommendedENBList(Vec<RecommendedENBList_Item>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct RecommendedENBsForPaging {
    pub recommended_enb_list: RecommendedENBList,
    #[asn(optional_idx = 0)]
    pub i_e_extensions: Option<RecommendedENBsForPagingiE_Extensions>,
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
    pub i_e_extensions: Option<RequestTypeiE_Extensions>,
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
    pub protocol_i_es: RerouteNASRequestprotocolIEs,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true)]
pub struct Reset {
    pub protocol_i_es: ResetprotocolIEs,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true)]
pub struct ResetAcknowledge {
    pub protocol_i_es: ResetAcknowledgeprotocolIEs,
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
    s1_Interface(ResetAll),
    #[asn(key = 1, extended = false)]
    partOfS1_Interface(UE_associatedLogicalS1_ConnectionListRes),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true)]
pub struct RetrieveUEInformation {
    pub protocol_i_es: RetrieveUEInformationprotocolIEs,
}

#[derive(Debug, AperCodec)]
#[asn(type = "INTEGER", lb = "0", ub = "255")]
pub struct Routing_ID(u8);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct S_TMSI {
    pub m_mec: MME_Code,
    pub m_tmsi: M_TMSI,
    #[asn(optional_idx = 0)]
    pub i_e_extensions: Option<S_TMSIiE_Extensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "CHOICE", lb = "0", ub = "2", extensible = true)]
pub enum S1AP_PDU {
    #[asn(key = 0, extended = false)]
    initiatingMessage(InitiatingMessage),
    #[asn(key = 1, extended = false)]
    successfulOutcome(SuccessfulOutcome),
    #[asn(key = 2, extended = false)]
    unsuccessfulOutcome(UnsuccessfulOutcome),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true)]
pub struct S1SetupFailure {
    pub protocol_i_es: S1SetupFailureprotocolIEs,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true)]
pub struct S1SetupRequest {
    pub protocol_i_es: S1SetupRequestprotocolIEs,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true)]
pub struct S1SetupResponse {
    pub protocol_i_es: S1SetupResponseprotocolIEs,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct SONConfigurationTransfer {
    pub targete_nb_id: TargeteNB_ID,
    pub sourcee_nb_id: SourceeNB_ID,
    pub s_on_information: SONInformation,
    #[asn(optional_idx = 0)]
    pub i_e_extensions: Option<SONConfigurationTransferiE_Extensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "CHOICE", lb = "0", ub = "1", extensible = true)]
pub enum SONInformation {
    #[asn(key = 0, extended = false)]
    sONInformationRequest(SONInformationRequest),
    #[asn(key = 1, extended = false)]
    sONInformationReply(SONInformationReply),
    #[asn(key = 0, extended = true)]
    sONInformation_Extension(SONInformation_Extension),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct SONInformation_Extension {
    #[asn(key_field = true)]
    pub id: ProtocolIE_ID,
    pub criticality: Criticality,
    pub value: SONInformation_Extensionvalue,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 2)]
pub struct SONInformationReply {
    #[asn(optional_idx = 0)]
    pub x2_tnl_configuration_info: Option<X2TNLConfigurationInfo>,
    #[asn(optional_idx = 1)]
    pub i_e_extensions: Option<SONInformationReplyiE_Extensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "CHOICE", lb = "0", ub = "0", extensible = true)]
pub enum SONInformationReport {
    #[asn(key = 0, extended = false)]
    rLFReportInformation(RLFReportInformation),
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
    cellLoadReporting(CellLoadReportingCause),
    #[asn(key = 0, extended = true)]
    multiCellLoadReporting(CellLoadReportingCause),
    #[asn(key = 1, extended = true)]
    eventTriggeredCellLoadReporting(CellLoadReportingCause),
    #[asn(key = 2, extended = true)]
    hOReporting(HOReportingCause),
    #[asn(key = 3, extended = true)]
    eutranCellActivation(CellActivationCause),
    #[asn(key = 4, extended = true)]
    energySavingsIndication(CellStateIndicationCause),
    #[asn(key = 5, extended = true)]
    failureEventReporting(FailureEventReportingCause),
}

#[derive(Debug, AperCodec)]
#[asn(type = "CHOICE", lb = "0", ub = "0", extensible = true)]
pub enum SONtransferRequestContainer {
    #[asn(key = 0, extended = false)]
    cellLoadReporting(NULL_52),
    #[asn(key = 0, extended = true)]
    multiCellLoadReporting(MultiCellLoadReportingRequest),
    #[asn(key = 1, extended = true)]
    eventTriggeredCellLoadReporting(EventTriggeredCellLoadReportingRequest),
    #[asn(key = 2, extended = true)]
    hOReporting(HOReport),
    #[asn(key = 3, extended = true)]
    eutranCellActivation(CellActivationRequest),
    #[asn(key = 4, extended = true)]
    energySavingsIndication(CellStateIndication),
    #[asn(key = 5, extended = true)]
    failureEventReporting(FailureEventReport),
}

#[derive(Debug, AperCodec)]
#[asn(type = "CHOICE", lb = "0", ub = "0", extensible = true)]
pub enum SONtransferResponseContainer {
    #[asn(key = 0, extended = false)]
    cellLoadReporting(CellLoadReportingResponse),
    #[asn(key = 0, extended = true)]
    multiCellLoadReporting(MultiCellLoadReportingResponse),
    #[asn(key = 1, extended = true)]
    eventTriggeredCellLoadReporting(EventTriggeredCellLoadReportingResponse),
    #[asn(key = 2, extended = true)]
    hOReporting(NULL_53),
    #[asn(key = 3, extended = true)]
    eutranCellActivation(CellActivationResponse),
    #[asn(key = 4, extended = true)]
    energySavingsIndication(NULL_54),
    #[asn(key = 5, extended = true)]
    failureEventReporting(NULL_55),
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
    pub i_e_extensions: Option<ScheduledCommunicationTimeiE_Extensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true)]
pub struct SecondaryRATDataUsageReport {
    pub protocol_i_es: SecondaryRATDataUsageReportprotocolIEs,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct SecondaryRATDataUsageReportItem {
    pub e_rab_id: E_RAB_ID,
    pub secondary_rat_type: SecondaryRATType,
    pub e_rab_usage_report_list: E_RABUsageReportList,
    #[asn(optional_idx = 0)]
    pub i_e_extensions: Option<SecondaryRATDataUsageReportItemiE_Extensions>,
}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "256"
)]
pub struct SecondaryRATDataUsageReportList(Vec<SecondaryRATDataUsageReportList_Item>);

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
    pub i_e_extensions: Option<SecurityContextiE_Extensions>,
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
    pub d_cn_id: DCN_ID,
    pub relative_dcn_capacity: RelativeMMECapacity,
    #[asn(optional_idx = 0)]
    pub i_e_extensions: Option<ServedDCNsItemiE_Extensions>,
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
    pub i_e_extensions: Option<ServedGUMMEIsItemiE_Extensions>,
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
    pub i_e_extensions: Option<SourceNgRanNode_IDiE_Extensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "OCTET-STRING")]
pub struct SourceNgRanNode_ToTargetNgRanNode_TransparentContainer(Vec<u8>);

#[derive(Debug, AperCodec)]
#[asn(type = "CHOICE", lb = "0", ub = "1", extensible = false)]
pub enum SourceNodeID {
    #[asn(key = 0, extended = false)]
    sourceNgRanNode_ID(SourceNgRanNode_ID),
    #[asn(key = 1, extended = false)]
    sourceNodeID_Extension(SourceNodeID_Extension),
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
    pub i_e_extensions: Option<SourceeNB_IDiE_Extensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 3)]
pub struct SourceeNB_ToTargeteNB_TransparentContainer {
    pub r_rc_container: RRC_Container,
    #[asn(optional_idx = 0)]
    pub e_rab_information_list: Option<E_RABInformationList>,
    pub target_cell_id: EUTRAN_CGI,
    #[asn(optional_idx = 1)]
    pub subscriber_profile_i_dfor_rfp: Option<SubscriberProfileIDforRFP>,
    pub u_e_history_information: UE_HistoryInformation,
    #[asn(optional_idx = 2)]
    pub i_e_extensions: Option<SourceeNB_ToTargeteNB_TransparentContaineriE_Extensions>,
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
    pub i_e_extensions: Option<Subscription_Based_UE_DifferentiationInfoiE_Extensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct SuccessfulOutcome {
    #[asn(key_field = true)]
    pub procedure_code: ProcedureCode,
    pub criticality: Criticality,
    pub value: SuccessfulOutcomevalue,
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
    pub t_ac: TAC,
    pub broadcast_plm_ns: BPLMNs,
    #[asn(optional_idx = 0)]
    pub i_e_extensions: Option<SupportedTAs_ItemiE_Extensions>,
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
    pub i_e_extensions: Option<SynchronisationInformationiE_Extensions>,
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
    pub t_a_listfor_mdt: TAListforMDT,
    #[asn(optional_idx = 0)]
    pub i_e_extensions: Option<TABasedMDTiE_Extensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct TABasedQMC {
    pub t_a_listfor_qmc: TAListforQMC,
    #[asn(optional_idx = 0)]
    pub i_e_extensions: Option<TABasedQMCiE_Extensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "OCTET-STRING", sz_extensible = false, sz_lb = "2", sz_ub = "2")]
pub struct TAC(Vec<u8>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct TAI {
    pub p_lm_nidentity: PLMNidentity,
    pub t_ac: TAC,
    #[asn(optional_idx = 0)]
    pub i_e_extensions: Option<TAIiE_Extensions>,
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
    pub t_ai: TAI,
    pub completed_cellin_tai: CompletedCellinTAI,
    #[asn(optional_idx = 0)]
    pub i_e_extensions: Option<TAI_Broadcast_ItemiE_Extensions>,
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
    pub t_ai: TAI,
    pub cancelled_cellin_tai: CancelledCellinTAI,
    #[asn(optional_idx = 0)]
    pub i_e_extensions: Option<TAI_Cancelled_ItemiE_Extensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct TAIBasedMDT {
    pub t_ai_listfor_mdt: TAIListforMDT,
    #[asn(optional_idx = 0)]
    pub i_e_extensions: Option<TAIBasedMDTiE_Extensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct TAIBasedQMC {
    pub t_ai_listfor_qmc: TAIListforQMC,
    #[asn(optional_idx = 0)]
    pub i_e_extensions: Option<TAIBasedQMCiE_Extensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct TAIItem {
    pub t_ai: TAI,
    #[asn(optional_idx = 0)]
    pub i_e_extensions: Option<TAIItemiE_Extensions>,
}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "256"
)]
pub struct TAIList(Vec<TAIList_Item>);

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
    targeteNB_ID(TargeteNB_ID),
    #[asn(key = 1, extended = false)]
    targetRNC_ID(TargetRNC_ID),
    #[asn(key = 2, extended = false)]
    cGI(CGI),
    #[asn(key = 0, extended = true)]
    targetgNgRanNode_ID(TargetNgRanNode_ID),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct TargetNgRanNode_ID {
    pub global_ran_node_id: Global_RAN_NODE_ID,
    pub selected_tai: FiveGSTAI,
    #[asn(optional_idx = 0)]
    pub i_e_extensions: Option<TargetNgRanNode_IDiE_Extensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "OCTET-STRING")]
pub struct TargetNgRanNode_ToSourceNgRanNode_TransparentContainer(Vec<u8>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 3)]
pub struct TargetRNC_ID {
    pub l_ai: LAI,
    #[asn(optional_idx = 0)]
    pub r_ac: Option<RAC>,
    pub r_nc_id: RNC_ID,
    #[asn(optional_idx = 1)]
    pub extended_rnc_id: Option<ExtendedRNC_ID>,
    #[asn(optional_idx = 2)]
    pub i_e_extensions: Option<TargetRNC_IDiE_Extensions>,
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
    pub i_e_extensions: Option<TargeteNB_IDiE_Extensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct TargeteNB_ToSourceeNB_TransparentContainer {
    pub r_rc_container: RRC_Container,
    #[asn(optional_idx = 0)]
    pub i_e_extensions: Option<TargeteNB_ToSourceeNB_TransparentContaineriE_Extensions>,
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
    pub i_e_extensions: Option<TimeSynchronisationInfoiE_Extensions>,
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
    pub u_erlf_report_container: OCTET_STRING_65,
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
    pub i_e_extensions: Option<TraceActivationiE_Extensions>,
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
    pub protocol_i_es: TraceFailureIndicationprotocolIEs,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true)]
pub struct TraceStart {
    pub protocol_i_es: TraceStartprotocolIEs,
}

#[derive(Debug, AperCodec)]
#[asn(type = "INTEGER", lb = "1", ub = "99")]
pub struct TrafficLoadReductionIndication(u8);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true)]
pub struct TransportInformation {
    pub transport_layer_address: TransportLayerAddress,
    pub u_l_gtp_teid: GTP_TEID,
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
    pub m_me_ue_s1ap_id: MME_UE_S1AP_ID,
    pub e_nb_ue_s1ap_id: ENB_UE_S1AP_ID,
    #[asn(optional_idx = 0)]
    pub i_e_extensions: Option<UE_S1AP_ID_pairiE_Extensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "CHOICE", lb = "0", ub = "1", extensible = true)]
pub enum UE_S1AP_IDs {
    #[asn(key = 0, extended = false)]
    uE_S1AP_ID_pair(UE_S1AP_ID_pair),
    #[asn(key = 1, extended = false)]
    mME_UE_S1AP_ID(MME_UE_S1AP_ID),
}

#[derive(Debug, AperCodec)]
#[asn(type = "INTEGER", lb = "0", ub = "255")]
pub struct UE_Usage_Type(u8);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 3)]
pub struct UE_associatedLogicalS1_ConnectionItem {
    #[asn(optional_idx = 0)]
    pub m_me_ue_s1ap_id: Option<MME_UE_S1AP_ID>,
    #[asn(optional_idx = 1)]
    pub e_nb_ue_s1ap_id: Option<ENB_UE_S1AP_ID>,
    #[asn(optional_idx = 2)]
    pub i_e_extensions: Option<UE_associatedLogicalS1_ConnectionItemiE_Extensions>,
}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "256"
)]
pub struct UE_associatedLogicalS1_ConnectionListRes(
    Vec<UE_associatedLogicalS1_ConnectionListRes_Item>,
);

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "256"
)]
pub struct UE_associatedLogicalS1_ConnectionListResAck(
    Vec<UE_associatedLogicalS1_ConnectionListResAck_Item>,
);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct UEAggregateMaximumBitrate {
    pub u_eaggregate_maximum_bit_rate_dl: BitRate,
    pub u_eaggregate_maximum_bit_rate_ul: BitRate,
    #[asn(optional_idx = 0)]
    pub i_e_extensions: Option<UEAggregateMaximumBitrateiE_Extensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct UEAppLayerMeasConfig {
    pub container_for_app_layer_meas_config: OCTET_STRING_66,
    pub area_scope_of_qmc: AreaScopeOfQMC,
    #[asn(optional_idx = 0)]
    pub i_e_extensions: Option<UEAppLayerMeasConfigiE_Extensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true)]
pub struct UECapabilityInfoIndication {
    pub protocol_i_es: UECapabilityInfoIndicationprotocolIEs,
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
    pub protocol_i_es: UEContextModificationConfirmprotocolIEs,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true)]
pub struct UEContextModificationFailure {
    pub protocol_i_es: UEContextModificationFailureprotocolIEs,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true)]
pub struct UEContextModificationIndication {
    pub protocol_i_es: UEContextModificationIndicationprotocolIEs,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true)]
pub struct UEContextModificationRequest {
    pub protocol_i_es: UEContextModificationRequestprotocolIEs,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true)]
pub struct UEContextModificationResponse {
    pub protocol_i_es: UEContextModificationResponseprotocolIEs,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true)]
pub struct UEContextReleaseCommand {
    pub protocol_i_es: UEContextReleaseCommandprotocolIEs,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true)]
pub struct UEContextReleaseComplete {
    pub protocol_i_es: UEContextReleaseCompleteprotocolIEs,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true)]
pub struct UEContextReleaseRequest {
    pub protocol_i_es: UEContextReleaseRequestprotocolIEs,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true)]
pub struct UEContextResumeFailure {
    pub protocol_i_es: UEContextResumeFailureprotocolIEs,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true)]
pub struct UEContextResumeRequest {
    pub protocol_i_es: UEContextResumeRequestprotocolIEs,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true)]
pub struct UEContextResumeResponse {
    pub protocol_i_es: UEContextResumeResponseprotocolIEs,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true)]
pub struct UEContextSuspendRequest {
    pub protocol_i_es: UEContextSuspendRequestprotocolIEs,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true)]
pub struct UEContextSuspendResponse {
    pub protocol_i_es: UEContextSuspendResponseprotocolIEs,
}

#[derive(Debug, AperCodec)]
#[asn(type = "BITSTRING", sz_extensible = false, sz_lb = "10", sz_ub = "10")]
pub struct UEIdentityIndexValue(BitVec<Msb0, u8>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true)]
pub struct UEInformationTransfer {
    pub protocol_i_es: UEInformationTransferprotocolIEs,
}

#[derive(Debug, AperCodec)]
#[asn(type = "CHOICE", lb = "0", ub = "1", extensible = true)]
pub enum UEPagingID {
    #[asn(key = 0, extended = false)]
    s_TMSI(S_TMSI),
    #[asn(key = 1, extended = false)]
    iMSI(IMSI),
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
    pub protocol_i_es: UERadioCapabilityIDMappingRequestprotocolIEs,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true)]
pub struct UERadioCapabilityIDMappingResponse {
    pub protocol_i_es: UERadioCapabilityIDMappingResponseprotocolIEs,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true)]
pub struct UERadioCapabilityMatchRequest {
    pub protocol_i_es: UERadioCapabilityMatchRequestprotocolIEs,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true)]
pub struct UERadioCapabilityMatchResponse {
    pub protocol_i_es: UERadioCapabilityMatchResponseprotocolIEs,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct UESecurityCapabilities {
    pub encryption_algorithms: EncryptionAlgorithms,
    pub integrity_protection_algorithms: IntegrityProtectionAlgorithms,
    #[asn(optional_idx = 0)]
    pub i_e_extensions: Option<UESecurityCapabilitiesiE_Extensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct UESidelinkAggregateMaximumBitrate {
    pub u_e_sidelink_aggregate_maximum_bit_rate: BitRate,
    #[asn(optional_idx = 0)]
    pub i_e_extensions: Option<UESidelinkAggregateMaximumBitrateiE_Extensions>,
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
    pub i_e_extensions: Option<UL_CP_SecurityInformationiE_Extensions>,
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
    pub value: UnsuccessfulOutcomevalue,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true)]
pub struct UplinkNASTransport {
    pub protocol_i_es: UplinkNASTransportprotocolIEs,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true)]
pub struct UplinkNonUEAssociatedLPPaTransport {
    pub protocol_i_es: UplinkNonUEAssociatedLPPaTransportprotocolIEs,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true)]
pub struct UplinkS1cdma2000tunnelling {
    pub protocol_i_es: UplinkS1cdma2000tunnellingprotocolIEs,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true)]
pub struct UplinkUEAssociatedLPPaTransport {
    pub protocol_i_es: UplinkUEAssociatedLPPaTransportprotocolIEs,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct UserLocationInformation {
    pub eutran_cgi: EUTRAN_CGI,
    pub tai: TAI,
    #[asn(optional_idx = 0)]
    pub i_e_extensions: Option<UserLocationInformationiE_Extensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 3)]
pub struct V2XServicesAuthorized {
    #[asn(optional_idx = 0)]
    pub vehicle_ue: Option<VehicleUE>,
    #[asn(optional_idx = 1)]
    pub pedestrian_ue: Option<PedestrianUE>,
    #[asn(optional_idx = 2)]
    pub i_e_extensions: Option<V2XServicesAuthorizediE_Extensions>,
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
    pub i_e_extensions: Option<WLANMeasurementConfigurationiE_Extensions>,
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
    pub i_e_extensions: Option<WUS_Assistance_InformationiE_Extensions>,
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
    cellIDList(ECGIList),
    #[asn(key = 1, extended = false)]
    trackingAreaListforWarning(TAIListforWarning),
    #[asn(key = 2, extended = false)]
    emergencyAreaIDList(EmergencyAreaIDList),
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
    pub protocol_i_es: WriteReplaceWarningRequestprotocolIEs,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true)]
pub struct WriteReplaceWarningResponse {
    pub protocol_i_es: WriteReplaceWarningResponseprotocolIEs,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct X2TNLConfigurationInfo {
    pub e_nbx2_transport_layer_addresses: ENBX2TLAs,
    #[asn(optional_idx = 0)]
    pub i_e_extensions: Option<X2TNLConfigurationInfoiE_Extensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "OCTET-STRING")]
pub struct OCTET_STRING_2(Vec<u8>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct Additional_GUTIiE_Extensions_Item {}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct Additional_GUTIiE_Extensions(Vec<Additional_GUTIiE_Extensions_Item>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct AllocationAndRetentionPriorityiE_Extensions_Item {}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct AllocationAndRetentionPriorityiE_Extensions(
    Vec<AllocationAndRetentionPriorityiE_Extensions_Item>,
);

#[derive(Debug, AperCodec)]
#[asn(type = "NULL")]
pub struct NULL_3;

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct AssistanceDataForCECapableUEsiE_Extensions_Item {}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct AssistanceDataForCECapableUEsiE_Extensions(
    Vec<AssistanceDataForCECapableUEsiE_Extensions_Item>,
);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct AssistanceDataForPagingiE_Extensions_Item {}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct AssistanceDataForPagingiE_Extensions(Vec<AssistanceDataForPagingiE_Extensions_Item>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct AssistanceDataForRecommendedCellsiE_Extensions_Item {}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct AssistanceDataForRecommendedCellsiE_Extensions(
    Vec<AssistanceDataForRecommendedCellsiE_Extensions_Item>,
);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct Bearers_SubjectToEarlyStatusTransfer_ItemiE_Extensions_Item {}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct Bearers_SubjectToEarlyStatusTransfer_ItemiE_Extensions(
    Vec<Bearers_SubjectToEarlyStatusTransfer_ItemiE_Extensions_Item>,
);

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum Bearers_SubjectToEarlyStatusTransferList_Itemvalue {
    #[asn(key = 322)]
    Bearers_SubjectToEarlyStatusTransfer_Item(Bearers_SubjectToEarlyStatusTransfer_Item),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct Bearers_SubjectToEarlyStatusTransferList_Item {
    #[asn(key_field = true)]
    pub id: ProtocolIE_ID,
    pub criticality: Criticality,
    pub value: Bearers_SubjectToEarlyStatusTransferList_Itemvalue,
}

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum Bearers_SubjectToStatusTransfer_ItemiE_Extensions_ItemextensionValue {
    #[asn(key = 179)]
    COUNTValueExtended(COUNTValueExtended),
    #[asn(key = 217)]
    COUNTvaluePDCP_SNlength18(COUNTvaluePDCP_SNlength18),
    #[asn(key = 181)]
    ReceiveStatusOfULPDCPSDUsExtended(ReceiveStatusOfULPDCPSDUsExtended),
    #[asn(key = 219)]
    ReceiveStatusOfULPDCPSDUsPDCP_SNlength18(ReceiveStatusOfULPDCPSDUsPDCP_SNlength18),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct Bearers_SubjectToStatusTransfer_ItemiE_Extensions_Item {
    #[asn(key_field = true)]
    pub id: ProtocolExtensionID,
    pub criticality: Criticality,
    pub extension_value: Bearers_SubjectToStatusTransfer_ItemiE_Extensions_ItemextensionValue,
}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct Bearers_SubjectToStatusTransfer_ItemiE_Extensions(
    Vec<Bearers_SubjectToStatusTransfer_ItemiE_Extensions_Item>,
);

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum Bearers_SubjectToStatusTransferList_Itemvalue {
    #[asn(key = 89)]
    Bearers_SubjectToStatusTransfer_Item(Bearers_SubjectToStatusTransfer_Item),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct Bearers_SubjectToStatusTransferList_Item {
    #[asn(key_field = true)]
    pub id: ProtocolIE_ID,
    pub criticality: Criticality,
    pub value: Bearers_SubjectToStatusTransferList_Itemvalue,
}

#[derive(Debug, AperCodec)]
#[asn(type = "ENUMERATED", extensible = true, lb = "0", ub = "0")]
pub struct ENUMERATED_4(u8);
impl ENUMERATED_4 {
    const TRUE: u8 = 0u8;
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct BluetoothMeasurementConfigurationiE_Extensions_Item {}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct BluetoothMeasurementConfigurationiE_Extensions(
    Vec<BluetoothMeasurementConfigurationiE_Extensions_Item>,
);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct CGIiE_Extensions_Item {}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct CGIiE_Extensions(Vec<CGIiE_Extensions_Item>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct CNTypeRestrictions_ItemiE_Extensions_Item {}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct CNTypeRestrictions_ItemiE_Extensions(Vec<CNTypeRestrictions_ItemiE_Extensions_Item>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct COUNTValueExtendediE_Extensions_Item {}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct COUNTValueExtendediE_Extensions(Vec<COUNTValueExtendediE_Extensions_Item>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct COUNTvalueiE_Extensions_Item {}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct COUNTvalueiE_Extensions(Vec<COUNTvalueiE_Extensions_Item>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct COUNTvaluePDCP_SNlength18iE_Extensions_Item {}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct COUNTvaluePDCP_SNlength18iE_Extensions(Vec<COUNTvaluePDCP_SNlength18iE_Extensions_Item>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct CSG_IdList_ItemiE_Extensions_Item {}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct CSG_IdList_ItemiE_Extensions(Vec<CSG_IdList_ItemiE_Extensions_Item>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct CSGMembershipInfoiE_Extensions_Item {}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct CSGMembershipInfoiE_Extensions(Vec<CSGMembershipInfoiE_Extensions_Item>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct CancelledCellinEAI_ItemiE_Extensions_Item {}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct CancelledCellinEAI_ItemiE_Extensions(Vec<CancelledCellinEAI_ItemiE_Extensions_Item>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct CancelledCellinTAI_ItemiE_Extensions_Item {}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct CancelledCellinTAI_ItemiE_Extensions(Vec<CancelledCellinTAI_ItemiE_Extensions_Item>);

#[derive(Debug, AperCodec)]
#[asn(type = "INTEGER", lb = "0", ub = "503")]
pub struct INTEGER_5(u16);

#[derive(Debug, AperCodec)]
#[asn(type = "OCTET-STRING")]
pub struct OCTET_STRING_6(Vec<u8>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct Cdma2000OneXSRVCCInfoiE_Extensions_Item {}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct Cdma2000OneXSRVCCInfoiE_Extensions(Vec<Cdma2000OneXSRVCCInfoiE_Extensions_Item>);

#[derive(Debug, AperCodec)]
#[asn(type = "INTEGER", lb = "1", ub = "60")]
pub struct INTEGER_7(u8);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct CellBasedMDTiE_Extensions_Item {}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct CellBasedMDTiE_Extensions(Vec<CellBasedMDTiE_Extensions_Item>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct CellBasedQMCiE_Extensions_Item {}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct CellBasedQMCiE_Extensions(Vec<CellBasedQMCiE_Extensions_Item>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct CellID_Broadcast_ItemiE_Extensions_Item {}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct CellID_Broadcast_ItemiE_Extensions(Vec<CellID_Broadcast_ItemiE_Extensions_Item>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct CellID_Cancelled_ItemiE_Extensions_Item {}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct CellID_Cancelled_ItemiE_Extensions(Vec<CellID_Cancelled_ItemiE_Extensions_Item>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct CellIdentifierAndCELevelForCECapableUEsiE_Extensions_Item {}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct CellIdentifierAndCELevelForCECapableUEsiE_Extensions(
    Vec<CellIdentifierAndCELevelForCECapableUEsiE_Extensions_Item>,
);

#[derive(Debug, AperCodec)]
#[asn(type = "OCTET-STRING")]
pub struct OCTET_STRING_8(Vec<u8>);

#[derive(Debug, AperCodec)]
#[asn(type = "OCTET-STRING")]
pub struct OCTET_STRING_9(Vec<u8>);

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum CellTrafficTraceprotocolIEs_Itemvalue {
    #[asn(key = 86)]
    E_UTRAN_Trace_ID(E_UTRAN_Trace_ID),
    #[asn(key = 8)]
    ENB_UE_S1AP_ID(ENB_UE_S1AP_ID),
    #[asn(key = 100)]
    EUTRAN_CGI(EUTRAN_CGI),
    #[asn(key = 0)]
    MME_UE_S1AP_ID(MME_UE_S1AP_ID),
    #[asn(key = 166)]
    PrivacyIndicator(PrivacyIndicator),
    #[asn(key = 131)]
    TransportLayerAddress(TransportLayerAddress),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct CellTrafficTraceprotocolIEs_Item {
    #[asn(key_field = true)]
    pub id: ProtocolIE_ID,
    pub criticality: Criticality,
    pub value: CellTrafficTraceprotocolIEs_Itemvalue,
}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "0",
    sz_ub = "65535"
)]
pub struct CellTrafficTraceprotocolIEs(Vec<CellTrafficTraceprotocolIEs_Item>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct CellTypeiE_Extensions_Item {}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct CellTypeiE_Extensions(Vec<CellTypeiE_Extensions_Item>);

#[derive(Debug, AperCodec)]
#[asn(type = "OCTET-STRING")]
pub struct OCTET_STRING_10(Vec<u8>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct CompletedCellinEAI_ItemiE_Extensions_Item {}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct CompletedCellinEAI_ItemiE_Extensions(Vec<CompletedCellinEAI_ItemiE_Extensions_Item>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct CompletedCellinTAI_ItemiE_Extensions_Item {}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct CompletedCellinTAI_ItemiE_Extensions(Vec<CompletedCellinTAI_ItemiE_Extensions_Item>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct ConnectedengNBItemiE_Extensions_Item {}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct ConnectedengNBItemiE_Extensions(Vec<ConnectedengNBItemiE_Extensions_Item>);

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum ConnectionEstablishmentIndicationprotocolIEs_Itemvalue {
    #[asn(key = 271)]
    CE_ModeBRestricted(CE_ModeBRestricted),
    #[asn(key = 253)]
    DL_CP_SecurityInformation(DL_CP_SecurityInformation),
    #[asn(key = 252)]
    E_RABLevelQoSParameters(E_RABLevelQoSParameters),
    #[asn(key = 8)]
    ENB_UE_S1AP_ID(ENB_UE_S1AP_ID),
    #[asn(key = 280)]
    EndIndication(EndIndication),
    #[asn(key = 251)]
    EnhancedCoverageRestricted(EnhancedCoverageRestricted),
    #[asn(key = 0)]
    MME_UE_S1AP_ID(MME_UE_S1AP_ID),
    #[asn(key = 278)]
    Subscription_Based_UE_DifferentiationInfo(Subscription_Based_UE_DifferentiationInfo),
    #[asn(key = 74)]
    UERadioCapability(UERadioCapability),
    #[asn(key = 314)]
    UERadioCapabilityID(UERadioCapabilityID),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct ConnectionEstablishmentIndicationprotocolIEs_Item {
    #[asn(key_field = true)]
    pub id: ProtocolIE_ID,
    pub criticality: Criticality,
    pub value: ConnectionEstablishmentIndicationprotocolIEs_Itemvalue,
}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "0",
    sz_ub = "65535"
)]
pub struct ConnectionEstablishmentIndicationprotocolIEs(
    Vec<ConnectionEstablishmentIndicationprotocolIEs_Item>,
);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct ContextatSourceiE_Extensions_Item {}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct ContextatSourceiE_Extensions(Vec<ContextatSourceiE_Extensions_Item>);

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
#[asn(type = "SEQUENCE", extensible = false)]
pub struct CriticalityDiagnostics_IE_ItemiE_Extensions_Item {}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct CriticalityDiagnostics_IE_ItemiE_Extensions(
    Vec<CriticalityDiagnostics_IE_ItemiE_Extensions_Item>,
);

#[derive(Debug, AperCodec)]
#[asn(type = "ENUMERATED", extensible = true, lb = "0", ub = "0")]
pub struct ENUMERATED_11(u8);
impl ENUMERATED_11 {
    const D_APS_HO_REQUIRED: u8 = 0u8;
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct DAPSRequestInfoiE_Extensions_Item {}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct DAPSRequestInfoiE_Extensions(Vec<DAPSRequestInfoiE_Extensions_Item>);

#[derive(Debug, AperCodec)]
#[asn(type = "ENUMERATED", extensible = true, lb = "0", ub = "1")]
pub struct ENUMERATED_12(u8);
impl ENUMERATED_12 {
    const D_APS_HO_ACCEPTED: u8 = 0u8;
    const D_APS_HO_NOT_ACCEPTED: u8 = 1u8;
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct DAPSResponseInfoiE_Extensions_Item {}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct DAPSResponseInfoiE_Extensions(Vec<DAPSResponseInfoiE_Extensions_Item>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct DAPSResponseInfoItemiE_Extensions_Item {}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct DAPSResponseInfoItemiE_Extensions(Vec<DAPSResponseInfoItemiE_Extensions_Item>);

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum DAPSResponseInfoList_Itemvalue {
    #[asn(key = 319)]
    DAPSResponseInfoItem(DAPSResponseInfoItem),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct DAPSResponseInfoList_Item {
    #[asn(key_field = true)]
    pub id: ProtocolIE_ID,
    pub criticality: Criticality,
    pub value: DAPSResponseInfoList_Itemvalue,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct DL_CP_SecurityInformationiE_Extensions_Item {}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct DL_CP_SecurityInformationiE_Extensions(Vec<DL_CP_SecurityInformationiE_Extensions_Item>);

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum DeactivateTraceprotocolIEs_Itemvalue {
    #[asn(key = 86)]
    E_UTRAN_Trace_ID(E_UTRAN_Trace_ID),
    #[asn(key = 8)]
    ENB_UE_S1AP_ID(ENB_UE_S1AP_ID),
    #[asn(key = 0)]
    MME_UE_S1AP_ID(MME_UE_S1AP_ID),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct DeactivateTraceprotocolIEs_Item {
    #[asn(key_field = true)]
    pub id: ProtocolIE_ID,
    pub criticality: Criticality,
    pub value: DeactivateTraceprotocolIEs_Itemvalue,
}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "0",
    sz_ub = "65535"
)]
pub struct DeactivateTraceprotocolIEs(Vec<DeactivateTraceprotocolIEs_Item>);

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum DownlinkNASTransportprotocolIEs_Itemvalue {
    #[asn(key = 299)]
    AdditionalRRMPriorityIndex(AdditionalRRMPriorityIndex),
    #[asn(key = 271)]
    CE_ModeBRestricted(CE_ModeBRestricted),
    #[asn(key = 249)]
    DLNASPDUDeliveryAckRequest(DLNASPDUDeliveryAckRequest),
    #[asn(key = 8)]
    ENB_UE_S1AP_ID(ENB_UE_S1AP_ID),
    #[asn(key = 280)]
    EndIndication(EndIndication),
    #[asn(key = 251)]
    EnhancedCoverageRestricted(EnhancedCoverageRestricted),
    #[asn(key = 41)]
    HandoverRestrictionList(HandoverRestrictionList),
    #[asn(key = 0)]
    MME_UE_S1AP_ID(MME_UE_S1AP_ID),
    #[asn(key = 26)]
    NAS_PDU(NAS_PDU),
    #[asn(key = 269)]
    NRUESecurityCapabilities(NRUESecurityCapabilities),
    #[asn(key = 283)]
    PendingDataIndication(PendingDataIndication),
    #[asn(key = 124)]
    SRVCCOperationPossible(SRVCCOperationPossible),
    #[asn(key = 106)]
    SubscriberProfileIDforRFP(SubscriberProfileIDforRFP),
    #[asn(key = 278)]
    Subscription_Based_UE_DifferentiationInfo(Subscription_Based_UE_DifferentiationInfo),
    #[asn(key = 275)]
    UECapabilityInfoRequest(UECapabilityInfoRequest),
    #[asn(key = 74)]
    UERadioCapability(UERadioCapability),
    #[asn(key = 314)]
    UERadioCapabilityID(UERadioCapabilityID),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct DownlinkNASTransportprotocolIEs_Item {
    #[asn(key_field = true)]
    pub id: ProtocolIE_ID,
    pub criticality: Criticality,
    pub value: DownlinkNASTransportprotocolIEs_Itemvalue,
}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "0",
    sz_ub = "65535"
)]
pub struct DownlinkNASTransportprotocolIEs(Vec<DownlinkNASTransportprotocolIEs_Item>);

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum DownlinkNonUEAssociatedLPPaTransportprotocolIEs_Itemvalue {
    #[asn(key = 147)]
    LPPa_PDU(LPPa_PDU),
    #[asn(key = 148)]
    Routing_ID(Routing_ID),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct DownlinkNonUEAssociatedLPPaTransportprotocolIEs_Item {
    #[asn(key_field = true)]
    pub id: ProtocolIE_ID,
    pub criticality: Criticality,
    pub value: DownlinkNonUEAssociatedLPPaTransportprotocolIEs_Itemvalue,
}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "0",
    sz_ub = "65535"
)]
pub struct DownlinkNonUEAssociatedLPPaTransportprotocolIEs(
    Vec<DownlinkNonUEAssociatedLPPaTransportprotocolIEs_Item>,
);

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum DownlinkS1cdma2000tunnellingprotocolIEs_Itemvalue {
    #[asn(key = 83)]
    Cdma2000HOStatus(Cdma2000HOStatus),
    #[asn(key = 70)]
    Cdma2000PDU(Cdma2000PDU),
    #[asn(key = 71)]
    Cdma2000RATType(Cdma2000RATType),
    #[asn(key = 12)]
    E_RABSubjecttoDataForwardingList(E_RABSubjecttoDataForwardingList),
    #[asn(key = 8)]
    ENB_UE_S1AP_ID(ENB_UE_S1AP_ID),
    #[asn(key = 0)]
    MME_UE_S1AP_ID(MME_UE_S1AP_ID),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct DownlinkS1cdma2000tunnellingprotocolIEs_Item {
    #[asn(key_field = true)]
    pub id: ProtocolIE_ID,
    pub criticality: Criticality,
    pub value: DownlinkS1cdma2000tunnellingprotocolIEs_Itemvalue,
}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "0",
    sz_ub = "65535"
)]
pub struct DownlinkS1cdma2000tunnellingprotocolIEs(
    Vec<DownlinkS1cdma2000tunnellingprotocolIEs_Item>,
);

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum DownlinkUEAssociatedLPPaTransportprotocolIEs_Itemvalue {
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
pub struct DownlinkUEAssociatedLPPaTransportprotocolIEs_Item {
    #[asn(key_field = true)]
    pub id: ProtocolIE_ID,
    pub criticality: Criticality,
    pub value: DownlinkUEAssociatedLPPaTransportprotocolIEs_Itemvalue,
}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "0",
    sz_ub = "65535"
)]
pub struct DownlinkUEAssociatedLPPaTransportprotocolIEs(
    Vec<DownlinkUEAssociatedLPPaTransportprotocolIEs_Item>,
);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct E_RABAdmittedItemiE_Extensions_Item {}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct E_RABAdmittedItemiE_Extensions(Vec<E_RABAdmittedItemiE_Extensions_Item>);

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum E_RABAdmittedList_Itemvalue {
    #[asn(key = 20)]
    E_RABAdmittedItem(E_RABAdmittedItem),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct E_RABAdmittedList_Item {
    #[asn(key_field = true)]
    pub id: ProtocolIE_ID,
    pub criticality: Criticality,
    pub value: E_RABAdmittedList_Itemvalue,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct E_RABDataForwardingItemiE_Extensions_Item {}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct E_RABDataForwardingItemiE_Extensions(Vec<E_RABDataForwardingItemiE_Extensions_Item>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct E_RABFailedToResumeItemResumeReqiE_Extensions_Item {}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct E_RABFailedToResumeItemResumeReqiE_Extensions(
    Vec<E_RABFailedToResumeItemResumeReqiE_Extensions_Item>,
);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct E_RABFailedToResumeItemResumeResiE_Extensions_Item {}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct E_RABFailedToResumeItemResumeResiE_Extensions(
    Vec<E_RABFailedToResumeItemResumeResiE_Extensions_Item>,
);

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum E_RABFailedToResumeListResumeReq_Itemvalue {
    #[asn(key = 236)]
    E_RABFailedToResumeItemResumeReq(E_RABFailedToResumeItemResumeReq),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct E_RABFailedToResumeListResumeReq_Item {
    #[asn(key_field = true)]
    pub id: ProtocolIE_ID,
    pub criticality: Criticality,
    pub value: E_RABFailedToResumeListResumeReq_Itemvalue,
}

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum E_RABFailedToResumeListResumeRes_Itemvalue {
    #[asn(key = 238)]
    E_RABFailedToResumeItemResumeRes(E_RABFailedToResumeItemResumeRes),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct E_RABFailedToResumeListResumeRes_Item {
    #[asn(key_field = true)]
    pub id: ProtocolIE_ID,
    pub criticality: Criticality,
    pub value: E_RABFailedToResumeListResumeRes_Itemvalue,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct E_RABFailedToSetupItemHOReqAckiE_Extensions_Item {}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct E_RABFailedToSetupItemHOReqAckiE_Extensions(
    Vec<E_RABFailedToSetupItemHOReqAckiE_Extensions_Item>,
);

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum E_RABFailedtoSetupListHOReqAck_Itemvalue {
    #[asn(key = 21)]
    E_RABFailedToSetupItemHOReqAck(E_RABFailedToSetupItemHOReqAck),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct E_RABFailedtoSetupListHOReqAck_Item {
    #[asn(key_field = true)]
    pub id: ProtocolIE_ID,
    pub criticality: Criticality,
    pub value: E_RABFailedtoSetupListHOReqAck_Itemvalue,
}

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum E_RABInformationList_Itemvalue {
    #[asn(key = 78)]
    E_RABInformationListItem(E_RABInformationListItem),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct E_RABInformationList_Item {
    #[asn(key_field = true)]
    pub id: ProtocolIE_ID,
    pub criticality: Criticality,
    pub value: E_RABInformationList_Itemvalue,
}

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum E_RABInformationListItemiE_Extensions_ItemextensionValue {
    #[asn(key = 317)]
    DAPSRequestInfo(DAPSRequestInfo),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct E_RABInformationListItemiE_Extensions_Item {
    #[asn(key_field = true)]
    pub id: ProtocolExtensionID,
    pub criticality: Criticality,
    pub extension_value: E_RABInformationListItemiE_Extensions_ItemextensionValue,
}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct E_RABInformationListItemiE_Extensions(Vec<E_RABInformationListItemiE_Extensions_Item>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct E_RABItemiE_Extensions_Item {}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct E_RABItemiE_Extensions(Vec<E_RABItemiE_Extensions_Item>);

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum E_RABLevelQoSParametersiE_Extensions_ItemextensionValue {
    #[asn(key = 274)]
    Packet_LossRate(Packet_LossRate),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct E_RABLevelQoSParametersiE_Extensions_Item {
    #[asn(key_field = true)]
    pub id: ProtocolExtensionID,
    pub criticality: Criticality,
    pub extension_value: E_RABLevelQoSParametersiE_Extensions_ItemextensionValue,
}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct E_RABLevelQoSParametersiE_Extensions(Vec<E_RABLevelQoSParametersiE_Extensions_Item>);

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum E_RABList_Itemvalue {
    #[asn(key = 35)]
    E_RABItem(E_RABItem),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct E_RABList_Item {
    #[asn(key_field = true)]
    pub id: ProtocolIE_ID,
    pub criticality: Criticality,
    pub value: E_RABList_Itemvalue,
}

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum E_RABModificationConfirmprotocolIEs_Itemvalue {
    #[asn(key = 146)]
    CSGMembershipStatus(CSGMembershipStatus),
    #[asn(key = 58)]
    CriticalityDiagnostics(CriticalityDiagnostics),
    #[asn(key = 210)]
    E_RABList(E_RABList),
    #[asn(key = 203)]
    E_RABModifyListBearerModConf(E_RABModifyListBearerModConf),
    #[asn(key = 8)]
    ENB_UE_S1AP_ID(ENB_UE_S1AP_ID),
    #[asn(key = 0)]
    MME_UE_S1AP_ID(MME_UE_S1AP_ID),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct E_RABModificationConfirmprotocolIEs_Item {
    #[asn(key_field = true)]
    pub id: ProtocolIE_ID,
    pub criticality: Criticality,
    pub value: E_RABModificationConfirmprotocolIEs_Itemvalue,
}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "0",
    sz_ub = "65535"
)]
pub struct E_RABModificationConfirmprotocolIEs(Vec<E_RABModificationConfirmprotocolIEs_Item>);

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum E_RABModificationIndicationprotocolIEs_Itemvalue {
    #[asn(key = 226)]
    CSGMembershipInfo(CSGMembershipInfo),
    #[asn(key = 201)]
    E_RABNotToBeModifiedListBearerModInd(E_RABNotToBeModifiedListBearerModInd),
    #[asn(key = 199)]
    E_RABToBeModifiedListBearerModInd(E_RABToBeModifiedListBearerModInd),
    #[asn(key = 8)]
    ENB_UE_S1AP_ID(ENB_UE_S1AP_ID),
    #[asn(key = 0)]
    MME_UE_S1AP_ID(MME_UE_S1AP_ID),
    #[asn(key = 264)]
    SecondaryRATDataUsageReportList(SecondaryRATDataUsageReportList),
    #[asn(key = 176)]
    TunnelInformation(TunnelInformation),
    #[asn(key = 189)]
    UserLocationInformation(UserLocationInformation),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct E_RABModificationIndicationprotocolIEs_Item {
    #[asn(key_field = true)]
    pub id: ProtocolIE_ID,
    pub criticality: Criticality,
    pub value: E_RABModificationIndicationprotocolIEs_Itemvalue,
}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "0",
    sz_ub = "65535"
)]
pub struct E_RABModificationIndicationprotocolIEs(Vec<E_RABModificationIndicationprotocolIEs_Item>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct E_RABModifyItemBearerModConfiE_Extensions_Item {}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct E_RABModifyItemBearerModConfiE_Extensions(
    Vec<E_RABModifyItemBearerModConfiE_Extensions_Item>,
);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct E_RABModifyItemBearerModResiE_Extensions_Item {}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct E_RABModifyItemBearerModResiE_Extensions(
    Vec<E_RABModifyItemBearerModResiE_Extensions_Item>,
);

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum E_RABModifyListBearerModConf_Itemvalue {
    #[asn(key = 204)]
    E_RABModifyItemBearerModConf(E_RABModifyItemBearerModConf),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct E_RABModifyListBearerModConf_Item {
    #[asn(key_field = true)]
    pub id: ProtocolIE_ID,
    pub criticality: Criticality,
    pub value: E_RABModifyListBearerModConf_Itemvalue,
}

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum E_RABModifyListBearerModRes_Itemvalue {
    #[asn(key = 37)]
    E_RABModifyItemBearerModRes(E_RABModifyItemBearerModRes),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct E_RABModifyListBearerModRes_Item {
    #[asn(key_field = true)]
    pub id: ProtocolIE_ID,
    pub criticality: Criticality,
    pub value: E_RABModifyListBearerModRes_Itemvalue,
}

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum E_RABModifyRequestprotocolIEs_Itemvalue {
    #[asn(key = 30)]
    E_RABToBeModifiedListBearerModReq(E_RABToBeModifiedListBearerModReq),
    #[asn(key = 8)]
    ENB_UE_S1AP_ID(ENB_UE_S1AP_ID),
    #[asn(key = 0)]
    MME_UE_S1AP_ID(MME_UE_S1AP_ID),
    #[asn(key = 268)]
    SecondaryRATDataUsageRequest(SecondaryRATDataUsageRequest),
    #[asn(key = 66)]
    UEAggregateMaximumBitrate(UEAggregateMaximumBitrate),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct E_RABModifyRequestprotocolIEs_Item {
    #[asn(key_field = true)]
    pub id: ProtocolIE_ID,
    pub criticality: Criticality,
    pub value: E_RABModifyRequestprotocolIEs_Itemvalue,
}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "0",
    sz_ub = "65535"
)]
pub struct E_RABModifyRequestprotocolIEs(Vec<E_RABModifyRequestprotocolIEs_Item>);

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum E_RABModifyResponseprotocolIEs_Itemvalue {
    #[asn(key = 58)]
    CriticalityDiagnostics(CriticalityDiagnostics),
    #[asn(key = 32)]
    E_RABList(E_RABList),
    #[asn(key = 31)]
    E_RABModifyListBearerModRes(E_RABModifyListBearerModRes),
    #[asn(key = 8)]
    ENB_UE_S1AP_ID(ENB_UE_S1AP_ID),
    #[asn(key = 0)]
    MME_UE_S1AP_ID(MME_UE_S1AP_ID),
    #[asn(key = 264)]
    SecondaryRATDataUsageReportList(SecondaryRATDataUsageReportList),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct E_RABModifyResponseprotocolIEs_Item {
    #[asn(key_field = true)]
    pub id: ProtocolIE_ID,
    pub criticality: Criticality,
    pub value: E_RABModifyResponseprotocolIEs_Itemvalue,
}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "0",
    sz_ub = "65535"
)]
pub struct E_RABModifyResponseprotocolIEs(Vec<E_RABModifyResponseprotocolIEs_Item>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct E_RABNotToBeModifiedItemBearerModIndiE_Extensions_Item {}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct E_RABNotToBeModifiedItemBearerModIndiE_Extensions(
    Vec<E_RABNotToBeModifiedItemBearerModIndiE_Extensions_Item>,
);

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum E_RABNotToBeModifiedListBearerModInd_Itemvalue {
    #[asn(key = 202)]
    E_RABNotToBeModifiedItemBearerModInd(E_RABNotToBeModifiedItemBearerModInd),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct E_RABNotToBeModifiedListBearerModInd_Item {
    #[asn(key_field = true)]
    pub id: ProtocolIE_ID,
    pub criticality: Criticality,
    pub value: E_RABNotToBeModifiedListBearerModInd_Itemvalue,
}

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum E_RABReleaseCommandprotocolIEs_Itemvalue {
    #[asn(key = 33)]
    E_RABList(E_RABList),
    #[asn(key = 8)]
    ENB_UE_S1AP_ID(ENB_UE_S1AP_ID),
    #[asn(key = 0)]
    MME_UE_S1AP_ID(MME_UE_S1AP_ID),
    #[asn(key = 26)]
    NAS_PDU(NAS_PDU),
    #[asn(key = 66)]
    UEAggregateMaximumBitrate(UEAggregateMaximumBitrate),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct E_RABReleaseCommandprotocolIEs_Item {
    #[asn(key_field = true)]
    pub id: ProtocolIE_ID,
    pub criticality: Criticality,
    pub value: E_RABReleaseCommandprotocolIEs_Itemvalue,
}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "0",
    sz_ub = "65535"
)]
pub struct E_RABReleaseCommandprotocolIEs(Vec<E_RABReleaseCommandprotocolIEs_Item>);

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum E_RABReleaseIndicationprotocolIEs_Itemvalue {
    #[asn(key = 110)]
    E_RABList(E_RABList),
    #[asn(key = 8)]
    ENB_UE_S1AP_ID(ENB_UE_S1AP_ID),
    #[asn(key = 0)]
    MME_UE_S1AP_ID(MME_UE_S1AP_ID),
    #[asn(key = 264)]
    SecondaryRATDataUsageReportList(SecondaryRATDataUsageReportList),
    #[asn(key = 189)]
    UserLocationInformation(UserLocationInformation),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct E_RABReleaseIndicationprotocolIEs_Item {
    #[asn(key_field = true)]
    pub id: ProtocolIE_ID,
    pub criticality: Criticality,
    pub value: E_RABReleaseIndicationprotocolIEs_Itemvalue,
}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "0",
    sz_ub = "65535"
)]
pub struct E_RABReleaseIndicationprotocolIEs(Vec<E_RABReleaseIndicationprotocolIEs_Item>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct E_RABReleaseItemBearerRelCompiE_Extensions_Item {}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct E_RABReleaseItemBearerRelCompiE_Extensions(
    Vec<E_RABReleaseItemBearerRelCompiE_Extensions_Item>,
);

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum E_RABReleaseListBearerRelComp_Itemvalue {
    #[asn(key = 15)]
    E_RABReleaseItemBearerRelComp(E_RABReleaseItemBearerRelComp),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct E_RABReleaseListBearerRelComp_Item {
    #[asn(key_field = true)]
    pub id: ProtocolIE_ID,
    pub criticality: Criticality,
    pub value: E_RABReleaseListBearerRelComp_Itemvalue,
}

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum E_RABReleaseResponseprotocolIEs_Itemvalue {
    #[asn(key = 58)]
    CriticalityDiagnostics(CriticalityDiagnostics),
    #[asn(key = 34)]
    E_RABList(E_RABList),
    #[asn(key = 69)]
    E_RABReleaseListBearerRelComp(E_RABReleaseListBearerRelComp),
    #[asn(key = 8)]
    ENB_UE_S1AP_ID(ENB_UE_S1AP_ID),
    #[asn(key = 0)]
    MME_UE_S1AP_ID(MME_UE_S1AP_ID),
    #[asn(key = 264)]
    SecondaryRATDataUsageReportList(SecondaryRATDataUsageReportList),
    #[asn(key = 189)]
    UserLocationInformation(UserLocationInformation),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct E_RABReleaseResponseprotocolIEs_Item {
    #[asn(key_field = true)]
    pub id: ProtocolIE_ID,
    pub criticality: Criticality,
    pub value: E_RABReleaseResponseprotocolIEs_Itemvalue,
}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "0",
    sz_ub = "65535"
)]
pub struct E_RABReleaseResponseprotocolIEs(Vec<E_RABReleaseResponseprotocolIEs_Item>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct E_RABSetupItemBearerSUResiE_Extensions_Item {}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct E_RABSetupItemBearerSUResiE_Extensions(Vec<E_RABSetupItemBearerSUResiE_Extensions_Item>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct E_RABSetupItemCtxtSUResiE_Extensions_Item {}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct E_RABSetupItemCtxtSUResiE_Extensions(Vec<E_RABSetupItemCtxtSUResiE_Extensions_Item>);

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum E_RABSetupListBearerSURes_Itemvalue {
    #[asn(key = 39)]
    E_RABSetupItemBearerSURes(E_RABSetupItemBearerSURes),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct E_RABSetupListBearerSURes_Item {
    #[asn(key_field = true)]
    pub id: ProtocolIE_ID,
    pub criticality: Criticality,
    pub value: E_RABSetupListBearerSURes_Itemvalue,
}

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum E_RABSetupListCtxtSURes_Itemvalue {
    #[asn(key = 50)]
    E_RABSetupItemCtxtSURes(E_RABSetupItemCtxtSURes),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct E_RABSetupListCtxtSURes_Item {
    #[asn(key_field = true)]
    pub id: ProtocolIE_ID,
    pub criticality: Criticality,
    pub value: E_RABSetupListCtxtSURes_Itemvalue,
}

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum E_RABSetupRequestprotocolIEs_Itemvalue {
    #[asn(key = 16)]
    E_RABToBeSetupListBearerSUReq(E_RABToBeSetupListBearerSUReq),
    #[asn(key = 8)]
    ENB_UE_S1AP_ID(ENB_UE_S1AP_ID),
    #[asn(key = 0)]
    MME_UE_S1AP_ID(MME_UE_S1AP_ID),
    #[asn(key = 66)]
    UEAggregateMaximumBitrate(UEAggregateMaximumBitrate),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct E_RABSetupRequestprotocolIEs_Item {
    #[asn(key_field = true)]
    pub id: ProtocolIE_ID,
    pub criticality: Criticality,
    pub value: E_RABSetupRequestprotocolIEs_Itemvalue,
}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "0",
    sz_ub = "65535"
)]
pub struct E_RABSetupRequestprotocolIEs(Vec<E_RABSetupRequestprotocolIEs_Item>);

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum E_RABSetupResponseprotocolIEs_Itemvalue {
    #[asn(key = 58)]
    CriticalityDiagnostics(CriticalityDiagnostics),
    #[asn(key = 29)]
    E_RABList(E_RABList),
    #[asn(key = 28)]
    E_RABSetupListBearerSURes(E_RABSetupListBearerSURes),
    #[asn(key = 8)]
    ENB_UE_S1AP_ID(ENB_UE_S1AP_ID),
    #[asn(key = 0)]
    MME_UE_S1AP_ID(MME_UE_S1AP_ID),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct E_RABSetupResponseprotocolIEs_Item {
    #[asn(key_field = true)]
    pub id: ProtocolIE_ID,
    pub criticality: Criticality,
    pub value: E_RABSetupResponseprotocolIEs_Itemvalue,
}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "0",
    sz_ub = "65535"
)]
pub struct E_RABSetupResponseprotocolIEs(Vec<E_RABSetupResponseprotocolIEs_Item>);

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum E_RABSubjecttoDataForwardingList_Itemvalue {
    #[asn(key = 14)]
    E_RABDataForwardingItem(E_RABDataForwardingItem),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct E_RABSubjecttoDataForwardingList_Item {
    #[asn(key_field = true)]
    pub id: ProtocolIE_ID,
    pub criticality: Criticality,
    pub value: E_RABSubjecttoDataForwardingList_Itemvalue,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct E_RABToBeModifiedItemBearerModIndiE_Extensions_Item {}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct E_RABToBeModifiedItemBearerModIndiE_Extensions(
    Vec<E_RABToBeModifiedItemBearerModIndiE_Extensions_Item>,
);

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum E_RABToBeModifiedItemBearerModReqiE_Extensions_ItemextensionValue {
    #[asn(key = 185)]
    TransportInformation(TransportInformation),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct E_RABToBeModifiedItemBearerModReqiE_Extensions_Item {
    #[asn(key_field = true)]
    pub id: ProtocolExtensionID,
    pub criticality: Criticality,
    pub extension_value: E_RABToBeModifiedItemBearerModReqiE_Extensions_ItemextensionValue,
}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct E_RABToBeModifiedItemBearerModReqiE_Extensions(
    Vec<E_RABToBeModifiedItemBearerModReqiE_Extensions_Item>,
);

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum E_RABToBeModifiedListBearerModInd_Itemvalue {
    #[asn(key = 200)]
    E_RABToBeModifiedItemBearerModInd(E_RABToBeModifiedItemBearerModInd),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct E_RABToBeModifiedListBearerModInd_Item {
    #[asn(key_field = true)]
    pub id: ProtocolIE_ID,
    pub criticality: Criticality,
    pub value: E_RABToBeModifiedListBearerModInd_Itemvalue,
}

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum E_RABToBeModifiedListBearerModReq_Itemvalue {
    #[asn(key = 36)]
    E_RABToBeModifiedItemBearerModReq(E_RABToBeModifiedItemBearerModReq),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct E_RABToBeModifiedListBearerModReq_Item {
    #[asn(key_field = true)]
    pub id: ProtocolIE_ID,
    pub criticality: Criticality,
    pub value: E_RABToBeModifiedListBearerModReq_Itemvalue,
}

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum E_RABToBeSetupItemBearerSUReqiE_Extensions_ItemextensionValue {
    #[asn(key = 233)]
    BearerType(BearerType),
    #[asn(key = 183)]
    Correlation_ID(Correlation_ID),
    #[asn(key = 305)]
    Ethernet_Type(Ethernet_Type),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct E_RABToBeSetupItemBearerSUReqiE_Extensions_Item {
    #[asn(key_field = true)]
    pub id: ProtocolExtensionID,
    pub criticality: Criticality,
    pub extension_value: E_RABToBeSetupItemBearerSUReqiE_Extensions_ItemextensionValue,
}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct E_RABToBeSetupItemBearerSUReqiE_Extensions(
    Vec<E_RABToBeSetupItemBearerSUReqiE_Extensions_Item>,
);

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum E_RABToBeSetupItemCtxtSUReqiE_Extensions_ItemextensionValue {
    #[asn(key = 233)]
    BearerType(BearerType),
    #[asn(key = 156)]
    Correlation_ID(Correlation_ID),
    #[asn(key = 305)]
    Ethernet_Type(Ethernet_Type),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct E_RABToBeSetupItemCtxtSUReqiE_Extensions_Item {
    #[asn(key_field = true)]
    pub id: ProtocolExtensionID,
    pub criticality: Criticality,
    pub extension_value: E_RABToBeSetupItemCtxtSUReqiE_Extensions_ItemextensionValue,
}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct E_RABToBeSetupItemCtxtSUReqiE_Extensions(
    Vec<E_RABToBeSetupItemCtxtSUReqiE_Extensions_Item>,
);

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum E_RABToBeSetupItemHOReqiE_Extensions_ItemextensionValue {
    #[asn(key = 233)]
    BearerType(BearerType),
    #[asn(key = 143)]
    Data_Forwarding_Not_Possible(Data_Forwarding_Not_Possible),
    #[asn(key = 305)]
    Ethernet_Type(Ethernet_Type),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct E_RABToBeSetupItemHOReqiE_Extensions_Item {
    #[asn(key_field = true)]
    pub id: ProtocolExtensionID,
    pub criticality: Criticality,
    pub extension_value: E_RABToBeSetupItemHOReqiE_Extensions_ItemextensionValue,
}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct E_RABToBeSetupItemHOReqiE_Extensions(Vec<E_RABToBeSetupItemHOReqiE_Extensions_Item>);

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum E_RABToBeSetupListBearerSUReq_Itemvalue {
    #[asn(key = 17)]
    E_RABToBeSetupItemBearerSUReq(E_RABToBeSetupItemBearerSUReq),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct E_RABToBeSetupListBearerSUReq_Item {
    #[asn(key_field = true)]
    pub id: ProtocolIE_ID,
    pub criticality: Criticality,
    pub value: E_RABToBeSetupListBearerSUReq_Itemvalue,
}

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum E_RABToBeSetupListCtxtSUReq_Itemvalue {
    #[asn(key = 52)]
    E_RABToBeSetupItemCtxtSUReq(E_RABToBeSetupItemCtxtSUReq),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct E_RABToBeSetupListCtxtSUReq_Item {
    #[asn(key_field = true)]
    pub id: ProtocolIE_ID,
    pub criticality: Criticality,
    pub value: E_RABToBeSetupListCtxtSUReq_Itemvalue,
}

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum E_RABToBeSetupListHOReq_Itemvalue {
    #[asn(key = 27)]
    E_RABToBeSetupItemHOReq(E_RABToBeSetupItemHOReq),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct E_RABToBeSetupListHOReq_Item {
    #[asn(key_field = true)]
    pub id: ProtocolIE_ID,
    pub criticality: Criticality,
    pub value: E_RABToBeSetupListHOReq_Itemvalue,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct E_RABToBeSwitchedDLItemiE_Extensions_Item {}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct E_RABToBeSwitchedDLItemiE_Extensions(Vec<E_RABToBeSwitchedDLItemiE_Extensions_Item>);

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum E_RABToBeSwitchedDLList_Itemvalue {
    #[asn(key = 23)]
    E_RABToBeSwitchedDLItem(E_RABToBeSwitchedDLItem),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct E_RABToBeSwitchedDLList_Item {
    #[asn(key_field = true)]
    pub id: ProtocolIE_ID,
    pub criticality: Criticality,
    pub value: E_RABToBeSwitchedDLList_Itemvalue,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct E_RABToBeSwitchedULItemiE_Extensions_Item {}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct E_RABToBeSwitchedULItemiE_Extensions(Vec<E_RABToBeSwitchedULItemiE_Extensions_Item>);

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum E_RABToBeSwitchedULList_Itemvalue {
    #[asn(key = 94)]
    E_RABToBeSwitchedULItem(E_RABToBeSwitchedULItem),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct E_RABToBeSwitchedULList_Item {
    #[asn(key_field = true)]
    pub id: ProtocolIE_ID,
    pub criticality: Criticality,
    pub value: E_RABToBeSwitchedULList_Itemvalue,
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
pub struct E_RABUsageReportItemiE_Extensions_Item {}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct E_RABUsageReportItemiE_Extensions(Vec<E_RABUsageReportItemiE_Extensions_Item>);

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum E_RABUsageReportList_Itemvalue {
    #[asn(key = 267)]
    E_RABUsageReportItem(E_RABUsageReportItem),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct E_RABUsageReportList_Item {
    #[asn(key_field = true)]
    pub id: ProtocolIE_ID,
    pub criticality: Criticality,
    pub value: E_RABUsageReportList_Itemvalue,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct EN_DCSONConfigurationTransferiE_Extensions_Item {}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct EN_DCSONConfigurationTransferiE_Extensions(
    Vec<EN_DCSONConfigurationTransferiE_Extensions_Item>,
);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct EN_DCSONeNBIdentificationiE_Extensions_Item {}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct EN_DCSONeNBIdentificationiE_Extensions(Vec<EN_DCSONeNBIdentificationiE_Extensions_Item>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct EN_DCSONengNBIdentificationiE_Extensions_Item {}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct EN_DCSONengNBIdentificationiE_Extensions(
    Vec<EN_DCSONengNBIdentificationiE_Extensions_Item>,
);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct EN_DCTransferTypeReplyiE_Extensions_Item {}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct EN_DCTransferTypeReplyiE_Extensions(Vec<EN_DCTransferTypeReplyiE_Extensions_Item>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct EN_DCTransferTypeRequestiE_Extensions_Item {}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct EN_DCTransferTypeRequestiE_Extensions(Vec<EN_DCTransferTypeRequestiE_Extensions_Item>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct ENB_EarlyStatusTransfer_TransparentContaineriE_Extensions_Item {}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct ENB_EarlyStatusTransfer_TransparentContaineriE_Extensions(
    Vec<ENB_EarlyStatusTransfer_TransparentContaineriE_Extensions_Item>,
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
pub struct ENB_StatusTransfer_TransparentContaineriE_Extensions_Item {}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct ENB_StatusTransfer_TransparentContaineriE_Extensions(
    Vec<ENB_StatusTransfer_TransparentContaineriE_Extensions_Item>,
);

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum ENBCPRelocationIndicationprotocolIEs_Itemvalue {
    #[asn(key = 8)]
    ENB_UE_S1AP_ID(ENB_UE_S1AP_ID),
    #[asn(key = 100)]
    EUTRAN_CGI(EUTRAN_CGI),
    #[asn(key = 96)]
    S_TMSI(S_TMSI),
    #[asn(key = 67)]
    TAI(TAI),
    #[asn(key = 254)]
    UL_CP_SecurityInformation(UL_CP_SecurityInformation),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct ENBCPRelocationIndicationprotocolIEs_Item {
    #[asn(key_field = true)]
    pub id: ProtocolIE_ID,
    pub criticality: Criticality,
    pub value: ENBCPRelocationIndicationprotocolIEs_Itemvalue,
}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "0",
    sz_ub = "65535"
)]
pub struct ENBCPRelocationIndicationprotocolIEs(Vec<ENBCPRelocationIndicationprotocolIEs_Item>);

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum ENBConfigurationTransferprotocolIEs_Itemvalue {
    #[asn(key = 294)]
    EN_DCSONConfigurationTransfer(EN_DCSONConfigurationTransfer),
    #[asn(key = 310)]
    IntersystemSONConfigurationTransfer(IntersystemSONConfigurationTransfer),
    #[asn(key = 129)]
    SONConfigurationTransfer(SONConfigurationTransfer),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct ENBConfigurationTransferprotocolIEs_Item {
    #[asn(key_field = true)]
    pub id: ProtocolIE_ID,
    pub criticality: Criticality,
    pub value: ENBConfigurationTransferprotocolIEs_Itemvalue,
}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "0",
    sz_ub = "65535"
)]
pub struct ENBConfigurationTransferprotocolIEs(Vec<ENBConfigurationTransferprotocolIEs_Item>);

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum ENBConfigurationUpdateprotocolIEs_Itemvalue {
    #[asn(key = 128)]
    CSG_IdList(CSG_IdList),
    #[asn(key = 293)]
    ConnectedengNBList(ConnectedengNBList),
    #[asn(key = 60)]
    ENBname(ENBname),
    #[asn(key = 234)]
    NB_IoT_DefaultPagingDRX(NB_IoT_DefaultPagingDRX),
    #[asn(key = 137)]
    PagingDRX(PagingDRX),
    #[asn(key = 64)]
    SupportedTAs(SupportedTAs),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct ENBConfigurationUpdateprotocolIEs_Item {
    #[asn(key_field = true)]
    pub id: ProtocolIE_ID,
    pub criticality: Criticality,
    pub value: ENBConfigurationUpdateprotocolIEs_Itemvalue,
}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "0",
    sz_ub = "65535"
)]
pub struct ENBConfigurationUpdateprotocolIEs(Vec<ENBConfigurationUpdateprotocolIEs_Item>);

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum ENBConfigurationUpdateAcknowledgeprotocolIEs_Itemvalue {
    #[asn(key = 58)]
    CriticalityDiagnostics(CriticalityDiagnostics),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct ENBConfigurationUpdateAcknowledgeprotocolIEs_Item {
    #[asn(key_field = true)]
    pub id: ProtocolIE_ID,
    pub criticality: Criticality,
    pub value: ENBConfigurationUpdateAcknowledgeprotocolIEs_Itemvalue,
}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "0",
    sz_ub = "65535"
)]
pub struct ENBConfigurationUpdateAcknowledgeprotocolIEs(
    Vec<ENBConfigurationUpdateAcknowledgeprotocolIEs_Item>,
);

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum ENBConfigurationUpdateFailureprotocolIEs_Itemvalue {
    #[asn(key = 2)]
    Cause(Cause),
    #[asn(key = 58)]
    CriticalityDiagnostics(CriticalityDiagnostics),
    #[asn(key = 65)]
    TimeToWait(TimeToWait),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct ENBConfigurationUpdateFailureprotocolIEs_Item {
    #[asn(key_field = true)]
    pub id: ProtocolIE_ID,
    pub criticality: Criticality,
    pub value: ENBConfigurationUpdateFailureprotocolIEs_Itemvalue,
}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "0",
    sz_ub = "65535"
)]
pub struct ENBConfigurationUpdateFailureprotocolIEs(
    Vec<ENBConfigurationUpdateFailureprotocolIEs_Item>,
);

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum ENBDirectInformationTransferprotocolIEs_Itemvalue {
    #[asn(key = 121)]
    Inter_SystemInformationTransferType(Inter_SystemInformationTransferType),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct ENBDirectInformationTransferprotocolIEs_Item {
    #[asn(key_field = true)]
    pub id: ProtocolIE_ID,
    pub criticality: Criticality,
    pub value: ENBDirectInformationTransferprotocolIEs_Itemvalue,
}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "0",
    sz_ub = "65535"
)]
pub struct ENBDirectInformationTransferprotocolIEs(
    Vec<ENBDirectInformationTransferprotocolIEs_Item>,
);

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum ENBEarlyStatusTransferprotocolIEs_Itemvalue {
    #[asn(key = 321)]
    ENB_EarlyStatusTransfer_TransparentContainer(ENB_EarlyStatusTransfer_TransparentContainer),
    #[asn(key = 8)]
    ENB_UE_S1AP_ID(ENB_UE_S1AP_ID),
    #[asn(key = 0)]
    MME_UE_S1AP_ID(MME_UE_S1AP_ID),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct ENBEarlyStatusTransferprotocolIEs_Item {
    #[asn(key_field = true)]
    pub id: ProtocolIE_ID,
    pub criticality: Criticality,
    pub value: ENBEarlyStatusTransferprotocolIEs_Itemvalue,
}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "0",
    sz_ub = "65535"
)]
pub struct ENBEarlyStatusTransferprotocolIEs(Vec<ENBEarlyStatusTransferprotocolIEs_Item>);

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum ENBStatusTransferprotocolIEs_Itemvalue {
    #[asn(key = 90)]
    ENB_StatusTransfer_TransparentContainer(ENB_StatusTransfer_TransparentContainer),
    #[asn(key = 8)]
    ENB_UE_S1AP_ID(ENB_UE_S1AP_ID),
    #[asn(key = 0)]
    MME_UE_S1AP_ID(MME_UE_S1AP_ID),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct ENBStatusTransferprotocolIEs_Item {
    #[asn(key_field = true)]
    pub id: ProtocolIE_ID,
    pub criticality: Criticality,
    pub value: ENBStatusTransferprotocolIEs_Itemvalue,
}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "0",
    sz_ub = "65535"
)]
pub struct ENBStatusTransferprotocolIEs(Vec<ENBStatusTransferprotocolIEs_Item>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct ENBX2ExtTLAiE_Extensions_Item {}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct ENBX2ExtTLAiE_Extensions(Vec<ENBX2ExtTLAiE_Extensions_Item>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct EUTRAN_CGIiE_Extensions_Item {}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct EUTRAN_CGIiE_Extensions(Vec<EUTRAN_CGIiE_Extensions_Item>);

#[derive(Debug, AperCodec)]
#[asn(type = "OCTET-STRING")]
pub struct OCTET_STRING_21(Vec<u8>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct EmergencyAreaID_Broadcast_ItemiE_Extensions_Item {}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct EmergencyAreaID_Broadcast_ItemiE_Extensions(
    Vec<EmergencyAreaID_Broadcast_ItemiE_Extensions_Item>,
);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct EmergencyAreaID_Cancelled_ItemiE_Extensions_Item {}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct EmergencyAreaID_Cancelled_ItemiE_Extensions(
    Vec<EmergencyAreaID_Cancelled_ItemiE_Extensions_Item>,
);

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum ErrorIndicationprotocolIEs_Itemvalue {
    #[asn(key = 2)]
    Cause(Cause),
    #[asn(key = 58)]
    CriticalityDiagnostics(CriticalityDiagnostics),
    #[asn(key = 8)]
    ENB_UE_S1AP_ID(ENB_UE_S1AP_ID),
    #[asn(key = 0)]
    MME_UE_S1AP_ID(MME_UE_S1AP_ID),
    #[asn(key = 96)]
    S_TMSI(S_TMSI),
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
#[asn(type = "SEQUENCE", extensible = false)]
pub struct ExpectedUEActivityBehaviouriE_Extensions_Item {}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct ExpectedUEActivityBehaviouriE_Extensions(
    Vec<ExpectedUEActivityBehaviouriE_Extensions_Item>,
);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct ExpectedUEBehaviouriE_Extensions_Item {}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct ExpectedUEBehaviouriE_Extensions(Vec<ExpectedUEBehaviouriE_Extensions_Item>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct FiveGSTAIiE_Extensions_Item {}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct FiveGSTAIiE_Extensions(Vec<FiveGSTAIiE_Extensions_Item>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct ForbiddenLAs_ItemiE_Extensions_Item {}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct ForbiddenLAs_ItemiE_Extensions(Vec<ForbiddenLAs_ItemiE_Extensions_Item>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct ForbiddenTAs_ItemiE_Extensions_Item {}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct ForbiddenTAs_ItemiE_Extensions(Vec<ForbiddenTAs_ItemiE_Extensions_Item>);

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum GBR_QosInformationiE_Extensions_ItemextensionValue {
    #[asn(key = 257)]
    ExtendedBitRate(ExtendedBitRate),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct GBR_QosInformationiE_Extensions_Item {
    #[asn(key_field = true)]
    pub id: ProtocolExtensionID,
    pub criticality: Criticality,
    pub extension_value: GBR_QosInformationiE_Extensions_ItemextensionValue,
}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct GBR_QosInformationiE_Extensions(Vec<GBR_QosInformationiE_Extensions_Item>);

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
pub struct GNBiE_Extensions_Item {}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct GNBiE_Extensions(Vec<GNBiE_Extensions_Item>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct GUMMEIiE_Extensions_Item {}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct GUMMEIiE_Extensions(Vec<GUMMEIiE_Extensions_Item>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct Global_ENB_IDiE_Extensions_Item {}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct Global_ENB_IDiE_Extensions(Vec<Global_ENB_IDiE_Extensions_Item>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct Global_GNB_IDiE_Extensions_Item {}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct Global_GNB_IDiE_Extensions(Vec<Global_GNB_IDiE_Extensions_Item>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct Global_en_gNB_IDiE_Extensions_Item {}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct Global_en_gNB_IDiE_Extensions(Vec<Global_en_gNB_IDiE_Extensions_Item>);

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum HandoverCancelprotocolIEs_Itemvalue {
    #[asn(key = 2)]
    Cause(Cause),
    #[asn(key = 8)]
    ENB_UE_S1AP_ID(ENB_UE_S1AP_ID),
    #[asn(key = 0)]
    MME_UE_S1AP_ID(MME_UE_S1AP_ID),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct HandoverCancelprotocolIEs_Item {
    #[asn(key_field = true)]
    pub id: ProtocolIE_ID,
    pub criticality: Criticality,
    pub value: HandoverCancelprotocolIEs_Itemvalue,
}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "0",
    sz_ub = "65535"
)]
pub struct HandoverCancelprotocolIEs(Vec<HandoverCancelprotocolIEs_Item>);

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum HandoverCancelAcknowledgeprotocolIEs_Itemvalue {
    #[asn(key = 58)]
    CriticalityDiagnostics(CriticalityDiagnostics),
    #[asn(key = 8)]
    ENB_UE_S1AP_ID(ENB_UE_S1AP_ID),
    #[asn(key = 0)]
    MME_UE_S1AP_ID(MME_UE_S1AP_ID),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct HandoverCancelAcknowledgeprotocolIEs_Item {
    #[asn(key_field = true)]
    pub id: ProtocolIE_ID,
    pub criticality: Criticality,
    pub value: HandoverCancelAcknowledgeprotocolIEs_Itemvalue,
}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "0",
    sz_ub = "65535"
)]
pub struct HandoverCancelAcknowledgeprotocolIEs(Vec<HandoverCancelAcknowledgeprotocolIEs_Item>);

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum HandoverCommandprotocolIEs_Itemvalue {
    #[asn(key = 58)]
    CriticalityDiagnostics(CriticalityDiagnostics),
    #[asn(key = 13)]
    E_RABList(E_RABList),
    #[asn(key = 12)]
    E_RABSubjecttoDataForwardingList(E_RABSubjecttoDataForwardingList),
    #[asn(key = 8)]
    ENB_UE_S1AP_ID(ENB_UE_S1AP_ID),
    #[asn(key = 1)]
    HandoverType(HandoverType),
    #[asn(key = 0)]
    MME_UE_S1AP_ID(MME_UE_S1AP_ID),
    #[asn(key = 135)]
    NASSecurityParametersfromE_UTRAN(NASSecurityParametersfromE_UTRAN),
    #[asn(key = 139)]
    Target_ToSource_TransparentContainer(Target_ToSource_TransparentContainer),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct HandoverCommandprotocolIEs_Item {
    #[asn(key_field = true)]
    pub id: ProtocolIE_ID,
    pub criticality: Criticality,
    pub value: HandoverCommandprotocolIEs_Itemvalue,
}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "0",
    sz_ub = "65535"
)]
pub struct HandoverCommandprotocolIEs(Vec<HandoverCommandprotocolIEs_Item>);

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum HandoverFailureprotocolIEs_Itemvalue {
    #[asn(key = 2)]
    Cause(Cause),
    #[asn(key = 58)]
    CriticalityDiagnostics(CriticalityDiagnostics),
    #[asn(key = 0)]
    MME_UE_S1AP_ID(MME_UE_S1AP_ID),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct HandoverFailureprotocolIEs_Item {
    #[asn(key_field = true)]
    pub id: ProtocolIE_ID,
    pub criticality: Criticality,
    pub value: HandoverFailureprotocolIEs_Itemvalue,
}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "0",
    sz_ub = "65535"
)]
pub struct HandoverFailureprotocolIEs(Vec<HandoverFailureprotocolIEs_Item>);

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum HandoverNotifyprotocolIEs_Itemvalue {
    #[asn(key = 8)]
    ENB_UE_S1AP_ID(ENB_UE_S1AP_ID),
    #[asn(key = 100)]
    EUTRAN_CGI(EUTRAN_CGI),
    #[asn(key = 186)]
    LHN_ID(LHN_ID),
    #[asn(key = 0)]
    MME_UE_S1AP_ID(MME_UE_S1AP_ID),
    #[asn(key = 320)]
    NotifySourceeNB(NotifySourceeNB),
    #[asn(key = 288)]
    PSCellInformation(PSCellInformation),
    #[asn(key = 67)]
    TAI(TAI),
    #[asn(key = 176)]
    TunnelInformation(TunnelInformation),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct HandoverNotifyprotocolIEs_Item {
    #[asn(key_field = true)]
    pub id: ProtocolIE_ID,
    pub criticality: Criticality,
    pub value: HandoverNotifyprotocolIEs_Itemvalue,
}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "0",
    sz_ub = "65535"
)]
pub struct HandoverNotifyprotocolIEs(Vec<HandoverNotifyprotocolIEs_Item>);

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum HandoverPreparationFailureprotocolIEs_Itemvalue {
    #[asn(key = 2)]
    Cause(Cause),
    #[asn(key = 58)]
    CriticalityDiagnostics(CriticalityDiagnostics),
    #[asn(key = 8)]
    ENB_UE_S1AP_ID(ENB_UE_S1AP_ID),
    #[asn(key = 0)]
    MME_UE_S1AP_ID(MME_UE_S1AP_ID),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct HandoverPreparationFailureprotocolIEs_Item {
    #[asn(key_field = true)]
    pub id: ProtocolIE_ID,
    pub criticality: Criticality,
    pub value: HandoverPreparationFailureprotocolIEs_Itemvalue,
}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "0",
    sz_ub = "65535"
)]
pub struct HandoverPreparationFailureprotocolIEs(Vec<HandoverPreparationFailureprotocolIEs_Item>);

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum HandoverRequestprotocolIEs_Itemvalue {
    #[asn(key = 299)]
    AdditionalRRMPriorityIndex(AdditionalRRMPriorityIndex),
    #[asn(key = 277)]
    AerialUEsubscriptionInformation(AerialUEsubscriptionInformation),
    #[asn(key = 271)]
    CE_ModeBRestricted(CE_ModeBRestricted),
    #[asn(key = 127)]
    CSG_Id(CSG_Id),
    #[asn(key = 146)]
    CSGMembershipStatus(CSGMembershipStatus),
    #[asn(key = 2)]
    Cause(Cause),
    #[asn(key = 53)]
    E_RABToBeSetupListHOReq(E_RABToBeSetupListHOReq),
    #[asn(key = 251)]
    EnhancedCoverageRestricted(EnhancedCoverageRestricted),
    #[asn(key = 196)]
    ExpectedUEBehaviour(ExpectedUEBehaviour),
    #[asn(key = 75)]
    GUMMEI(GUMMEI),
    #[asn(key = 41)]
    HandoverRestrictionList(HandoverRestrictionList),
    #[asn(key = 1)]
    HandoverType(HandoverType),
    #[asn(key = 301)]
    IAB_Authorized(IAB_Authorized),
    #[asn(key = 177)]
    MDTPLMNList(MDTPLMNList),
    #[asn(key = 0)]
    MME_UE_S1AP_ID(MME_UE_S1AP_ID),
    #[asn(key = 165)]
    ManagementBasedMDTAllowed(ManagementBasedMDTAllowed),
    #[asn(key = 192)]
    Masked_IMEISV(Masked_IMEISV),
    #[asn(key = 136)]
    NASSecurityParameterstoE_UTRAN(NASSecurityParameterstoE_UTRAN),
    #[asn(key = 269)]
    NRUESecurityCapabilities(NRUESecurityCapabilities),
    #[asn(key = 307)]
    NRUESidelinkAggregateMaximumBitrate(NRUESidelinkAggregateMaximumBitrate),
    #[asn(key = 306)]
    NRV2XServicesAuthorized(NRV2XServicesAuthorized),
    #[asn(key = 308)]
    PC5QoSParameters(PC5QoSParameters),
    #[asn(key = 283)]
    PendingDataIndication(PendingDataIndication),
    #[asn(key = 195)]
    ProSeAuthorized(ProSeAuthorized),
    #[asn(key = 98)]
    RequestType(RequestType),
    #[asn(key = 124)]
    SRVCCOperationPossible(SRVCCOperationPossible),
    #[asn(key = 40)]
    SecurityContext(SecurityContext),
    #[asn(key = 104)]
    Source_ToTarget_TransparentContainer(Source_ToTarget_TransparentContainer),
    #[asn(key = 278)]
    Subscription_Based_UE_DifferentiationInfo(Subscription_Based_UE_DifferentiationInfo),
    #[asn(key = 25)]
    TraceActivation(TraceActivation),
    #[asn(key = 66)]
    UEAggregateMaximumBitrate(UEAggregateMaximumBitrate),
    #[asn(key = 314)]
    UERadioCapabilityID(UERadioCapabilityID),
    #[asn(key = 107)]
    UESecurityCapabilities(UESecurityCapabilities),
    #[asn(key = 248)]
    UESidelinkAggregateMaximumBitrate(UESidelinkAggregateMaximumBitrate),
    #[asn(key = 241)]
    UEUserPlaneCIoTSupportIndicator(UEUserPlaneCIoTSupportIndicator),
    #[asn(key = 240)]
    V2XServicesAuthorized(V2XServicesAuthorized),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct HandoverRequestprotocolIEs_Item {
    #[asn(key_field = true)]
    pub id: ProtocolIE_ID,
    pub criticality: Criticality,
    pub value: HandoverRequestprotocolIEs_Itemvalue,
}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "0",
    sz_ub = "65535"
)]
pub struct HandoverRequestprotocolIEs(Vec<HandoverRequestprotocolIEs_Item>);

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum HandoverRequestAcknowledgeprotocolIEs_Itemvalue {
    #[asn(key = 242)]
    CE_mode_B_SupportIndicator(CE_mode_B_SupportIndicator),
    #[asn(key = 127)]
    CSG_Id(CSG_Id),
    #[asn(key = 145)]
    CellAccessMode(CellAccessMode),
    #[asn(key = 58)]
    CriticalityDiagnostics(CriticalityDiagnostics),
    #[asn(key = 18)]
    E_RABAdmittedList(E_RABAdmittedList),
    #[asn(key = 19)]
    E_RABFailedtoSetupListHOReqAck(E_RABFailedtoSetupListHOReqAck),
    #[asn(key = 8)]
    ENB_UE_S1AP_ID(ENB_UE_S1AP_ID),
    #[asn(key = 0)]
    MME_UE_S1AP_ID(MME_UE_S1AP_ID),
    #[asn(key = 123)]
    Target_ToSource_TransparentContainer(Target_ToSource_TransparentContainer),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct HandoverRequestAcknowledgeprotocolIEs_Item {
    #[asn(key_field = true)]
    pub id: ProtocolIE_ID,
    pub criticality: Criticality,
    pub value: HandoverRequestAcknowledgeprotocolIEs_Itemvalue,
}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "0",
    sz_ub = "65535"
)]
pub struct HandoverRequestAcknowledgeprotocolIEs(Vec<HandoverRequestAcknowledgeprotocolIEs_Item>);

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum HandoverRequiredprotocolIEs_Itemvalue {
    #[asn(key = 127)]
    CSG_Id(CSG_Id),
    #[asn(key = 2)]
    Cause(Cause),
    #[asn(key = 145)]
    CellAccessMode(CellAccessMode),
    #[asn(key = 79)]
    Direct_Forwarding_Path_Availability(Direct_Forwarding_Path_Availability),
    #[asn(key = 8)]
    ENB_UE_S1AP_ID(ENB_UE_S1AP_ID),
    #[asn(key = 1)]
    HandoverType(HandoverType),
    #[asn(key = 0)]
    MME_UE_S1AP_ID(MME_UE_S1AP_ID),
    #[asn(key = 132)]
    MSClassmark2(MSClassmark2),
    #[asn(key = 133)]
    MSClassmark3(MSClassmark3),
    #[asn(key = 150)]
    PS_ServiceNotAvailable(PS_ServiceNotAvailable),
    #[asn(key = 125)]
    SRVCCHOIndication(SRVCCHOIndication),
    #[asn(key = 138)]
    Source_ToTarget_TransparentContainer(Source_ToTarget_TransparentContainer),
    #[asn(key = 4)]
    TargetID(TargetID),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct HandoverRequiredprotocolIEs_Item {
    #[asn(key_field = true)]
    pub id: ProtocolIE_ID,
    pub criticality: Criticality,
    pub value: HandoverRequiredprotocolIEs_Itemvalue,
}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "0",
    sz_ub = "65535"
)]
pub struct HandoverRequiredprotocolIEs(Vec<HandoverRequiredprotocolIEs_Item>);

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum HandoverRestrictionListiE_Extensions_ItemextensionValue {
    #[asn(key = 282)]
    CNTypeRestrictions(CNTypeRestrictions),
    #[asn(key = 287)]
    NRrestrictionin5GS(NRrestrictionin5GS),
    #[asn(key = 261)]
    NRrestrictioninEPSasSecondaryRAT(NRrestrictioninEPSasSecondaryRAT),
    #[asn(key = 290)]
    PLMNidentity(PLMNidentity),
    #[asn(key = 270)]
    UnlicensedSpectrumRestriction(UnlicensedSpectrumRestriction),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct HandoverRestrictionListiE_Extensions_Item {
    #[asn(key_field = true)]
    pub id: ProtocolExtensionID,
    pub criticality: Criticality,
    pub extension_value: HandoverRestrictionListiE_Extensions_ItemextensionValue,
}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct HandoverRestrictionListiE_Extensions(Vec<HandoverRestrictionListiE_Extensions_Item>);

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum HandoverSuccessprotocolIEs_Itemvalue {
    #[asn(key = 8)]
    ENB_UE_S1AP_ID(ENB_UE_S1AP_ID),
    #[asn(key = 0)]
    MME_UE_S1AP_ID(MME_UE_S1AP_ID),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct HandoverSuccessprotocolIEs_Item {
    #[asn(key_field = true)]
    pub id: ProtocolIE_ID,
    pub criticality: Criticality,
    pub value: HandoverSuccessprotocolIEs_Itemvalue,
}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "0",
    sz_ub = "65535"
)]
pub struct HandoverSuccessprotocolIEs(Vec<HandoverSuccessprotocolIEs_Item>);

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
pub enum ImmediateMDTiE_Extensions_ItemextensionValue {
    #[asn(key = 284)]
    BluetoothMeasurementConfiguration(BluetoothMeasurementConfiguration),
    #[asn(key = 171)]
    M3Configuration(M3Configuration),
    #[asn(key = 172)]
    M4Configuration(M4Configuration),
    #[asn(key = 173)]
    M5Configuration(M5Configuration),
    #[asn(key = 220)]
    M6Configuration(M6Configuration),
    #[asn(key = 221)]
    M7Configuration(M7Configuration),
    #[asn(key = 174)]
    MDT_Location_Info(MDT_Location_Info),
    #[asn(key = 285)]
    WLANMeasurementConfiguration(WLANMeasurementConfiguration),
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
#[asn(type = "SEQUENCE", extensible = false)]
pub struct InformationOnRecommendedCellsAndENBsForPagingiE_Extensions_Item {}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct InformationOnRecommendedCellsAndENBsForPagingiE_Extensions(
    Vec<InformationOnRecommendedCellsAndENBsForPagingiE_Extensions_Item>,
);

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum InitialContextSetupFailureprotocolIEs_Itemvalue {
    #[asn(key = 2)]
    Cause(Cause),
    #[asn(key = 58)]
    CriticalityDiagnostics(CriticalityDiagnostics),
    #[asn(key = 8)]
    ENB_UE_S1AP_ID(ENB_UE_S1AP_ID),
    #[asn(key = 0)]
    MME_UE_S1AP_ID(MME_UE_S1AP_ID),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct InitialContextSetupFailureprotocolIEs_Item {
    #[asn(key_field = true)]
    pub id: ProtocolIE_ID,
    pub criticality: Criticality,
    pub value: InitialContextSetupFailureprotocolIEs_Itemvalue,
}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "0",
    sz_ub = "65535"
)]
pub struct InitialContextSetupFailureprotocolIEs(Vec<InitialContextSetupFailureprotocolIEs_Item>);

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum InitialContextSetupRequestprotocolIEs_Itemvalue {
    #[asn(key = 187)]
    AdditionalCSFallbackIndicator(AdditionalCSFallbackIndicator),
    #[asn(key = 299)]
    AdditionalRRMPriorityIndex(AdditionalRRMPriorityIndex),
    #[asn(key = 277)]
    AerialUEsubscriptionInformation(AerialUEsubscriptionInformation),
    #[asn(key = 271)]
    CE_ModeBRestricted(CE_ModeBRestricted),
    #[asn(key = 108)]
    CSFallbackIndicator(CSFallbackIndicator),
    #[asn(key = 146)]
    CSGMembershipStatus(CSGMembershipStatus),
    #[asn(key = 24)]
    E_RABToBeSetupListCtxtSUReq(E_RABToBeSetupListCtxtSUReq),
    #[asn(key = 8)]
    ENB_UE_S1AP_ID(ENB_UE_S1AP_ID),
    #[asn(key = 251)]
    EnhancedCoverageRestricted(EnhancedCoverageRestricted),
    #[asn(key = 196)]
    ExpectedUEBehaviour(ExpectedUEBehaviour),
    #[asn(key = 75)]
    GUMMEI(GUMMEI),
    #[asn(key = 41)]
    HandoverRestrictionList(HandoverRestrictionList),
    #[asn(key = 301)]
    IAB_Authorized(IAB_Authorized),
    #[asn(key = 159)]
    LAI(LAI),
    #[asn(key = 177)]
    MDTPLMNList(MDTPLMNList),
    #[asn(key = 0)]
    MME_UE_S1AP_ID(MME_UE_S1AP_ID),
    #[asn(key = 165)]
    ManagementBasedMDTAllowed(ManagementBasedMDTAllowed),
    #[asn(key = 192)]
    Masked_IMEISV(Masked_IMEISV),
    #[asn(key = 269)]
    NRUESecurityCapabilities(NRUESecurityCapabilities),
    #[asn(key = 307)]
    NRUESidelinkAggregateMaximumBitrate(NRUESidelinkAggregateMaximumBitrate),
    #[asn(key = 306)]
    NRV2XServicesAuthorized(NRV2XServicesAuthorized),
    #[asn(key = 308)]
    PC5QoSParameters(PC5QoSParameters),
    #[asn(key = 283)]
    PendingDataIndication(PendingDataIndication),
    #[asn(key = 195)]
    ProSeAuthorized(ProSeAuthorized),
    #[asn(key = 124)]
    SRVCCOperationPossible(SRVCCOperationPossible),
    #[asn(key = 73)]
    SecurityKey(SecurityKey),
    #[asn(key = 106)]
    SubscriberProfileIDforRFP(SubscriberProfileIDforRFP),
    #[asn(key = 278)]
    Subscription_Based_UE_DifferentiationInfo(Subscription_Based_UE_DifferentiationInfo),
    #[asn(key = 25)]
    TraceActivation(TraceActivation),
    #[asn(key = 66)]
    UEAggregateMaximumBitrate(UEAggregateMaximumBitrate),
    #[asn(key = 74)]
    UERadioCapability(UERadioCapability),
    #[asn(key = 314)]
    UERadioCapabilityID(UERadioCapabilityID),
    #[asn(key = 107)]
    UESecurityCapabilities(UESecurityCapabilities),
    #[asn(key = 248)]
    UESidelinkAggregateMaximumBitrate(UESidelinkAggregateMaximumBitrate),
    #[asn(key = 241)]
    UEUserPlaneCIoTSupportIndicator(UEUserPlaneCIoTSupportIndicator),
    #[asn(key = 240)]
    V2XServicesAuthorized(V2XServicesAuthorized),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct InitialContextSetupRequestprotocolIEs_Item {
    #[asn(key_field = true)]
    pub id: ProtocolIE_ID,
    pub criticality: Criticality,
    pub value: InitialContextSetupRequestprotocolIEs_Itemvalue,
}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "0",
    sz_ub = "65535"
)]
pub struct InitialContextSetupRequestprotocolIEs(Vec<InitialContextSetupRequestprotocolIEs_Item>);

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum InitialContextSetupResponseprotocolIEs_Itemvalue {
    #[asn(key = 58)]
    CriticalityDiagnostics(CriticalityDiagnostics),
    #[asn(key = 48)]
    E_RABList(E_RABList),
    #[asn(key = 51)]
    E_RABSetupListCtxtSURes(E_RABSetupListCtxtSURes),
    #[asn(key = 8)]
    ENB_UE_S1AP_ID(ENB_UE_S1AP_ID),
    #[asn(key = 0)]
    MME_UE_S1AP_ID(MME_UE_S1AP_ID),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct InitialContextSetupResponseprotocolIEs_Item {
    #[asn(key_field = true)]
    pub id: ProtocolIE_ID,
    pub criticality: Criticality,
    pub value: InitialContextSetupResponseprotocolIEs_Itemvalue,
}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "0",
    sz_ub = "65535"
)]
pub struct InitialContextSetupResponseprotocolIEs(Vec<InitialContextSetupResponseprotocolIEs_Item>);

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum InitialUEMessageprotocolIEs_Itemvalue {
    #[asn(key = 242)]
    CE_mode_B_SupportIndicator(CE_mode_B_SupportIndicator),
    #[asn(key = 127)]
    CSG_Id(CSG_Id),
    #[asn(key = 145)]
    CellAccessMode(CellAccessMode),
    #[asn(key = 250)]
    Coverage_Level(Coverage_Level),
    #[asn(key = 246)]
    DCN_ID(DCN_ID),
    #[asn(key = 281)]
    EDT_Session(EDT_Session),
    #[asn(key = 8)]
    ENB_UE_S1AP_ID(ENB_UE_S1AP_ID),
    #[asn(key = 100)]
    EUTRAN_CGI(EUTRAN_CGI),
    #[asn(key = 75)]
    GUMMEI(GUMMEI),
    #[asn(key = 170)]
    GUMMEIType(GUMMEIType),
    #[asn(key = 302)]
    IAB_Node_Indication(IAB_Node_Indication),
    #[asn(key = 186)]
    LHN_ID(LHN_ID),
    #[asn(key = 223)]
    MME_Group_ID(MME_Group_ID),
    #[asn(key = 26)]
    NAS_PDU(NAS_PDU),
    #[asn(key = 134)]
    RRC_Establishment_Cause(RRC_Establishment_Cause),
    #[asn(key = 160)]
    RelayNode_Indicator(RelayNode_Indicator),
    #[asn(key = 96)]
    S_TMSI(S_TMSI),
    #[asn(key = 67)]
    TAI(TAI),
    #[asn(key = 184)]
    TransportLayerAddress(TransportLayerAddress),
    #[asn(key = 176)]
    TunnelInformation(TunnelInformation),
    #[asn(key = 263)]
    UE_Application_Layer_Measurement_Capability(UE_Application_Layer_Measurement_Capability),
    #[asn(key = 230)]
    UE_Usage_Type(UE_Usage_Type),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct InitialUEMessageprotocolIEs_Item {
    #[asn(key_field = true)]
    pub id: ProtocolIE_ID,
    pub criticality: Criticality,
    pub value: InitialUEMessageprotocolIEs_Itemvalue,
}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "0",
    sz_ub = "65535"
)]
pub struct InitialUEMessageprotocolIEs(Vec<InitialUEMessageprotocolIEs_Item>);

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum InitiatingMessagevalue {
    #[asn(key = 42)]
    CellTrafficTrace(CellTrafficTrace),
    #[asn(key = 54)]
    ConnectionEstablishmentIndication(ConnectionEstablishmentIndication),
    #[asn(key = 26)]
    DeactivateTrace(DeactivateTrace),
    #[asn(key = 11)]
    DownlinkNASTransport(DownlinkNASTransport),
    #[asn(key = 46)]
    DownlinkNonUEAssociatedLPPaTransport(DownlinkNonUEAssociatedLPPaTransport),
    #[asn(key = 19)]
    DownlinkS1cdma2000tunnelling(DownlinkS1cdma2000tunnelling),
    #[asn(key = 44)]
    DownlinkUEAssociatedLPPaTransport(DownlinkUEAssociatedLPPaTransport),
    #[asn(key = 50)]
    E_RABModificationIndication(E_RABModificationIndication),
    #[asn(key = 6)]
    E_RABModifyRequest(E_RABModifyRequest),
    #[asn(key = 7)]
    E_RABReleaseCommand(E_RABReleaseCommand),
    #[asn(key = 8)]
    E_RABReleaseIndication(E_RABReleaseIndication),
    #[asn(key = 5)]
    E_RABSetupRequest(E_RABSetupRequest),
    #[asn(key = 60)]
    ENBCPRelocationIndication(ENBCPRelocationIndication),
    #[asn(key = 40)]
    ENBConfigurationTransfer(ENBConfigurationTransfer),
    #[asn(key = 29)]
    ENBConfigurationUpdate(ENBConfigurationUpdate),
    #[asn(key = 37)]
    ENBDirectInformationTransfer(ENBDirectInformationTransfer),
    #[asn(key = 65)]
    ENBEarlyStatusTransfer(ENBEarlyStatusTransfer),
    #[asn(key = 24)]
    ENBStatusTransfer(ENBStatusTransfer),
    #[asn(key = 15)]
    ErrorIndication(ErrorIndication),
    #[asn(key = 4)]
    HandoverCancel(HandoverCancel),
    #[asn(key = 2)]
    HandoverNotify(HandoverNotify),
    #[asn(key = 1)]
    HandoverRequest(HandoverRequest),
    #[asn(key = 0)]
    HandoverRequired(HandoverRequired),
    #[asn(key = 64)]
    HandoverSuccess(HandoverSuccess),
    #[asn(key = 9)]
    InitialContextSetupRequest(InitialContextSetupRequest),
    #[asn(key = 12)]
    InitialUEMessage(InitialUEMessage),
    #[asn(key = 43)]
    KillRequest(KillRequest),
    #[asn(key = 33)]
    LocationReport(LocationReport),
    #[asn(key = 31)]
    LocationReportingControl(LocationReportingControl),
    #[asn(key = 32)]
    LocationReportingFailureIndication(LocationReportingFailureIndication),
    #[asn(key = 61)]
    MMECPRelocationIndication(MMECPRelocationIndication),
    #[asn(key = 41)]
    MMEConfigurationTransfer(MMEConfigurationTransfer),
    #[asn(key = 30)]
    MMEConfigurationUpdate(MMEConfigurationUpdate),
    #[asn(key = 38)]
    MMEDirectInformationTransfer(MMEDirectInformationTransfer),
    #[asn(key = 66)]
    MMEEarlyStatusTransfer(MMEEarlyStatusTransfer),
    #[asn(key = 25)]
    MMEStatusTransfer(MMEStatusTransfer),
    #[asn(key = 57)]
    NASDeliveryIndication(NASDeliveryIndication),
    #[asn(key = 16)]
    NASNonDeliveryIndication(NASNonDeliveryIndication),
    #[asn(key = 34)]
    OverloadStart(OverloadStart),
    #[asn(key = 35)]
    OverloadStop(OverloadStop),
    #[asn(key = 51)]
    PWSFailureIndication(PWSFailureIndication),
    #[asn(key = 49)]
    PWSRestartIndication(PWSRestartIndication),
    #[asn(key = 10)]
    Paging(Paging),
    #[asn(key = 3)]
    PathSwitchRequest(PathSwitchRequest),
    #[asn(key = 39)]
    PrivateMessage(PrivateMessage),
    #[asn(key = 52)]
    RerouteNASRequest(RerouteNASRequest),
    #[asn(key = 14)]
    Reset(Reset),
    #[asn(key = 58)]
    RetrieveUEInformation(RetrieveUEInformation),
    #[asn(key = 17)]
    S1SetupRequest(S1SetupRequest),
    #[asn(key = 62)]
    SecondaryRATDataUsageReport(SecondaryRATDataUsageReport),
    #[asn(key = 28)]
    TraceFailureIndication(TraceFailureIndication),
    #[asn(key = 27)]
    TraceStart(TraceStart),
    #[asn(key = 22)]
    UECapabilityInfoIndication(UECapabilityInfoIndication),
    #[asn(key = 53)]
    UEContextModificationIndication(UEContextModificationIndication),
    #[asn(key = 21)]
    UEContextModificationRequest(UEContextModificationRequest),
    #[asn(key = 23)]
    UEContextReleaseCommand(UEContextReleaseCommand),
    #[asn(key = 18)]
    UEContextReleaseRequest(UEContextReleaseRequest),
    #[asn(key = 56)]
    UEContextResumeRequest(UEContextResumeRequest),
    #[asn(key = 55)]
    UEContextSuspendRequest(UEContextSuspendRequest),
    #[asn(key = 59)]
    UEInformationTransfer(UEInformationTransfer),
    #[asn(key = 63)]
    UERadioCapabilityIDMappingRequest(UERadioCapabilityIDMappingRequest),
    #[asn(key = 48)]
    UERadioCapabilityMatchRequest(UERadioCapabilityMatchRequest),
    #[asn(key = 13)]
    UplinkNASTransport(UplinkNASTransport),
    #[asn(key = 47)]
    UplinkNonUEAssociatedLPPaTransport(UplinkNonUEAssociatedLPPaTransport),
    #[asn(key = 20)]
    UplinkS1cdma2000tunnelling(UplinkS1cdma2000tunnelling),
    #[asn(key = 45)]
    UplinkUEAssociatedLPPaTransport(UplinkUEAssociatedLPPaTransport),
    #[asn(key = 36)]
    WriteReplaceWarningRequest(WriteReplaceWarningRequest),
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
pub struct InterSystemMeasurementItemiE_Extensions_Item {}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct InterSystemMeasurementItemiE_Extensions(
    Vec<InterSystemMeasurementItemiE_Extensions_Item>,
);

#[derive(Debug, AperCodec)]
#[asn(type = "INTEGER", lb = "1", ub = "100")]
pub struct INTEGER_35(u8);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct InterSystemMeasurementParametersiE_Extensions_Item {}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct InterSystemMeasurementParametersiE_Extensions(
    Vec<InterSystemMeasurementParametersiE_Extensions_Item>,
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
pub struct IntersystemMeasurementConfigurationiE_Extensions_Item {}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct IntersystemMeasurementConfigurationiE_Extensions(
    Vec<IntersystemMeasurementConfigurationiE_Extensions_Item>,
);

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum KillRequestprotocolIEs_Itemvalue {
    #[asn(key = 191)]
    KillAllWarningMessages(KillAllWarningMessages),
    #[asn(key = 111)]
    MessageIdentifier(MessageIdentifier),
    #[asn(key = 112)]
    SerialNumber(SerialNumber),
    #[asn(key = 113)]
    WarningAreaList(WarningAreaList),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct KillRequestprotocolIEs_Item {
    #[asn(key_field = true)]
    pub id: ProtocolIE_ID,
    pub criticality: Criticality,
    pub value: KillRequestprotocolIEs_Itemvalue,
}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "0",
    sz_ub = "65535"
)]
pub struct KillRequestprotocolIEs(Vec<KillRequestprotocolIEs_Item>);

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum KillResponseprotocolIEs_Itemvalue {
    #[asn(key = 141)]
    BroadcastCancelledAreaList(BroadcastCancelledAreaList),
    #[asn(key = 58)]
    CriticalityDiagnostics(CriticalityDiagnostics),
    #[asn(key = 111)]
    MessageIdentifier(MessageIdentifier),
    #[asn(key = 112)]
    SerialNumber(SerialNumber),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct KillResponseprotocolIEs_Item {
    #[asn(key_field = true)]
    pub id: ProtocolIE_ID,
    pub criticality: Criticality,
    pub value: KillResponseprotocolIEs_Itemvalue,
}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "0",
    sz_ub = "65535"
)]
pub struct KillResponseprotocolIEs(Vec<KillResponseprotocolIEs_Item>);

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
#[asn(type = "OPEN")]
pub enum LastVisitedEUTRANCellInformationiE_Extensions_ItemextensionValue {
    #[asn(key = 168)]
    Cause(Cause),
    #[asn(key = 167)]
    Time_UE_StayedInCell_EnhancedGranularity(Time_UE_StayedInCell_EnhancedGranularity),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct LastVisitedEUTRANCellInformationiE_Extensions_Item {
    #[asn(key_field = true)]
    pub id: ProtocolExtensionID,
    pub criticality: Criticality,
    pub extension_value: LastVisitedEUTRANCellInformationiE_Extensions_ItemextensionValue,
}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct LastVisitedEUTRANCellInformationiE_Extensions(
    Vec<LastVisitedEUTRANCellInformationiE_Extensions_Item>,
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
pub struct ListeningSubframePatterniE_Extensions_Item {}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct ListeningSubframePatterniE_Extensions(Vec<ListeningSubframePatterniE_Extensions_Item>);

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum LocationReportprotocolIEs_Itemvalue {
    #[asn(key = 8)]
    ENB_UE_S1AP_ID(ENB_UE_S1AP_ID),
    #[asn(key = 100)]
    EUTRAN_CGI(EUTRAN_CGI),
    #[asn(key = 0)]
    MME_UE_S1AP_ID(MME_UE_S1AP_ID),
    #[asn(key = 288)]
    PSCellInformation(PSCellInformation),
    #[asn(key = 98)]
    RequestType(RequestType),
    #[asn(key = 67)]
    TAI(TAI),
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
pub enum LocationReportingControlprotocolIEs_Itemvalue {
    #[asn(key = 8)]
    ENB_UE_S1AP_ID(ENB_UE_S1AP_ID),
    #[asn(key = 0)]
    MME_UE_S1AP_ID(MME_UE_S1AP_ID),
    #[asn(key = 98)]
    RequestType(RequestType),
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
pub enum LocationReportingFailureIndicationprotocolIEs_Itemvalue {
    #[asn(key = 2)]
    Cause(Cause),
    #[asn(key = 8)]
    ENB_UE_S1AP_ID(ENB_UE_S1AP_ID),
    #[asn(key = 0)]
    MME_UE_S1AP_ID(MME_UE_S1AP_ID),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct LocationReportingFailureIndicationprotocolIEs_Item {
    #[asn(key_field = true)]
    pub id: ProtocolIE_ID,
    pub criticality: Criticality,
    pub value: LocationReportingFailureIndicationprotocolIEs_Itemvalue,
}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "0",
    sz_ub = "65535"
)]
pub struct LocationReportingFailureIndicationprotocolIEs(
    Vec<LocationReportingFailureIndicationprotocolIEs_Item>,
);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct LoggedMBSFNMDTiE_Extensions_Item {}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct LoggedMBSFNMDTiE_Extensions(Vec<LoggedMBSFNMDTiE_Extensions_Item>);

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum LoggedMDTiE_Extensions_ItemextensionValue {
    #[asn(key = 284)]
    BluetoothMeasurementConfiguration(BluetoothMeasurementConfiguration),
    #[asn(key = 285)]
    WLANMeasurementConfiguration(WLANMeasurementConfiguration),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct LoggedMDTiE_Extensions_Item {
    #[asn(key_field = true)]
    pub id: ProtocolExtensionID,
    pub criticality: Criticality,
    pub extension_value: LoggedMDTiE_Extensions_ItemextensionValue,
}

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
pub struct M1PeriodicReportingiE_Extensions_Item {}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct M1PeriodicReportingiE_Extensions(Vec<M1PeriodicReportingiE_Extensions_Item>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct M1ThresholdEventA2iE_Extensions_Item {}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct M1ThresholdEventA2iE_Extensions(Vec<M1ThresholdEventA2iE_Extensions_Item>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct M3ConfigurationiE_Extensions_Item {}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct M3ConfigurationiE_Extensions(Vec<M3ConfigurationiE_Extensions_Item>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct M4ConfigurationiE_Extensions_Item {}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct M4ConfigurationiE_Extensions(Vec<M4ConfigurationiE_Extensions_Item>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct M5ConfigurationiE_Extensions_Item {}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct M5ConfigurationiE_Extensions(Vec<M5ConfigurationiE_Extensions_Item>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct M6ConfigurationiE_Extensions_Item {}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct M6ConfigurationiE_Extensions(Vec<M6ConfigurationiE_Extensions_Item>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct M7ConfigurationiE_Extensions_Item {}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct M7ConfigurationiE_Extensions(Vec<M7ConfigurationiE_Extensions_Item>);

#[derive(Debug, AperCodec)]
#[asn(type = "INTEGER", lb = "0", ub = "255")]
pub struct INTEGER_42(u8);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct MBSFN_ResultToLogInfoiE_Extensions_Item {}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct MBSFN_ResultToLogInfoiE_Extensions(Vec<MBSFN_ResultToLogInfoiE_Extensions_Item>);

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum MDT_ConfigurationiE_Extensions_ItemextensionValue {
    #[asn(key = 178)]
    MDTPLMNList(MDTPLMNList),
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
#[asn(type = "OPEN")]
pub enum MDTMode_Extensionvalue {
    #[asn(key = 197)]
    LoggedMBSFNMDT(LoggedMBSFNMDT),
}

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum MMECPRelocationIndicationprotocolIEs_Itemvalue {
    #[asn(key = 8)]
    ENB_UE_S1AP_ID(ENB_UE_S1AP_ID),
    #[asn(key = 0)]
    MME_UE_S1AP_ID(MME_UE_S1AP_ID),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct MMECPRelocationIndicationprotocolIEs_Item {
    #[asn(key_field = true)]
    pub id: ProtocolIE_ID,
    pub criticality: Criticality,
    pub value: MMECPRelocationIndicationprotocolIEs_Itemvalue,
}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "0",
    sz_ub = "65535"
)]
pub struct MMECPRelocationIndicationprotocolIEs(Vec<MMECPRelocationIndicationprotocolIEs_Item>);

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum MMEConfigurationTransferprotocolIEs_Itemvalue {
    #[asn(key = 295)]
    EN_DCSONConfigurationTransfer(EN_DCSONConfigurationTransfer),
    #[asn(key = 309)]
    IntersystemSONConfigurationTransfer(IntersystemSONConfigurationTransfer),
    #[asn(key = 130)]
    SONConfigurationTransfer(SONConfigurationTransfer),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct MMEConfigurationTransferprotocolIEs_Item {
    #[asn(key_field = true)]
    pub id: ProtocolIE_ID,
    pub criticality: Criticality,
    pub value: MMEConfigurationTransferprotocolIEs_Itemvalue,
}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "0",
    sz_ub = "65535"
)]
pub struct MMEConfigurationTransferprotocolIEs(Vec<MMEConfigurationTransferprotocolIEs_Item>);

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum MMEConfigurationUpdateprotocolIEs_Itemvalue {
    #[asn(key = 61)]
    MMEname(MMEname),
    #[asn(key = 87)]
    RelativeMMECapacity(RelativeMMECapacity),
    #[asn(key = 247)]
    ServedDCNs(ServedDCNs),
    #[asn(key = 105)]
    ServedGUMMEIs(ServedGUMMEIs),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct MMEConfigurationUpdateprotocolIEs_Item {
    #[asn(key_field = true)]
    pub id: ProtocolIE_ID,
    pub criticality: Criticality,
    pub value: MMEConfigurationUpdateprotocolIEs_Itemvalue,
}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "0",
    sz_ub = "65535"
)]
pub struct MMEConfigurationUpdateprotocolIEs(Vec<MMEConfigurationUpdateprotocolIEs_Item>);

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum MMEConfigurationUpdateAcknowledgeprotocolIEs_Itemvalue {
    #[asn(key = 58)]
    CriticalityDiagnostics(CriticalityDiagnostics),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct MMEConfigurationUpdateAcknowledgeprotocolIEs_Item {
    #[asn(key_field = true)]
    pub id: ProtocolIE_ID,
    pub criticality: Criticality,
    pub value: MMEConfigurationUpdateAcknowledgeprotocolIEs_Itemvalue,
}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "0",
    sz_ub = "65535"
)]
pub struct MMEConfigurationUpdateAcknowledgeprotocolIEs(
    Vec<MMEConfigurationUpdateAcknowledgeprotocolIEs_Item>,
);

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum MMEConfigurationUpdateFailureprotocolIEs_Itemvalue {
    #[asn(key = 2)]
    Cause(Cause),
    #[asn(key = 58)]
    CriticalityDiagnostics(CriticalityDiagnostics),
    #[asn(key = 65)]
    TimeToWait(TimeToWait),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct MMEConfigurationUpdateFailureprotocolIEs_Item {
    #[asn(key_field = true)]
    pub id: ProtocolIE_ID,
    pub criticality: Criticality,
    pub value: MMEConfigurationUpdateFailureprotocolIEs_Itemvalue,
}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "0",
    sz_ub = "65535"
)]
pub struct MMEConfigurationUpdateFailureprotocolIEs(
    Vec<MMEConfigurationUpdateFailureprotocolIEs_Item>,
);

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum MMEDirectInformationTransferprotocolIEs_Itemvalue {
    #[asn(key = 122)]
    Inter_SystemInformationTransferType(Inter_SystemInformationTransferType),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct MMEDirectInformationTransferprotocolIEs_Item {
    #[asn(key_field = true)]
    pub id: ProtocolIE_ID,
    pub criticality: Criticality,
    pub value: MMEDirectInformationTransferprotocolIEs_Itemvalue,
}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "0",
    sz_ub = "65535"
)]
pub struct MMEDirectInformationTransferprotocolIEs(
    Vec<MMEDirectInformationTransferprotocolIEs_Item>,
);

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum MMEEarlyStatusTransferprotocolIEs_Itemvalue {
    #[asn(key = 321)]
    ENB_EarlyStatusTransfer_TransparentContainer(ENB_EarlyStatusTransfer_TransparentContainer),
    #[asn(key = 8)]
    ENB_UE_S1AP_ID(ENB_UE_S1AP_ID),
    #[asn(key = 0)]
    MME_UE_S1AP_ID(MME_UE_S1AP_ID),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct MMEEarlyStatusTransferprotocolIEs_Item {
    #[asn(key_field = true)]
    pub id: ProtocolIE_ID,
    pub criticality: Criticality,
    pub value: MMEEarlyStatusTransferprotocolIEs_Itemvalue,
}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "0",
    sz_ub = "65535"
)]
pub struct MMEEarlyStatusTransferprotocolIEs(Vec<MMEEarlyStatusTransferprotocolIEs_Item>);

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum MMEStatusTransferprotocolIEs_Itemvalue {
    #[asn(key = 90)]
    ENB_StatusTransfer_TransparentContainer(ENB_StatusTransfer_TransparentContainer),
    #[asn(key = 8)]
    ENB_UE_S1AP_ID(ENB_UE_S1AP_ID),
    #[asn(key = 0)]
    MME_UE_S1AP_ID(MME_UE_S1AP_ID),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct MMEStatusTransferprotocolIEs_Item {
    #[asn(key_field = true)]
    pub id: ProtocolIE_ID,
    pub criticality: Criticality,
    pub value: MMEStatusTransferprotocolIEs_Itemvalue,
}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "0",
    sz_ub = "65535"
)]
pub struct MMEStatusTransferprotocolIEs(Vec<MMEStatusTransferprotocolIEs_Item>);

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
pub struct MutingPatternInformationiE_Extensions_Item {}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct MutingPatternInformationiE_Extensions(Vec<MutingPatternInformationiE_Extensions_Item>);

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum NASDeliveryIndicationprotocolIEs_Itemvalue {
    #[asn(key = 8)]
    ENB_UE_S1AP_ID(ENB_UE_S1AP_ID),
    #[asn(key = 0)]
    MME_UE_S1AP_ID(MME_UE_S1AP_ID),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct NASDeliveryIndicationprotocolIEs_Item {
    #[asn(key_field = true)]
    pub id: ProtocolIE_ID,
    pub criticality: Criticality,
    pub value: NASDeliveryIndicationprotocolIEs_Itemvalue,
}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "0",
    sz_ub = "65535"
)]
pub struct NASDeliveryIndicationprotocolIEs(Vec<NASDeliveryIndicationprotocolIEs_Item>);

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum NASNonDeliveryIndicationprotocolIEs_Itemvalue {
    #[asn(key = 2)]
    Cause(Cause),
    #[asn(key = 8)]
    ENB_UE_S1AP_ID(ENB_UE_S1AP_ID),
    #[asn(key = 0)]
    MME_UE_S1AP_ID(MME_UE_S1AP_ID),
    #[asn(key = 26)]
    NAS_PDU(NAS_PDU),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct NASNonDeliveryIndicationprotocolIEs_Item {
    #[asn(key_field = true)]
    pub id: ProtocolIE_ID,
    pub criticality: Criticality,
    pub value: NASNonDeliveryIndicationprotocolIEs_Itemvalue,
}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "0",
    sz_ub = "65535"
)]
pub struct NASNonDeliveryIndicationprotocolIEs(Vec<NASNonDeliveryIndicationprotocolIEs_Item>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct NB_IoT_Paging_eDRXInformationiE_Extensions_Item {}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct NB_IoT_Paging_eDRXInformationiE_Extensions(
    Vec<NB_IoT_Paging_eDRXInformationiE_Extensions_Item>,
);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct NG_eNBiE_Extensions_Item {}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct NG_eNBiE_Extensions(Vec<NG_eNBiE_Extensions_Item>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct NR_CGIiE_Extensions_Item {}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct NR_CGIiE_Extensions(Vec<NR_CGIiE_Extensions_Item>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct NRUESecurityCapabilitiesiE_Extensions_Item {}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct NRUESecurityCapabilitiesiE_Extensions(Vec<NRUESecurityCapabilitiesiE_Extensions_Item>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct NRUESidelinkAggregateMaximumBitrateiE_Extensions_Item {}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct NRUESidelinkAggregateMaximumBitrateiE_Extensions(
    Vec<NRUESidelinkAggregateMaximumBitrateiE_Extensions_Item>,
);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct NRV2XServicesAuthorizediE_Extensions_Item {}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct NRV2XServicesAuthorizediE_Extensions(Vec<NRV2XServicesAuthorizediE_Extensions_Item>);

#[derive(Debug, AperCodec)]
#[asn(type = "OCTET-STRING")]
pub struct OCTET_STRING_47(Vec<u8>);

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum OverloadStartprotocolIEs_Itemvalue {
    #[asn(key = 154)]
    GUMMEIList(GUMMEIList),
    #[asn(key = 101)]
    OverloadResponse(OverloadResponse),
    #[asn(key = 161)]
    TrafficLoadReductionIndication(TrafficLoadReductionIndication),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct OverloadStartprotocolIEs_Item {
    #[asn(key_field = true)]
    pub id: ProtocolIE_ID,
    pub criticality: Criticality,
    pub value: OverloadStartprotocolIEs_Itemvalue,
}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "0",
    sz_ub = "65535"
)]
pub struct OverloadStartprotocolIEs(Vec<OverloadStartprotocolIEs_Item>);

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum OverloadStopprotocolIEs_Itemvalue {
    #[asn(key = 154)]
    GUMMEIList(GUMMEIList),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct OverloadStopprotocolIEs_Item {
    #[asn(key_field = true)]
    pub id: ProtocolIE_ID,
    pub criticality: Criticality,
    pub value: OverloadStopprotocolIEs_Itemvalue,
}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "0",
    sz_ub = "65535"
)]
pub struct OverloadStopprotocolIEs(Vec<OverloadStopprotocolIEs_Item>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct PC5FlowBitRatesiE_Extensions_Item {}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct PC5FlowBitRatesiE_Extensions(Vec<PC5FlowBitRatesiE_Extensions_Item>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct PC5QoSFlowItemiE_Extensions_Item {}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct PC5QoSFlowItemiE_Extensions(Vec<PC5QoSFlowItemiE_Extensions_Item>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct PC5QoSParametersiE_Extensions_Item {}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct PC5QoSParametersiE_Extensions(Vec<PC5QoSParametersiE_Extensions_Item>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct PLMNAreaBasedQMCiE_Extensions_Item {}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct PLMNAreaBasedQMCiE_Extensions(Vec<PLMNAreaBasedQMCiE_Extensions_Item>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct PSCellInformationiE_Extensions_Item {}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct PSCellInformationiE_Extensions(Vec<PSCellInformationiE_Extensions_Item>);

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum PWSFailureIndicationprotocolIEs_Itemvalue {
    #[asn(key = 59)]
    Global_ENB_ID(Global_ENB_ID),
    #[asn(key = 222)]
    PWSfailedECGIList(PWSfailedECGIList),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct PWSFailureIndicationprotocolIEs_Item {
    #[asn(key_field = true)]
    pub id: ProtocolIE_ID,
    pub criticality: Criticality,
    pub value: PWSFailureIndicationprotocolIEs_Itemvalue,
}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "0",
    sz_ub = "65535"
)]
pub struct PWSFailureIndicationprotocolIEs(Vec<PWSFailureIndicationprotocolIEs_Item>);

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum PWSRestartIndicationprotocolIEs_Itemvalue {
    #[asn(key = 182)]
    ECGIListForRestart(ECGIListForRestart),
    #[asn(key = 190)]
    EmergencyAreaIDListForRestart(EmergencyAreaIDListForRestart),
    #[asn(key = 59)]
    Global_ENB_ID(Global_ENB_ID),
    #[asn(key = 188)]
    TAIListForRestart(TAIListForRestart),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct PWSRestartIndicationprotocolIEs_Item {
    #[asn(key_field = true)]
    pub id: ProtocolIE_ID,
    pub criticality: Criticality,
    pub value: PWSRestartIndicationprotocolIEs_Itemvalue,
}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "0",
    sz_ub = "65535"
)]
pub struct PWSRestartIndicationprotocolIEs(Vec<PWSRestartIndicationprotocolIEs_Item>);

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum PagingprotocolIEs_Itemvalue {
    #[asn(key = 211)]
    AssistanceDataForPaging(AssistanceDataForPaging),
    #[asn(key = 271)]
    CE_ModeBRestricted(CE_ModeBRestricted),
    #[asn(key = 109)]
    CNDomain(CNDomain),
    #[asn(key = 128)]
    CSG_IdList(CSG_IdList),
    #[asn(key = 304)]
    DataSize(DataSize),
    #[asn(key = 251)]
    EnhancedCoverageRestricted(EnhancedCoverageRestricted),
    #[asn(key = 231)]
    Extended_UEIdentityIndexValue(Extended_UEIdentityIndexValue),
    #[asn(key = 239)]
    NB_IoT_Paging_eDRXInformation(NB_IoT_Paging_eDRXInformation),
    #[asn(key = 324)]
    NB_IoT_PagingDRX(NB_IoT_PagingDRX),
    #[asn(key = 244)]
    NB_IoT_UEIdentityIndexValue(NB_IoT_UEIdentityIndexValue),
    #[asn(key = 227)]
    Paging_eDRXInformation(Paging_eDRXInformation),
    #[asn(key = 44)]
    PagingDRX(PagingDRX),
    #[asn(key = 151)]
    PagingPriority(PagingPriority),
    #[asn(key = 46)]
    TAIList(TAIList),
    #[asn(key = 80)]
    UEIdentityIndexValue(UEIdentityIndexValue),
    #[asn(key = 43)]
    UEPagingID(UEPagingID),
    #[asn(key = 198)]
    UERadioCapabilityForPaging(UERadioCapabilityForPaging),
    #[asn(key = 323)]
    WUS_Assistance_Information(WUS_Assistance_Information),
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
#[asn(type = "SEQUENCE", extensible = false)]
pub struct Paging_eDRXInformationiE_Extensions_Item {}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct Paging_eDRXInformationiE_Extensions(Vec<Paging_eDRXInformationiE_Extensions_Item>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct PagingAttemptInformationiE_Extensions_Item {}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct PagingAttemptInformationiE_Extensions(Vec<PagingAttemptInformationiE_Extensions_Item>);

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum PathSwitchRequestprotocolIEs_Itemvalue {
    #[asn(key = 127)]
    CSG_Id(CSG_Id),
    #[asn(key = 146)]
    CSGMembershipStatus(CSGMembershipStatus),
    #[asn(key = 145)]
    CellAccessMode(CellAccessMode),
    #[asn(key = 22)]
    E_RABToBeSwitchedDLList(E_RABToBeSwitchedDLList),
    #[asn(key = 8)]
    ENB_UE_S1AP_ID(ENB_UE_S1AP_ID),
    #[asn(key = 100)]
    EUTRAN_CGI(EUTRAN_CGI),
    #[asn(key = 157)]
    GUMMEI(GUMMEI),
    #[asn(key = 186)]
    LHN_ID(LHN_ID),
    #[asn(key = 88)]
    MME_UE_S1AP_ID(MME_UE_S1AP_ID),
    #[asn(key = 269)]
    NRUESecurityCapabilities(NRUESecurityCapabilities),
    #[asn(key = 288)]
    PSCellInformation(PSCellInformation),
    #[asn(key = 245)]
    RRC_Establishment_Cause(RRC_Establishment_Cause),
    #[asn(key = 67)]
    TAI(TAI),
    #[asn(key = 176)]
    TunnelInformation(TunnelInformation),
    #[asn(key = 107)]
    UESecurityCapabilities(UESecurityCapabilities),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct PathSwitchRequestprotocolIEs_Item {
    #[asn(key_field = true)]
    pub id: ProtocolIE_ID,
    pub criticality: Criticality,
    pub value: PathSwitchRequestprotocolIEs_Itemvalue,
}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "0",
    sz_ub = "65535"
)]
pub struct PathSwitchRequestprotocolIEs(Vec<PathSwitchRequestprotocolIEs_Item>);

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum PathSwitchRequestAcknowledgeprotocolIEs_Itemvalue {
    #[asn(key = 299)]
    AdditionalRRMPriorityIndex(AdditionalRRMPriorityIndex),
    #[asn(key = 277)]
    AerialUEsubscriptionInformation(AerialUEsubscriptionInformation),
    #[asn(key = 271)]
    CE_ModeBRestricted(CE_ModeBRestricted),
    #[asn(key = 146)]
    CSGMembershipStatus(CSGMembershipStatus),
    #[asn(key = 58)]
    CriticalityDiagnostics(CriticalityDiagnostics),
    #[asn(key = 33)]
    E_RABList(E_RABList),
    #[asn(key = 95)]
    E_RABToBeSwitchedULList(E_RABToBeSwitchedULList),
    #[asn(key = 8)]
    ENB_UE_S1AP_ID(ENB_UE_S1AP_ID),
    #[asn(key = 251)]
    EnhancedCoverageRestricted(EnhancedCoverageRestricted),
    #[asn(key = 41)]
    HandoverRestrictionList(HandoverRestrictionList),
    #[asn(key = 0)]
    MME_UE_S1AP_ID(MME_UE_S1AP_ID),
    #[asn(key = 269)]
    NRUESecurityCapabilities(NRUESecurityCapabilities),
    #[asn(key = 307)]
    NRUESidelinkAggregateMaximumBitrate(NRUESidelinkAggregateMaximumBitrate),
    #[asn(key = 306)]
    NRV2XServicesAuthorized(NRV2XServicesAuthorized),
    #[asn(key = 308)]
    PC5QoSParameters(PC5QoSParameters),
    #[asn(key = 283)]
    PendingDataIndication(PendingDataIndication),
    #[asn(key = 195)]
    ProSeAuthorized(ProSeAuthorized),
    #[asn(key = 40)]
    SecurityContext(SecurityContext),
    #[asn(key = 278)]
    Subscription_Based_UE_DifferentiationInfo(Subscription_Based_UE_DifferentiationInfo),
    #[asn(key = 66)]
    UEAggregateMaximumBitrate(UEAggregateMaximumBitrate),
    #[asn(key = 314)]
    UERadioCapabilityID(UERadioCapabilityID),
    #[asn(key = 248)]
    UESidelinkAggregateMaximumBitrate(UESidelinkAggregateMaximumBitrate),
    #[asn(key = 241)]
    UEUserPlaneCIoTSupportIndicator(UEUserPlaneCIoTSupportIndicator),
    #[asn(key = 240)]
    V2XServicesAuthorized(V2XServicesAuthorized),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct PathSwitchRequestAcknowledgeprotocolIEs_Item {
    #[asn(key_field = true)]
    pub id: ProtocolIE_ID,
    pub criticality: Criticality,
    pub value: PathSwitchRequestAcknowledgeprotocolIEs_Itemvalue,
}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "0",
    sz_ub = "65535"
)]
pub struct PathSwitchRequestAcknowledgeprotocolIEs(
    Vec<PathSwitchRequestAcknowledgeprotocolIEs_Item>,
);

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum PathSwitchRequestFailureprotocolIEs_Itemvalue {
    #[asn(key = 2)]
    Cause(Cause),
    #[asn(key = 58)]
    CriticalityDiagnostics(CriticalityDiagnostics),
    #[asn(key = 8)]
    ENB_UE_S1AP_ID(ENB_UE_S1AP_ID),
    #[asn(key = 0)]
    MME_UE_S1AP_ID(MME_UE_S1AP_ID),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct PathSwitchRequestFailureprotocolIEs_Item {
    #[asn(key_field = true)]
    pub id: ProtocolIE_ID,
    pub criticality: Criticality,
    pub value: PathSwitchRequestFailureprotocolIEs_Itemvalue,
}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "0",
    sz_ub = "65535"
)]
pub struct PathSwitchRequestFailureprotocolIEs(Vec<PathSwitchRequestFailureprotocolIEs_Item>);

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
pub enum ProSeAuthorizediE_Extensions_ItemextensionValue {
    #[asn(key = 216)]
    ProSeUEtoNetworkRelaying(ProSeUEtoNetworkRelaying),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct ProSeAuthorizediE_Extensions_Item {
    #[asn(key_field = true)]
    pub id: ProtocolExtensionID,
    pub criticality: Criticality,
    pub extension_value: ProSeAuthorizediE_Extensions_ItemextensionValue,
}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct ProSeAuthorizediE_Extensions(Vec<ProSeAuthorizediE_Extensions_Item>);

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
pub struct RIMTransferiE_Extensions_Item {}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct RIMTransferiE_Extensions(Vec<RIMTransferiE_Extensions_Item>);

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum RLFReportInformationiE_Extensions_ItemextensionValue {
    #[asn(key = 313)]
    NB_IoT_RLF_Report_Container(NB_IoT_RLF_Report_Container),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct RLFReportInformationiE_Extensions_Item {
    #[asn(key_field = true)]
    pub id: ProtocolExtensionID,
    pub criticality: Criticality,
    pub extension_value: RLFReportInformationiE_Extensions_ItemextensionValue,
}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct RLFReportInformationiE_Extensions(Vec<RLFReportInformationiE_Extensions_Item>);

#[derive(Debug, AperCodec)]
#[asn(type = "INTEGER", lb = "0", ub = "4095")]
pub struct INTEGER_51(u16);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct RecommendedCellItemiE_Extensions_Item {}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct RecommendedCellItemiE_Extensions(Vec<RecommendedCellItemiE_Extensions_Item>);

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum RecommendedCellList_Itemvalue {
    #[asn(key = 214)]
    RecommendedCellItem(RecommendedCellItem),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct RecommendedCellList_Item {
    #[asn(key_field = true)]
    pub id: ProtocolIE_ID,
    pub criticality: Criticality,
    pub value: RecommendedCellList_Itemvalue,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct RecommendedCellsForPagingiE_Extensions_Item {}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct RecommendedCellsForPagingiE_Extensions(Vec<RecommendedCellsForPagingiE_Extensions_Item>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct RecommendedENBItemiE_Extensions_Item {}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct RecommendedENBItemiE_Extensions(Vec<RecommendedENBItemiE_Extensions_Item>);

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum RecommendedENBList_Itemvalue {
    #[asn(key = 215)]
    RecommendedENBItem(RecommendedENBItem),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct RecommendedENBList_Item {
    #[asn(key_field = true)]
    pub id: ProtocolIE_ID,
    pub criticality: Criticality,
    pub value: RecommendedENBList_Itemvalue,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct RecommendedENBsForPagingiE_Extensions_Item {}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct RecommendedENBsForPagingiE_Extensions(Vec<RecommendedENBsForPagingiE_Extensions_Item>);

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum RequestTypeiE_Extensions_ItemextensionValue {
    #[asn(key = 298)]
    RequestTypeAdditionalInfo(RequestTypeAdditionalInfo),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct RequestTypeiE_Extensions_Item {
    #[asn(key_field = true)]
    pub id: ProtocolExtensionID,
    pub criticality: Criticality,
    pub extension_value: RequestTypeiE_Extensions_ItemextensionValue,
}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct RequestTypeiE_Extensions(Vec<RequestTypeiE_Extensions_Item>);

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum RerouteNASRequestprotocolIEs_Itemvalue {
    #[asn(key = 224)]
    Additional_GUTI(Additional_GUTI),
    #[asn(key = 8)]
    ENB_UE_S1AP_ID(ENB_UE_S1AP_ID),
    #[asn(key = 223)]
    MME_Group_ID(MME_Group_ID),
    #[asn(key = 0)]
    MME_UE_S1AP_ID(MME_UE_S1AP_ID),
    #[asn(key = 230)]
    UE_Usage_Type(UE_Usage_Type),
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
#[asn(type = "OPEN")]
pub enum ResetprotocolIEs_Itemvalue {
    #[asn(key = 2)]
    Cause(Cause),
    #[asn(key = 92)]
    ResetType(ResetType),
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
pub enum ResetAcknowledgeprotocolIEs_Itemvalue {
    #[asn(key = 58)]
    CriticalityDiagnostics(CriticalityDiagnostics),
    #[asn(key = 93)]
    UE_associatedLogicalS1_ConnectionListResAck(UE_associatedLogicalS1_ConnectionListResAck),
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
pub enum RetrieveUEInformationprotocolIEs_Itemvalue {
    #[asn(key = 96)]
    S_TMSI(S_TMSI),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct RetrieveUEInformationprotocolIEs_Item {
    #[asn(key_field = true)]
    pub id: ProtocolIE_ID,
    pub criticality: Criticality,
    pub value: RetrieveUEInformationprotocolIEs_Itemvalue,
}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "0",
    sz_ub = "65535"
)]
pub struct RetrieveUEInformationprotocolIEs(Vec<RetrieveUEInformationprotocolIEs_Item>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct S_TMSIiE_Extensions_Item {}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct S_TMSIiE_Extensions(Vec<S_TMSIiE_Extensions_Item>);

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum S1SetupFailureprotocolIEs_Itemvalue {
    #[asn(key = 2)]
    Cause(Cause),
    #[asn(key = 58)]
    CriticalityDiagnostics(CriticalityDiagnostics),
    #[asn(key = 65)]
    TimeToWait(TimeToWait),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct S1SetupFailureprotocolIEs_Item {
    #[asn(key_field = true)]
    pub id: ProtocolIE_ID,
    pub criticality: Criticality,
    pub value: S1SetupFailureprotocolIEs_Itemvalue,
}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "0",
    sz_ub = "65535"
)]
pub struct S1SetupFailureprotocolIEs(Vec<S1SetupFailureprotocolIEs_Item>);

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum S1SetupRequestprotocolIEs_Itemvalue {
    #[asn(key = 128)]
    CSG_IdList(CSG_IdList),
    #[asn(key = 291)]
    ConnectedengNBList(ConnectedengNBList),
    #[asn(key = 60)]
    ENBname(ENBname),
    #[asn(key = 59)]
    Global_ENB_ID(Global_ENB_ID),
    #[asn(key = 234)]
    NB_IoT_DefaultPagingDRX(NB_IoT_DefaultPagingDRX),
    #[asn(key = 137)]
    PagingDRX(PagingDRX),
    #[asn(key = 64)]
    SupportedTAs(SupportedTAs),
    #[asn(key = 228)]
    UE_RetentionInformation(UE_RetentionInformation),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct S1SetupRequestprotocolIEs_Item {
    #[asn(key_field = true)]
    pub id: ProtocolIE_ID,
    pub criticality: Criticality,
    pub value: S1SetupRequestprotocolIEs_Itemvalue,
}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "0",
    sz_ub = "65535"
)]
pub struct S1SetupRequestprotocolIEs(Vec<S1SetupRequestprotocolIEs_Item>);

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum S1SetupResponseprotocolIEs_Itemvalue {
    #[asn(key = 58)]
    CriticalityDiagnostics(CriticalityDiagnostics),
    #[asn(key = 303)]
    IAB_Supported(IAB_Supported),
    #[asn(key = 163)]
    MMERelaySupportIndicator(MMERelaySupportIndicator),
    #[asn(key = 61)]
    MMEname(MMEname),
    #[asn(key = 87)]
    RelativeMMECapacity(RelativeMMECapacity),
    #[asn(key = 247)]
    ServedDCNs(ServedDCNs),
    #[asn(key = 105)]
    ServedGUMMEIs(ServedGUMMEIs),
    #[asn(key = 228)]
    UE_RetentionInformation(UE_RetentionInformation),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct S1SetupResponseprotocolIEs_Item {
    #[asn(key_field = true)]
    pub id: ProtocolIE_ID,
    pub criticality: Criticality,
    pub value: S1SetupResponseprotocolIEs_Itemvalue,
}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "0",
    sz_ub = "65535"
)]
pub struct S1SetupResponseprotocolIEs(Vec<S1SetupResponseprotocolIEs_Item>);

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum SONConfigurationTransferiE_Extensions_ItemextensionValue {
    #[asn(key = 209)]
    SynchronisationInformation(SynchronisationInformation),
    #[asn(key = 152)]
    X2TNLConfigurationInfo(X2TNLConfigurationInfo),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct SONConfigurationTransferiE_Extensions_Item {
    #[asn(key_field = true)]
    pub id: ProtocolExtensionID,
    pub criticality: Criticality,
    pub extension_value: SONConfigurationTransferiE_Extensions_ItemextensionValue,
}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct SONConfigurationTransferiE_Extensions(Vec<SONConfigurationTransferiE_Extensions_Item>);

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum SONInformation_Extensionvalue {
    #[asn(key = 206)]
    SONInformationReport(SONInformationReport),
}

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum SONInformationReplyiE_Extensions_ItemextensionValue {
    #[asn(key = 208)]
    MutingPatternInformation(MutingPatternInformation),
    #[asn(key = 149)]
    TimeSynchronisationInfo(TimeSynchronisationInfo),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct SONInformationReplyiE_Extensions_Item {
    #[asn(key_field = true)]
    pub id: ProtocolExtensionID,
    pub criticality: Criticality,
    pub extension_value: SONInformationReplyiE_Extensions_ItemextensionValue,
}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct SONInformationReplyiE_Extensions(Vec<SONInformationReplyiE_Extensions_Item>);

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
pub struct ScheduledCommunicationTimeiE_Extensions_Item {}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct ScheduledCommunicationTimeiE_Extensions(
    Vec<ScheduledCommunicationTimeiE_Extensions_Item>,
);

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum SecondaryRATDataUsageReportprotocolIEs_Itemvalue {
    #[asn(key = 8)]
    ENB_UE_S1AP_ID(ENB_UE_S1AP_ID),
    #[asn(key = 266)]
    HandoverFlag(HandoverFlag),
    #[asn(key = 0)]
    MME_UE_S1AP_ID(MME_UE_S1AP_ID),
    #[asn(key = 264)]
    SecondaryRATDataUsageReportList(SecondaryRATDataUsageReportList),
    #[asn(key = 297)]
    TimeSinceSecondaryNodeRelease(TimeSinceSecondaryNodeRelease),
    #[asn(key = 189)]
    UserLocationInformation(UserLocationInformation),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct SecondaryRATDataUsageReportprotocolIEs_Item {
    #[asn(key_field = true)]
    pub id: ProtocolIE_ID,
    pub criticality: Criticality,
    pub value: SecondaryRATDataUsageReportprotocolIEs_Itemvalue,
}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "0",
    sz_ub = "65535"
)]
pub struct SecondaryRATDataUsageReportprotocolIEs(Vec<SecondaryRATDataUsageReportprotocolIEs_Item>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct SecondaryRATDataUsageReportItemiE_Extensions_Item {}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct SecondaryRATDataUsageReportItemiE_Extensions(
    Vec<SecondaryRATDataUsageReportItemiE_Extensions_Item>,
);

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum SecondaryRATDataUsageReportList_Itemvalue {
    #[asn(key = 265)]
    SecondaryRATDataUsageReportItem(SecondaryRATDataUsageReportItem),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct SecondaryRATDataUsageReportList_Item {
    #[asn(key_field = true)]
    pub id: ProtocolIE_ID,
    pub criticality: Criticality,
    pub value: SecondaryRATDataUsageReportList_Itemvalue,
}

#[derive(Debug, AperCodec)]
#[asn(type = "INTEGER", lb = "0", ub = "7")]
pub struct INTEGER_59(u8);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct SecurityContextiE_Extensions_Item {}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct SecurityContextiE_Extensions(Vec<SecurityContextiE_Extensions_Item>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct ServedDCNsItemiE_Extensions_Item {}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct ServedDCNsItemiE_Extensions(Vec<ServedDCNsItemiE_Extensions_Item>);

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum ServedGUMMEIsItemiE_Extensions_ItemextensionValue {
    #[asn(key = 170)]
    GUMMEIType(GUMMEIType),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct ServedGUMMEIsItemiE_Extensions_Item {
    #[asn(key_field = true)]
    pub id: ProtocolExtensionID,
    pub criticality: Criticality,
    pub extension_value: ServedGUMMEIsItemiE_Extensions_ItemextensionValue,
}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct ServedGUMMEIsItemiE_Extensions(Vec<ServedGUMMEIsItemiE_Extensions_Item>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct SourceNgRanNode_IDiE_Extensions_Item {}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct SourceNgRanNode_IDiE_Extensions(Vec<SourceNgRanNode_IDiE_Extensions_Item>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct SourceeNB_IDiE_Extensions_Item {}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct SourceeNB_IDiE_Extensions(Vec<SourceeNB_IDiE_Extensions_Item>);

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum SourceeNB_ToTargeteNB_TransparentContaineriE_Extensions_ItemextensionValue {
    #[asn(key = 299)]
    AdditionalRRMPriorityIndex(AdditionalRRMPriorityIndex),
    #[asn(key = 300)]
    ContextatSource(ContextatSource),
    #[asn(key = 326)]
    EmergencyIndicator(EmergencyIndicator),
    #[asn(key = 296)]
    IMSvoiceEPSfallbackfrom5G(IMSvoiceEPSfallbackfrom5G),
    #[asn(key = 311)]
    IntersystemMeasurementConfiguration(IntersystemMeasurementConfiguration),
    #[asn(key = 175)]
    MobilityInformation(MobilityInformation),
    #[asn(key = 312)]
    SourceNodeID(SourceNodeID),
    #[asn(key = 194)]
    UE_HistoryInformationFromTheUE(UE_HistoryInformationFromTheUE),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct SourceeNB_ToTargeteNB_TransparentContaineriE_Extensions_Item {
    #[asn(key_field = true)]
    pub id: ProtocolExtensionID,
    pub criticality: Criticality,
    pub extension_value: SourceeNB_ToTargeteNB_TransparentContaineriE_Extensions_ItemextensionValue,
}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct SourceeNB_ToTargeteNB_TransparentContaineriE_Extensions(
    Vec<SourceeNB_ToTargeteNB_TransparentContaineriE_Extensions_Item>,
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
pub struct Subscription_Based_UE_DifferentiationInfoiE_Extensions_Item {}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct Subscription_Based_UE_DifferentiationInfoiE_Extensions(
    Vec<Subscription_Based_UE_DifferentiationInfoiE_Extensions_Item>,
);

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum SuccessfulOutcomevalue {
    #[asn(key = 50)]
    E_RABModificationConfirm(E_RABModificationConfirm),
    #[asn(key = 6)]
    E_RABModifyResponse(E_RABModifyResponse),
    #[asn(key = 7)]
    E_RABReleaseResponse(E_RABReleaseResponse),
    #[asn(key = 5)]
    E_RABSetupResponse(E_RABSetupResponse),
    #[asn(key = 29)]
    ENBConfigurationUpdateAcknowledge(ENBConfigurationUpdateAcknowledge),
    #[asn(key = 4)]
    HandoverCancelAcknowledge(HandoverCancelAcknowledge),
    #[asn(key = 0)]
    HandoverCommand(HandoverCommand),
    #[asn(key = 1)]
    HandoverRequestAcknowledge(HandoverRequestAcknowledge),
    #[asn(key = 9)]
    InitialContextSetupResponse(InitialContextSetupResponse),
    #[asn(key = 43)]
    KillResponse(KillResponse),
    #[asn(key = 30)]
    MMEConfigurationUpdateAcknowledge(MMEConfigurationUpdateAcknowledge),
    #[asn(key = 3)]
    PathSwitchRequestAcknowledge(PathSwitchRequestAcknowledge),
    #[asn(key = 14)]
    ResetAcknowledge(ResetAcknowledge),
    #[asn(key = 17)]
    S1SetupResponse(S1SetupResponse),
    #[asn(key = 53)]
    UEContextModificationConfirm(UEContextModificationConfirm),
    #[asn(key = 21)]
    UEContextModificationResponse(UEContextModificationResponse),
    #[asn(key = 23)]
    UEContextReleaseComplete(UEContextReleaseComplete),
    #[asn(key = 56)]
    UEContextResumeResponse(UEContextResumeResponse),
    #[asn(key = 55)]
    UEContextSuspendResponse(UEContextSuspendResponse),
    #[asn(key = 63)]
    UERadioCapabilityIDMappingResponse(UERadioCapabilityIDMappingResponse),
    #[asn(key = 48)]
    UERadioCapabilityMatchResponse(UERadioCapabilityMatchResponse),
    #[asn(key = 36)]
    WriteReplaceWarningResponse(WriteReplaceWarningResponse),
}

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum SupportedTAs_ItemiE_Extensions_ItemextensionValue {
    #[asn(key = 232)]
    RAT_Type(RAT_Type),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct SupportedTAs_ItemiE_Extensions_Item {
    #[asn(key_field = true)]
    pub id: ProtocolExtensionID,
    pub criticality: Criticality,
    pub extension_value: SupportedTAs_ItemiE_Extensions_ItemextensionValue,
}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct SupportedTAs_ItemiE_Extensions(Vec<SupportedTAs_ItemiE_Extensions_Item>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct SynchronisationInformationiE_Extensions_Item {}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct SynchronisationInformationiE_Extensions(
    Vec<SynchronisationInformationiE_Extensions_Item>,
);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct TABasedMDTiE_Extensions_Item {}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct TABasedMDTiE_Extensions(Vec<TABasedMDTiE_Extensions_Item>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct TABasedQMCiE_Extensions_Item {}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct TABasedQMCiE_Extensions(Vec<TABasedQMCiE_Extensions_Item>);

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
#[asn(type = "SEQUENCE", extensible = false)]
pub struct TAI_Broadcast_ItemiE_Extensions_Item {}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct TAI_Broadcast_ItemiE_Extensions(Vec<TAI_Broadcast_ItemiE_Extensions_Item>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct TAI_Cancelled_ItemiE_Extensions_Item {}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct TAI_Cancelled_ItemiE_Extensions(Vec<TAI_Cancelled_ItemiE_Extensions_Item>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct TAIBasedMDTiE_Extensions_Item {}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct TAIBasedMDTiE_Extensions(Vec<TAIBasedMDTiE_Extensions_Item>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct TAIBasedQMCiE_Extensions_Item {}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct TAIBasedQMCiE_Extensions(Vec<TAIBasedQMCiE_Extensions_Item>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct TAIItemiE_Extensions_Item {}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct TAIItemiE_Extensions(Vec<TAIItemiE_Extensions_Item>);

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum TAIList_Itemvalue {
    #[asn(key = 47)]
    TAIItem(TAIItem),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct TAIList_Item {
    #[asn(key_field = true)]
    pub id: ProtocolIE_ID,
    pub criticality: Criticality,
    pub value: TAIList_Itemvalue,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct TargetNgRanNode_IDiE_Extensions_Item {}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct TargetNgRanNode_IDiE_Extensions(Vec<TargetNgRanNode_IDiE_Extensions_Item>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct TargetRNC_IDiE_Extensions_Item {}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct TargetRNC_IDiE_Extensions(Vec<TargetRNC_IDiE_Extensions_Item>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct TargeteNB_IDiE_Extensions_Item {}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct TargeteNB_IDiE_Extensions(Vec<TargeteNB_IDiE_Extensions_Item>);

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum TargeteNB_ToSourceeNB_TransparentContaineriE_Extensions_ItemextensionValue {
    #[asn(key = 318)]
    DAPSResponseInfoList(DAPSResponseInfoList),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct TargeteNB_ToSourceeNB_TransparentContaineriE_Extensions_Item {
    #[asn(key_field = true)]
    pub id: ProtocolExtensionID,
    pub criticality: Criticality,
    pub extension_value: TargeteNB_ToSourceeNB_TransparentContaineriE_Extensions_ItemextensionValue,
}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct TargeteNB_ToSourceeNB_TransparentContaineriE_Extensions(
    Vec<TargeteNB_ToSourceeNB_TransparentContaineriE_Extensions_Item>,
);

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum TimeSynchronisationInfoiE_Extensions_ItemextensionValue {
    #[asn(key = 207)]
    MutingAvailabilityIndication(MutingAvailabilityIndication),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct TimeSynchronisationInfoiE_Extensions_Item {
    #[asn(key_field = true)]
    pub id: ProtocolExtensionID,
    pub criticality: Criticality,
    pub extension_value: TimeSynchronisationInfoiE_Extensions_ItemextensionValue,
}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct TimeSynchronisationInfoiE_Extensions(Vec<TimeSynchronisationInfoiE_Extensions_Item>);

#[derive(Debug, AperCodec)]
#[asn(type = "OCTET-STRING")]
pub struct OCTET_STRING_65(Vec<u8>);

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum TraceActivationiE_Extensions_ItemextensionValue {
    #[asn(key = 162)]
    MDT_Configuration(MDT_Configuration),
    #[asn(key = 316)]
    MDT_ConfigurationNR(MDT_ConfigurationNR),
    #[asn(key = 262)]
    UEAppLayerMeasConfig(UEAppLayerMeasConfig),
    #[asn(key = 325)]
    URI_Address(URI_Address),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct TraceActivationiE_Extensions_Item {
    #[asn(key_field = true)]
    pub id: ProtocolExtensionID,
    pub criticality: Criticality,
    pub extension_value: TraceActivationiE_Extensions_ItemextensionValue,
}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct TraceActivationiE_Extensions(Vec<TraceActivationiE_Extensions_Item>);

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum TraceFailureIndicationprotocolIEs_Itemvalue {
    #[asn(key = 2)]
    Cause(Cause),
    #[asn(key = 86)]
    E_UTRAN_Trace_ID(E_UTRAN_Trace_ID),
    #[asn(key = 8)]
    ENB_UE_S1AP_ID(ENB_UE_S1AP_ID),
    #[asn(key = 0)]
    MME_UE_S1AP_ID(MME_UE_S1AP_ID),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct TraceFailureIndicationprotocolIEs_Item {
    #[asn(key_field = true)]
    pub id: ProtocolIE_ID,
    pub criticality: Criticality,
    pub value: TraceFailureIndicationprotocolIEs_Itemvalue,
}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "0",
    sz_ub = "65535"
)]
pub struct TraceFailureIndicationprotocolIEs(Vec<TraceFailureIndicationprotocolIEs_Item>);

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum TraceStartprotocolIEs_Itemvalue {
    #[asn(key = 8)]
    ENB_UE_S1AP_ID(ENB_UE_S1AP_ID),
    #[asn(key = 0)]
    MME_UE_S1AP_ID(MME_UE_S1AP_ID),
    #[asn(key = 25)]
    TraceActivation(TraceActivation),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct TraceStartprotocolIEs_Item {
    #[asn(key_field = true)]
    pub id: ProtocolIE_ID,
    pub criticality: Criticality,
    pub value: TraceStartprotocolIEs_Itemvalue,
}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "0",
    sz_ub = "65535"
)]
pub struct TraceStartprotocolIEs(Vec<TraceStartprotocolIEs_Item>);

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
#[asn(type = "SEQUENCE", extensible = false)]
pub struct UE_S1AP_ID_pairiE_Extensions_Item {}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct UE_S1AP_ID_pairiE_Extensions(Vec<UE_S1AP_ID_pairiE_Extensions_Item>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct UE_associatedLogicalS1_ConnectionItemiE_Extensions_Item {}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct UE_associatedLogicalS1_ConnectionItemiE_Extensions(
    Vec<UE_associatedLogicalS1_ConnectionItemiE_Extensions_Item>,
);

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum UE_associatedLogicalS1_ConnectionListRes_Itemvalue {
    #[asn(key = 91)]
    UE_associatedLogicalS1_ConnectionItem(UE_associatedLogicalS1_ConnectionItem),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct UE_associatedLogicalS1_ConnectionListRes_Item {
    #[asn(key_field = true)]
    pub id: ProtocolIE_ID,
    pub criticality: Criticality,
    pub value: UE_associatedLogicalS1_ConnectionListRes_Itemvalue,
}

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum UE_associatedLogicalS1_ConnectionListResAck_Itemvalue {
    #[asn(key = 91)]
    UE_associatedLogicalS1_ConnectionItem(UE_associatedLogicalS1_ConnectionItem),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct UE_associatedLogicalS1_ConnectionListResAck_Item {
    #[asn(key_field = true)]
    pub id: ProtocolIE_ID,
    pub criticality: Criticality,
    pub value: UE_associatedLogicalS1_ConnectionListResAck_Itemvalue,
}

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum UEAggregateMaximumBitrateiE_Extensions_ItemextensionValue {
    #[asn(key = 260)]
    ExtendedBitRate(ExtendedBitRate),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct UEAggregateMaximumBitrateiE_Extensions_Item {
    #[asn(key_field = true)]
    pub id: ProtocolExtensionID,
    pub criticality: Criticality,
    pub extension_value: UEAggregateMaximumBitrateiE_Extensions_ItemextensionValue,
}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct UEAggregateMaximumBitrateiE_Extensions(Vec<UEAggregateMaximumBitrateiE_Extensions_Item>);

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
pub enum UEAppLayerMeasConfigiE_Extensions_ItemextensionValue {
    #[asn(key = 276)]
    ServiceType(ServiceType),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct UEAppLayerMeasConfigiE_Extensions_Item {
    #[asn(key_field = true)]
    pub id: ProtocolExtensionID,
    pub criticality: Criticality,
    pub extension_value: UEAppLayerMeasConfigiE_Extensions_ItemextensionValue,
}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct UEAppLayerMeasConfigiE_Extensions(Vec<UEAppLayerMeasConfigiE_Extensions_Item>);

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum UECapabilityInfoIndicationprotocolIEs_Itemvalue {
    #[asn(key = 8)]
    ENB_UE_S1AP_ID(ENB_UE_S1AP_ID),
    #[asn(key = 272)]
    LTE_M_Indication(LTE_M_Indication),
    #[asn(key = 0)]
    MME_UE_S1AP_ID(MME_UE_S1AP_ID),
    #[asn(key = 263)]
    UE_Application_Layer_Measurement_Capability(UE_Application_Layer_Measurement_Capability),
    #[asn(key = 74)]
    UERadioCapability(UERadioCapability),
    #[asn(key = 198)]
    UERadioCapabilityForPaging(UERadioCapabilityForPaging),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct UECapabilityInfoIndicationprotocolIEs_Item {
    #[asn(key_field = true)]
    pub id: ProtocolIE_ID,
    pub criticality: Criticality,
    pub value: UECapabilityInfoIndicationprotocolIEs_Itemvalue,
}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "0",
    sz_ub = "65535"
)]
pub struct UECapabilityInfoIndicationprotocolIEs(Vec<UECapabilityInfoIndicationprotocolIEs_Item>);

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum UEContextModificationConfirmprotocolIEs_Itemvalue {
    #[asn(key = 146)]
    CSGMembershipStatus(CSGMembershipStatus),
    #[asn(key = 58)]
    CriticalityDiagnostics(CriticalityDiagnostics),
    #[asn(key = 8)]
    ENB_UE_S1AP_ID(ENB_UE_S1AP_ID),
    #[asn(key = 0)]
    MME_UE_S1AP_ID(MME_UE_S1AP_ID),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct UEContextModificationConfirmprotocolIEs_Item {
    #[asn(key_field = true)]
    pub id: ProtocolIE_ID,
    pub criticality: Criticality,
    pub value: UEContextModificationConfirmprotocolIEs_Itemvalue,
}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "0",
    sz_ub = "65535"
)]
pub struct UEContextModificationConfirmprotocolIEs(
    Vec<UEContextModificationConfirmprotocolIEs_Item>,
);

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum UEContextModificationFailureprotocolIEs_Itemvalue {
    #[asn(key = 2)]
    Cause(Cause),
    #[asn(key = 58)]
    CriticalityDiagnostics(CriticalityDiagnostics),
    #[asn(key = 8)]
    ENB_UE_S1AP_ID(ENB_UE_S1AP_ID),
    #[asn(key = 0)]
    MME_UE_S1AP_ID(MME_UE_S1AP_ID),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct UEContextModificationFailureprotocolIEs_Item {
    #[asn(key_field = true)]
    pub id: ProtocolIE_ID,
    pub criticality: Criticality,
    pub value: UEContextModificationFailureprotocolIEs_Itemvalue,
}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "0",
    sz_ub = "65535"
)]
pub struct UEContextModificationFailureprotocolIEs(
    Vec<UEContextModificationFailureprotocolIEs_Item>,
);

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum UEContextModificationIndicationprotocolIEs_Itemvalue {
    #[asn(key = 226)]
    CSGMembershipInfo(CSGMembershipInfo),
    #[asn(key = 8)]
    ENB_UE_S1AP_ID(ENB_UE_S1AP_ID),
    #[asn(key = 0)]
    MME_UE_S1AP_ID(MME_UE_S1AP_ID),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct UEContextModificationIndicationprotocolIEs_Item {
    #[asn(key_field = true)]
    pub id: ProtocolIE_ID,
    pub criticality: Criticality,
    pub value: UEContextModificationIndicationprotocolIEs_Itemvalue,
}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "0",
    sz_ub = "65535"
)]
pub struct UEContextModificationIndicationprotocolIEs(
    Vec<UEContextModificationIndicationprotocolIEs_Item>,
);

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum UEContextModificationRequestprotocolIEs_Itemvalue {
    #[asn(key = 187)]
    AdditionalCSFallbackIndicator(AdditionalCSFallbackIndicator),
    #[asn(key = 299)]
    AdditionalRRMPriorityIndex(AdditionalRRMPriorityIndex),
    #[asn(key = 277)]
    AerialUEsubscriptionInformation(AerialUEsubscriptionInformation),
    #[asn(key = 108)]
    CSFallbackIndicator(CSFallbackIndicator),
    #[asn(key = 146)]
    CSGMembershipStatus(CSGMembershipStatus),
    #[asn(key = 8)]
    ENB_UE_S1AP_ID(ENB_UE_S1AP_ID),
    #[asn(key = 301)]
    IAB_Authorized(IAB_Authorized),
    #[asn(key = 159)]
    LAI(LAI),
    #[asn(key = 0)]
    MME_UE_S1AP_ID(MME_UE_S1AP_ID),
    #[asn(key = 269)]
    NRUESecurityCapabilities(NRUESecurityCapabilities),
    #[asn(key = 307)]
    NRUESidelinkAggregateMaximumBitrate(NRUESidelinkAggregateMaximumBitrate),
    #[asn(key = 306)]
    NRV2XServicesAuthorized(NRV2XServicesAuthorized),
    #[asn(key = 308)]
    PC5QoSParameters(PC5QoSParameters),
    #[asn(key = 195)]
    ProSeAuthorized(ProSeAuthorized),
    #[asn(key = 243)]
    SRVCCOperationNotPossible(SRVCCOperationNotPossible),
    #[asn(key = 124)]
    SRVCCOperationPossible(SRVCCOperationPossible),
    #[asn(key = 73)]
    SecurityKey(SecurityKey),
    #[asn(key = 106)]
    SubscriberProfileIDforRFP(SubscriberProfileIDforRFP),
    #[asn(key = 66)]
    UEAggregateMaximumBitrate(UEAggregateMaximumBitrate),
    #[asn(key = 314)]
    UERadioCapabilityID(UERadioCapabilityID),
    #[asn(key = 107)]
    UESecurityCapabilities(UESecurityCapabilities),
    #[asn(key = 248)]
    UESidelinkAggregateMaximumBitrate(UESidelinkAggregateMaximumBitrate),
    #[asn(key = 240)]
    V2XServicesAuthorized(V2XServicesAuthorized),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct UEContextModificationRequestprotocolIEs_Item {
    #[asn(key_field = true)]
    pub id: ProtocolIE_ID,
    pub criticality: Criticality,
    pub value: UEContextModificationRequestprotocolIEs_Itemvalue,
}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "0",
    sz_ub = "65535"
)]
pub struct UEContextModificationRequestprotocolIEs(
    Vec<UEContextModificationRequestprotocolIEs_Item>,
);

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum UEContextModificationResponseprotocolIEs_Itemvalue {
    #[asn(key = 58)]
    CriticalityDiagnostics(CriticalityDiagnostics),
    #[asn(key = 8)]
    ENB_UE_S1AP_ID(ENB_UE_S1AP_ID),
    #[asn(key = 0)]
    MME_UE_S1AP_ID(MME_UE_S1AP_ID),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct UEContextModificationResponseprotocolIEs_Item {
    #[asn(key_field = true)]
    pub id: ProtocolIE_ID,
    pub criticality: Criticality,
    pub value: UEContextModificationResponseprotocolIEs_Itemvalue,
}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "0",
    sz_ub = "65535"
)]
pub struct UEContextModificationResponseprotocolIEs(
    Vec<UEContextModificationResponseprotocolIEs_Item>,
);

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum UEContextReleaseCommandprotocolIEs_Itemvalue {
    #[asn(key = 2)]
    Cause(Cause),
    #[asn(key = 99)]
    UE_S1AP_IDs(UE_S1AP_IDs),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct UEContextReleaseCommandprotocolIEs_Item {
    #[asn(key_field = true)]
    pub id: ProtocolIE_ID,
    pub criticality: Criticality,
    pub value: UEContextReleaseCommandprotocolIEs_Itemvalue,
}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "0",
    sz_ub = "65535"
)]
pub struct UEContextReleaseCommandprotocolIEs(Vec<UEContextReleaseCommandprotocolIEs_Item>);

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum UEContextReleaseCompleteprotocolIEs_Itemvalue {
    #[asn(key = 212)]
    CellIdentifierAndCELevelForCECapableUEs(CellIdentifierAndCELevelForCECapableUEs),
    #[asn(key = 58)]
    CriticalityDiagnostics(CriticalityDiagnostics),
    #[asn(key = 8)]
    ENB_UE_S1AP_ID(ENB_UE_S1AP_ID),
    #[asn(key = 213)]
    InformationOnRecommendedCellsAndENBsForPaging(InformationOnRecommendedCellsAndENBsForPaging),
    #[asn(key = 0)]
    MME_UE_S1AP_ID(MME_UE_S1AP_ID),
    #[asn(key = 264)]
    SecondaryRATDataUsageReportList(SecondaryRATDataUsageReportList),
    #[asn(key = 297)]
    TimeSinceSecondaryNodeRelease(TimeSinceSecondaryNodeRelease),
    #[asn(key = 189)]
    UserLocationInformation(UserLocationInformation),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct UEContextReleaseCompleteprotocolIEs_Item {
    #[asn(key_field = true)]
    pub id: ProtocolIE_ID,
    pub criticality: Criticality,
    pub value: UEContextReleaseCompleteprotocolIEs_Itemvalue,
}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "0",
    sz_ub = "65535"
)]
pub struct UEContextReleaseCompleteprotocolIEs(Vec<UEContextReleaseCompleteprotocolIEs_Item>);

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum UEContextReleaseRequestprotocolIEs_Itemvalue {
    #[asn(key = 2)]
    Cause(Cause),
    #[asn(key = 8)]
    ENB_UE_S1AP_ID(ENB_UE_S1AP_ID),
    #[asn(key = 164)]
    GWContextReleaseIndication(GWContextReleaseIndication),
    #[asn(key = 0)]
    MME_UE_S1AP_ID(MME_UE_S1AP_ID),
    #[asn(key = 264)]
    SecondaryRATDataUsageReportList(SecondaryRATDataUsageReportList),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct UEContextReleaseRequestprotocolIEs_Item {
    #[asn(key_field = true)]
    pub id: ProtocolIE_ID,
    pub criticality: Criticality,
    pub value: UEContextReleaseRequestprotocolIEs_Itemvalue,
}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "0",
    sz_ub = "65535"
)]
pub struct UEContextReleaseRequestprotocolIEs(Vec<UEContextReleaseRequestprotocolIEs_Item>);

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum UEContextResumeFailureprotocolIEs_Itemvalue {
    #[asn(key = 2)]
    Cause(Cause),
    #[asn(key = 58)]
    CriticalityDiagnostics(CriticalityDiagnostics),
    #[asn(key = 8)]
    ENB_UE_S1AP_ID(ENB_UE_S1AP_ID),
    #[asn(key = 0)]
    MME_UE_S1AP_ID(MME_UE_S1AP_ID),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct UEContextResumeFailureprotocolIEs_Item {
    #[asn(key_field = true)]
    pub id: ProtocolIE_ID,
    pub criticality: Criticality,
    pub value: UEContextResumeFailureprotocolIEs_Itemvalue,
}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "0",
    sz_ub = "65535"
)]
pub struct UEContextResumeFailureprotocolIEs(Vec<UEContextResumeFailureprotocolIEs_Item>);

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum UEContextResumeRequestprotocolIEs_Itemvalue {
    #[asn(key = 235)]
    E_RABFailedToResumeListResumeReq(E_RABFailedToResumeListResumeReq),
    #[asn(key = 8)]
    ENB_UE_S1AP_ID(ENB_UE_S1AP_ID),
    #[asn(key = 0)]
    MME_UE_S1AP_ID(MME_UE_S1AP_ID),
    #[asn(key = 245)]
    RRC_Establishment_Cause(RRC_Establishment_Cause),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct UEContextResumeRequestprotocolIEs_Item {
    #[asn(key_field = true)]
    pub id: ProtocolIE_ID,
    pub criticality: Criticality,
    pub value: UEContextResumeRequestprotocolIEs_Itemvalue,
}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "0",
    sz_ub = "65535"
)]
pub struct UEContextResumeRequestprotocolIEs(Vec<UEContextResumeRequestprotocolIEs_Item>);

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum UEContextResumeResponseprotocolIEs_Itemvalue {
    #[asn(key = 58)]
    CriticalityDiagnostics(CriticalityDiagnostics),
    #[asn(key = 237)]
    E_RABFailedToResumeListResumeRes(E_RABFailedToResumeListResumeRes),
    #[asn(key = 8)]
    ENB_UE_S1AP_ID(ENB_UE_S1AP_ID),
    #[asn(key = 0)]
    MME_UE_S1AP_ID(MME_UE_S1AP_ID),
    #[asn(key = 283)]
    PendingDataIndication(PendingDataIndication),
    #[asn(key = 40)]
    SecurityContext(SecurityContext),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct UEContextResumeResponseprotocolIEs_Item {
    #[asn(key_field = true)]
    pub id: ProtocolIE_ID,
    pub criticality: Criticality,
    pub value: UEContextResumeResponseprotocolIEs_Itemvalue,
}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "0",
    sz_ub = "65535"
)]
pub struct UEContextResumeResponseprotocolIEs(Vec<UEContextResumeResponseprotocolIEs_Item>);

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum UEContextSuspendRequestprotocolIEs_Itemvalue {
    #[asn(key = 212)]
    CellIdentifierAndCELevelForCECapableUEs(CellIdentifierAndCELevelForCECapableUEs),
    #[asn(key = 8)]
    ENB_UE_S1AP_ID(ENB_UE_S1AP_ID),
    #[asn(key = 213)]
    InformationOnRecommendedCellsAndENBsForPaging(InformationOnRecommendedCellsAndENBsForPaging),
    #[asn(key = 0)]
    MME_UE_S1AP_ID(MME_UE_S1AP_ID),
    #[asn(key = 264)]
    SecondaryRATDataUsageReportList(SecondaryRATDataUsageReportList),
    #[asn(key = 297)]
    TimeSinceSecondaryNodeRelease(TimeSinceSecondaryNodeRelease),
    #[asn(key = 189)]
    UserLocationInformation(UserLocationInformation),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct UEContextSuspendRequestprotocolIEs_Item {
    #[asn(key_field = true)]
    pub id: ProtocolIE_ID,
    pub criticality: Criticality,
    pub value: UEContextSuspendRequestprotocolIEs_Itemvalue,
}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "0",
    sz_ub = "65535"
)]
pub struct UEContextSuspendRequestprotocolIEs(Vec<UEContextSuspendRequestprotocolIEs_Item>);

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum UEContextSuspendResponseprotocolIEs_Itemvalue {
    #[asn(key = 58)]
    CriticalityDiagnostics(CriticalityDiagnostics),
    #[asn(key = 8)]
    ENB_UE_S1AP_ID(ENB_UE_S1AP_ID),
    #[asn(key = 0)]
    MME_UE_S1AP_ID(MME_UE_S1AP_ID),
    #[asn(key = 40)]
    SecurityContext(SecurityContext),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct UEContextSuspendResponseprotocolIEs_Item {
    #[asn(key_field = true)]
    pub id: ProtocolIE_ID,
    pub criticality: Criticality,
    pub value: UEContextSuspendResponseprotocolIEs_Itemvalue,
}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "0",
    sz_ub = "65535"
)]
pub struct UEContextSuspendResponseprotocolIEs(Vec<UEContextSuspendResponseprotocolIEs_Item>);

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum UEInformationTransferprotocolIEs_Itemvalue {
    #[asn(key = 252)]
    E_RABLevelQoSParameters(E_RABLevelQoSParameters),
    #[asn(key = 283)]
    PendingDataIndication(PendingDataIndication),
    #[asn(key = 96)]
    S_TMSI(S_TMSI),
    #[asn(key = 278)]
    Subscription_Based_UE_DifferentiationInfo(Subscription_Based_UE_DifferentiationInfo),
    #[asn(key = 74)]
    UERadioCapability(UERadioCapability),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct UEInformationTransferprotocolIEs_Item {
    #[asn(key_field = true)]
    pub id: ProtocolIE_ID,
    pub criticality: Criticality,
    pub value: UEInformationTransferprotocolIEs_Itemvalue,
}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "0",
    sz_ub = "65535"
)]
pub struct UEInformationTransferprotocolIEs(Vec<UEInformationTransferprotocolIEs_Item>);

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum UERadioCapabilityIDMappingRequestprotocolIEs_Itemvalue {
    #[asn(key = 314)]
    UERadioCapabilityID(UERadioCapabilityID),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct UERadioCapabilityIDMappingRequestprotocolIEs_Item {
    #[asn(key_field = true)]
    pub id: ProtocolIE_ID,
    pub criticality: Criticality,
    pub value: UERadioCapabilityIDMappingRequestprotocolIEs_Itemvalue,
}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "0",
    sz_ub = "65535"
)]
pub struct UERadioCapabilityIDMappingRequestprotocolIEs(
    Vec<UERadioCapabilityIDMappingRequestprotocolIEs_Item>,
);

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum UERadioCapabilityIDMappingResponseprotocolIEs_Itemvalue {
    #[asn(key = 58)]
    CriticalityDiagnostics(CriticalityDiagnostics),
    #[asn(key = 74)]
    UERadioCapability(UERadioCapability),
    #[asn(key = 314)]
    UERadioCapabilityID(UERadioCapabilityID),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct UERadioCapabilityIDMappingResponseprotocolIEs_Item {
    #[asn(key_field = true)]
    pub id: ProtocolIE_ID,
    pub criticality: Criticality,
    pub value: UERadioCapabilityIDMappingResponseprotocolIEs_Itemvalue,
}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "0",
    sz_ub = "65535"
)]
pub struct UERadioCapabilityIDMappingResponseprotocolIEs(
    Vec<UERadioCapabilityIDMappingResponseprotocolIEs_Item>,
);

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum UERadioCapabilityMatchRequestprotocolIEs_Itemvalue {
    #[asn(key = 8)]
    ENB_UE_S1AP_ID(ENB_UE_S1AP_ID),
    #[asn(key = 0)]
    MME_UE_S1AP_ID(MME_UE_S1AP_ID),
    #[asn(key = 74)]
    UERadioCapability(UERadioCapability),
    #[asn(key = 314)]
    UERadioCapabilityID(UERadioCapabilityID),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct UERadioCapabilityMatchRequestprotocolIEs_Item {
    #[asn(key_field = true)]
    pub id: ProtocolIE_ID,
    pub criticality: Criticality,
    pub value: UERadioCapabilityMatchRequestprotocolIEs_Itemvalue,
}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "0",
    sz_ub = "65535"
)]
pub struct UERadioCapabilityMatchRequestprotocolIEs(
    Vec<UERadioCapabilityMatchRequestprotocolIEs_Item>,
);

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum UERadioCapabilityMatchResponseprotocolIEs_Itemvalue {
    #[asn(key = 58)]
    CriticalityDiagnostics(CriticalityDiagnostics),
    #[asn(key = 8)]
    ENB_UE_S1AP_ID(ENB_UE_S1AP_ID),
    #[asn(key = 0)]
    MME_UE_S1AP_ID(MME_UE_S1AP_ID),
    #[asn(key = 169)]
    VoiceSupportMatchIndicator(VoiceSupportMatchIndicator),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct UERadioCapabilityMatchResponseprotocolIEs_Item {
    #[asn(key_field = true)]
    pub id: ProtocolIE_ID,
    pub criticality: Criticality,
    pub value: UERadioCapabilityMatchResponseprotocolIEs_Itemvalue,
}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "0",
    sz_ub = "65535"
)]
pub struct UERadioCapabilityMatchResponseprotocolIEs(
    Vec<UERadioCapabilityMatchResponseprotocolIEs_Item>,
);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct UESecurityCapabilitiesiE_Extensions_Item {}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct UESecurityCapabilitiesiE_Extensions(Vec<UESecurityCapabilitiesiE_Extensions_Item>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct UESidelinkAggregateMaximumBitrateiE_Extensions_Item {}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct UESidelinkAggregateMaximumBitrateiE_Extensions(
    Vec<UESidelinkAggregateMaximumBitrateiE_Extensions_Item>,
);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct UL_CP_SecurityInformationiE_Extensions_Item {}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct UL_CP_SecurityInformationiE_Extensions(Vec<UL_CP_SecurityInformationiE_Extensions_Item>);

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum UnsuccessfulOutcomevalue {
    #[asn(key = 29)]
    ENBConfigurationUpdateFailure(ENBConfigurationUpdateFailure),
    #[asn(key = 1)]
    HandoverFailure(HandoverFailure),
    #[asn(key = 0)]
    HandoverPreparationFailure(HandoverPreparationFailure),
    #[asn(key = 9)]
    InitialContextSetupFailure(InitialContextSetupFailure),
    #[asn(key = 30)]
    MMEConfigurationUpdateFailure(MMEConfigurationUpdateFailure),
    #[asn(key = 3)]
    PathSwitchRequestFailure(PathSwitchRequestFailure),
    #[asn(key = 17)]
    S1SetupFailure(S1SetupFailure),
    #[asn(key = 21)]
    UEContextModificationFailure(UEContextModificationFailure),
    #[asn(key = 56)]
    UEContextResumeFailure(UEContextResumeFailure),
}

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum UplinkNASTransportprotocolIEs_Itemvalue {
    #[asn(key = 8)]
    ENB_UE_S1AP_ID(ENB_UE_S1AP_ID),
    #[asn(key = 100)]
    EUTRAN_CGI(EUTRAN_CGI),
    #[asn(key = 186)]
    LHN_ID(LHN_ID),
    #[asn(key = 0)]
    MME_UE_S1AP_ID(MME_UE_S1AP_ID),
    #[asn(key = 26)]
    NAS_PDU(NAS_PDU),
    #[asn(key = 288)]
    PSCellInformation(PSCellInformation),
    #[asn(key = 67)]
    TAI(TAI),
    #[asn(key = 184)]
    TransportLayerAddress(TransportLayerAddress),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct UplinkNASTransportprotocolIEs_Item {
    #[asn(key_field = true)]
    pub id: ProtocolIE_ID,
    pub criticality: Criticality,
    pub value: UplinkNASTransportprotocolIEs_Itemvalue,
}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "0",
    sz_ub = "65535"
)]
pub struct UplinkNASTransportprotocolIEs(Vec<UplinkNASTransportprotocolIEs_Item>);

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum UplinkNonUEAssociatedLPPaTransportprotocolIEs_Itemvalue {
    #[asn(key = 147)]
    LPPa_PDU(LPPa_PDU),
    #[asn(key = 148)]
    Routing_ID(Routing_ID),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct UplinkNonUEAssociatedLPPaTransportprotocolIEs_Item {
    #[asn(key_field = true)]
    pub id: ProtocolIE_ID,
    pub criticality: Criticality,
    pub value: UplinkNonUEAssociatedLPPaTransportprotocolIEs_Itemvalue,
}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "0",
    sz_ub = "65535"
)]
pub struct UplinkNonUEAssociatedLPPaTransportprotocolIEs(
    Vec<UplinkNonUEAssociatedLPPaTransportprotocolIEs_Item>,
);

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum UplinkS1cdma2000tunnellingprotocolIEs_Itemvalue {
    #[asn(key = 84)]
    Cdma2000HORequiredIndication(Cdma2000HORequiredIndication),
    #[asn(key = 97)]
    Cdma2000OneXRAND(Cdma2000OneXRAND),
    #[asn(key = 102)]
    Cdma2000OneXSRVCCInfo(Cdma2000OneXSRVCCInfo),
    #[asn(key = 70)]
    Cdma2000PDU(Cdma2000PDU),
    #[asn(key = 71)]
    Cdma2000RATType(Cdma2000RATType),
    #[asn(key = 72)]
    Cdma2000SectorID(Cdma2000SectorID),
    #[asn(key = 8)]
    ENB_UE_S1AP_ID(ENB_UE_S1AP_ID),
    #[asn(key = 140)]
    EUTRANRoundTripDelayEstimationInfo(EUTRANRoundTripDelayEstimationInfo),
    #[asn(key = 0)]
    MME_UE_S1AP_ID(MME_UE_S1AP_ID),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct UplinkS1cdma2000tunnellingprotocolIEs_Item {
    #[asn(key_field = true)]
    pub id: ProtocolIE_ID,
    pub criticality: Criticality,
    pub value: UplinkS1cdma2000tunnellingprotocolIEs_Itemvalue,
}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "0",
    sz_ub = "65535"
)]
pub struct UplinkS1cdma2000tunnellingprotocolIEs(Vec<UplinkS1cdma2000tunnellingprotocolIEs_Item>);

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum UplinkUEAssociatedLPPaTransportprotocolIEs_Itemvalue {
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
pub struct UplinkUEAssociatedLPPaTransportprotocolIEs_Item {
    #[asn(key_field = true)]
    pub id: ProtocolIE_ID,
    pub criticality: Criticality,
    pub value: UplinkUEAssociatedLPPaTransportprotocolIEs_Itemvalue,
}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "0",
    sz_ub = "65535"
)]
pub struct UplinkUEAssociatedLPPaTransportprotocolIEs(
    Vec<UplinkUEAssociatedLPPaTransportprotocolIEs_Item>,
);

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum UserLocationInformationiE_Extensions_ItemextensionValue {
    #[asn(key = 288)]
    PSCellInformation(PSCellInformation),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct UserLocationInformationiE_Extensions_Item {
    #[asn(key_field = true)]
    pub id: ProtocolExtensionID,
    pub criticality: Criticality,
    pub extension_value: UserLocationInformationiE_Extensions_ItemextensionValue,
}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct UserLocationInformationiE_Extensions(Vec<UserLocationInformationiE_Extensions_Item>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct V2XServicesAuthorizediE_Extensions_Item {}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct V2XServicesAuthorizediE_Extensions(Vec<V2XServicesAuthorizediE_Extensions_Item>);

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
pub struct WLANMeasurementConfigurationiE_Extensions_Item {}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct WLANMeasurementConfigurationiE_Extensions(
    Vec<WLANMeasurementConfigurationiE_Extensions_Item>,
);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct WUS_Assistance_InformationiE_Extensions_Item {}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct WUS_Assistance_InformationiE_Extensions(
    Vec<WUS_Assistance_InformationiE_Extensions_Item>,
);

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum WriteReplaceWarningRequestprotocolIEs_Itemvalue {
    #[asn(key = 142)]
    ConcurrentWarningMessageIndicator(ConcurrentWarningMessageIndicator),
    #[asn(key = 118)]
    DataCodingScheme(DataCodingScheme),
    #[asn(key = 144)]
    ExtendedRepetitionPeriod(ExtendedRepetitionPeriod),
    #[asn(key = 111)]
    MessageIdentifier(MessageIdentifier),
    #[asn(key = 115)]
    NumberofBroadcastRequest(NumberofBroadcastRequest),
    #[asn(key = 114)]
    RepetitionPeriod(RepetitionPeriod),
    #[asn(key = 112)]
    SerialNumber(SerialNumber),
    #[asn(key = 286)]
    WarningAreaCoordinates(WarningAreaCoordinates),
    #[asn(key = 113)]
    WarningAreaList(WarningAreaList),
    #[asn(key = 119)]
    WarningMessageContents(WarningMessageContents),
    #[asn(key = 117)]
    WarningSecurityInfo(WarningSecurityInfo),
    #[asn(key = 116)]
    WarningType(WarningType),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct WriteReplaceWarningRequestprotocolIEs_Item {
    #[asn(key_field = true)]
    pub id: ProtocolIE_ID,
    pub criticality: Criticality,
    pub value: WriteReplaceWarningRequestprotocolIEs_Itemvalue,
}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "0",
    sz_ub = "65535"
)]
pub struct WriteReplaceWarningRequestprotocolIEs(Vec<WriteReplaceWarningRequestprotocolIEs_Item>);

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum WriteReplaceWarningResponseprotocolIEs_Itemvalue {
    #[asn(key = 120)]
    BroadcastCompletedAreaList(BroadcastCompletedAreaList),
    #[asn(key = 58)]
    CriticalityDiagnostics(CriticalityDiagnostics),
    #[asn(key = 111)]
    MessageIdentifier(MessageIdentifier),
    #[asn(key = 112)]
    SerialNumber(SerialNumber),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct WriteReplaceWarningResponseprotocolIEs_Item {
    #[asn(key_field = true)]
    pub id: ProtocolIE_ID,
    pub criticality: Criticality,
    pub value: WriteReplaceWarningResponseprotocolIEs_Itemvalue,
}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "0",
    sz_ub = "65535"
)]
pub struct WriteReplaceWarningResponseprotocolIEs(Vec<WriteReplaceWarningResponseprotocolIEs_Item>);

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum X2TNLConfigurationInfoiE_Extensions_ItemextensionValue {
    #[asn(key = 193)]
    ENBIndirectX2TransportLayerAddresses(ENBIndirectX2TransportLayerAddresses),
    #[asn(key = 153)]
    ENBX2ExtTLAs(ENBX2ExtTLAs),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct X2TNLConfigurationInfoiE_Extensions_Item {
    #[asn(key_field = true)]
    pub id: ProtocolExtensionID,
    pub criticality: Criticality,
    pub extension_value: X2TNLConfigurationInfoiE_Extensions_ItemextensionValue,
}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct X2TNLConfigurationInfoiE_Extensions(Vec<X2TNLConfigurationInfoiE_Extensions_Item>);

fn main() {
    eprintln!("S1AP");
}
