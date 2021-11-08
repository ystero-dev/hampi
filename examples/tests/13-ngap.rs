#![allow(non_camel_case_types, dead_code)]

use asn_codecs_derive::*;
use bitvec::order::Msb0;
use bitvec::vec::BitVec;

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct AMF_TNLAssociationSetupItem {
    pub a_mf_tnl_association_address: CPTransportLayerInformation,
    #[asn(optional_idx = 0)]
    pub i_e_extensions: Option<AMF_TNLAssociationSetupItemiE_Extensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "32")]
pub struct AMF_TNLAssociationSetupList(Vec<AMF_TNLAssociationSetupItem>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 2)]
pub struct AMF_TNLAssociationToAddItem {
    pub a_mf_tnl_association_address: CPTransportLayerInformation,
    #[asn(optional_idx = 0)]
    pub t_nl_association_usage: Option<TNLAssociationUsage>,
    pub t_nl_address_weight_factor: TNLAddressWeightFactor,
    #[asn(optional_idx = 1)]
    pub i_e_extensions: Option<AMF_TNLAssociationToAddItemiE_Extensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "32")]
pub struct AMF_TNLAssociationToAddList(Vec<AMF_TNLAssociationToAddItem>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct AMF_TNLAssociationToRemoveItem {
    pub a_mf_tnl_association_address: CPTransportLayerInformation,
    #[asn(optional_idx = 0)]
    pub i_e_extensions: Option<AMF_TNLAssociationToRemoveItemiE_Extensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "32")]
pub struct AMF_TNLAssociationToRemoveList(Vec<AMF_TNLAssociationToRemoveItem>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 3)]
pub struct AMF_TNLAssociationToUpdateItem {
    pub a_mf_tnl_association_address: CPTransportLayerInformation,
    #[asn(optional_idx = 0)]
    pub t_nl_association_usage: Option<TNLAssociationUsage>,
    #[asn(optional_idx = 1)]
    pub t_nl_address_weight_factor: Option<TNLAddressWeightFactor>,
    #[asn(optional_idx = 2)]
    pub i_e_extensions: Option<AMF_TNLAssociationToUpdateItemiE_Extensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "32")]
pub struct AMF_TNLAssociationToUpdateList(Vec<AMF_TNLAssociationToUpdateItem>);

#[derive(Debug, AperCodec)]
#[asn(type = "INTEGER", lb = "0", ub = "1099511627775")]
pub struct AMF_UE_NGAP_ID(u64);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true)]
pub struct AMFCPRelocationIndication {
    pub protocol_i_es: AMFCPRelocationIndicationprotocolIEs,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true)]
pub struct AMFConfigurationUpdate {
    pub protocol_i_es: AMFConfigurationUpdateprotocolIEs,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true)]
pub struct AMFConfigurationUpdateAcknowledge {
    pub protocol_i_es: AMFConfigurationUpdateAcknowledgeprotocolIEs,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true)]
pub struct AMFConfigurationUpdateFailure {
    pub protocol_i_es: AMFConfigurationUpdateFailureprotocolIEs,
}

#[derive(Debug, AperCodec)]
#[asn(
    type = "PrintableString",
    sz_extensible = true,
    sz_lb = "1",
    sz_ub = "150"
)]
pub struct AMFName(String);

#[derive(Debug, AperCodec)]
#[asn(type = "UTF8String", sz_extensible = true, sz_lb = "1", sz_ub = "150")]
pub struct AMFNameUTF8String(String);

#[derive(Debug, AperCodec)]
#[asn(
    type = "VisibleString",
    sz_extensible = true,
    sz_lb = "1",
    sz_ub = "150"
)]
pub struct AMFNameVisibleString(String);

#[derive(Debug, AperCodec)]
#[asn(type = "CHOICE", lb = "0", ub = "2", extensible = false)]
pub enum AMFPagingTarget {
    #[asn(key = 0, extended = false)]
    globalRANNodeID(GlobalRANNodeID),
    #[asn(key = 1, extended = false)]
    tAI(TAI),
    #[asn(key = 2, extended = false)]
    choice_Extensions(AMFPagingTargetchoice_Extensions),
}

#[derive(Debug, AperCodec)]
#[asn(type = "BITSTRING", sz_extensible = false, sz_lb = "6", sz_ub = "6")]
pub struct AMFPointer(BitVec<Msb0, u8>);

#[derive(Debug, AperCodec)]
#[asn(type = "BITSTRING", sz_extensible = false, sz_lb = "8", sz_ub = "8")]
pub struct AMFRegionID(BitVec<Msb0, u8>);

#[derive(Debug, AperCodec)]
#[asn(type = "BITSTRING", sz_extensible = false, sz_lb = "10", sz_ub = "10")]
pub struct AMFSetID(BitVec<Msb0, u8>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true)]
pub struct AMFStatusIndication {
    pub protocol_i_es: AMFStatusIndicationprotocolIEs,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 2)]
pub struct AdditionalDLUPTNLInformationForHOItem {
    pub additional_dl_ngu_up_tnl_information: UPTransportLayerInformation,
    pub additional_qos_flow_setup_response_list: QosFlowListWithDataForwarding,
    #[asn(optional_idx = 0)]
    pub additional_dl_forwarding_uptnl_information: Option<UPTransportLayerInformation>,
    #[asn(optional_idx = 1)]
    pub i_e_extensions: Option<AdditionalDLUPTNLInformationForHOItemiE_Extensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "3")]
pub struct AdditionalDLUPTNLInformationForHOList(Vec<AdditionalDLUPTNLInformationForHOItem>);

#[derive(Debug, AperCodec)]
#[asn(type = "ENUMERATED", extensible = true, lb = "0", ub = "0")]
pub struct AdditionalQosFlowInformation(u8);
impl AdditionalQosFlowInformation {
    const MORE_LIKELY: u8 = 0u8;
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct AllocationAndRetentionPriority {
    pub priority_level_arp: PriorityLevelARP,
    pub pre_emption_capability: Pre_emptionCapability,
    pub pre_emption_vulnerability: Pre_emptionVulnerability,
    #[asn(optional_idx = 0)]
    pub i_e_extensions: Option<AllocationAndRetentionPriorityiE_Extensions>,
}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "256"
)]
pub struct Allowed_CAG_List_per_PLMN(Vec<CAG_ID>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct Allowed_PNI_NPN_Item {
    pub p_lmn_identity: PLMNIdentity,
    pub p_ni_npn_restricted: ENUMERATED_2,
    pub allowed_cag_list_per_plmn: Allowed_CAG_List_per_PLMN,
    #[asn(optional_idx = 0)]
    pub i_e_extensions: Option<Allowed_PNI_NPN_ItemiE_Extensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "16")]
pub struct Allowed_PNI_NPN_List(Vec<Allowed_PNI_NPN_Item>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "8")]
pub struct AllowedNSSAI(Vec<AllowedNSSAI_Item>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct AllowedNSSAI_Item {
    pub s_nssai: S_NSSAI,
    #[asn(optional_idx = 0)]
    pub i_e_extensions: Option<AllowedNSSAI_ItemiE_Extensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "16")]
pub struct AllowedTACs(Vec<TAC>);

#[derive(Debug, AperCodec)]
#[asn(type = "INTEGER", lb = "1", ub = "8", extensible = true)]
pub struct AlternativeQoSParaSetIndex(u8);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 5)]
pub struct AlternativeQoSParaSetItem {
    pub alternative_qo_s_para_set_index: AlternativeQoSParaSetIndex,
    #[asn(optional_idx = 0)]
    pub guaranteed_flow_bit_rate_dl: Option<BitRate>,
    #[asn(optional_idx = 1)]
    pub guaranteed_flow_bit_rate_ul: Option<BitRate>,
    #[asn(optional_idx = 2)]
    pub packet_delay_budget: Option<PacketDelayBudget>,
    #[asn(optional_idx = 3)]
    pub packet_error_rate: Option<PacketErrorRate>,
    #[asn(optional_idx = 4)]
    pub i_e_extensions: Option<AlternativeQoSParaSetItemiE_Extensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "8")]
pub struct AlternativeQoSParaSetList(Vec<AlternativeQoSParaSetItem>);

#[derive(Debug, AperCodec)]
#[asn(type = "INTEGER", lb = "0", ub = "8", extensible = true)]
pub struct AlternativeQoSParaSetNotifyIndex(u8);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 4)]
pub struct AreaOfInterest {
    #[asn(optional_idx = 0)]
    pub area_of_interest_tai_list: Option<AreaOfInterestTAIList>,
    #[asn(optional_idx = 1)]
    pub area_of_interest_cell_list: Option<AreaOfInterestCellList>,
    #[asn(optional_idx = 2)]
    pub area_of_interest_ran_node_list: Option<AreaOfInterestRANNodeList>,
    #[asn(optional_idx = 3)]
    pub i_e_extensions: Option<AreaOfInterestiE_Extensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct AreaOfInterestCellItem {
    pub n_gran_cgi: NGRAN_CGI,
    #[asn(optional_idx = 0)]
    pub i_e_extensions: Option<AreaOfInterestCellItemiE_Extensions>,
}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "256"
)]
pub struct AreaOfInterestCellList(Vec<AreaOfInterestCellItem>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct AreaOfInterestItem {
    pub area_of_interest: AreaOfInterest,
    pub location_reporting_reference_id: LocationReportingReferenceID,
    #[asn(optional_idx = 0)]
    pub i_e_extensions: Option<AreaOfInterestItemiE_Extensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "64")]
pub struct AreaOfInterestList(Vec<AreaOfInterestItem>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct AreaOfInterestRANNodeItem {
    pub global_ran_node_id: GlobalRANNodeID,
    #[asn(optional_idx = 0)]
    pub i_e_extensions: Option<AreaOfInterestRANNodeItemiE_Extensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "64")]
pub struct AreaOfInterestRANNodeList(Vec<AreaOfInterestRANNodeItem>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct AreaOfInterestTAIItem {
    pub t_ai: TAI,
    #[asn(optional_idx = 0)]
    pub i_e_extensions: Option<AreaOfInterestTAIItemiE_Extensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "16")]
pub struct AreaOfInterestTAIList(Vec<AreaOfInterestTAIItem>);

#[derive(Debug, AperCodec)]
#[asn(type = "CHOICE", lb = "0", ub = "4", extensible = false)]
pub enum AreaScopeOfMDT_EUTRA {
    #[asn(key = 0, extended = false)]
    cellBased(CellBasedMDT_EUTRA),
    #[asn(key = 1, extended = false)]
    tABased(TABasedMDT),
    #[asn(key = 2, extended = false)]
    pLMNWide(NULL_3),
    #[asn(key = 3, extended = false)]
    tAIBased(TAIBasedMDT),
    #[asn(key = 4, extended = false)]
    choice_Extensions(AreaScopeOfMDT_EUTRAchoice_Extensions),
}

#[derive(Debug, AperCodec)]
#[asn(type = "CHOICE", lb = "0", ub = "4", extensible = false)]
pub enum AreaScopeOfMDT_NR {
    #[asn(key = 0, extended = false)]
    cellBased(CellBasedMDT_NR),
    #[asn(key = 1, extended = false)]
    tABased(TABasedMDT),
    #[asn(key = 2, extended = false)]
    pLMNWide(NULL_4),
    #[asn(key = 3, extended = false)]
    tAIBased(TAIBasedMDT),
    #[asn(key = 4, extended = false)]
    choice_Extensions(AreaScopeOfMDT_NRchoice_Extensions),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 2)]
pub struct AreaScopeOfNeighCellsItem {
    pub nr_frequency_info: NRFrequencyInfo,
    #[asn(optional_idx = 0)]
    pub pci_list_for_mdt: Option<PCIListForMDT>,
    #[asn(optional_idx = 1)]
    pub i_e_extensions: Option<AreaScopeOfNeighCellsItemiE_Extensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "8")]
pub struct AreaScopeOfNeighCellsList(Vec<AreaScopeOfNeighCellsItem>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 3)]
pub struct AssistanceDataForPaging {
    #[asn(optional_idx = 0)]
    pub assistance_data_for_recommended_cells: Option<AssistanceDataForRecommendedCells>,
    #[asn(optional_idx = 1)]
    pub paging_attempt_information: Option<PagingAttemptInformation>,
    #[asn(optional_idx = 2)]
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
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 2)]
pub struct AssociatedQosFlowItem {
    pub qos_flow_identifier: QosFlowIdentifier,
    #[asn(optional_idx = 0)]
    pub qos_flow_mapping_indication: Option<ENUMERATED_5>,
    #[asn(optional_idx = 1)]
    pub i_e_extensions: Option<AssociatedQosFlowItemiE_Extensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "64")]
pub struct AssociatedQosFlowList(Vec<AssociatedQosFlowItem>);

#[derive(Debug, AperCodec)]
#[asn(type = "ENUMERATED", extensible = true, lb = "0", ub = "0")]
pub struct AuthenticatedIndication(u8);
impl AuthenticatedIndication {
    const TRUE: u8 = 0u8;
}

#[derive(Debug, AperCodec)]
#[asn(type = "INTEGER", lb = "0", ub = "4095", extensible = true)]
pub struct AveragingWindow(u16);

#[derive(Debug, AperCodec)]
#[asn(type = "INTEGER", lb = "0", ub = "4000000000000", extensible = true)]
pub struct BitRate(u64);

#[derive(Debug, AperCodec)]
#[asn(type = "ENUMERATED", extensible = true, lb = "0", ub = "0")]
pub struct BluetoothMeasConfig(u8);
impl BluetoothMeasConfig {
    const SETUP: u8 = 0u8;
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct BluetoothMeasConfigNameItem {
    pub bluetooth_name: BluetoothName,
    #[asn(optional_idx = 0)]
    pub i_e_extensions: Option<BluetoothMeasConfigNameItemiE_Extensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "4")]
pub struct BluetoothMeasConfigNameList(Vec<BluetoothMeasConfigNameItem>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 3)]
pub struct BluetoothMeasurementConfiguration {
    pub bluetooth_meas_config: BluetoothMeasConfig,
    #[asn(optional_idx = 0)]
    pub bluetooth_meas_config_name_list: Option<BluetoothMeasConfigNameList>,
    #[asn(optional_idx = 1)]
    pub bt_rssi: Option<ENUMERATED_6>,
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
#[asn(type = "CHOICE", lb = "0", ub = "6", extensible = false)]
pub enum BroadcastCancelledAreaList {
    #[asn(key = 0, extended = false)]
    cellIDCancelledEUTRA(CellIDCancelledEUTRA),
    #[asn(key = 1, extended = false)]
    tAICancelledEUTRA(TAICancelledEUTRA),
    #[asn(key = 2, extended = false)]
    emergencyAreaIDCancelledEUTRA(EmergencyAreaIDCancelledEUTRA),
    #[asn(key = 3, extended = false)]
    cellIDCancelledNR(CellIDCancelledNR),
    #[asn(key = 4, extended = false)]
    tAICancelledNR(TAICancelledNR),
    #[asn(key = 5, extended = false)]
    emergencyAreaIDCancelledNR(EmergencyAreaIDCancelledNR),
    #[asn(key = 6, extended = false)]
    choice_Extensions(BroadcastCancelledAreaListchoice_Extensions),
}

#[derive(Debug, AperCodec)]
#[asn(type = "CHOICE", lb = "0", ub = "6", extensible = false)]
pub enum BroadcastCompletedAreaList {
    #[asn(key = 0, extended = false)]
    cellIDBroadcastEUTRA(CellIDBroadcastEUTRA),
    #[asn(key = 1, extended = false)]
    tAIBroadcastEUTRA(TAIBroadcastEUTRA),
    #[asn(key = 2, extended = false)]
    emergencyAreaIDBroadcastEUTRA(EmergencyAreaIDBroadcastEUTRA),
    #[asn(key = 3, extended = false)]
    cellIDBroadcastNR(CellIDBroadcastNR),
    #[asn(key = 4, extended = false)]
    tAIBroadcastNR(TAIBroadcastNR),
    #[asn(key = 5, extended = false)]
    emergencyAreaIDBroadcastNR(EmergencyAreaIDBroadcastNR),
    #[asn(key = 6, extended = false)]
    choice_Extensions(BroadcastCompletedAreaListchoice_Extensions),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct BroadcastPLMNItem {
    pub p_lmn_identity: PLMNIdentity,
    pub t_ai_slice_support_list: SliceSupportList,
    #[asn(optional_idx = 0)]
    pub i_e_extensions: Option<BroadcastPLMNItemiE_Extensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "12")]
pub struct BroadcastPLMNList(Vec<BroadcastPLMNItem>);

#[derive(Debug, AperCodec)]
#[asn(type = "OCTET-STRING")]
pub struct BurstArrivalTime(Vec<u8>);

#[derive(Debug, AperCodec)]
#[asn(type = "BITSTRING", sz_extensible = false, sz_lb = "32", sz_ub = "32")]
pub struct CAG_ID(BitVec<Msb0, u8>);

#[derive(Debug, AperCodec)]
#[asn(type = "ENUMERATED", extensible = true, lb = "0", ub = "0")]
pub struct CEmodeBSupport_Indicator(u8);
impl CEmodeBSupport_Indicator {
    const SUPPORTED: u8 = 0u8;
}

#[derive(Debug, AperCodec)]
#[asn(type = "ENUMERATED", extensible = true, lb = "0", ub = "1")]
pub struct CEmodeBrestricted(u8);
impl CEmodeBrestricted {
    const RESTRICTED: u8 = 0u8;
    const NOT_RESTRICTED: u8 = 1u8;
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 2)]
pub struct CNAssistedRANTuning {
    #[asn(optional_idx = 0)]
    pub expected_ue_behaviour: Option<ExpectedUEBehaviour>,
    #[asn(optional_idx = 1)]
    pub i_e_extensions: Option<CNAssistedRANTuningiE_Extensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "15")]
pub struct CNTypeRestrictionsForEquivalent(Vec<CNTypeRestrictionsForEquivalentItem>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct CNTypeRestrictionsForEquivalentItem {
    pub plmn_identity: PLMNIdentity,
    pub cn_type: ENUMERATED_7,
    #[asn(optional_idx = 0)]
    pub i_e_extensions: Option<CNTypeRestrictionsForEquivalentItemiE_Extensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "ENUMERATED", extensible = true, lb = "0", ub = "0")]
pub struct CNTypeRestrictionsForServing(u8);
impl CNTypeRestrictionsForServing {
    const EPC_FORBIDDEN: u8 = 0u8;
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct COUNTValueForPDCP_SN12 {
    pub p_dcp_sn12: INTEGER_8,
    pub h_fn_pdcp_sn12: INTEGER_9,
    #[asn(optional_idx = 0)]
    pub i_e_extensions: Option<COUNTValueForPDCP_SN12iE_Extensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct COUNTValueForPDCP_SN18 {
    pub p_dcp_sn18: INTEGER_10,
    pub h_fn_pdcp_sn18: INTEGER_11,
    #[asn(optional_idx = 0)]
    pub i_e_extensions: Option<COUNTValueForPDCP_SN18iE_Extensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "CHOICE", lb = "0", ub = "1", extensible = false)]
pub enum CPTransportLayerInformation {
    #[asn(key = 0, extended = false)]
    endpointIPAddress(TransportLayerAddress),
    #[asn(key = 1, extended = false)]
    choice_Extensions(CPTransportLayerInformationchoice_Extensions),
}

#[derive(Debug, AperCodec)]
#[asn(type = "ENUMERATED", extensible = true, lb = "0", ub = "0")]
pub struct CancelAllWarningMessages(u8);
impl CancelAllWarningMessages {
    const TRUE: u8 = 0u8;
}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct CancelledCellsInEAI_EUTRA(Vec<CancelledCellsInEAI_EUTRA_Item>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct CancelledCellsInEAI_EUTRA_Item {
    pub e_utra_cgi: EUTRA_CGI,
    pub number_of_broadcasts: NumberOfBroadcasts,
    #[asn(optional_idx = 0)]
    pub i_e_extensions: Option<CancelledCellsInEAI_EUTRA_ItemiE_Extensions>,
}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct CancelledCellsInEAI_NR(Vec<CancelledCellsInEAI_NR_Item>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct CancelledCellsInEAI_NR_Item {
    pub n_r_cgi: NR_CGI,
    pub number_of_broadcasts: NumberOfBroadcasts,
    #[asn(optional_idx = 0)]
    pub i_e_extensions: Option<CancelledCellsInEAI_NR_ItemiE_Extensions>,
}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct CancelledCellsInTAI_EUTRA(Vec<CancelledCellsInTAI_EUTRA_Item>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct CancelledCellsInTAI_EUTRA_Item {
    pub e_utra_cgi: EUTRA_CGI,
    pub number_of_broadcasts: NumberOfBroadcasts,
    #[asn(optional_idx = 0)]
    pub i_e_extensions: Option<CancelledCellsInTAI_EUTRA_ItemiE_Extensions>,
}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct CancelledCellsInTAI_NR(Vec<CancelledCellsInTAI_NR_Item>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct CancelledCellsInTAI_NR_Item {
    pub n_r_cgi: NR_CGI,
    pub number_of_broadcasts: NumberOfBroadcasts,
    #[asn(optional_idx = 0)]
    pub i_e_extensions: Option<CancelledCellsInTAI_NR_ItemiE_Extensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "CHOICE", lb = "0", ub = "2", extensible = false)]
pub enum CandidateCell {
    #[asn(key = 0, extended = false)]
    candidateCGI(CandidateCellID),
    #[asn(key = 1, extended = false)]
    candidatePCI(CandidatePCI),
    #[asn(key = 2, extended = false)]
    choice_Extensions(CandidateCellchoice_Extensions),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct CandidateCellID {
    pub candidate_cell_id: NR_CGI,
    #[asn(optional_idx = 0)]
    pub i_e_extensions: Option<CandidateCellIDiE_Extensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct CandidateCellItem {
    pub candidate_cell: CandidateCell,
    #[asn(optional_idx = 0)]
    pub i_e_extensions: Option<CandidateCellItemiE_Extensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "32")]
pub struct CandidateCellList(Vec<CandidateCellItem>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct CandidatePCI {
    pub candidate_pci: INTEGER_12,
    pub candidate_nrarfcn: INTEGER_13,
    #[asn(optional_idx = 0)]
    pub i_e_extensions: Option<CandidatePCIiE_Extensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "CHOICE", lb = "0", ub = "5", extensible = false)]
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
    #[asn(key = 5, extended = false)]
    choice_Extensions(Causechoice_Extensions),
}

#[derive(Debug, AperCodec)]
#[asn(type = "ENUMERATED", extensible = true, lb = "0", ub = "5")]
pub struct CauseMisc(u8);
impl CauseMisc {
    const CONTROL_PROCESSING_OVERLOAD: u8 = 0u8;
    const NOT_ENOUGH_USER_PLANE_PROCESSING_RESOURCES: u8 = 1u8;
    const HARDWARE_FAILURE: u8 = 2u8;
    const OM_INTERVENTION: u8 = 3u8;
    const UNKNOWN_PLMN_OR_SNPN: u8 = 4u8;
    const UNSPECIFIED: u8 = 5u8;
}

#[derive(Debug, AperCodec)]
#[asn(type = "ENUMERATED", extensible = true, lb = "0", ub = "3")]
pub struct CauseNas(u8);
impl CauseNas {
    const NORMAL_RELEASE: u8 = 0u8;
    const AUTHENTICATION_FAILURE: u8 = 1u8;
    const DEREGISTER: u8 = 2u8;
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
#[asn(type = "ENUMERATED", extensible = true, lb = "0", ub = "44")]
pub struct CauseRadioNetwork(u8);
impl CauseRadioNetwork {
    const UNSPECIFIED: u8 = 0u8;
    const TXNRELOCOVERALL_EXPIRY: u8 = 1u8;
    const SUCCESSFUL_HANDOVER: u8 = 2u8;
    const RELEASE_DUE_TO_NGRAN_GENERATED_REASON: u8 = 3u8;
    const RELEASE_DUE_TO_5GC_GENERATED_REASON: u8 = 4u8;
    const HANDOVER_CANCELLED: u8 = 5u8;
    const PARTIAL_HANDOVER: u8 = 6u8;
    const HO_FAILURE_IN_TARGET_5GC_NGRAN_NODE_OR_TARGET_SYSTEM: u8 = 7u8;
    const HO_TARGET_NOT_ALLOWED: u8 = 8u8;
    const TNGRELOCOVERALL_EXPIRY: u8 = 9u8;
    const TNGRELOCPREP_EXPIRY: u8 = 10u8;
    const CELL_NOT_AVAILABLE: u8 = 11u8;
    const UNKNOWN_TARGET_ID: u8 = 12u8;
    const NO_RADIO_RESOURCES_AVAILABLE_IN_TARGET_CELL: u8 = 13u8;
    const UNKNOWN_LOCAL_UE_NGAP_ID: u8 = 14u8;
    const INCONSISTENT_REMOTE_UE_NGAP_ID: u8 = 15u8;
    const HANDOVER_DESIRABLE_FOR_RADIO_REASON: u8 = 16u8;
    const TIME_CRITICAL_HANDOVER: u8 = 17u8;
    const RESOURCE_OPTIMISATION_HANDOVER: u8 = 18u8;
    const REDUCE_LOAD_IN_SERVING_CELL: u8 = 19u8;
    const USER_INACTIVITY: u8 = 20u8;
    const RADIO_CONNECTION_WITH_UE_LOST: u8 = 21u8;
    const RADIO_RESOURCES_NOT_AVAILABLE: u8 = 22u8;
    const INVALID_QOS_COMBINATION: u8 = 23u8;
    const FAILURE_IN_RADIO_INTERFACE_PROCEDURE: u8 = 24u8;
    const INTERACTION_WITH_OTHER_PROCEDURE: u8 = 25u8;
    const UNKNOWN_PDU_SESSION_ID: u8 = 26u8;
    const UNKOWN_QOS_FLOW_ID: u8 = 27u8;
    const MULTIPLE_PDU_SESSION_ID_INSTANCES: u8 = 28u8;
    const MULTIPLE_QOS_FLOW_ID_INSTANCES: u8 = 29u8;
    const ENCRYPTION_AND_OR_INTEGRITY_PROTECTION_ALGORITHMS_NOT_SUPPORTED: u8 = 30u8;
    const NG_INTRA_SYSTEM_HANDOVER_TRIGGERED: u8 = 31u8;
    const NG_INTER_SYSTEM_HANDOVER_TRIGGERED: u8 = 32u8;
    const XN_HANDOVER_TRIGGERED: u8 = 33u8;
    const NOT_SUPPORTED_5QI_VALUE: u8 = 34u8;
    const UE_CONTEXT_TRANSFER: u8 = 35u8;
    const IMS_VOICE_EPS_FALLBACK_OR_RAT_FALLBACK_TRIGGERED: u8 = 36u8;
    const UP_INTEGRITY_PROTECTION_NOT_POSSIBLE: u8 = 37u8;
    const UP_CONFIDENTIALITY_PROTECTION_NOT_POSSIBLE: u8 = 38u8;
    const SLICE_NOT_SUPPORTED: u8 = 39u8;
    const UE_IN_RRC_INACTIVE_STATE_NOT_REACHABLE: u8 = 40u8;
    const REDIRECTION: u8 = 41u8;
    const RESOURCES_NOT_AVAILABLE_FOR_THE_SLICE: u8 = 42u8;
    const UE_MAX_INTEGRITY_PROTECTED_DATA_RATE_REASON: u8 = 43u8;
    const RELEASE_DUE_TO_CN_DETECTED_MOBILITY: u8 = 44u8;
}

#[derive(Debug, AperCodec)]
#[asn(type = "ENUMERATED", extensible = true, lb = "0", ub = "1")]
pub struct CauseTransport(u8);
impl CauseTransport {
    const TRANSPORT_RESOURCE_UNAVAILABLE: u8 = 0u8;
    const UNSPECIFIED: u8 = 1u8;
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct Cell_CAGInformation {
    pub n_gran_cgi: NGRAN_CGI,
    pub cell_cag_list: CellCAGList,
    #[asn(optional_idx = 0)]
    pub i_e_extensions: Option<Cell_CAGInformationiE_Extensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct CellBasedMDT_EUTRA {
    pub cell_id_listfor_mdt: CellIdListforMDT_EUTRA,
    #[asn(optional_idx = 0)]
    pub i_e_extensions: Option<CellBasedMDT_EUTRAiE_Extensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct CellBasedMDT_NR {
    pub cell_id_listfor_mdt: CellIdListforMDT_NR,
    #[asn(optional_idx = 0)]
    pub i_e_extensions: Option<CellBasedMDT_NRiE_Extensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "64")]
pub struct CellCAGList(Vec<CAG_ID>);

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct CellIDBroadcastEUTRA(Vec<CellIDBroadcastEUTRA_Item>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct CellIDBroadcastEUTRA_Item {
    pub e_utra_cgi: EUTRA_CGI,
    #[asn(optional_idx = 0)]
    pub i_e_extensions: Option<CellIDBroadcastEUTRA_ItemiE_Extensions>,
}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct CellIDBroadcastNR(Vec<CellIDBroadcastNR_Item>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct CellIDBroadcastNR_Item {
    pub n_r_cgi: NR_CGI,
    #[asn(optional_idx = 0)]
    pub i_e_extensions: Option<CellIDBroadcastNR_ItemiE_Extensions>,
}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct CellIDCancelledEUTRA(Vec<CellIDCancelledEUTRA_Item>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct CellIDCancelledEUTRA_Item {
    pub e_utra_cgi: EUTRA_CGI,
    pub number_of_broadcasts: NumberOfBroadcasts,
    #[asn(optional_idx = 0)]
    pub i_e_extensions: Option<CellIDCancelledEUTRA_ItemiE_Extensions>,
}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct CellIDCancelledNR(Vec<CellIDCancelledNR_Item>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct CellIDCancelledNR_Item {
    pub n_r_cgi: NR_CGI,
    pub number_of_broadcasts: NumberOfBroadcasts,
    #[asn(optional_idx = 0)]
    pub i_e_extensions: Option<CellIDCancelledNR_ItemiE_Extensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "CHOICE", lb = "0", ub = "2", extensible = false)]
pub enum CellIDListForRestart {
    #[asn(key = 0, extended = false)]
    eUTRA_CGIListforRestart(EUTRA_CGIList),
    #[asn(key = 1, extended = false)]
    nR_CGIListforRestart(NR_CGIList),
    #[asn(key = 2, extended = false)]
    choice_Extensions(CellIDListForRestartchoice_Extensions),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "32")]
pub struct CellIdListforMDT_EUTRA(Vec<EUTRA_CGI>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "32")]
pub struct CellIdListforMDT_NR(Vec<NR_CGI>);

#[derive(Debug, AperCodec)]
#[asn(type = "ENUMERATED", extensible = true, lb = "0", ub = "3")]
pub struct CellSize(u8);
impl CellSize {
    const VERYSMALL: u8 = 0u8;
    const SMALL: u8 = 1u8;
    const MEDIUM: u8 = 2u8;
    const LARGE: u8 = 3u8;
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true)]
pub struct CellTrafficTrace {
    pub protocol_i_es: CellTrafficTraceprotocolIEs,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct CellType {
    pub cell_size: CellSize,
    #[asn(optional_idx = 0)]
    pub i_e_extensions: Option<CellTypeiE_Extensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "OCTET-STRING")]
pub struct CommonNetworkInstance(Vec<u8>);

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct CompletedCellsInEAI_EUTRA(Vec<CompletedCellsInEAI_EUTRA_Item>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct CompletedCellsInEAI_EUTRA_Item {
    pub e_utra_cgi: EUTRA_CGI,
    #[asn(optional_idx = 0)]
    pub i_e_extensions: Option<CompletedCellsInEAI_EUTRA_ItemiE_Extensions>,
}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct CompletedCellsInEAI_NR(Vec<CompletedCellsInEAI_NR_Item>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct CompletedCellsInEAI_NR_Item {
    pub n_r_cgi: NR_CGI,
    #[asn(optional_idx = 0)]
    pub i_e_extensions: Option<CompletedCellsInEAI_NR_ItemiE_Extensions>,
}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct CompletedCellsInTAI_EUTRA(Vec<CompletedCellsInTAI_EUTRA_Item>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct CompletedCellsInTAI_EUTRA_Item {
    pub e_utra_cgi: EUTRA_CGI,
    #[asn(optional_idx = 0)]
    pub i_e_extensions: Option<CompletedCellsInTAI_EUTRA_ItemiE_Extensions>,
}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct CompletedCellsInTAI_NR(Vec<CompletedCellsInTAI_NR_Item>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct CompletedCellsInTAI_NR_Item {
    pub n_r_cgi: NR_CGI,
    #[asn(optional_idx = 0)]
    pub i_e_extensions: Option<CompletedCellsInTAI_NR_ItemiE_Extensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "ENUMERATED", extensible = true, lb = "0", ub = "0")]
pub struct ConcurrentWarningMessageInd(u8);
impl ConcurrentWarningMessageInd {
    const TRUE: u8 = 0u8;
}

#[derive(Debug, AperCodec)]
#[asn(type = "ENUMERATED", extensible = true, lb = "0", ub = "2")]
pub struct ConfidentialityProtectionIndication(u8);
impl ConfidentialityProtectionIndication {
    const REQUIRED: u8 = 0u8;
    const PREFERRED: u8 = 1u8;
    const NOT_NEEDED: u8 = 2u8;
}

#[derive(Debug, AperCodec)]
#[asn(type = "ENUMERATED", extensible = true, lb = "0", ub = "1")]
pub struct ConfidentialityProtectionResult(u8);
impl ConfidentialityProtectionResult {
    const PERFORMED: u8 = 0u8;
    const NOT_PERFORMED: u8 = 1u8;
}

#[derive(Debug, AperCodec)]
#[asn(
    type = "OCTET-STRING",
    sz_extensible = false,
    sz_lb = "128",
    sz_ub = "128"
)]
pub struct ConfiguredNSSAI(Vec<u8>);

#[derive(Debug, AperCodec)]
#[asn(type = "ENUMERATED", extensible = true, lb = "0", ub = "0")]
pub struct ConfiguredTACIndication(u8);
impl ConfiguredTACIndication {
    const TRUE: u8 = 0u8;
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true)]
pub struct ConnectionEstablishmentIndication {
    pub protocol_i_es: ConnectionEstablishmentIndicationprotocolIEs,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 4)]
pub struct CoreNetworkAssistanceInformationForInactive {
    pub u_e_identity_index_value: UEIdentityIndexValue,
    #[asn(optional_idx = 0)]
    pub u_e_specific_drx: Option<PagingDRX>,
    pub periodic_registration_update_timer: PeriodicRegistrationUpdateTimer,
    #[asn(optional_idx = 1)]
    pub m_ico_mode_indication: Option<MICOModeIndication>,
    pub t_ai_list_for_inactive: TAIListForInactive,
    #[asn(optional_idx = 2)]
    pub expected_ue_behaviour: Option<ExpectedUEBehaviour>,
    #[asn(optional_idx = 3)]
    pub i_e_extensions: Option<CoreNetworkAssistanceInformationForInactiveiE_Extensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "OCTET-STRING")]
pub struct CoverageEnhancementLevel(Vec<u8>);

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
    pub d_aps_indicator: ENUMERATED_14,
    #[asn(optional_idx = 0)]
    pub i_e_extensions: Option<DAPSRequestInfoiE_Extensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct DAPSResponseInfo {
    pub dapsresponseindicator: ENUMERATED_15,
    #[asn(optional_idx = 0)]
    pub i_e_extensions: Option<DAPSResponseInfoiE_Extensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct DAPSResponseInfoItem {
    pub d_rb_id: DRB_ID,
    pub d_aps_response_info: DAPSResponseInfo,
    #[asn(optional_idx = 0)]
    pub i_e_extension: Option<DAPSResponseInfoItemiE_Extension>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "32")]
pub struct DAPSResponseInfoList(Vec<DAPSResponseInfoItem>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct DL_CP_SecurityInformation {
    pub dl_nas_mac: DL_NAS_MAC,
    #[asn(optional_idx = 0)]
    pub i_e_extensions: Option<DL_CP_SecurityInformationiE_Extensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "BITSTRING", sz_extensible = false, sz_lb = "16", sz_ub = "16")]
pub struct DL_NAS_MAC(BitVec<Msb0, u8>);

#[derive(Debug, AperCodec)]
#[asn(type = "ENUMERATED", extensible = true, lb = "0", ub = "0")]
pub struct DL_NGU_TNLInformationReused(u8);
impl DL_NGU_TNLInformationReused {
    const TRUE: u8 = 0u8;
}

#[derive(Debug, AperCodec)]
#[asn(type = "ENUMERATED", extensible = true, lb = "0", ub = "0")]
pub struct DLForwarding(u8);
impl DLForwarding {
    const DL_FORWARDING_PROPOSED: u8 = 0u8;
}

#[derive(Debug, AperCodec)]
#[asn(type = "INTEGER", lb = "1", ub = "32", extensible = true)]
pub struct DRB_ID(u8);

#[derive(Debug, AperCodec)]
#[asn(type = "CHOICE", lb = "0", ub = "2", extensible = false)]
pub enum DRBStatusDL {
    #[asn(key = 0, extended = false)]
    dRBStatusDL12(DRBStatusDL12),
    #[asn(key = 1, extended = false)]
    dRBStatusDL18(DRBStatusDL18),
    #[asn(key = 2, extended = false)]
    choice_Extensions(DRBStatusDLchoice_Extensions),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct DRBStatusDL12 {
    pub d_l_count_value: COUNTValueForPDCP_SN12,
    #[asn(optional_idx = 0)]
    pub i_e_extension: Option<DRBStatusDL12iE_Extension>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct DRBStatusDL18 {
    pub d_l_count_value: COUNTValueForPDCP_SN18,
    #[asn(optional_idx = 0)]
    pub i_e_extension: Option<DRBStatusDL18iE_Extension>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "CHOICE", lb = "0", ub = "2", extensible = false)]
pub enum DRBStatusUL {
    #[asn(key = 0, extended = false)]
    dRBStatusUL12(DRBStatusUL12),
    #[asn(key = 1, extended = false)]
    dRBStatusUL18(DRBStatusUL18),
    #[asn(key = 2, extended = false)]
    choice_Extensions(DRBStatusULchoice_Extensions),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 2)]
pub struct DRBStatusUL12 {
    pub u_l_count_value: COUNTValueForPDCP_SN12,
    #[asn(optional_idx = 0)]
    pub receive_status_of_ul_pdcp_sd_us: Option<BIT_STRING_16>,
    #[asn(optional_idx = 1)]
    pub i_e_extension: Option<DRBStatusUL12iE_Extension>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 2)]
pub struct DRBStatusUL18 {
    pub u_l_count_value: COUNTValueForPDCP_SN18,
    #[asn(optional_idx = 0)]
    pub receive_status_of_ul_pdcp_sd_us: Option<BIT_STRING_17>,
    #[asn(optional_idx = 1)]
    pub i_e_extension: Option<DRBStatusUL18iE_Extension>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct DRBsSubjectToEarlyStatusTransfer_Item {
    pub d_rb_id: DRB_ID,
    pub first_dlcount: DRBStatusDL,
    #[asn(optional_idx = 0)]
    pub i_e_extension: Option<DRBsSubjectToEarlyStatusTransfer_ItemiE_Extension>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "32")]
pub struct DRBsSubjectToEarlyStatusTransfer_List(Vec<DRBsSubjectToEarlyStatusTransfer_Item>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct DRBsSubjectToStatusTransferItem {
    pub d_rb_id: DRB_ID,
    pub d_rb_status_ul: DRBStatusUL,
    pub d_rb_status_dl: DRBStatusDL,
    #[asn(optional_idx = 0)]
    pub i_e_extension: Option<DRBsSubjectToStatusTransferItemiE_Extension>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "32")]
pub struct DRBsSubjectToStatusTransferList(Vec<DRBsSubjectToStatusTransferItem>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct DRBsToQosFlowsMappingItem {
    pub d_rb_id: DRB_ID,
    pub associated_qos_flow_list: AssociatedQosFlowList,
    #[asn(optional_idx = 0)]
    pub i_e_extensions: Option<DRBsToQosFlowsMappingItemiE_Extensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "32")]
pub struct DRBsToQosFlowsMappingList(Vec<DRBsToQosFlowsMappingItem>);

#[derive(Debug, AperCodec)]
#[asn(type = "BITSTRING", sz_extensible = false, sz_lb = "8", sz_ub = "8")]
pub struct DataCodingScheme(BitVec<Msb0, u8>);

#[derive(Debug, AperCodec)]
#[asn(type = "ENUMERATED", extensible = true, lb = "0", ub = "0")]
pub struct DataForwardingAccepted(u8);
impl DataForwardingAccepted {
    const DATA_FORWARDING_ACCEPTED: u8 = 0u8;
}

#[derive(Debug, AperCodec)]
#[asn(type = "ENUMERATED", extensible = true, lb = "0", ub = "0")]
pub struct DataForwardingNotPossible(u8);
impl DataForwardingNotPossible {
    const DATA_FORWARDING_NOT_POSSIBLE: u8 = 0u8;
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 3)]
pub struct DataForwardingResponseDRBItem {
    pub d_rb_id: DRB_ID,
    #[asn(optional_idx = 0)]
    pub d_l_forwarding_up_tnl_information: Option<UPTransportLayerInformation>,
    #[asn(optional_idx = 1)]
    pub u_l_forwarding_up_tnl_information: Option<UPTransportLayerInformation>,
    #[asn(optional_idx = 2)]
    pub i_e_extensions: Option<DataForwardingResponseDRBItemiE_Extensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "32")]
pub struct DataForwardingResponseDRBList(Vec<DataForwardingResponseDRBItem>);

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "256"
)]
pub struct DataForwardingResponseERABList(Vec<DataForwardingResponseERABListItem>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct DataForwardingResponseERABListItem {
    pub e_rab_id: E_RAB_ID,
    pub d_l_forwarding_up_tnl_information: UPTransportLayerInformation,
    #[asn(optional_idx = 0)]
    pub i_e_extensions: Option<DataForwardingResponseERABListItemiE_Extensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true)]
pub struct DeactivateTrace {
    pub protocol_i_es: DeactivateTraceprotocolIEs,
}

#[derive(Debug, AperCodec)]
#[asn(type = "ENUMERATED", extensible = true, lb = "0", ub = "1")]
pub struct DelayCritical(u8);
impl DelayCritical {
    const DELAY_CRITICAL: u8 = 0u8;
    const NON_DELAY_CRITICAL: u8 = 1u8;
}

#[derive(Debug, AperCodec)]
#[asn(type = "ENUMERATED", extensible = true, lb = "0", ub = "0")]
pub struct DirectForwardingPathAvailability(u8);
impl DirectForwardingPathAvailability {
    const DIRECT_PATH_AVAILABLE: u8 = 0u8;
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true)]
pub struct DownlinkNASTransport {
    pub protocol_i_es: DownlinkNASTransportprotocolIEs,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true)]
pub struct DownlinkNonUEAssociatedNRPPaTransport {
    pub protocol_i_es: DownlinkNonUEAssociatedNRPPaTransportprotocolIEs,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true)]
pub struct DownlinkRANConfigurationTransfer {
    pub protocol_i_es: DownlinkRANConfigurationTransferprotocolIEs,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true)]
pub struct DownlinkRANEarlyStatusTransfer {
    pub protocol_i_es: DownlinkRANEarlyStatusTransferprotocolIEs,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true)]
pub struct DownlinkRANStatusTransfer {
    pub protocol_i_es: DownlinkRANStatusTransferprotocolIEs,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true)]
pub struct DownlinkRIMInformationTransfer {
    pub protocol_i_es: DownlinkRIMInformationTransferprotocolIEs,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true)]
pub struct DownlinkUEAssociatedNRPPaTransport {
    pub protocol_i_es: DownlinkUEAssociatedNRPPaTransportprotocolIEs,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 5)]
pub struct Dynamic5QIDescriptor {
    pub priority_level_qos: PriorityLevelQos,
    pub packet_delay_budget: PacketDelayBudget,
    pub packet_error_rate: PacketErrorRate,
    #[asn(optional_idx = 0)]
    pub five_qi: Option<FiveQI>,
    #[asn(optional_idx = 1)]
    pub delay_critical: Option<DelayCritical>,
    #[asn(optional_idx = 2)]
    pub averaging_window: Option<AveragingWindow>,
    #[asn(optional_idx = 3)]
    pub maximum_data_burst_volume: Option<MaximumDataBurstVolume>,
    #[asn(optional_idx = 4)]
    pub i_e_extensions: Option<Dynamic5QIDescriptoriE_Extensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "INTEGER", lb = "0", ub = "15", extensible = true)]
pub struct E_RAB_ID(u8);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 2)]
pub struct E_RABInformationItem {
    pub e_rab_id: E_RAB_ID,
    #[asn(optional_idx = 0)]
    pub d_l_forwarding: Option<DLForwarding>,
    #[asn(optional_idx = 1)]
    pub i_e_extensions: Option<E_RABInformationItemiE_Extensions>,
}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "256"
)]
pub struct E_RABInformationList(Vec<E_RABInformationItem>);

#[derive(Debug, AperCodec)]
#[asn(type = "ENUMERATED", extensible = true, lb = "0", ub = "0")]
pub struct EDT_Session(u8);
impl EDT_Session {
    const TRUE: u8 = 0u8;
}

#[derive(Debug, AperCodec)]
#[asn(type = "OCTET-STRING")]
pub struct EN_DCSONConfigurationTransfer(Vec<u8>);

#[derive(Debug, AperCodec)]
#[asn(type = "CHOICE", lb = "0", ub = "4", extensible = false)]
pub enum ENB_ID {
    #[asn(key = 0, extended = false)]
    macroENB_ID(BIT_STRING_18),
    #[asn(key = 1, extended = false)]
    homeENB_ID(BIT_STRING_19),
    #[asn(key = 2, extended = false)]
    short_macroENB_ID(BIT_STRING_20),
    #[asn(key = 3, extended = false)]
    long_macroENB_ID(BIT_STRING_21),
    #[asn(key = 4, extended = false)]
    choice_Extensions(ENB_IDchoice_Extensions),
}

#[derive(Debug, AperCodec)]
#[asn(type = "OCTET-STRING", sz_extensible = false, sz_lb = "2", sz_ub = "2")]
pub struct EPS_TAC(Vec<u8>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct EPS_TAI {
    pub p_lmn_identity: PLMNIdentity,
    pub e_ps_tac: EPS_TAC,
    #[asn(optional_idx = 0)]
    pub i_e_extensions: Option<EPS_TAIiE_Extensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct EUTRA_CGI {
    pub p_lmn_identity: PLMNIdentity,
    pub e_utra_cell_identity: EUTRACellIdentity,
    #[asn(optional_idx = 0)]
    pub i_e_extensions: Option<EUTRA_CGIiE_Extensions>,
}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "256"
)]
pub struct EUTRA_CGIList(Vec<EUTRA_CGI>);

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct EUTRA_CGIListForWarning(Vec<EUTRA_CGI>);

#[derive(Debug, AperCodec)]
#[asn(type = "BITSTRING", sz_extensible = false, sz_lb = "28", sz_ub = "28")]
pub struct EUTRACellIdentity(BitVec<Msb0, u8>);

#[derive(Debug, AperCodec)]
#[asn(type = "BITSTRING", sz_extensible = true, sz_lb = "16", sz_ub = "16")]
pub struct EUTRAencryptionAlgorithms(BitVec<Msb0, u8>);

#[derive(Debug, AperCodec)]
#[asn(type = "BITSTRING", sz_extensible = true, sz_lb = "16", sz_ub = "16")]
pub struct EUTRAintegrityProtectionAlgorithms(BitVec<Msb0, u8>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct EarlyStatusTransfer_TransparentContainer {
    pub procedure_stage: ProcedureStageChoice,
    #[asn(optional_idx = 0)]
    pub i_e_extensions: Option<EarlyStatusTransfer_TransparentContaineriE_Extensions>,
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
pub struct EmergencyAreaIDBroadcastEUTRA(Vec<EmergencyAreaIDBroadcastEUTRA_Item>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct EmergencyAreaIDBroadcastEUTRA_Item {
    pub emergency_area_id: EmergencyAreaID,
    pub completed_cells_in_eai_eutra: CompletedCellsInEAI_EUTRA,
    #[asn(optional_idx = 0)]
    pub i_e_extensions: Option<EmergencyAreaIDBroadcastEUTRA_ItemiE_Extensions>,
}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct EmergencyAreaIDBroadcastNR(Vec<EmergencyAreaIDBroadcastNR_Item>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct EmergencyAreaIDBroadcastNR_Item {
    pub emergency_area_id: EmergencyAreaID,
    pub completed_cells_in_eai_nr: CompletedCellsInEAI_NR,
    #[asn(optional_idx = 0)]
    pub i_e_extensions: Option<EmergencyAreaIDBroadcastNR_ItemiE_Extensions>,
}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct EmergencyAreaIDCancelledEUTRA(Vec<EmergencyAreaIDCancelledEUTRA_Item>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct EmergencyAreaIDCancelledEUTRA_Item {
    pub emergency_area_id: EmergencyAreaID,
    pub cancelled_cells_in_eai_eutra: CancelledCellsInEAI_EUTRA,
    #[asn(optional_idx = 0)]
    pub i_e_extensions: Option<EmergencyAreaIDCancelledEUTRA_ItemiE_Extensions>,
}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct EmergencyAreaIDCancelledNR(Vec<EmergencyAreaIDCancelledNR_Item>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct EmergencyAreaIDCancelledNR_Item {
    pub emergency_area_id: EmergencyAreaID,
    pub cancelled_cells_in_eai_nr: CancelledCellsInEAI_NR,
    #[asn(optional_idx = 0)]
    pub i_e_extensions: Option<EmergencyAreaIDCancelledNR_ItemiE_Extensions>,
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
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 2)]
pub struct EmergencyFallbackIndicator {
    pub emergency_fallback_request_indicator: EmergencyFallbackRequestIndicator,
    #[asn(optional_idx = 0)]
    pub emergency_service_target_cn: Option<EmergencyServiceTargetCN>,
    #[asn(optional_idx = 1)]
    pub i_e_extensions: Option<EmergencyFallbackIndicatoriE_Extensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "ENUMERATED", extensible = true, lb = "0", ub = "0")]
pub struct EmergencyFallbackRequestIndicator(u8);
impl EmergencyFallbackRequestIndicator {
    const EMERGENCY_FALLBACK_REQUESTED: u8 = 0u8;
}

#[derive(Debug, AperCodec)]
#[asn(type = "ENUMERATED", extensible = true, lb = "0", ub = "1")]
pub struct EmergencyServiceTargetCN(u8);
impl EmergencyServiceTargetCN {
    const FIVE_GC: u8 = 0u8;
    const EPC: u8 = 1u8;
}

#[derive(Debug, AperCodec)]
#[asn(type = "ENUMERATED", extensible = true, lb = "0", ub = "1")]
pub struct EndIndication(u8);
impl EndIndication {
    const NO_FURTHER_DATA: u8 = 0u8;
    const FURTHER_DATA_EXISTS: u8 = 1u8;
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false, optional_fields = 1)]
pub struct EndpointIPAddressAndPort {
    pub endpoint_ip_address: TransportLayerAddress,
    pub port_number: PortNumber,
    #[asn(optional_idx = 0)]
    pub i_e_extensions: Option<EndpointIPAddressAndPortiE_Extensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "ENUMERATED", extensible = true, lb = "0", ub = "0")]
pub struct Enhanced_CoverageRestriction(u8);
impl Enhanced_CoverageRestriction {
    const RESTRICTED: u8 = 0u8;
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "15")]
pub struct EquivalentPLMNs(Vec<PLMNIdentity>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true)]
pub struct ErrorIndication {
    pub protocol_i_es: ErrorIndicationprotocolIEs,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct EventL1LoggedMDTConfig {
    pub l1_threshold: MeasurementThresholdL1LoggedMDT,
    pub hysteresis: Hysteresis,
    pub time_to_trigger: TimeToTrigger,
    #[asn(optional_idx = 0)]
    pub i_e_extensions: Option<EventL1LoggedMDTConfigiE_Extensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "CHOICE", lb = "0", ub = "2", extensible = false)]
pub enum EventTrigger {
    #[asn(key = 0, extended = false)]
    outOfCoverage(ENUMERATED_22),
    #[asn(key = 1, extended = false)]
    eventL1LoggedMDTConfig(EventL1LoggedMDTConfig),
    #[asn(key = 2, extended = false)]
    choice_Extensions(EventTriggerchoice_Extensions),
}

#[derive(Debug, AperCodec)]
#[asn(type = "ENUMERATED", extensible = true, lb = "0", ub = "5")]
pub struct EventType(u8);
impl EventType {
    const DIRECT: u8 = 0u8;
    const CHANGE_OF_SERVE_CELL: u8 = 1u8;
    const UE_PRESENCE_IN_AREA_OF_INTEREST: u8 = 2u8;
    const STOP_CHANGE_OF_SERVE_CELL: u8 = 3u8;
    const STOP_UE_PRESENCE_IN_AREA_OF_INTEREST: u8 = 4u8;
    const CANCEL_LOCATION_REPORTING_FOR_THE_UE: u8 = 5u8;
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
    pub source_of_ue_activity_behaviour_information: Option<SourceOfUEActivityBehaviourInformation>,
    #[asn(optional_idx = 3)]
    pub i_e_extensions: Option<ExpectedUEActivityBehaviouriE_Extensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 5)]
pub struct ExpectedUEBehaviour {
    #[asn(optional_idx = 0)]
    pub expected_ue_activity_behaviour: Option<ExpectedUEActivityBehaviour>,
    #[asn(optional_idx = 1)]
    pub expected_ho_interval: Option<ExpectedHOInterval>,
    #[asn(optional_idx = 2)]
    pub expected_ue_mobility: Option<ExpectedUEMobility>,
    #[asn(optional_idx = 3)]
    pub expected_ue_moving_trajectory: Option<ExpectedUEMovingTrajectory>,
    #[asn(optional_idx = 4)]
    pub i_e_extensions: Option<ExpectedUEBehaviouriE_Extensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "ENUMERATED", extensible = true, lb = "0", ub = "1")]
pub struct ExpectedUEMobility(u8);
impl ExpectedUEMobility {
    const STATIONARY: u8 = 0u8;
    const MOBILE: u8 = 1u8;
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "16")]
pub struct ExpectedUEMovingTrajectory(Vec<ExpectedUEMovingTrajectoryItem>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 2)]
pub struct ExpectedUEMovingTrajectoryItem {
    pub n_gran_cgi: NGRAN_CGI,
    #[asn(optional_idx = 0)]
    pub time_stayed_in_cell: Option<INTEGER_23>,
    #[asn(optional_idx = 1)]
    pub i_e_extensions: Option<ExpectedUEMovingTrajectoryItemiE_Extensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 3)]
pub struct Extended_AMFName {
    #[asn(optional_idx = 0)]
    pub a_mf_name_visible_string: Option<AMFNameVisibleString>,
    #[asn(optional_idx = 1)]
    pub a_mf_name_utf8_string: Option<AMFNameUTF8String>,
    #[asn(optional_idx = 2)]
    pub i_e_extensions: Option<Extended_AMFNameiE_Extensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "INTEGER", lb = "0", ub = "255")]
pub struct Extended_ConnectedTime(u8);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 3)]
pub struct Extended_RANNodeName {
    #[asn(optional_idx = 0)]
    pub r_an_node_name_visible_string: Option<RANNodeNameVisibleString>,
    #[asn(optional_idx = 1)]
    pub r_an_node_name_utf8_string: Option<RANNodeNameUTF8String>,
    #[asn(optional_idx = 2)]
    pub i_e_extensions: Option<Extended_RANNodeNameiE_Extensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "INTEGER", lb = "1", ub = "65535", extensible = true)]
pub struct ExtendedPacketDelayBudget(u16);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct ExtendedRATRestrictionInformation {
    pub primary_rat_restriction: BIT_STRING_24,
    pub secondary_rat_restriction: BIT_STRING_25,
    #[asn(optional_idx = 0)]
    pub i_e_extensions: Option<ExtendedRATRestrictionInformationiE_Extensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "INTEGER", lb = "4096", ub = "65535")]
pub struct ExtendedRNC_ID(u16);

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct ExtendedSliceSupportList(Vec<SliceSupportItem>);

#[derive(Debug, AperCodec)]
#[asn(type = "BITSTRING", sz_extensible = false, sz_lb = "16", sz_ub = "16")]
pub struct ExtendedUEIdentityIndexValue(BitVec<Msb0, u8>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct FailureIndication {
    pub u_erlf_report_container: UERLFReportContainer,
    #[asn(optional_idx = 0)]
    pub i_e_extensions: Option<FailureIndicationiE_Extensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct FirstDLCount {
    pub d_r_bs_subject_to_early_status_transfer: DRBsSubjectToEarlyStatusTransfer_List,
    #[asn(optional_idx = 0)]
    pub i_e_extension: Option<FirstDLCountiE_Extension>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct FiveG_S_TMSI {
    pub a_mf_set_id: AMFSetID,
    pub a_mf_pointer: AMFPointer,
    pub five_g_tmsi: FiveG_TMSI,
    #[asn(optional_idx = 0)]
    pub i_e_extensions: Option<FiveG_S_TMSIiE_Extensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "OCTET-STRING", sz_extensible = false, sz_lb = "4", sz_ub = "4")]
pub struct FiveG_TMSI(Vec<u8>);

#[derive(Debug, AperCodec)]
#[asn(type = "INTEGER", lb = "0", ub = "255", extensible = true)]
pub struct FiveQI(u8);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "16")]
pub struct ForbiddenAreaInformation(Vec<ForbiddenAreaInformation_Item>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct ForbiddenAreaInformation_Item {
    pub p_lmn_identity: PLMNIdentity,
    pub forbidden_ta_cs: ForbiddenTACs,
    #[asn(optional_idx = 0)]
    pub i_e_extensions: Option<ForbiddenAreaInformation_ItemiE_Extensions>,
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
#[asn(type = "SEQUENCE", extensible = false, optional_fields = 1)]
pub struct FromEUTRANtoNGRAN {
    pub sourcee_nbid: IntersystemSONeNBID,
    pub target_ngra_nnode_id: IntersystemSONNGRANnodeID,
    #[asn(optional_idx = 0)]
    pub i_e_extensions: Option<FromEUTRANtoNGRANiE_Extensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false, optional_fields = 1)]
pub struct FromNGRANtoEUTRAN {
    pub source_ngra_nnode_id: IntersystemSONNGRANnodeID,
    pub targete_nbid: IntersystemSONeNBID,
    #[asn(optional_idx = 0)]
    pub i_e_extensions: Option<FromNGRANtoEUTRANiE_Extensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 4)]
pub struct GBR_QosInformation {
    pub maximum_flow_bit_rate_dl: BitRate,
    pub maximum_flow_bit_rate_ul: BitRate,
    pub guaranteed_flow_bit_rate_dl: BitRate,
    pub guaranteed_flow_bit_rate_ul: BitRate,
    #[asn(optional_idx = 0)]
    pub notification_control: Option<NotificationControl>,
    #[asn(optional_idx = 1)]
    pub maximum_packet_loss_rate_dl: Option<PacketLossRate>,
    #[asn(optional_idx = 2)]
    pub maximum_packet_loss_rate_ul: Option<PacketLossRate>,
    #[asn(optional_idx = 3)]
    pub i_e_extensions: Option<GBR_QosInformationiE_Extensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "CHOICE", lb = "0", ub = "1", extensible = false)]
pub enum GNB_ID {
    #[asn(key = 0, extended = false)]
    gNB_ID(BIT_STRING_26),
    #[asn(key = 1, extended = false)]
    choice_Extensions(GNB_IDchoice_Extensions),
}

#[derive(Debug, AperCodec)]
#[asn(type = "BITSTRING", sz_extensible = false, sz_lb = "22", sz_ub = "22")]
pub struct GNBSetID(BitVec<Msb0, u8>);

#[derive(Debug, AperCodec)]
#[asn(type = "OCTET-STRING", sz_extensible = false, sz_lb = "4", sz_ub = "4")]
pub struct GTP_TEID(Vec<u8>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct GTPTunnel {
    pub transport_layer_address: TransportLayerAddress,
    pub g_tp_teid: GTP_TEID,
    #[asn(optional_idx = 0)]
    pub i_e_extensions: Option<GTPTunneliE_Extensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct GUAMI {
    pub p_lmn_identity: PLMNIdentity,
    pub a_mf_region_id: AMFRegionID,
    pub a_mf_set_id: AMFSetID,
    pub a_mf_pointer: AMFPointer,
    #[asn(optional_idx = 0)]
    pub i_e_extensions: Option<GUAMIiE_Extensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "ENUMERATED", extensible = true, lb = "0", ub = "1")]
pub struct GUAMIType(u8);
impl GUAMIType {
    const NATIVE: u8 = 0u8;
    const MAPPED: u8 = 1u8;
}

#[derive(Debug, AperCodec)]
#[asn(type = "OCTET-STRING")]
pub struct GlobalCable_ID(Vec<u8>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct GlobalENB_ID {
    pub p_lm_nidentity: PLMNIdentity,
    pub e_nb_id: ENB_ID,
    #[asn(optional_idx = 0)]
    pub i_e_extensions: Option<GlobalENB_IDiE_Extensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct GlobalGNB_ID {
    pub p_lmn_identity: PLMNIdentity,
    pub g_nb_id: GNB_ID,
    #[asn(optional_idx = 0)]
    pub i_e_extensions: Option<GlobalGNB_IDiE_Extensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 2)]
pub struct GlobalLine_ID {
    pub global_line_identity: GlobalLineIdentity,
    #[asn(optional_idx = 0)]
    pub line_type: Option<LineType>,
    #[asn(optional_idx = 1)]
    pub i_e_extensions: Option<GlobalLine_IDiE_Extensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "OCTET-STRING")]
pub struct GlobalLineIdentity(Vec<u8>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct GlobalN3IWF_ID {
    pub p_lmn_identity: PLMNIdentity,
    pub n3_iwf_id: N3IWF_ID,
    #[asn(optional_idx = 0)]
    pub i_e_extensions: Option<GlobalN3IWF_IDiE_Extensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct GlobalNgENB_ID {
    pub p_lmn_identity: PLMNIdentity,
    pub ng_enb_id: NgENB_ID,
    #[asn(optional_idx = 0)]
    pub i_e_extensions: Option<GlobalNgENB_IDiE_Extensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "CHOICE", lb = "0", ub = "3", extensible = false)]
pub enum GlobalRANNodeID {
    #[asn(key = 0, extended = false)]
    globalGNB_ID(GlobalGNB_ID),
    #[asn(key = 1, extended = false)]
    globalNgENB_ID(GlobalNgENB_ID),
    #[asn(key = 2, extended = false)]
    globalN3IWF_ID(GlobalN3IWF_ID),
    #[asn(key = 3, extended = false)]
    choice_Extensions(GlobalRANNodeIDchoice_Extensions),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct GlobalTNGF_ID {
    pub p_lmn_identity: PLMNIdentity,
    pub t_ngf_id: TNGF_ID,
    #[asn(optional_idx = 0)]
    pub i_e_extensions: Option<GlobalTNGF_IDiE_Extensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct GlobalTWIF_ID {
    pub p_lmn_identity: PLMNIdentity,
    pub t_wif_id: TWIF_ID,
    #[asn(optional_idx = 0)]
    pub i_e_extensions: Option<GlobalTWIF_IDiE_Extensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct GlobalW_AGF_ID {
    pub p_lmn_identity: PLMNIdentity,
    pub w_agf_id: W_AGF_ID,
    #[asn(optional_idx = 0)]
    pub i_e_extensions: Option<GlobalW_AGF_IDiE_Extensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "OCTET-STRING")]
pub struct HFCNode_ID(Vec<u8>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 6)]
pub struct HOReport {
    pub handover_report_type: ENUMERATED_27,
    pub handover_cause: Cause,
    pub sourcecell_cgi: NGRAN_CGI,
    pub targetcell_cgi: NGRAN_CGI,
    #[asn(optional_idx = 0)]
    pub reestablishmentcell_cgi: Option<NGRAN_CGI>,
    #[asn(optional_idx = 1)]
    pub sourcecell_c_rnti: Option<BIT_STRING_28>,
    #[asn(optional_idx = 2)]
    pub targetcellin_e_utran: Option<EUTRA_CGI>,
    #[asn(optional_idx = 3)]
    pub mobility_information: Option<MobilityInformation>,
    #[asn(optional_idx = 4)]
    pub u_erlf_report_container: Option<UERLFReportContainer>,
    #[asn(optional_idx = 5)]
    pub i_e_extensions: Option<HOReportiE_Extensions>,
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
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 4)]
pub struct HandoverCommandTransfer {
    #[asn(optional_idx = 0)]
    pub d_l_forwarding_up_tnl_information: Option<UPTransportLayerInformation>,
    #[asn(optional_idx = 1)]
    pub qos_flow_to_be_forwarded_list: Option<QosFlowToBeForwardedList>,
    #[asn(optional_idx = 2)]
    pub data_forwarding_response_drb_list: Option<DataForwardingResponseDRBList>,
    #[asn(optional_idx = 3)]
    pub i_e_extensions: Option<HandoverCommandTransferiE_Extensions>,
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
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct HandoverPreparationUnsuccessfulTransfer {
    pub cause: Cause,
    #[asn(optional_idx = 0)]
    pub i_e_extensions: Option<HandoverPreparationUnsuccessfulTransferiE_Extensions>,
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
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 5)]
pub struct HandoverRequestAcknowledgeTransfer {
    pub d_l_ngu_up_tnl_information: UPTransportLayerInformation,
    #[asn(optional_idx = 0)]
    pub d_l_forwarding_up_tnl_information: Option<UPTransportLayerInformation>,
    #[asn(optional_idx = 1)]
    pub security_result: Option<SecurityResult>,
    pub qos_flow_setup_response_list: QosFlowListWithDataForwarding,
    #[asn(optional_idx = 2)]
    pub qos_flow_failed_to_setup_list: Option<QosFlowListWithCause>,
    #[asn(optional_idx = 3)]
    pub data_forwarding_response_drb_list: Option<DataForwardingResponseDRBList>,
    #[asn(optional_idx = 4)]
    pub i_e_extensions: Option<HandoverRequestAcknowledgeTransferiE_Extensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true)]
pub struct HandoverRequired {
    pub protocol_i_es: HandoverRequiredprotocolIEs,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 2)]
pub struct HandoverRequiredTransfer {
    #[asn(optional_idx = 0)]
    pub direct_forwarding_path_availability: Option<DirectForwardingPathAvailability>,
    #[asn(optional_idx = 1)]
    pub i_e_extensions: Option<HandoverRequiredTransferiE_Extensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 2)]
pub struct HandoverResourceAllocationUnsuccessfulTransfer {
    pub cause: Cause,
    #[asn(optional_idx = 0)]
    pub criticality_diagnostics: Option<CriticalityDiagnostics>,
    #[asn(optional_idx = 1)]
    pub i_e_extensions: Option<HandoverResourceAllocationUnsuccessfulTransferiE_Extensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true)]
pub struct HandoverSuccess {
    pub protocol_i_es: HandoverSuccessprotocolIEs,
}

#[derive(Debug, AperCodec)]
#[asn(type = "ENUMERATED", extensible = true, lb = "0", ub = "2")]
pub struct HandoverType(u8);
impl HandoverType {
    const INTRA5GS: u8 = 0u8;
    const FIVEGS_TO_EPS: u8 = 1u8;
    const EPS_TO_5GS: u8 = 2u8;
}

#[derive(Debug, AperCodec)]
#[asn(type = "INTEGER", lb = "0", ub = "30")]
pub struct Hysteresis(u8);

#[derive(Debug, AperCodec)]
#[asn(type = "ENUMERATED", extensible = true, lb = "0", ub = "1")]
pub struct IAB_Authorized(u8);
impl IAB_Authorized {
    const AUTHORIZED: u8 = 0u8;
    const NOT_AUTHORIZED: u8 = 1u8;
}

#[derive(Debug, AperCodec)]
#[asn(type = "ENUMERATED", extensible = true, lb = "0", ub = "0")]
pub struct IAB_Supported(u8);
impl IAB_Supported {
    const TRUE: u8 = 0u8;
}

#[derive(Debug, AperCodec)]
#[asn(type = "ENUMERATED", extensible = true, lb = "0", ub = "0")]
pub struct IABNodeIndication(u8);
impl IABNodeIndication {
    const TRUE: u8 = 0u8;
}

#[derive(Debug, AperCodec)]
#[asn(type = "ENUMERATED", extensible = true, lb = "0", ub = "1")]
pub struct IMSVoiceSupportIndicator(u8);
impl IMSVoiceSupportIndicator {
    const SUPPORTED: u8 = 0u8;
    const NOT_SUPPORTED: u8 = 1u8;
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 10)]
pub struct ImmediateMDTNr {
    pub measurements_to_activate: MeasurementsToActivate,
    #[asn(optional_idx = 0)]
    pub m1_configuration: Option<M1Configuration>,
    #[asn(optional_idx = 1)]
    pub m4_configuration: Option<M4Configuration>,
    #[asn(optional_idx = 2)]
    pub m5_configuration: Option<M5Configuration>,
    #[asn(optional_idx = 3)]
    pub m6_configuration: Option<M6Configuration>,
    #[asn(optional_idx = 4)]
    pub m7_configuration: Option<M7Configuration>,
    #[asn(optional_idx = 5)]
    pub bluetooth_measurement_configuration: Option<BluetoothMeasurementConfiguration>,
    #[asn(optional_idx = 6)]
    pub w_lan_measurement_configuration: Option<WLANMeasurementConfiguration>,
    #[asn(optional_idx = 7)]
    pub m_dt_location_info: Option<MDT_Location_Info>,
    #[asn(optional_idx = 8)]
    pub sensor_measurement_configuration: Option<SensorMeasurementConfiguration>,
    #[asn(optional_idx = 9)]
    pub i_e_extensions: Option<ImmediateMDTNriE_Extensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "INTEGER", lb = "1", ub = "256", extensible = true)]
pub struct IndexToRFSP(u16);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct InfoOnRecommendedCellsAndRANNodesForPaging {
    pub recommended_cells_for_paging: RecommendedCellsForPaging,
    pub recommend_ran_nodes_for_paging: RecommendedRANNodesForPaging,
    #[asn(optional_idx = 0)]
    pub i_e_extensions: Option<InfoOnRecommendedCellsAndRANNodesForPagingiE_Extensions>,
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
#[asn(type = "ENUMERATED", extensible = true, lb = "0", ub = "2")]
pub struct IntegrityProtectionIndication(u8);
impl IntegrityProtectionIndication {
    const REQUIRED: u8 = 0u8;
    const PREFERRED: u8 = 1u8;
    const NOT_NEEDED: u8 = 2u8;
}

#[derive(Debug, AperCodec)]
#[asn(type = "ENUMERATED", extensible = true, lb = "0", ub = "1")]
pub struct IntegrityProtectionResult(u8);
impl IntegrityProtectionResult {
    const PERFORMED: u8 = 0u8;
    const NOT_PERFORMED: u8 = 1u8;
}

#[derive(Debug, AperCodec)]
#[asn(type = "INTEGER", lb = "1", ub = "16", extensible = true)]
pub struct IntendedNumberOfPagingAttempts(u8);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 2)]
pub struct InterSystemFailureIndication {
    #[asn(optional_idx = 0)]
    pub u_erlf_report_container: Option<UERLFReportContainer>,
    #[asn(optional_idx = 1)]
    pub i_e_extensions: Option<InterSystemFailureIndicationiE_Extensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct InterSystemHOReport {
    pub handover_report_type: InterSystemHandoverReportType,
    #[asn(optional_idx = 0)]
    pub i_e_extensions: Option<InterSystemHOReportiE_Extensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "CHOICE", lb = "0", ub = "2", extensible = false)]
pub enum InterSystemHandoverReportType {
    #[asn(key = 0, extended = false)]
    tooearlyIntersystemHO(TooearlyIntersystemHO),
    #[asn(key = 1, extended = false)]
    intersystemUnnecessaryHO(IntersystemUnnecessaryHO),
    #[asn(key = 2, extended = false)]
    choice_Extensions(InterSystemHandoverReportTypechoice_Extensions),
}

#[derive(Debug, AperCodec)]
#[asn(type = "BITSTRING", sz_extensible = false, sz_lb = "8", sz_ub = "8")]
pub struct InterfacesToTrace(BitVec<Msb0, u8>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct IntersystemSONConfigurationTransfer {
    pub transfer_type: IntersystemSONTransferType,
    pub intersystem_son_information: IntersystemSONInformation,
    #[asn(optional_idx = 0)]
    pub i_e_extensions: Option<IntersystemSONConfigurationTransferiE_Extensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "CHOICE", lb = "0", ub = "1", extensible = false)]
pub enum IntersystemSONInformation {
    #[asn(key = 0, extended = false)]
    intersystemSONInformationReport(IntersystemSONInformationReport),
    #[asn(key = 1, extended = false)]
    choice_Extensions(IntersystemSONInformationchoice_Extensions),
}

#[derive(Debug, AperCodec)]
#[asn(type = "CHOICE", lb = "0", ub = "2", extensible = false)]
pub enum IntersystemSONInformationReport {
    #[asn(key = 0, extended = false)]
    hOReportInformation(InterSystemHOReport),
    #[asn(key = 1, extended = false)]
    failureIndicationInformation(InterSystemFailureIndication),
    #[asn(key = 2, extended = false)]
    choice_Extensions(IntersystemSONInformationReportchoice_Extensions),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct IntersystemSONNGRANnodeID {
    pub global_ran_node_id: GlobalRANNodeID,
    pub selected_tai: TAI,
    #[asn(optional_idx = 0)]
    pub i_e_extensions: Option<IntersystemSONNGRANnodeIDiE_Extensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "CHOICE", lb = "0", ub = "2", extensible = false)]
pub enum IntersystemSONTransferType {
    #[asn(key = 0, extended = false)]
    fromEUTRANtoNGRAN(FromEUTRANtoNGRAN),
    #[asn(key = 1, extended = false)]
    fromNGRANtoEUTRAN(FromNGRANtoEUTRAN),
    #[asn(key = 2, extended = false)]
    choice_Extensions(IntersystemSONTransferTypechoice_Extensions),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct IntersystemSONeNBID {
    pub globale_nbid: GlobalENB_ID,
    pub selected_epstai: EPS_TAI,
    #[asn(optional_idx = 0)]
    pub i_e_extensions: Option<IntersystemSONeNBIDiE_Extensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct IntersystemUnnecessaryHO {
    pub sourcecell_id: NGRAN_CGI,
    pub targetcell_id: EUTRA_CGI,
    pub early_iratho: ENUMERATED_29,
    pub candidate_cell_list: CandidateCellList,
    #[asn(optional_idx = 0)]
    pub i_e_extensions: Option<IntersystemUnnecessaryHOiE_Extensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "OCTET-STRING", sz_extensible = false, sz_lb = "2", sz_ub = "2")]
pub struct LAC(Vec<u8>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct LAI {
    pub p_lm_nidentity: PLMNIdentity,
    pub l_ac: LAC,
    #[asn(optional_idx = 0)]
    pub i_e_extensions: Option<LAIiE_Extensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "ENUMERATED", extensible = true, lb = "0", ub = "0")]
pub struct LTEM_Indication(u8);
impl LTEM_Indication {
    const LTE_M: u8 = 0u8;
}

#[derive(Debug, AperCodec)]
#[asn(type = "OCTET-STRING")]
pub struct LTEUERLFReportContainer(Vec<u8>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct LTEUESidelinkAggregateMaximumBitrate {
    pub u_e_sidelink_aggregate_maximum_bit_rate: BitRate,
    #[asn(optional_idx = 0)]
    pub i_e_extensions: Option<LTEUESidelinkAggregateMaximumBitrateiE_Extensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 3)]
pub struct LTEV2XServicesAuthorized {
    #[asn(optional_idx = 0)]
    pub vehicle_ue: Option<VehicleUE>,
    #[asn(optional_idx = 1)]
    pub pedestrian_ue: Option<PedestrianUE>,
    #[asn(optional_idx = 2)]
    pub i_e_extensions: Option<LTEV2XServicesAuthorizediE_Extensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "CHOICE", lb = "0", ub = "4", extensible = false)]
pub enum LastVisitedCellInformation {
    #[asn(key = 0, extended = false)]
    nGRANCell(LastVisitedNGRANCellInformation),
    #[asn(key = 1, extended = false)]
    eUTRANCell(LastVisitedEUTRANCellInformation),
    #[asn(key = 2, extended = false)]
    uTRANCell(LastVisitedUTRANCellInformation),
    #[asn(key = 3, extended = false)]
    gERANCell(LastVisitedGERANCellInformation),
    #[asn(key = 4, extended = false)]
    choice_Extensions(LastVisitedCellInformationchoice_Extensions),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct LastVisitedCellItem {
    pub last_visited_cell_information: LastVisitedCellInformation,
    #[asn(optional_idx = 0)]
    pub i_e_extensions: Option<LastVisitedCellItemiE_Extensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "OCTET-STRING")]
pub struct LastVisitedEUTRANCellInformation(Vec<u8>);

#[derive(Debug, AperCodec)]
#[asn(type = "OCTET-STRING")]
pub struct LastVisitedGERANCellInformation(Vec<u8>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 3)]
pub struct LastVisitedNGRANCellInformation {
    pub global_cell_id: NGRAN_CGI,
    pub cell_type: CellType,
    pub time_ue_stayed_in_cell: TimeUEStayedInCell,
    #[asn(optional_idx = 0)]
    pub time_ue_stayed_in_cell_enhanced_granularity: Option<TimeUEStayedInCellEnhancedGranularity>,
    #[asn(optional_idx = 1)]
    pub h_o_cause_value: Option<Cause>,
    #[asn(optional_idx = 2)]
    pub i_e_extensions: Option<LastVisitedNGRANCellInformationiE_Extensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "OCTET-STRING")]
pub struct LastVisitedUTRANCellInformation(Vec<u8>);

#[derive(Debug, AperCodec)]
#[asn(type = "ENUMERATED", extensible = true, lb = "0", ub = "1")]
pub struct LineType(u8);
impl LineType {
    const DSL: u8 = 0u8;
    const PON: u8 = 1u8;
}

#[derive(Debug, AperCodec)]
#[asn(type = "ENUMERATED", extensible = true, lb = "0", ub = "2")]
pub struct Links_to_log(u8);
impl Links_to_log {
    const UPLINK: u8 = 0u8;
    const DOWNLINK: u8 = 1u8;
    const BOTH_UPLINK_AND_DOWNLINK: u8 = 2u8;
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true)]
pub struct LocationReport {
    pub protocol_i_es: LocationReportprotocolIEs,
}

#[derive(Debug, AperCodec)]
#[asn(type = "ENUMERATED", extensible = true, lb = "0", ub = "0")]
pub struct LocationReportingAdditionalInfo(u8);
impl LocationReportingAdditionalInfo {
    const INCLUDE_PS_CELL: u8 = 0u8;
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
#[asn(type = "INTEGER", lb = "1", ub = "64", extensible = true)]
pub struct LocationReportingReferenceID(u8);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 3)]
pub struct LocationReportingRequestType {
    pub event_type: EventType,
    pub report_area: ReportArea,
    #[asn(optional_idx = 0)]
    pub area_of_interest_list: Option<AreaOfInterestList>,
    #[asn(optional_idx = 1)]
    pub location_reporting_reference_id_to_be_cancelled: Option<LocationReportingReferenceID>,
    #[asn(optional_idx = 2)]
    pub i_e_extensions: Option<LocationReportingRequestTypeiE_Extensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 5)]
pub struct LoggedMDTNr {
    pub logging_interval: LoggingInterval,
    pub logging_duration: LoggingDuration,
    pub logged_mdt_trigger: LoggedMDTTrigger,
    #[asn(optional_idx = 0)]
    pub bluetooth_measurement_configuration: Option<BluetoothMeasurementConfiguration>,
    #[asn(optional_idx = 1)]
    pub w_lan_measurement_configuration: Option<WLANMeasurementConfiguration>,
    #[asn(optional_idx = 2)]
    pub sensor_measurement_configuration: Option<SensorMeasurementConfiguration>,
    #[asn(optional_idx = 3)]
    pub area_scope_of_neigh_cells_list: Option<AreaScopeOfNeighCellsList>,
    #[asn(optional_idx = 4)]
    pub i_e_extensions: Option<LoggedMDTNriE_Extensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "CHOICE", lb = "0", ub = "2", extensible = false)]
pub enum LoggedMDTTrigger {
    #[asn(key = 0, extended = false)]
    periodical(NULL_30),
    #[asn(key = 1, extended = false)]
    eventTrigger(EventTrigger),
    #[asn(key = 2, extended = false)]
    choice_Extensions(LoggedMDTTriggerchoice_Extensions),
}

#[derive(Debug, AperCodec)]
#[asn(type = "ENUMERATED", extensible = true, lb = "0", ub = "5")]
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
#[asn(type = "ENUMERATED", extensible = true, lb = "0", ub = "10")]
pub struct LoggingInterval(u8);
impl LoggingInterval {
    const MS320: u8 = 0u8;
    const MS640: u8 = 1u8;
    const MS1280: u8 = 2u8;
    const MS2560: u8 = 3u8;
    const MS5120: u8 = 4u8;
    const MS10240: u8 = 5u8;
    const MS20480: u8 = 6u8;
    const MS30720: u8 = 7u8;
    const MS40960: u8 = 8u8;
    const MS61440: u8 = 9u8;
    const INFINITY: u8 = 10u8;
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 3)]
pub struct M1Configuration {
    pub m1reporting_trigger: M1ReportingTrigger,
    #[asn(optional_idx = 0)]
    pub m1threshold_event_a2: Option<M1ThresholdEventA2>,
    #[asn(optional_idx = 1)]
    pub m1periodic_reporting: Option<M1PeriodicReporting>,
    #[asn(optional_idx = 2)]
    pub i_e_extensions: Option<M1ConfigurationiE_Extensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct M1PeriodicReporting {
    pub report_interval: ReportIntervalMDT,
    pub report_amount: ReportAmountMDT,
    #[asn(optional_idx = 0)]
    pub i_e_extensions: Option<M1PeriodicReportingiE_Extensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "ENUMERATED", extensible = true, lb = "0", ub = "2")]
pub struct M1ReportingTrigger(u8);
impl M1ReportingTrigger {
    const PERIODIC: u8 = 0u8;
    const A2EVENTTRIGGERED: u8 = 1u8;
    const A2EVENTTRIGGERED_PERIODIC: u8 = 2u8;
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct M1ThresholdEventA2 {
    pub m1_threshold_type: M1ThresholdType,
    #[asn(optional_idx = 0)]
    pub i_e_extensions: Option<M1ThresholdEventA2iE_Extensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "CHOICE", lb = "0", ub = "3", extensible = false)]
pub enum M1ThresholdType {
    #[asn(key = 0, extended = false)]
    threshold_RSRP(Threshold_RSRP),
    #[asn(key = 1, extended = false)]
    threshold_RSRQ(Threshold_RSRQ),
    #[asn(key = 2, extended = false)]
    threshold_SINR(Threshold_SINR),
    #[asn(key = 3, extended = false)]
    choice_Extensions(M1ThresholdTypechoice_Extensions),
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
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct M6Configuration {
    pub m6report_interval: M6report_Interval,
    pub m6_links_to_log: Links_to_log,
    #[asn(optional_idx = 0)]
    pub i_e_extensions: Option<M6ConfigurationiE_Extensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "ENUMERATED", extensible = true, lb = "0", ub = "13")]
pub struct M6report_Interval(u8);
impl M6report_Interval {
    const MS120: u8 = 0u8;
    const MS240: u8 = 1u8;
    const MS480: u8 = 2u8;
    const MS640: u8 = 3u8;
    const MS1024: u8 = 4u8;
    const MS2048: u8 = 5u8;
    const MS5120: u8 = 6u8;
    const MS10240: u8 = 7u8;
    const MS20480: u8 = 8u8;
    const MS40960: u8 = 9u8;
    const MIN1: u8 = 10u8;
    const MIN6: u8 = 11u8;
    const MIN12: u8 = 12u8;
    const MIN30: u8 = 13u8;
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
#[asn(type = "ENUMERATED", extensible = true, lb = "0", ub = "2")]
pub struct MDT_Activation(u8);
impl MDT_Activation {
    const IMMEDIATE_MDT_ONLY: u8 = 0u8;
    const LOGGED_MDT_ONLY: u8 = 1u8;
    const IMMEDIATE_MDT_AND_TRACE: u8 = 2u8;
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 3)]
pub struct MDT_Configuration {
    #[asn(optional_idx = 0)]
    pub mdt_config_nr: Option<MDT_Configuration_NR>,
    #[asn(optional_idx = 1)]
    pub mdt_config_eutra: Option<MDT_Configuration_EUTRA>,
    #[asn(optional_idx = 2)]
    pub i_e_extensions: Option<MDT_ConfigurationiE_Extensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 2)]
pub struct MDT_Configuration_EUTRA {
    pub mdt_activation: MDT_Activation,
    pub area_scope_of_mdt: AreaScopeOfMDT_EUTRA,
    pub m_dt_mode: MDTModeEutra,
    #[asn(optional_idx = 0)]
    pub signalling_based_mdtplmn_list: Option<MDTPLMNList>,
    #[asn(optional_idx = 1)]
    pub i_e_extensions: Option<MDT_Configuration_EUTRAiE_Extensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 2)]
pub struct MDT_Configuration_NR {
    pub mdt_activation: MDT_Activation,
    pub area_scope_of_mdt: AreaScopeOfMDT_NR,
    pub m_dt_mode_nr: MDTModeNr,
    #[asn(optional_idx = 0)]
    pub signalling_based_mdtplmn_list: Option<MDTPLMNList>,
    #[asn(optional_idx = 1)]
    pub i_e_extensions: Option<MDT_Configuration_NRiE_Extensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct MDT_Location_Info {
    pub m_dt_location_information: MDT_Location_Information,
    #[asn(optional_idx = 0)]
    pub i_e_extensions: Option<MDT_Location_InfoiE_Extensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "BITSTRING", sz_extensible = false, sz_lb = "8", sz_ub = "8")]
pub struct MDT_Location_Information(BitVec<Msb0, u8>);

#[derive(Debug, AperCodec)]
#[asn(type = "OCTET-STRING")]
pub struct MDTModeEutra(Vec<u8>);

#[derive(Debug, AperCodec)]
#[asn(type = "CHOICE", lb = "0", ub = "2", extensible = false)]
pub enum MDTModeNr {
    #[asn(key = 0, extended = false)]
    immediateMDTNr(ImmediateMDTNr),
    #[asn(key = 1, extended = false)]
    loggedMDTNr(LoggedMDTNr),
    #[asn(key = 2, extended = false)]
    choice_Extensions(MDTModeNrchoice_Extensions),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "16")]
pub struct MDTPLMNList(Vec<PLMNIdentity>);

#[derive(Debug, AperCodec)]
#[asn(type = "ENUMERATED", extensible = true, lb = "0", ub = "0")]
pub struct MICOModeIndication(u8);
impl MICOModeIndication {
    const TRUE: u8 = 0u8;
}

#[derive(Debug, AperCodec)]
#[asn(type = "BITSTRING", sz_extensible = false, sz_lb = "64", sz_ub = "64")]
pub struct MaskedIMEISV(BitVec<Msb0, u8>);

#[derive(Debug, AperCodec)]
#[asn(type = "INTEGER", lb = "0", ub = "4095", extensible = true)]
pub struct MaximumDataBurstVolume(u16);

#[derive(Debug, AperCodec)]
#[asn(type = "ENUMERATED", extensible = true, lb = "0", ub = "1")]
pub struct MaximumIntegrityProtectedDataRate(u8);
impl MaximumIntegrityProtectedDataRate {
    const BITRATE64KBS: u8 = 0u8;
    const MAXIMUM_UE_RATE: u8 = 1u8;
}

#[derive(Debug, AperCodec)]
#[asn(type = "CHOICE", lb = "0", ub = "2", extensible = false)]
pub enum MeasurementThresholdL1LoggedMDT {
    #[asn(key = 0, extended = false)]
    threshold_RSRP(Threshold_RSRP),
    #[asn(key = 1, extended = false)]
    threshold_RSRQ(Threshold_RSRQ),
    #[asn(key = 2, extended = false)]
    choice_Extensions(MeasurementThresholdL1LoggedMDTchoice_Extensions),
}

#[derive(Debug, AperCodec)]
#[asn(type = "BITSTRING", sz_extensible = false, sz_lb = "8", sz_ub = "8")]
pub struct MeasurementsToActivate(BitVec<Msb0, u8>);

#[derive(Debug, AperCodec)]
#[asn(type = "BITSTRING", sz_extensible = false, sz_lb = "16", sz_ub = "16")]
pub struct MessageIdentifier(BitVec<Msb0, u8>);

#[derive(Debug, AperCodec)]
#[asn(type = "BITSTRING", sz_extensible = false, sz_lb = "16", sz_ub = "16")]
pub struct MobilityInformation(BitVec<Msb0, u8>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 5)]
pub struct MobilityRestrictionList {
    pub serving_plmn: PLMNIdentity,
    #[asn(optional_idx = 0)]
    pub equivalent_plm_ns: Option<EquivalentPLMNs>,
    #[asn(optional_idx = 1)]
    pub r_at_restrictions: Option<RATRestrictions>,
    #[asn(optional_idx = 2)]
    pub forbidden_area_information: Option<ForbiddenAreaInformation>,
    #[asn(optional_idx = 3)]
    pub service_area_information: Option<ServiceAreaInformation>,
    #[asn(optional_idx = 4)]
    pub i_e_extensions: Option<MobilityRestrictionListiE_Extensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "CHOICE", lb = "0", ub = "1", extensible = false)]
pub enum N3IWF_ID {
    #[asn(key = 0, extended = false)]
    n3IWF_ID(BIT_STRING_31),
    #[asn(key = 1, extended = false)]
    choice_Extensions(N3IWF_IDchoice_Extensions),
}

#[derive(Debug, AperCodec)]
#[asn(type = "OCTET-STRING")]
pub struct NAS_PDU(Vec<u8>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true)]
pub struct NASNonDeliveryIndication {
    pub protocol_i_es: NASNonDeliveryIndicationprotocolIEs,
}

#[derive(Debug, AperCodec)]
#[asn(type = "OCTET-STRING")]
pub struct NASSecurityParametersFromNGRAN(Vec<u8>);

#[derive(Debug, AperCodec)]
#[asn(type = "ENUMERATED", extensible = true, lb = "0", ub = "3")]
pub struct NB_IoT_DefaultPagingDRX(u8);
impl NB_IoT_DefaultPagingDRX {
    const RF128: u8 = 0u8;
    const RF256: u8 = 1u8;
    const RF512: u8 = 2u8;
    const RF1024: u8 = 3u8;
}

#[derive(Debug, AperCodec)]
#[asn(type = "ENUMERATED", extensible = true, lb = "0", ub = "15")]
pub struct NB_IoT_Paging_TimeWindow(u8);
impl NB_IoT_Paging_TimeWindow {
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
#[asn(type = "ENUMERATED", extensible = true, lb = "0", ub = "13")]
pub struct NB_IoT_Paging_eDRXCycle(u8);
impl NB_IoT_Paging_eDRXCycle {
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
pub struct NB_IoT_Paging_eDRXInfo {
    pub n_b_io_t_paging_e_drx_cycle: NB_IoT_Paging_eDRXCycle,
    #[asn(optional_idx = 0)]
    pub n_b_io_t_paging_time_window: Option<NB_IoT_Paging_TimeWindow>,
    #[asn(optional_idx = 1)]
    pub i_e_extensions: Option<NB_IoT_Paging_eDRXInfoiE_Extensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "ENUMERATED", extensible = true, lb = "0", ub = "5")]
pub struct NB_IoT_PagingDRX(u8);
impl NB_IoT_PagingDRX {
    const RF32: u8 = 0u8;
    const RF64: u8 = 1u8;
    const RF128: u8 = 2u8;
    const RF256: u8 = 3u8;
    const RF512: u8 = 4u8;
    const RF1024: u8 = 5u8;
}

#[derive(Debug, AperCodec)]
#[asn(type = "INTEGER", lb = "0", ub = "255", extensible = true)]
pub struct NB_IoT_UEPriority(u8);

#[derive(Debug, AperCodec)]
#[asn(type = "CHOICE", lb = "0", ub = "2", extensible = true)]
pub enum NGAP_PDU {
    #[asn(key = 0, extended = false)]
    initiatingMessage(InitiatingMessage),
    #[asn(key = 1, extended = false)]
    successfulOutcome(SuccessfulOutcome),
    #[asn(key = 2, extended = false)]
    unsuccessfulOutcome(UnsuccessfulOutcome),
}

#[derive(Debug, AperCodec)]
#[asn(type = "CHOICE", lb = "0", ub = "2", extensible = false)]
pub enum NGRAN_CGI {
    #[asn(key = 0, extended = false)]
    nR_CGI(NR_CGI),
    #[asn(key = 1, extended = false)]
    eUTRA_CGI(EUTRA_CGI),
    #[asn(key = 2, extended = false)]
    choice_Extensions(NGRAN_CGIchoice_Extensions),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false, optional_fields = 2)]
pub struct NGRAN_TNLAssociationToRemoveItem {
    pub t_nl_association_transport_layer_address: CPTransportLayerInformation,
    #[asn(optional_idx = 0)]
    pub t_nl_association_transport_layer_address_amf: Option<CPTransportLayerInformation>,
    #[asn(optional_idx = 1)]
    pub i_e_extensions: Option<NGRAN_TNLAssociationToRemoveItemiE_Extensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "32")]
pub struct NGRAN_TNLAssociationToRemoveList(Vec<NGRAN_TNLAssociationToRemoveItem>);

#[derive(Debug, AperCodec)]
#[asn(type = "OCTET-STRING", sz_extensible = false, sz_lb = "8", sz_ub = "8")]
pub struct NGRANTraceID(Vec<u8>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true)]
pub struct NGReset {
    pub protocol_i_es: NGResetprotocolIEs,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true)]
pub struct NGResetAcknowledge {
    pub protocol_i_es: NGResetAcknowledgeprotocolIEs,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true)]
pub struct NGSetupFailure {
    pub protocol_i_es: NGSetupFailureprotocolIEs,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true)]
pub struct NGSetupRequest {
    pub protocol_i_es: NGSetupRequestprotocolIEs,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true)]
pub struct NGSetupResponse {
    pub protocol_i_es: NGSetupResponseprotocolIEs,
}

#[derive(Debug, AperCodec)]
#[asn(type = "BITSTRING", sz_extensible = false, sz_lb = "44", sz_ub = "44")]
pub struct NID(BitVec<Msb0, u8>);

#[derive(Debug, AperCodec)]
#[asn(type = "CHOICE", lb = "0", ub = "1", extensible = false)]
pub enum NPN_AccessInformation {
    #[asn(key = 0, extended = false)]
    pNI_NPN_Access_Information(CellCAGList),
    #[asn(key = 1, extended = false)]
    choice_Extensions(NPN_AccessInformationchoice_Extensions),
}

#[derive(Debug, AperCodec)]
#[asn(type = "CHOICE", lb = "0", ub = "2", extensible = false)]
pub enum NPN_MobilityInformation {
    #[asn(key = 0, extended = false)]
    sNPN_MobilityInformation(SNPN_MobilityInformation),
    #[asn(key = 1, extended = false)]
    pNI_NPN_MobilityInformation(PNI_NPN_MobilityInformation),
    #[asn(key = 2, extended = false)]
    choice_Extensions(NPN_MobilityInformationchoice_Extensions),
}

#[derive(Debug, AperCodec)]
#[asn(type = "CHOICE", lb = "0", ub = "1", extensible = false)]
pub enum NPN_PagingAssistanceInformation {
    #[asn(key = 0, extended = false)]
    pNI_NPN_PagingAssistance(Allowed_PNI_NPN_List),
    #[asn(key = 1, extended = false)]
    choice_Extensions(NPN_PagingAssistanceInformationchoice_Extensions),
}

#[derive(Debug, AperCodec)]
#[asn(type = "CHOICE", lb = "0", ub = "1", extensible = false)]
pub enum NPN_Support {
    #[asn(key = 0, extended = false)]
    sNPN(NID),
    #[asn(key = 1, extended = false)]
    choice_Extensions(NPN_Supportchoice_Extensions),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct NR_CGI {
    pub p_lmn_identity: PLMNIdentity,
    pub n_r_cell_identity: NRCellIdentity,
    #[asn(optional_idx = 0)]
    pub i_e_extensions: Option<NR_CGIiE_Extensions>,
}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "16384"
)]
pub struct NR_CGIList(Vec<NR_CGI>);

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct NR_CGIListForWarning(Vec<NR_CGI>);

#[derive(Debug, AperCodec)]
#[asn(type = "INTEGER", lb = "0", ub = "1007", extensible = true)]
pub struct NR_PCI(u16);

#[derive(Debug, AperCodec)]
#[asn(type = "INTEGER", lb = "0", ub = "3279165")]
pub struct NRARFCN(u32);

#[derive(Debug, AperCodec)]
#[asn(type = "BITSTRING", sz_extensible = false, sz_lb = "36", sz_ub = "36")]
pub struct NRCellIdentity(BitVec<Msb0, u8>);

#[derive(Debug, AperCodec)]
#[asn(type = "INTEGER", lb = "1", ub = "1024", extensible = true)]
pub struct NRFrequencyBand(u16);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "32")]
pub struct NRFrequencyBand_List(Vec<NRFrequencyBandItem>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct NRFrequencyBandItem {
    pub nr_frequency_band: NRFrequencyBand,
    #[asn(optional_idx = 0)]
    pub i_e_extension: Option<NRFrequencyBandItemiE_Extension>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct NRFrequencyInfo {
    pub nr_arfcn: NRARFCN,
    pub frequency_band_list: NRFrequencyBand_List,
    #[asn(optional_idx = 0)]
    pub i_e_extension: Option<NRFrequencyInfoiE_Extension>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "OCTET-STRING")]
pub struct NRMobilityHistoryReport(Vec<u8>);

#[derive(Debug, AperCodec)]
#[asn(type = "OCTET-STRING")]
pub struct NRPPa_PDU(Vec<u8>);

#[derive(Debug, AperCodec)]
#[asn(type = "OCTET-STRING")]
pub struct NRUERLFReportContainer(Vec<u8>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct NRUESidelinkAggregateMaximumBitrate {
    pub u_e_sidelink_aggregate_maximum_bit_rate: BitRate,
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
#[asn(type = "INTEGER", lb = "1", ub = "256", extensible = true)]
pub struct NetworkInstance(u16);

#[derive(Debug, AperCodec)]
#[asn(type = "ENUMERATED", extensible = true, lb = "0", ub = "0")]
pub struct NewSecurityContextInd(u8);
impl NewSecurityContextInd {
    const TRUE: u8 = 0u8;
}

#[derive(Debug, AperCodec)]
#[asn(type = "INTEGER", lb = "0", ub = "7")]
pub struct NextHopChainingCount(u8);

#[derive(Debug, AperCodec)]
#[asn(type = "ENUMERATED", extensible = true, lb = "0", ub = "1")]
pub struct NextPagingAreaScope(u8);
impl NextPagingAreaScope {
    const SAME: u8 = 0u8;
    const CHANGED: u8 = 1u8;
}

#[derive(Debug, AperCodec)]
#[asn(type = "CHOICE", lb = "0", ub = "3", extensible = false)]
pub enum NgENB_ID {
    #[asn(key = 0, extended = false)]
    macroNgENB_ID(BIT_STRING_32),
    #[asn(key = 1, extended = false)]
    shortMacroNgENB_ID(BIT_STRING_33),
    #[asn(key = 2, extended = false)]
    longMacroNgENB_ID(BIT_STRING_34),
    #[asn(key = 3, extended = false)]
    choice_Extensions(NgENB_IDchoice_Extensions),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 4)]
pub struct NonDynamic5QIDescriptor {
    pub five_qi: FiveQI,
    #[asn(optional_idx = 0)]
    pub priority_level_qos: Option<PriorityLevelQos>,
    #[asn(optional_idx = 1)]
    pub averaging_window: Option<AveragingWindow>,
    #[asn(optional_idx = 2)]
    pub maximum_data_burst_volume: Option<MaximumDataBurstVolume>,
    #[asn(optional_idx = 3)]
    pub i_e_extensions: Option<NonDynamic5QIDescriptoriE_Extensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "16")]
pub struct NotAllowedTACs(Vec<TAC>);

#[derive(Debug, AperCodec)]
#[asn(type = "ENUMERATED", extensible = true, lb = "0", ub = "1")]
pub struct NotificationCause(u8);
impl NotificationCause {
    const FULFILLED: u8 = 0u8;
    const NOT_FULFILLED: u8 = 1u8;
}

#[derive(Debug, AperCodec)]
#[asn(type = "ENUMERATED", extensible = true, lb = "0", ub = "0")]
pub struct NotificationControl(u8);
impl NotificationControl {
    const NOTIFICATION_REQUESTED: u8 = 0u8;
}

#[derive(Debug, AperCodec)]
#[asn(type = "ENUMERATED", extensible = true, lb = "0", ub = "0")]
pub struct NotifySourceNGRANNode(u8);
impl NotifySourceNGRANNode {
    const NOTIFY_SOURCE: u8 = 0u8;
}

#[derive(Debug, AperCodec)]
#[asn(type = "INTEGER", lb = "0", ub = "65535")]
pub struct NumberOfBroadcasts(u16);

#[derive(Debug, AperCodec)]
#[asn(type = "INTEGER", lb = "0", ub = "65535")]
pub struct NumberOfBroadcastsRequested(u16);

#[derive(Debug, AperCodec)]
#[asn(type = "ENUMERATED", extensible = true, lb = "0", ub = "3")]
pub struct OverloadAction(u8);
impl OverloadAction {
    const REJECT_NON_EMERGENCY_MO_DT: u8 = 0u8;
    const REJECT_RRC_CR_SIGNALLING: u8 = 1u8;
    const PERMIT_EMERGENCY_SESSIONS_AND_MOBILE_TERMINATED_SERVICES_ONLY: u8 = 2u8;
    const PERMIT_HIGH_PRIORITY_SESSIONS_AND_MOBILE_TERMINATED_SERVICES_ONLY: u8 = 3u8;
}

#[derive(Debug, AperCodec)]
#[asn(type = "CHOICE", lb = "0", ub = "1", extensible = false)]
pub enum OverloadResponse {
    #[asn(key = 0, extended = false)]
    overloadAction(OverloadAction),
    #[asn(key = 1, extended = false)]
    choice_Extensions(OverloadResponsechoice_Extensions),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true)]
pub struct OverloadStart {
    pub protocol_i_es: OverloadStartprotocolIEs,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 3)]
pub struct OverloadStartNSSAIItem {
    pub slice_overload_list: SliceOverloadList,
    #[asn(optional_idx = 0)]
    pub slice_overload_response: Option<OverloadResponse>,
    #[asn(optional_idx = 1)]
    pub slice_traffic_load_reduction_indication: Option<TrafficLoadReductionIndication>,
    #[asn(optional_idx = 2)]
    pub i_e_extensions: Option<OverloadStartNSSAIItemiE_Extensions>,
}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "1024"
)]
pub struct OverloadStartNSSAIList(Vec<OverloadStartNSSAIItem>);

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
    pub pc5_link_aggregate_bit_rates: Option<BitRate>,
    #[asn(optional_idx = 1)]
    pub i_e_extensions: Option<PC5QoSParametersiE_Extensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "32")]
pub struct PCIListForMDT(Vec<NR_PCI>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct PDUSessionAggregateMaximumBitRate {
    pub p_du_session_aggregate_maximum_bit_rate_dl: BitRate,
    pub p_du_session_aggregate_maximum_bit_rate_ul: BitRate,
    #[asn(optional_idx = 0)]
    pub i_e_extensions: Option<PDUSessionAggregateMaximumBitRateiE_Extensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "INTEGER", lb = "0", ub = "255")]
pub struct PDUSessionID(u8);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct PDUSessionResourceAdmittedItem {
    pub p_du_session_id: PDUSessionID,
    pub handover_request_acknowledge_transfer: OCTET_STRING_35,
    #[asn(optional_idx = 0)]
    pub i_e_extensions: Option<PDUSessionResourceAdmittedItemiE_Extensions>,
}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "256"
)]
pub struct PDUSessionResourceAdmittedList(Vec<PDUSessionResourceAdmittedItem>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct PDUSessionResourceFailedToModifyItemModCfm {
    pub p_du_session_id: PDUSessionID,
    pub p_du_session_resource_modify_indication_unsuccessful_transfer: OCTET_STRING_36,
    #[asn(optional_idx = 0)]
    pub i_e_extensions: Option<PDUSessionResourceFailedToModifyItemModCfmiE_Extensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct PDUSessionResourceFailedToModifyItemModRes {
    pub p_du_session_id: PDUSessionID,
    pub p_du_session_resource_modify_unsuccessful_transfer: OCTET_STRING_37,
    #[asn(optional_idx = 0)]
    pub i_e_extensions: Option<PDUSessionResourceFailedToModifyItemModResiE_Extensions>,
}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "256"
)]
pub struct PDUSessionResourceFailedToModifyListModCfm(
    Vec<PDUSessionResourceFailedToModifyItemModCfm>,
);

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "256"
)]
pub struct PDUSessionResourceFailedToModifyListModRes(
    Vec<PDUSessionResourceFailedToModifyItemModRes>,
);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct PDUSessionResourceFailedToResumeItemRESReq {
    pub p_du_session_id: PDUSessionID,
    pub cause: Cause,
    #[asn(optional_idx = 0)]
    pub i_e_extensions: Option<PDUSessionResourceFailedToResumeItemRESReqiE_Extensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct PDUSessionResourceFailedToResumeItemRESRes {
    pub p_du_session_id: PDUSessionID,
    pub cause: Cause,
    #[asn(optional_idx = 0)]
    pub i_e_extensions: Option<PDUSessionResourceFailedToResumeItemRESResiE_Extensions>,
}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "256"
)]
pub struct PDUSessionResourceFailedToResumeListRESReq(
    Vec<PDUSessionResourceFailedToResumeItemRESReq>,
);

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "256"
)]
pub struct PDUSessionResourceFailedToResumeListRESRes(
    Vec<PDUSessionResourceFailedToResumeItemRESRes>,
);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct PDUSessionResourceFailedToSetupItemCxtFail {
    pub p_du_session_id: PDUSessionID,
    pub p_du_session_resource_setup_unsuccessful_transfer: OCTET_STRING_38,
    #[asn(optional_idx = 0)]
    pub i_e_extensions: Option<PDUSessionResourceFailedToSetupItemCxtFailiE_Extensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct PDUSessionResourceFailedToSetupItemCxtRes {
    pub p_du_session_id: PDUSessionID,
    pub p_du_session_resource_setup_unsuccessful_transfer: OCTET_STRING_39,
    #[asn(optional_idx = 0)]
    pub i_e_extensions: Option<PDUSessionResourceFailedToSetupItemCxtResiE_Extensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct PDUSessionResourceFailedToSetupItemHOAck {
    pub p_du_session_id: PDUSessionID,
    pub handover_resource_allocation_unsuccessful_transfer: OCTET_STRING_40,
    #[asn(optional_idx = 0)]
    pub i_e_extensions: Option<PDUSessionResourceFailedToSetupItemHOAckiE_Extensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct PDUSessionResourceFailedToSetupItemPSReq {
    pub p_du_session_id: PDUSessionID,
    pub path_switch_request_setup_failed_transfer: OCTET_STRING_41,
    #[asn(optional_idx = 0)]
    pub i_e_extensions: Option<PDUSessionResourceFailedToSetupItemPSReqiE_Extensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct PDUSessionResourceFailedToSetupItemSURes {
    pub p_du_session_id: PDUSessionID,
    pub p_du_session_resource_setup_unsuccessful_transfer: OCTET_STRING_42,
    #[asn(optional_idx = 0)]
    pub i_e_extensions: Option<PDUSessionResourceFailedToSetupItemSUResiE_Extensions>,
}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "256"
)]
pub struct PDUSessionResourceFailedToSetupListCxtFail(
    Vec<PDUSessionResourceFailedToSetupItemCxtFail>,
);

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "256"
)]
pub struct PDUSessionResourceFailedToSetupListCxtRes(
    Vec<PDUSessionResourceFailedToSetupItemCxtRes>,
);

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "256"
)]
pub struct PDUSessionResourceFailedToSetupListHOAck(Vec<PDUSessionResourceFailedToSetupItemHOAck>);

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "256"
)]
pub struct PDUSessionResourceFailedToSetupListPSReq(Vec<PDUSessionResourceFailedToSetupItemPSReq>);

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "256"
)]
pub struct PDUSessionResourceFailedToSetupListSURes(Vec<PDUSessionResourceFailedToSetupItemSURes>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct PDUSessionResourceHandoverItem {
    pub p_du_session_id: PDUSessionID,
    pub handover_command_transfer: OCTET_STRING_43,
    #[asn(optional_idx = 0)]
    pub i_e_extensions: Option<PDUSessionResourceHandoverItemiE_Extensions>,
}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "256"
)]
pub struct PDUSessionResourceHandoverList(Vec<PDUSessionResourceHandoverItem>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 2)]
pub struct PDUSessionResourceInformationItem {
    pub p_du_session_id: PDUSessionID,
    pub qos_flow_information_list: QosFlowInformationList,
    #[asn(optional_idx = 0)]
    pub d_r_bs_to_qos_flows_mapping_list: Option<DRBsToQosFlowsMappingList>,
    #[asn(optional_idx = 1)]
    pub i_e_extensions: Option<PDUSessionResourceInformationItemiE_Extensions>,
}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "256"
)]
pub struct PDUSessionResourceInformationList(Vec<PDUSessionResourceInformationItem>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct PDUSessionResourceItemCxtRelCpl {
    pub p_du_session_id: PDUSessionID,
    #[asn(optional_idx = 0)]
    pub i_e_extensions: Option<PDUSessionResourceItemCxtRelCpliE_Extensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct PDUSessionResourceItemCxtRelReq {
    pub p_du_session_id: PDUSessionID,
    #[asn(optional_idx = 0)]
    pub i_e_extensions: Option<PDUSessionResourceItemCxtRelReqiE_Extensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct PDUSessionResourceItemHORqd {
    pub p_du_session_id: PDUSessionID,
    pub handover_required_transfer: OCTET_STRING_44,
    #[asn(optional_idx = 0)]
    pub i_e_extensions: Option<PDUSessionResourceItemHORqdiE_Extensions>,
}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "256"
)]
pub struct PDUSessionResourceListCxtRelCpl(Vec<PDUSessionResourceItemCxtRelCpl>);

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "256"
)]
pub struct PDUSessionResourceListCxtRelReq(Vec<PDUSessionResourceItemCxtRelReq>);

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "256"
)]
pub struct PDUSessionResourceListHORqd(Vec<PDUSessionResourceItemHORqd>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true)]
pub struct PDUSessionResourceModifyConfirm {
    pub protocol_i_es: PDUSessionResourceModifyConfirmprotocolIEs,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 3)]
pub struct PDUSessionResourceModifyConfirmTransfer {
    pub qos_flow_modify_confirm_list: QosFlowModifyConfirmList,
    pub u_lngu_up_tnl_information: UPTransportLayerInformation,
    #[asn(optional_idx = 0)]
    pub additional_ng_uuptnl_information: Option<UPTransportLayerInformationPairList>,
    #[asn(optional_idx = 1)]
    pub qos_flow_failed_to_modify_list: Option<QosFlowListWithCause>,
    #[asn(optional_idx = 2)]
    pub i_e_extensions: Option<PDUSessionResourceModifyConfirmTransferiE_Extensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true)]
pub struct PDUSessionResourceModifyIndication {
    pub protocol_i_es: PDUSessionResourceModifyIndicationprotocolIEs,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 2)]
pub struct PDUSessionResourceModifyIndicationTransfer {
    pub d_l_qos_flow_per_tnl_information: QosFlowPerTNLInformation,
    #[asn(optional_idx = 0)]
    pub additional_dl_qos_flow_per_tnl_information: Option<QosFlowPerTNLInformationList>,
    #[asn(optional_idx = 1)]
    pub i_e_extensions: Option<PDUSessionResourceModifyIndicationTransferiE_Extensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct PDUSessionResourceModifyIndicationUnsuccessfulTransfer {
    pub cause: Cause,
    #[asn(optional_idx = 0)]
    pub i_e_extensions: Option<PDUSessionResourceModifyIndicationUnsuccessfulTransferiE_Extensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct PDUSessionResourceModifyItemModCfm {
    pub p_du_session_id: PDUSessionID,
    pub p_du_session_resource_modify_confirm_transfer: OCTET_STRING_45,
    #[asn(optional_idx = 0)]
    pub i_e_extensions: Option<PDUSessionResourceModifyItemModCfmiE_Extensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct PDUSessionResourceModifyItemModInd {
    pub p_du_session_id: PDUSessionID,
    pub p_du_session_resource_modify_indication_transfer: OCTET_STRING_46,
    #[asn(optional_idx = 0)]
    pub i_e_extensions: Option<PDUSessionResourceModifyItemModIndiE_Extensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 2)]
pub struct PDUSessionResourceModifyItemModReq {
    pub p_du_session_id: PDUSessionID,
    #[asn(optional_idx = 0)]
    pub n_as_pdu: Option<NAS_PDU>,
    pub p_du_session_resource_modify_request_transfer: OCTET_STRING_47,
    #[asn(optional_idx = 1)]
    pub i_e_extensions: Option<PDUSessionResourceModifyItemModReqiE_Extensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct PDUSessionResourceModifyItemModRes {
    pub p_du_session_id: PDUSessionID,
    pub p_du_session_resource_modify_response_transfer: OCTET_STRING_48,
    #[asn(optional_idx = 0)]
    pub i_e_extensions: Option<PDUSessionResourceModifyItemModResiE_Extensions>,
}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "256"
)]
pub struct PDUSessionResourceModifyListModCfm(Vec<PDUSessionResourceModifyItemModCfm>);

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "256"
)]
pub struct PDUSessionResourceModifyListModInd(Vec<PDUSessionResourceModifyItemModInd>);

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "256"
)]
pub struct PDUSessionResourceModifyListModReq(Vec<PDUSessionResourceModifyItemModReq>);

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "256"
)]
pub struct PDUSessionResourceModifyListModRes(Vec<PDUSessionResourceModifyItemModRes>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true)]
pub struct PDUSessionResourceModifyRequest {
    pub protocol_i_es: PDUSessionResourceModifyRequestprotocolIEs,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true)]
pub struct PDUSessionResourceModifyRequestTransfer {
    pub protocol_i_es: PDUSessionResourceModifyRequestTransferprotocolIEs,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true)]
pub struct PDUSessionResourceModifyResponse {
    pub protocol_i_es: PDUSessionResourceModifyResponseprotocolIEs,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 6)]
pub struct PDUSessionResourceModifyResponseTransfer {
    #[asn(optional_idx = 0)]
    pub d_l_ngu_up_tnl_information: Option<UPTransportLayerInformation>,
    #[asn(optional_idx = 1)]
    pub u_l_ngu_up_tnl_information: Option<UPTransportLayerInformation>,
    #[asn(optional_idx = 2)]
    pub qos_flow_add_or_modify_response_list: Option<QosFlowAddOrModifyResponseList>,
    #[asn(optional_idx = 3)]
    pub additional_dl_qos_flow_per_tnl_information: Option<QosFlowPerTNLInformationList>,
    #[asn(optional_idx = 4)]
    pub qos_flow_failed_to_add_or_modify_list: Option<QosFlowListWithCause>,
    #[asn(optional_idx = 5)]
    pub i_e_extensions: Option<PDUSessionResourceModifyResponseTransferiE_Extensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 2)]
pub struct PDUSessionResourceModifyUnsuccessfulTransfer {
    pub cause: Cause,
    #[asn(optional_idx = 0)]
    pub criticality_diagnostics: Option<CriticalityDiagnostics>,
    #[asn(optional_idx = 1)]
    pub i_e_extensions: Option<PDUSessionResourceModifyUnsuccessfulTransferiE_Extensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true)]
pub struct PDUSessionResourceNotify {
    pub protocol_i_es: PDUSessionResourceNotifyprotocolIEs,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct PDUSessionResourceNotifyItem {
    pub p_du_session_id: PDUSessionID,
    pub p_du_session_resource_notify_transfer: OCTET_STRING_49,
    #[asn(optional_idx = 0)]
    pub i_e_extensions: Option<PDUSessionResourceNotifyItemiE_Extensions>,
}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "256"
)]
pub struct PDUSessionResourceNotifyList(Vec<PDUSessionResourceNotifyItem>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct PDUSessionResourceNotifyReleasedTransfer {
    pub cause: Cause,
    #[asn(optional_idx = 0)]
    pub i_e_extensions: Option<PDUSessionResourceNotifyReleasedTransferiE_Extensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 3)]
pub struct PDUSessionResourceNotifyTransfer {
    #[asn(optional_idx = 0)]
    pub qos_flow_notify_list: Option<QosFlowNotifyList>,
    #[asn(optional_idx = 1)]
    pub qos_flow_released_list: Option<QosFlowListWithCause>,
    #[asn(optional_idx = 2)]
    pub i_e_extensions: Option<PDUSessionResourceNotifyTransferiE_Extensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true)]
pub struct PDUSessionResourceReleaseCommand {
    pub protocol_i_es: PDUSessionResourceReleaseCommandprotocolIEs,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct PDUSessionResourceReleaseCommandTransfer {
    pub cause: Cause,
    #[asn(optional_idx = 0)]
    pub i_e_extensions: Option<PDUSessionResourceReleaseCommandTransferiE_Extensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true)]
pub struct PDUSessionResourceReleaseResponse {
    pub protocol_i_es: PDUSessionResourceReleaseResponseprotocolIEs,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct PDUSessionResourceReleaseResponseTransfer {
    #[asn(optional_idx = 0)]
    pub i_e_extensions: Option<PDUSessionResourceReleaseResponseTransferiE_Extensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct PDUSessionResourceReleasedItemNot {
    pub p_du_session_id: PDUSessionID,
    pub p_du_session_resource_notify_released_transfer: OCTET_STRING_50,
    #[asn(optional_idx = 0)]
    pub i_e_extensions: Option<PDUSessionResourceReleasedItemNotiE_Extensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct PDUSessionResourceReleasedItemPSAck {
    pub p_du_session_id: PDUSessionID,
    pub path_switch_request_unsuccessful_transfer: OCTET_STRING_51,
    #[asn(optional_idx = 0)]
    pub i_e_extensions: Option<PDUSessionResourceReleasedItemPSAckiE_Extensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct PDUSessionResourceReleasedItemPSFail {
    pub p_du_session_id: PDUSessionID,
    pub path_switch_request_unsuccessful_transfer: OCTET_STRING_52,
    #[asn(optional_idx = 0)]
    pub i_e_extensions: Option<PDUSessionResourceReleasedItemPSFailiE_Extensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct PDUSessionResourceReleasedItemRelRes {
    pub p_du_session_id: PDUSessionID,
    pub p_du_session_resource_release_response_transfer: OCTET_STRING_53,
    #[asn(optional_idx = 0)]
    pub i_e_extensions: Option<PDUSessionResourceReleasedItemRelResiE_Extensions>,
}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "256"
)]
pub struct PDUSessionResourceReleasedListNot(Vec<PDUSessionResourceReleasedItemNot>);

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "256"
)]
pub struct PDUSessionResourceReleasedListPSAck(Vec<PDUSessionResourceReleasedItemPSAck>);

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "256"
)]
pub struct PDUSessionResourceReleasedListPSFail(Vec<PDUSessionResourceReleasedItemPSFail>);

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "256"
)]
pub struct PDUSessionResourceReleasedListRelRes(Vec<PDUSessionResourceReleasedItemRelRes>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct PDUSessionResourceResumeItemRESReq {
    pub p_du_session_id: PDUSessionID,
    pub u_e_context_resume_request_transfer: OCTET_STRING_54,
    #[asn(optional_idx = 0)]
    pub i_e_extensions: Option<PDUSessionResourceResumeItemRESReqiE_Extensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct PDUSessionResourceResumeItemRESRes {
    pub p_du_session_id: PDUSessionID,
    pub u_e_context_resume_response_transfer: OCTET_STRING_55,
    #[asn(optional_idx = 0)]
    pub i_e_extensions: Option<PDUSessionResourceResumeItemRESResiE_Extensions>,
}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "256"
)]
pub struct PDUSessionResourceResumeListRESReq(Vec<PDUSessionResourceResumeItemRESReq>);

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "256"
)]
pub struct PDUSessionResourceResumeListRESRes(Vec<PDUSessionResourceResumeItemRESRes>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct PDUSessionResourceSecondaryRATUsageItem {
    pub p_du_session_id: PDUSessionID,
    pub secondary_rat_data_usage_report_transfer: OCTET_STRING_56,
    #[asn(optional_idx = 0)]
    pub i_e_extensions: Option<PDUSessionResourceSecondaryRATUsageItemiE_Extensions>,
}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "256"
)]
pub struct PDUSessionResourceSecondaryRATUsageList(Vec<PDUSessionResourceSecondaryRATUsageItem>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 2)]
pub struct PDUSessionResourceSetupItemCxtReq {
    pub p_du_session_id: PDUSessionID,
    #[asn(optional_idx = 0)]
    pub n_as_pdu: Option<NAS_PDU>,
    pub s_nssai: S_NSSAI,
    pub p_du_session_resource_setup_request_transfer: OCTET_STRING_57,
    #[asn(optional_idx = 1)]
    pub i_e_extensions: Option<PDUSessionResourceSetupItemCxtReqiE_Extensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct PDUSessionResourceSetupItemCxtRes {
    pub p_du_session_id: PDUSessionID,
    pub p_du_session_resource_setup_response_transfer: OCTET_STRING_58,
    #[asn(optional_idx = 0)]
    pub i_e_extensions: Option<PDUSessionResourceSetupItemCxtResiE_Extensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct PDUSessionResourceSetupItemHOReq {
    pub p_du_session_id: PDUSessionID,
    pub s_nssai: S_NSSAI,
    pub handover_request_transfer: OCTET_STRING_59,
    #[asn(optional_idx = 0)]
    pub i_e_extensions: Option<PDUSessionResourceSetupItemHOReqiE_Extensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 2)]
pub struct PDUSessionResourceSetupItemSUReq {
    pub p_du_session_id: PDUSessionID,
    #[asn(optional_idx = 0)]
    pub p_du_session_nas_pdu: Option<NAS_PDU>,
    pub s_nssai: S_NSSAI,
    pub p_du_session_resource_setup_request_transfer: OCTET_STRING_60,
    #[asn(optional_idx = 1)]
    pub i_e_extensions: Option<PDUSessionResourceSetupItemSUReqiE_Extensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct PDUSessionResourceSetupItemSURes {
    pub p_du_session_id: PDUSessionID,
    pub p_du_session_resource_setup_response_transfer: OCTET_STRING_61,
    #[asn(optional_idx = 0)]
    pub i_e_extensions: Option<PDUSessionResourceSetupItemSUResiE_Extensions>,
}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "256"
)]
pub struct PDUSessionResourceSetupListCxtReq(Vec<PDUSessionResourceSetupItemCxtReq>);

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "256"
)]
pub struct PDUSessionResourceSetupListCxtRes(Vec<PDUSessionResourceSetupItemCxtRes>);

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "256"
)]
pub struct PDUSessionResourceSetupListHOReq(Vec<PDUSessionResourceSetupItemHOReq>);

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "256"
)]
pub struct PDUSessionResourceSetupListSUReq(Vec<PDUSessionResourceSetupItemSUReq>);

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "256"
)]
pub struct PDUSessionResourceSetupListSURes(Vec<PDUSessionResourceSetupItemSURes>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true)]
pub struct PDUSessionResourceSetupRequest {
    pub protocol_i_es: PDUSessionResourceSetupRequestprotocolIEs,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true)]
pub struct PDUSessionResourceSetupRequestTransfer {
    pub protocol_i_es: PDUSessionResourceSetupRequestTransferprotocolIEs,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true)]
pub struct PDUSessionResourceSetupResponse {
    pub protocol_i_es: PDUSessionResourceSetupResponseprotocolIEs,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 4)]
pub struct PDUSessionResourceSetupResponseTransfer {
    pub d_l_qos_flow_per_tnl_information: QosFlowPerTNLInformation,
    #[asn(optional_idx = 0)]
    pub additional_dl_qos_flow_per_tnl_information: Option<QosFlowPerTNLInformationList>,
    #[asn(optional_idx = 1)]
    pub security_result: Option<SecurityResult>,
    #[asn(optional_idx = 2)]
    pub qos_flow_failed_to_setup_list: Option<QosFlowListWithCause>,
    #[asn(optional_idx = 3)]
    pub i_e_extensions: Option<PDUSessionResourceSetupResponseTransferiE_Extensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 2)]
pub struct PDUSessionResourceSetupUnsuccessfulTransfer {
    pub cause: Cause,
    #[asn(optional_idx = 0)]
    pub criticality_diagnostics: Option<CriticalityDiagnostics>,
    #[asn(optional_idx = 1)]
    pub i_e_extensions: Option<PDUSessionResourceSetupUnsuccessfulTransferiE_Extensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct PDUSessionResourceSuspendItemSUSReq {
    pub p_du_session_id: PDUSessionID,
    pub u_e_context_suspend_request_transfer: OCTET_STRING_62,
    #[asn(optional_idx = 0)]
    pub i_e_extensions: Option<PDUSessionResourceSuspendItemSUSReqiE_Extensions>,
}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "256"
)]
pub struct PDUSessionResourceSuspendListSUSReq(Vec<PDUSessionResourceSuspendItemSUSReq>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct PDUSessionResourceSwitchedItem {
    pub p_du_session_id: PDUSessionID,
    pub path_switch_request_acknowledge_transfer: OCTET_STRING_63,
    #[asn(optional_idx = 0)]
    pub i_e_extensions: Option<PDUSessionResourceSwitchedItemiE_Extensions>,
}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "256"
)]
pub struct PDUSessionResourceSwitchedList(Vec<PDUSessionResourceSwitchedItem>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct PDUSessionResourceToBeSwitchedDLItem {
    pub p_du_session_id: PDUSessionID,
    pub path_switch_request_transfer: OCTET_STRING_64,
    #[asn(optional_idx = 0)]
    pub i_e_extensions: Option<PDUSessionResourceToBeSwitchedDLItemiE_Extensions>,
}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "256"
)]
pub struct PDUSessionResourceToBeSwitchedDLList(Vec<PDUSessionResourceToBeSwitchedDLItem>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct PDUSessionResourceToReleaseItemHOCmd {
    pub p_du_session_id: PDUSessionID,
    pub handover_preparation_unsuccessful_transfer: OCTET_STRING_65,
    #[asn(optional_idx = 0)]
    pub i_e_extensions: Option<PDUSessionResourceToReleaseItemHOCmdiE_Extensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct PDUSessionResourceToReleaseItemRelCmd {
    pub p_du_session_id: PDUSessionID,
    pub p_du_session_resource_release_command_transfer: OCTET_STRING_66,
    #[asn(optional_idx = 0)]
    pub i_e_extensions: Option<PDUSessionResourceToReleaseItemRelCmdiE_Extensions>,
}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "256"
)]
pub struct PDUSessionResourceToReleaseListHOCmd(Vec<PDUSessionResourceToReleaseItemHOCmd>);

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "256"
)]
pub struct PDUSessionResourceToReleaseListRelCmd(Vec<PDUSessionResourceToReleaseItemRelCmd>);

#[derive(Debug, AperCodec)]
#[asn(type = "ENUMERATED", extensible = true, lb = "0", ub = "4")]
pub struct PDUSessionType(u8);
impl PDUSessionType {
    const IPV4: u8 = 0u8;
    const IPV6: u8 = 1u8;
    const IPV4V6: u8 = 2u8;
    const ETHERNET: u8 = 3u8;
    const UNSTRUCTURED: u8 = 4u8;
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct PDUSessionUsageReport {
    pub r_at_type: ENUMERATED_67,
    pub p_du_session_timed_report_list: VolumeTimedReportList,
    #[asn(optional_idx = 0)]
    pub i_e_extensions: Option<PDUSessionUsageReportiE_Extensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "OCTET-STRING", sz_extensible = false, sz_lb = "3", sz_ub = "3")]
pub struct PLMNIdentity(Vec<u8>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct PLMNSupportItem {
    pub p_lmn_identity: PLMNIdentity,
    pub slice_support_list: SliceSupportList,
    #[asn(optional_idx = 0)]
    pub i_e_extensions: Option<PLMNSupportItemiE_Extensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "12")]
pub struct PLMNSupportList(Vec<PLMNSupportItem>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct PNI_NPN_MobilityInformation {
    pub allowed_pni_npi_list: Allowed_PNI_NPN_List,
    #[asn(optional_idx = 0)]
    pub i_e_extensions: Option<PNI_NPN_MobilityInformationiE_Extensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true)]
pub struct PWSCancelRequest {
    pub protocol_i_es: PWSCancelRequestprotocolIEs,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true)]
pub struct PWSCancelResponse {
    pub protocol_i_es: PWSCancelResponseprotocolIEs,
}

#[derive(Debug, AperCodec)]
#[asn(type = "CHOICE", lb = "0", ub = "2", extensible = false)]
pub enum PWSFailedCellIDList {
    #[asn(key = 0, extended = false)]
    eUTRA_CGI_PWSFailedList(EUTRA_CGIList),
    #[asn(key = 1, extended = false)]
    nR_CGI_PWSFailedList(NR_CGIList),
    #[asn(key = 2, extended = false)]
    choice_Extensions(PWSFailedCellIDListchoice_Extensions),
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
#[asn(type = "INTEGER", lb = "0", ub = "1023", extensible = true)]
pub struct PacketDelayBudget(u16);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct PacketErrorRate {
    pub p_er_scalar: INTEGER_68,
    pub p_er_exponent: INTEGER_69,
    #[asn(optional_idx = 0)]
    pub i_e_extensions: Option<PacketErrorRateiE_Extensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "INTEGER", lb = "0", ub = "1000", extensible = true)]
pub struct PacketLossRate(u16);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true)]
pub struct Paging {
    pub protocol_i_es: PagingprotocolIEs,
}

#[derive(Debug, AperCodec)]
#[asn(type = "ENUMERATED", extensible = true, lb = "0", ub = "15")]
pub struct Paging_Time_Window(u8);
impl Paging_Time_Window {
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
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct PagingAssisDataforCEcapabUE {
    pub e_utra_cgi: EUTRA_CGI,
    pub coverage_enhancement_level: CoverageEnhancementLevel,
    #[asn(optional_idx = 0)]
    pub i_e_extensions: Option<PagingAssisDataforCEcapabUEiE_Extensions>,
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
#[asn(type = "ENUMERATED", extensible = true, lb = "0", ub = "0")]
pub struct PagingOrigin(u8);
impl PagingOrigin {
    const NON_3GPP: u8 = 0u8;
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
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 2)]
pub struct PagingeDRXInformation {
    pub paging_e_drx_cycle: Paging_eDRX_Cycle,
    #[asn(optional_idx = 0)]
    pub paging_time_window: Option<Paging_Time_Window>,
    #[asn(optional_idx = 1)]
    pub i_e_extensions: Option<PagingeDRXInformationiE_Extensions>,
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
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 3)]
pub struct PathSwitchRequestAcknowledgeTransfer {
    #[asn(optional_idx = 0)]
    pub u_l_ngu_up_tnl_information: Option<UPTransportLayerInformation>,
    #[asn(optional_idx = 1)]
    pub security_indication: Option<SecurityIndication>,
    #[asn(optional_idx = 2)]
    pub i_e_extensions: Option<PathSwitchRequestAcknowledgeTransferiE_Extensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true)]
pub struct PathSwitchRequestFailure {
    pub protocol_i_es: PathSwitchRequestFailureprotocolIEs,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct PathSwitchRequestSetupFailedTransfer {
    pub cause: Cause,
    #[asn(optional_idx = 0)]
    pub i_e_extensions: Option<PathSwitchRequestSetupFailedTransferiE_Extensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 3)]
pub struct PathSwitchRequestTransfer {
    pub d_l_ngu_up_tnl_information: UPTransportLayerInformation,
    #[asn(optional_idx = 0)]
    pub d_l_ngu_tnl_information_reused: Option<DL_NGU_TNLInformationReused>,
    #[asn(optional_idx = 1)]
    pub user_plane_security_information: Option<UserPlaneSecurityInformation>,
    pub qos_flow_accepted_list: QosFlowAcceptedList,
    #[asn(optional_idx = 2)]
    pub i_e_extensions: Option<PathSwitchRequestTransferiE_Extensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct PathSwitchRequestUnsuccessfulTransfer {
    pub cause: Cause,
    #[asn(optional_idx = 0)]
    pub i_e_extensions: Option<PathSwitchRequestUnsuccessfulTransferiE_Extensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "ENUMERATED", extensible = true, lb = "0", ub = "1")]
pub struct PedestrianUE(u8);
impl PedestrianUE {
    const AUTHORIZED: u8 = 0u8;
    const NOT_AUTHORIZED: u8 = 1u8;
}

#[derive(Debug, AperCodec)]
#[asn(type = "BITSTRING", sz_extensible = false, sz_lb = "8", sz_ub = "8")]
pub struct PeriodicRegistrationUpdateTimer(BitVec<Msb0, u8>);

#[derive(Debug, AperCodec)]
#[asn(type = "INTEGER", lb = "0", ub = "640000", extensible = true)]
pub struct Periodicity(u32);

#[derive(Debug, AperCodec)]
#[asn(type = "OCTET-STRING", sz_extensible = false, sz_lb = "2", sz_ub = "2")]
pub struct PortNumber(Vec<u8>);

#[derive(Debug, AperCodec)]
#[asn(type = "ENUMERATED", extensible = true, lb = "0", ub = "1")]
pub struct Pre_emptionCapability(u8);
impl Pre_emptionCapability {
    const SHALL_NOT_TRIGGER_PRE_EMPTION: u8 = 0u8;
    const MAY_TRIGGER_PRE_EMPTION: u8 = 1u8;
}

#[derive(Debug, AperCodec)]
#[asn(type = "ENUMERATED", extensible = true, lb = "0", ub = "1")]
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
#[asn(type = "INTEGER", lb = "1", ub = "15")]
pub struct PriorityLevelARP(u8);

#[derive(Debug, AperCodec)]
#[asn(type = "INTEGER", lb = "1", ub = "127", extensible = true)]
pub struct PriorityLevelQos(u8);

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
    local(INTEGER_70),
    #[asn(key = 1, extended = false)]
    global(OBJECT_IDENTIFIER_71),
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
#[asn(type = "CHOICE", lb = "0", ub = "1", extensible = false)]
pub enum ProcedureStageChoice {
    #[asn(key = 0, extended = false)]
    first_dl_count(FirstDLCount),
    #[asn(key = 1, extended = false)]
    choice_Extensions(ProcedureStageChoicechoice_Extensions),
}

#[derive(Debug, AperCodec)]
#[asn(type = "INTEGER", lb = "0", ub = "65535")]
pub struct ProtocolExtensionID(u16);

#[derive(Debug, AperCodec)]
#[asn(type = "INTEGER", lb = "0", ub = "65535")]
pub struct ProtocolIE_ID(u16);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct QoSFlowsUsageReport_Item {
    pub qos_flow_identifier: QosFlowIdentifier,
    pub r_at_type: ENUMERATED_72,
    pub qo_s_flows_timed_report_list: VolumeTimedReportList,
    #[asn(optional_idx = 0)]
    pub i_e_extensions: Option<QoSFlowsUsageReport_ItemiE_Extensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "64")]
pub struct QoSFlowsUsageReportList(Vec<QoSFlowsUsageReport_Item>);

#[derive(Debug, AperCodec)]
#[asn(type = "CHOICE", lb = "0", ub = "2", extensible = false)]
pub enum QosCharacteristics {
    #[asn(key = 0, extended = false)]
    nonDynamic5QI(NonDynamic5QIDescriptor),
    #[asn(key = 1, extended = false)]
    dynamic5QI(Dynamic5QIDescriptor),
    #[asn(key = 2, extended = false)]
    choice_Extensions(QosCharacteristicschoice_Extensions),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct QosFlowAcceptedItem {
    pub qos_flow_identifier: QosFlowIdentifier,
    #[asn(optional_idx = 0)]
    pub i_e_extensions: Option<QosFlowAcceptedItemiE_Extensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "64")]
pub struct QosFlowAcceptedList(Vec<QosFlowAcceptedItem>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 3)]
pub struct QosFlowAddOrModifyRequestItem {
    pub qos_flow_identifier: QosFlowIdentifier,
    #[asn(optional_idx = 0)]
    pub qos_flow_level_qos_parameters: Option<QosFlowLevelQosParameters>,
    #[asn(optional_idx = 1)]
    pub e_rab_id: Option<E_RAB_ID>,
    #[asn(optional_idx = 2)]
    pub i_e_extensions: Option<QosFlowAddOrModifyRequestItemiE_Extensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "64")]
pub struct QosFlowAddOrModifyRequestList(Vec<QosFlowAddOrModifyRequestItem>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct QosFlowAddOrModifyResponseItem {
    pub qos_flow_identifier: QosFlowIdentifier,
    #[asn(optional_idx = 0)]
    pub i_e_extensions: Option<QosFlowAddOrModifyResponseItemiE_Extensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "64")]
pub struct QosFlowAddOrModifyResponseList(Vec<QosFlowAddOrModifyResponseItem>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 4)]
pub struct QosFlowFeedbackItem {
    pub qos_flow_identifier: QosFlowIdentifier,
    #[asn(optional_idx = 0)]
    pub update_feedback: Option<UpdateFeedback>,
    #[asn(optional_idx = 1)]
    pub c_npacket_delay_budget_dl: Option<ExtendedPacketDelayBudget>,
    #[asn(optional_idx = 2)]
    pub c_npacket_delay_budget_ul: Option<ExtendedPacketDelayBudget>,
    #[asn(optional_idx = 3)]
    pub i_e_extensions: Option<QosFlowFeedbackItemiE_Extensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "64")]
pub struct QosFlowFeedbackList(Vec<QosFlowFeedbackItem>);

#[derive(Debug, AperCodec)]
#[asn(type = "INTEGER", lb = "0", ub = "63", extensible = true)]
pub struct QosFlowIdentifier(u8);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 2)]
pub struct QosFlowInformationItem {
    pub qos_flow_identifier: QosFlowIdentifier,
    #[asn(optional_idx = 0)]
    pub d_l_forwarding: Option<DLForwarding>,
    #[asn(optional_idx = 1)]
    pub i_e_extensions: Option<QosFlowInformationItemiE_Extensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "64")]
pub struct QosFlowInformationList(Vec<QosFlowInformationItem>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 2)]
pub struct QosFlowItemWithDataForwarding {
    pub qos_flow_identifier: QosFlowIdentifier,
    #[asn(optional_idx = 0)]
    pub data_forwarding_accepted: Option<DataForwardingAccepted>,
    #[asn(optional_idx = 1)]
    pub i_e_extensions: Option<QosFlowItemWithDataForwardingiE_Extensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 4)]
pub struct QosFlowLevelQosParameters {
    pub qos_characteristics: QosCharacteristics,
    pub allocation_and_retention_priority: AllocationAndRetentionPriority,
    #[asn(optional_idx = 0)]
    pub g_br_qos_information: Option<GBR_QosInformation>,
    #[asn(optional_idx = 1)]
    pub reflective_qos_attribute: Option<ReflectiveQosAttribute>,
    #[asn(optional_idx = 2)]
    pub additional_qos_flow_information: Option<AdditionalQosFlowInformation>,
    #[asn(optional_idx = 3)]
    pub i_e_extensions: Option<QosFlowLevelQosParametersiE_Extensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "64")]
pub struct QosFlowListWithCause(Vec<QosFlowWithCauseItem>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "64")]
pub struct QosFlowListWithDataForwarding(Vec<QosFlowItemWithDataForwarding>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct QosFlowModifyConfirmItem {
    pub qos_flow_identifier: QosFlowIdentifier,
    #[asn(optional_idx = 0)]
    pub i_e_extensions: Option<QosFlowModifyConfirmItemiE_Extensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "64")]
pub struct QosFlowModifyConfirmList(Vec<QosFlowModifyConfirmItem>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct QosFlowNotifyItem {
    pub qos_flow_identifier: QosFlowIdentifier,
    pub notification_cause: NotificationCause,
    #[asn(optional_idx = 0)]
    pub i_e_extensions: Option<QosFlowNotifyItemiE_Extensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "64")]
pub struct QosFlowNotifyList(Vec<QosFlowNotifyItem>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 2)]
pub struct QosFlowParametersItem {
    pub qos_flow_identifier: QosFlowIdentifier,
    #[asn(optional_idx = 0)]
    pub alternative_qo_s_para_set_list: Option<AlternativeQoSParaSetList>,
    #[asn(optional_idx = 1)]
    pub i_e_extensions: Option<QosFlowParametersItemiE_Extensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "64")]
pub struct QosFlowParametersList(Vec<QosFlowParametersItem>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct QosFlowPerTNLInformation {
    pub u_p_transport_layer_information: UPTransportLayerInformation,
    pub associated_qos_flow_list: AssociatedQosFlowList,
    #[asn(optional_idx = 0)]
    pub i_e_extensions: Option<QosFlowPerTNLInformationiE_Extensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct QosFlowPerTNLInformationItem {
    pub qos_flow_per_tnl_information: QosFlowPerTNLInformation,
    #[asn(optional_idx = 0)]
    pub i_e_extensions: Option<QosFlowPerTNLInformationItemiE_Extensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "3")]
pub struct QosFlowPerTNLInformationList(Vec<QosFlowPerTNLInformationItem>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 2)]
pub struct QosFlowSetupRequestItem {
    pub qos_flow_identifier: QosFlowIdentifier,
    pub qos_flow_level_qos_parameters: QosFlowLevelQosParameters,
    #[asn(optional_idx = 0)]
    pub e_rab_id: Option<E_RAB_ID>,
    #[asn(optional_idx = 1)]
    pub i_e_extensions: Option<QosFlowSetupRequestItemiE_Extensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "64")]
pub struct QosFlowSetupRequestList(Vec<QosFlowSetupRequestItem>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct QosFlowToBeForwardedItem {
    pub qos_flow_identifier: QosFlowIdentifier,
    #[asn(optional_idx = 0)]
    pub i_e_extensions: Option<QosFlowToBeForwardedItemiE_Extensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "64")]
pub struct QosFlowToBeForwardedList(Vec<QosFlowToBeForwardedItem>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct QosFlowWithCauseItem {
    pub qos_flow_identifier: QosFlowIdentifier,
    pub cause: Cause,
    #[asn(optional_idx = 0)]
    pub i_e_extensions: Option<QosFlowWithCauseItemiE_Extensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "INTEGER", lb = "1", ub = "1800", extensible = true)]
pub struct QosMonitoringReportingFrequency(u16);

#[derive(Debug, AperCodec)]
#[asn(type = "ENUMERATED", extensible = true, lb = "0", ub = "2")]
pub struct QosMonitoringRequest(u8);
impl QosMonitoringRequest {
    const UL: u8 = 0u8;
    const DL: u8 = 1u8;
    const BOTH: u8 = 2u8;
}

#[derive(Debug, AperCodec)]
#[asn(type = "INTEGER", lb = "0", ub = "4294967295")]
pub struct RAN_UE_NGAP_ID(u32);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true)]
pub struct RANCPRelocationIndication {
    pub protocol_i_es: RANCPRelocationIndicationprotocolIEs,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true)]
pub struct RANConfigurationUpdate {
    pub protocol_i_es: RANConfigurationUpdateprotocolIEs,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true)]
pub struct RANConfigurationUpdateAcknowledge {
    pub protocol_i_es: RANConfigurationUpdateAcknowledgeprotocolIEs,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true)]
pub struct RANConfigurationUpdateFailure {
    pub protocol_i_es: RANConfigurationUpdateFailureprotocolIEs,
}

#[derive(Debug, AperCodec)]
#[asn(
    type = "PrintableString",
    sz_extensible = true,
    sz_lb = "1",
    sz_ub = "150"
)]
pub struct RANNodeName(String);

#[derive(Debug, AperCodec)]
#[asn(type = "UTF8String", sz_extensible = true, sz_lb = "1", sz_ub = "150")]
pub struct RANNodeNameUTF8String(String);

#[derive(Debug, AperCodec)]
#[asn(
    type = "VisibleString",
    sz_extensible = true,
    sz_lb = "1",
    sz_ub = "150"
)]
pub struct RANNodeNameVisibleString(String);

#[derive(Debug, AperCodec)]
#[asn(type = "INTEGER", lb = "1", ub = "256")]
pub struct RANPagingPriority(u16);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct RANStatusTransfer_TransparentContainer {
    pub d_r_bs_subject_to_status_transfer_list: DRBsSubjectToStatusTransferList,
    #[asn(optional_idx = 0)]
    pub i_e_extensions: Option<RANStatusTransfer_TransparentContaineriE_Extensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "ENUMERATED", extensible = true, lb = "0", ub = "1")]
pub struct RAT_Information(u8);
impl RAT_Information {
    const UNLICENSED: u8 = 0u8;
    const NB_IO_T: u8 = 1u8;
}

#[derive(Debug, AperCodec)]
#[asn(type = "BITSTRING", sz_extensible = true, sz_lb = "8", sz_ub = "8")]
pub struct RATRestrictionInformation(BitVec<Msb0, u8>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "16")]
pub struct RATRestrictions(Vec<RATRestrictions_Item>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct RATRestrictions_Item {
    pub p_lmn_identity: PLMNIdentity,
    pub r_at_restriction_information: RATRestrictionInformation,
    #[asn(optional_idx = 0)]
    pub i_e_extensions: Option<RATRestrictions_ItemiE_Extensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "OCTET-STRING")]
pub struct RGLevelWirelineAccessCharacteristics(Vec<u8>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct RIMInformation {
    pub targetg_nb_set_id: GNBSetID,
    pub r_im_rs_detection: ENUMERATED_73,
    #[asn(optional_idx = 0)]
    pub i_e_extensions: Option<RIMInformationiE_Extensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct RIMInformationTransfer {
    pub target_ran_node_id: TargetRANNodeID,
    pub source_ran_node_id: SourceRANNodeID,
    pub r_im_information: RIMInformation,
    #[asn(optional_idx = 0)]
    pub i_e_extensions: Option<RIMInformationTransferiE_Extensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "INTEGER", lb = "0", ub = "4095")]
pub struct RNC_ID(u16);

#[derive(Debug, AperCodec)]
#[asn(type = "OCTET-STRING")]
pub struct RRCContainer(Vec<u8>);

#[derive(Debug, AperCodec)]
#[asn(type = "ENUMERATED", extensible = true, lb = "0", ub = "9")]
pub struct RRCEstablishmentCause(u8);
impl RRCEstablishmentCause {
    const EMERGENCY: u8 = 0u8;
    const HIGH_PRIORITY_ACCESS: u8 = 1u8;
    const MT_ACCESS: u8 = 2u8;
    const MO_SIGNALLING: u8 = 3u8;
    const MO_DATA: u8 = 4u8;
    const MO_VOICE_CALL: u8 = 5u8;
    const MO_VIDEO_CALL: u8 = 6u8;
    const MO_SMS: u8 = 7u8;
    const MPS_PRIORITY_ACCESS: u8 = 8u8;
    const MCS_PRIORITY_ACCESS: u8 = 9u8;
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true)]
pub struct RRCInactiveTransitionReport {
    pub protocol_i_es: RRCInactiveTransitionReportprotocolIEs,
}

#[derive(Debug, AperCodec)]
#[asn(type = "ENUMERATED", extensible = true, lb = "0", ub = "2")]
pub struct RRCInactiveTransitionReportRequest(u8);
impl RRCInactiveTransitionReportRequest {
    const SUBSEQUENT_STATE_TRANSITION_REPORT: u8 = 0u8;
    const SINGLE_RRC_CONNECTED_STATE_REPORT: u8 = 1u8;
    const CANCEL_REPORT: u8 = 2u8;
}

#[derive(Debug, AperCodec)]
#[asn(type = "ENUMERATED", extensible = true, lb = "0", ub = "1")]
pub struct RRCState(u8);
impl RRCState {
    const INACTIVE: u8 = 0u8;
    const CONNECTED: u8 = 1u8;
}

#[derive(Debug, AperCodec)]
#[asn(type = "ENUMERATED", extensible = true, lb = "0", ub = "1")]
pub struct RSN(u8);
impl RSN {
    const V1: u8 = 0u8;
    const V2: u8 = 1u8;
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
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 2)]
pub struct RecommendedCellItem {
    pub n_gran_cgi: NGRAN_CGI,
    #[asn(optional_idx = 0)]
    pub time_stayed_in_cell: Option<INTEGER_74>,
    #[asn(optional_idx = 1)]
    pub i_e_extensions: Option<RecommendedCellItemiE_Extensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "16")]
pub struct RecommendedCellList(Vec<RecommendedCellItem>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct RecommendedCellsForPaging {
    pub recommended_cell_list: RecommendedCellList,
    #[asn(optional_idx = 0)]
    pub i_e_extensions: Option<RecommendedCellsForPagingiE_Extensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct RecommendedRANNodeItem {
    pub a_mf_paging_target: AMFPagingTarget,
    #[asn(optional_idx = 0)]
    pub i_e_extensions: Option<RecommendedRANNodeItemiE_Extensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "16")]
pub struct RecommendedRANNodeList(Vec<RecommendedRANNodeItem>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct RecommendedRANNodesForPaging {
    pub recommended_ran_node_list: RecommendedRANNodeList,
    #[asn(optional_idx = 0)]
    pub i_e_extensions: Option<RecommendedRANNodesForPagingiE_Extensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "ENUMERATED", extensible = true, lb = "0", ub = "1")]
pub struct RedirectionVoiceFallback(u8);
impl RedirectionVoiceFallback {
    const POSSIBLE: u8 = 0u8;
    const NOT_POSSIBLE: u8 = 1u8;
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct RedundantPDUSessionInformation {
    pub r_sn: RSN,
    #[asn(optional_idx = 0)]
    pub i_e_extensions: Option<RedundantPDUSessionInformationiE_Extensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "ENUMERATED", lb = "0", ub = "1")]
pub struct RedundantQosFlowIndicator(u8);
impl RedundantQosFlowIndicator {
    const TRUE: u8 = 0u8;
    const FALSE: u8 = 1u8;
}

#[derive(Debug, AperCodec)]
#[asn(type = "ENUMERATED", extensible = true, lb = "0", ub = "0")]
pub struct ReflectiveQosAttribute(u8);
impl ReflectiveQosAttribute {
    const SUBJECT_TO: u8 = 0u8;
}

#[derive(Debug, AperCodec)]
#[asn(
    type = "OCTET-STRING",
    sz_extensible = false,
    sz_lb = "32",
    sz_ub = "32"
)]
pub struct RejectedNSSAIinPLMN(Vec<u8>);

#[derive(Debug, AperCodec)]
#[asn(
    type = "OCTET-STRING",
    sz_extensible = false,
    sz_lb = "32",
    sz_ub = "32"
)]
pub struct RejectedNSSAIinTA(Vec<u8>);

#[derive(Debug, AperCodec)]
#[asn(type = "INTEGER", lb = "0", ub = "255")]
pub struct RelativeAMFCapacity(u8);

#[derive(Debug, AperCodec)]
#[asn(type = "INTEGER", lb = "0", ub = "131071")]
pub struct RepetitionPeriod(u32);

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
    const CELL: u8 = 0u8;
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
#[asn(type = "SEQUENCE", extensible = true)]
pub struct RerouteNASRequest {
    pub protocol_i_es: RerouteNASRequestprotocolIEs,
}

#[derive(Debug, AperCodec)]
#[asn(type = "ENUMERATED", extensible = true, lb = "0", ub = "0")]
pub struct ResetAll(u8);
impl ResetAll {
    const RESET_ALL: u8 = 0u8;
}

#[derive(Debug, AperCodec)]
#[asn(type = "CHOICE", lb = "0", ub = "2", extensible = false)]
pub enum ResetType {
    #[asn(key = 0, extended = false)]
    nG_Interface(ResetAll),
    #[asn(key = 1, extended = false)]
    partOfNG_Interface(UE_associatedLogicalNG_connectionList),
    #[asn(key = 2, extended = false)]
    choice_Extensions(ResetTypechoice_Extensions),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true)]
pub struct RetrieveUEInformation {
    pub protocol_i_es: RetrieveUEInformationprotocolIEs,
}

#[derive(Debug, AperCodec)]
#[asn(type = "OCTET-STRING")]
pub struct RoutingID(Vec<u8>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 2)]
pub struct S_NSSAI {
    pub s_st: SST,
    #[asn(optional_idx = 0)]
    pub s_d: Option<SD>,
    #[asn(optional_idx = 1)]
    pub i_e_extensions: Option<S_NSSAIiE_Extensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "2")]
pub struct SCTP_TLAs(Vec<TransportLayerAddress>);

#[derive(Debug, AperCodec)]
#[asn(type = "OCTET-STRING", sz_extensible = false, sz_lb = "3", sz_ub = "3")]
pub struct SD(Vec<u8>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct SNPN_MobilityInformation {
    pub serving_nid: NID,
    #[asn(optional_idx = 0)]
    pub i_e_extensions: Option<SNPN_MobilityInformationiE_Extensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 2)]
pub struct SONConfigurationTransfer {
    pub target_ran_node_id: TargetRANNodeID,
    pub source_ran_node_id: SourceRANNodeID,
    pub s_on_information: SONInformation,
    #[asn(optional_idx = 0)]
    pub xn_tnl_configuration_info: Option<XnTNLConfigurationInfo>,
    #[asn(optional_idx = 1)]
    pub i_e_extensions: Option<SONConfigurationTransferiE_Extensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "CHOICE", lb = "0", ub = "2", extensible = false)]
pub enum SONInformation {
    #[asn(key = 0, extended = false)]
    sONInformationRequest(SONInformationRequest),
    #[asn(key = 1, extended = false)]
    sONInformationReply(SONInformationReply),
    #[asn(key = 2, extended = false)]
    choice_Extensions(SONInformationchoice_Extensions),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 2)]
pub struct SONInformationReply {
    #[asn(optional_idx = 0)]
    pub xn_tnl_configuration_info: Option<XnTNLConfigurationInfo>,
    #[asn(optional_idx = 1)]
    pub i_e_extensions: Option<SONInformationReplyiE_Extensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "CHOICE", lb = "0", ub = "2", extensible = false)]
pub enum SONInformationReport {
    #[asn(key = 0, extended = false)]
    failureIndicationInformation(FailureIndication),
    #[asn(key = 1, extended = false)]
    hOReportInformation(HOReport),
    #[asn(key = 2, extended = false)]
    choice_Extensions(SONInformationReportchoice_Extensions),
}

#[derive(Debug, AperCodec)]
#[asn(type = "ENUMERATED", extensible = true, lb = "0", ub = "0")]
pub struct SONInformationRequest(u8);
impl SONInformationRequest {
    const XN_TNL_CONFIGURATION_INFO: u8 = 0u8;
}

#[derive(Debug, AperCodec)]
#[asn(type = "ENUMERATED", extensible = true, lb = "0", ub = "1")]
pub struct SRVCCOperationPossible(u8);
impl SRVCCOperationPossible {
    const POSSIBLE: u8 = 0u8;
    const NOT_POSSIBLE: u8 = 1u8;
}

#[derive(Debug, AperCodec)]
#[asn(type = "OCTET-STRING", sz_extensible = false, sz_lb = "1", sz_ub = "1")]
pub struct SST(Vec<u8>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 4)]
pub struct ScheduledCommunicationTime {
    #[asn(optional_idx = 0)]
    pub dayof_week: Option<BIT_STRING_75>,
    #[asn(optional_idx = 1)]
    pub timeof_day_start: Option<INTEGER_76>,
    #[asn(optional_idx = 2)]
    pub timeof_day_end: Option<INTEGER_77>,
    #[asn(optional_idx = 3)]
    pub i_e_extensions: Option<ScheduledCommunicationTimeiE_Extensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true)]
pub struct SecondaryRATDataUsageReport {
    pub protocol_i_es: SecondaryRATDataUsageReportprotocolIEs,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 2)]
pub struct SecondaryRATDataUsageReportTransfer {
    #[asn(optional_idx = 0)]
    pub secondary_rat_usage_information: Option<SecondaryRATUsageInformation>,
    #[asn(optional_idx = 1)]
    pub i_e_extensions: Option<SecondaryRATDataUsageReportTransferiE_Extensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 3)]
pub struct SecondaryRATUsageInformation {
    #[asn(optional_idx = 0)]
    pub p_du_session_usage_report: Option<PDUSessionUsageReport>,
    #[asn(optional_idx = 1)]
    pub qos_flows_usage_report_list: Option<QoSFlowsUsageReportList>,
    #[asn(optional_idx = 2)]
    pub i_e_extension: Option<SecondaryRATUsageInformationiE_Extension>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct SecurityContext {
    pub next_hop_chaining_count: NextHopChainingCount,
    pub next_hop_nh: SecurityKey,
    #[asn(optional_idx = 0)]
    pub i_e_extensions: Option<SecurityContextiE_Extensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 2)]
pub struct SecurityIndication {
    pub integrity_protection_indication: IntegrityProtectionIndication,
    pub confidentiality_protection_indication: ConfidentialityProtectionIndication,
    #[asn(optional_idx = 0)]
    pub maximum_integrity_protected_data_rate_ul: Option<MaximumIntegrityProtectedDataRate>,
    #[asn(optional_idx = 1)]
    pub i_e_extensions: Option<SecurityIndicationiE_Extensions>,
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
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct SecurityResult {
    pub integrity_protection_result: IntegrityProtectionResult,
    pub confidentiality_protection_result: ConfidentialityProtectionResult,
    #[asn(optional_idx = 0)]
    pub i_e_extensions: Option<SecurityResultiE_Extensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "ENUMERATED", extensible = true, lb = "0", ub = "0")]
pub struct SensorMeasConfig(u8);
impl SensorMeasConfig {
    const SETUP: u8 = 0u8;
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct SensorMeasConfigNameItem {
    pub sensor_name_config: SensorNameConfig,
    #[asn(optional_idx = 0)]
    pub i_e_extensions: Option<SensorMeasConfigNameItemiE_Extensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "3")]
pub struct SensorMeasConfigNameList(Vec<SensorMeasConfigNameItem>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 2)]
pub struct SensorMeasurementConfiguration {
    pub sensor_meas_config: SensorMeasConfig,
    #[asn(optional_idx = 0)]
    pub sensor_meas_config_name_list: Option<SensorMeasConfigNameList>,
    #[asn(optional_idx = 1)]
    pub i_e_extensions: Option<SensorMeasurementConfigurationiE_Extensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "CHOICE", lb = "0", ub = "3", extensible = false)]
pub enum SensorNameConfig {
    #[asn(key = 0, extended = false)]
    uncompensatedBarometricConfig(ENUMERATED_78),
    #[asn(key = 1, extended = false)]
    ueSpeedConfig(ENUMERATED_79),
    #[asn(key = 2, extended = false)]
    ueOrientationConfig(ENUMERATED_80),
    #[asn(key = 3, extended = false)]
    choice_Extensions(SensorNameConfigchoice_Extensions),
}

#[derive(Debug, AperCodec)]
#[asn(type = "BITSTRING", sz_extensible = false, sz_lb = "16", sz_ub = "16")]
pub struct SerialNumber(BitVec<Msb0, u8>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 2)]
pub struct ServedGUAMIItem {
    pub g_uami: GUAMI,
    #[asn(optional_idx = 0)]
    pub backup_amf_name: Option<AMFName>,
    #[asn(optional_idx = 1)]
    pub i_e_extensions: Option<ServedGUAMIItemiE_Extensions>,
}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "256"
)]
pub struct ServedGUAMIList(Vec<ServedGUAMIItem>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "16")]
pub struct ServiceAreaInformation(Vec<ServiceAreaInformation_Item>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 3)]
pub struct ServiceAreaInformation_Item {
    pub p_lmn_identity: PLMNIdentity,
    #[asn(optional_idx = 0)]
    pub allowed_ta_cs: Option<AllowedTACs>,
    #[asn(optional_idx = 1)]
    pub not_allowed_ta_cs: Option<NotAllowedTACs>,
    #[asn(optional_idx = 2)]
    pub i_e_extensions: Option<ServiceAreaInformation_ItemiE_Extensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "INTEGER", lb = "0", ub = "4294967295")]
pub struct SgNB_UE_X2AP_ID(u32);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct SliceOverloadItem {
    pub s_nssai: S_NSSAI,
    #[asn(optional_idx = 0)]
    pub i_e_extensions: Option<SliceOverloadItemiE_Extensions>,
}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "1024"
)]
pub struct SliceOverloadList(Vec<SliceOverloadItem>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct SliceSupportItem {
    pub s_nssai: S_NSSAI,
    #[asn(optional_idx = 0)]
    pub i_e_extensions: Option<SliceSupportItemiE_Extensions>,
}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "1024"
)]
pub struct SliceSupportList(Vec<SliceSupportItem>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 4)]
pub struct SourceNGRANNode_ToTargetNGRANNode_TransparentContainer {
    pub r_rc_container: RRCContainer,
    #[asn(optional_idx = 0)]
    pub p_du_session_resource_information_list: Option<PDUSessionResourceInformationList>,
    #[asn(optional_idx = 1)]
    pub e_rab_information_list: Option<E_RABInformationList>,
    pub target_cell_id: NGRAN_CGI,
    #[asn(optional_idx = 2)]
    pub index_to_rfsp: Option<IndexToRFSP>,
    pub u_e_history_information: UEHistoryInformation,
    #[asn(optional_idx = 3)]
    pub i_e_extensions: Option<SourceNGRANNode_ToTargetNGRANNode_TransparentContaineriE_Extensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "ENUMERATED", extensible = true, lb = "0", ub = "1")]
pub struct SourceOfUEActivityBehaviourInformation(u8);
impl SourceOfUEActivityBehaviourInformation {
    const SUBSCRIPTION_INFORMATION: u8 = 0u8;
    const STATISTICS: u8 = 1u8;
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct SourceRANNodeID {
    pub global_ran_node_id: GlobalRANNodeID,
    pub selected_tai: TAI,
    #[asn(optional_idx = 0)]
    pub i_e_extensions: Option<SourceRANNodeIDiE_Extensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 4)]
pub struct SourceToTarget_AMFInformationReroute {
    #[asn(optional_idx = 0)]
    pub configured_nssai: Option<ConfiguredNSSAI>,
    #[asn(optional_idx = 1)]
    pub rejected_nssa_iin_plmn: Option<RejectedNSSAIinPLMN>,
    #[asn(optional_idx = 2)]
    pub rejected_nssa_iin_ta: Option<RejectedNSSAIinTA>,
    #[asn(optional_idx = 3)]
    pub i_e_extensions: Option<SourceToTarget_AMFInformationRerouteiE_Extensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "OCTET-STRING")]
pub struct SourceToTarget_TransparentContainer(Vec<u8>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct SuccessfulOutcome {
    #[asn(key_field = true)]
    pub procedure_code: ProcedureCode,
    pub criticality: Criticality,
    pub value: SuccessfulOutcomevalue,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct SupportedTAItem {
    pub t_ac: TAC,
    pub broadcast_plmn_list: BroadcastPLMNList,
    #[asn(optional_idx = 0)]
    pub i_e_extensions: Option<SupportedTAItemiE_Extensions>,
}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "256"
)]
pub struct SupportedTAList(Vec<SupportedTAItem>);

#[derive(Debug, AperCodec)]
#[asn(type = "ENUMERATED", extensible = true, lb = "0", ub = "0")]
pub struct Suspend_Request_Indication(u8);
impl Suspend_Request_Indication {
    const SUSPEND_REQUESTED: u8 = 0u8;
}

#[derive(Debug, AperCodec)]
#[asn(type = "ENUMERATED", extensible = true, lb = "0", ub = "0")]
pub struct Suspend_Response_Indication(u8);
impl Suspend_Response_Indication {
    const SUSPEND_INDICATED: u8 = 0u8;
}

#[derive(Debug, AperCodec)]
#[asn(type = "ENUMERATED", extensible = true, lb = "0", ub = "0")]
pub struct SuspendIndicator(u8);
impl SuspendIndicator {
    const TRUE: u8 = 0u8;
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct TABasedMDT {
    pub t_a_listfor_mdt: TAListforMDT,
    #[asn(optional_idx = 0)]
    pub i_e_extensions: Option<TABasedMDTiE_Extensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "OCTET-STRING", sz_extensible = false, sz_lb = "3", sz_ub = "3")]
pub struct TAC(Vec<u8>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct TAI {
    pub p_lmn_identity: PLMNIdentity,
    pub t_ac: TAC,
    #[asn(optional_idx = 0)]
    pub i_e_extensions: Option<TAIiE_Extensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct TAIBasedMDT {
    pub t_ai_listfor_mdt: TAIListforMDT,
    #[asn(optional_idx = 0)]
    pub i_e_extensions: Option<TAIBasedMDTiE_Extensions>,
}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct TAIBroadcastEUTRA(Vec<TAIBroadcastEUTRA_Item>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct TAIBroadcastEUTRA_Item {
    pub t_ai: TAI,
    pub completed_cells_in_tai_eutra: CompletedCellsInTAI_EUTRA,
    #[asn(optional_idx = 0)]
    pub i_e_extensions: Option<TAIBroadcastEUTRA_ItemiE_Extensions>,
}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct TAIBroadcastNR(Vec<TAIBroadcastNR_Item>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct TAIBroadcastNR_Item {
    pub t_ai: TAI,
    pub completed_cells_in_tai_nr: CompletedCellsInTAI_NR,
    #[asn(optional_idx = 0)]
    pub i_e_extensions: Option<TAIBroadcastNR_ItemiE_Extensions>,
}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct TAICancelledEUTRA(Vec<TAICancelledEUTRA_Item>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct TAICancelledEUTRA_Item {
    pub t_ai: TAI,
    pub cancelled_cells_in_tai_eutra: CancelledCellsInTAI_EUTRA,
    #[asn(optional_idx = 0)]
    pub i_e_extensions: Option<TAICancelledEUTRA_ItemiE_Extensions>,
}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct TAICancelledNR(Vec<TAICancelledNR_Item>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct TAICancelledNR_Item {
    pub t_ai: TAI,
    pub cancelled_cells_in_tai_nr: CancelledCellsInTAI_NR,
    #[asn(optional_idx = 0)]
    pub i_e_extensions: Option<TAICancelledNR_ItemiE_Extensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "16")]
pub struct TAIListForInactive(Vec<TAIListForInactiveItem>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct TAIListForInactiveItem {
    pub t_ai: TAI,
    #[asn(optional_idx = 0)]
    pub i_e_extensions: Option<TAIListForInactiveItemiE_Extensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "16")]
pub struct TAIListForPaging(Vec<TAIListForPagingItem>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct TAIListForPagingItem {
    pub t_ai: TAI,
    #[asn(optional_idx = 0)]
    pub i_e_extensions: Option<TAIListForPagingItemiE_Extensions>,
}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "2048"
)]
pub struct TAIListForRestart(Vec<TAI>);

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct TAIListForWarning(Vec<TAI>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "8")]
pub struct TAIListforMDT(Vec<TAI>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "8")]
pub struct TAListforMDT(Vec<TAC>);

#[derive(Debug, AperCodec)]
#[asn(type = "OCTET-STRING")]
pub struct TNAP_ID(Vec<u8>);

#[derive(Debug, AperCodec)]
#[asn(type = "CHOICE", lb = "0", ub = "1", extensible = false)]
pub enum TNGF_ID {
    #[asn(key = 0, extended = false)]
    tNGF_ID(BIT_STRING_81),
    #[asn(key = 1, extended = false)]
    choice_Extensions(TNGF_IDchoice_Extensions),
}

#[derive(Debug, AperCodec)]
#[asn(type = "INTEGER", lb = "0", ub = "255")]
pub struct TNLAddressWeightFactor(u8);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct TNLAssociationItem {
    pub t_nl_association_address: CPTransportLayerInformation,
    pub cause: Cause,
    #[asn(optional_idx = 0)]
    pub i_e_extensions: Option<TNLAssociationItemiE_Extensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "32")]
pub struct TNLAssociationList(Vec<TNLAssociationItem>);

#[derive(Debug, AperCodec)]
#[asn(type = "ENUMERATED", extensible = true, lb = "0", ub = "2")]
pub struct TNLAssociationUsage(u8);
impl TNLAssociationUsage {
    const UE: u8 = 0u8;
    const NON_UE: u8 = 1u8;
    const BOTH: u8 = 2u8;
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 2)]
pub struct TSCAssistanceInformation {
    pub periodicity: Periodicity,
    #[asn(optional_idx = 0)]
    pub burst_arrival_time: Option<BurstArrivalTime>,
    #[asn(optional_idx = 1)]
    pub i_e_extensions: Option<TSCAssistanceInformationiE_Extensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 3)]
pub struct TSCTrafficCharacteristics {
    #[asn(optional_idx = 0)]
    pub t_sc_assistance_information_dl: Option<TSCAssistanceInformation>,
    #[asn(optional_idx = 1)]
    pub t_sc_assistance_information_ul: Option<TSCAssistanceInformation>,
    #[asn(optional_idx = 2)]
    pub i_e_extensions: Option<TSCTrafficCharacteristicsiE_Extensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "OCTET-STRING")]
pub struct TWAP_ID(Vec<u8>);

#[derive(Debug, AperCodec)]
#[asn(type = "CHOICE", lb = "0", ub = "1", extensible = false)]
pub enum TWIF_ID {
    #[asn(key = 0, extended = false)]
    tWIF_ID(BIT_STRING_82),
    #[asn(key = 1, extended = false)]
    choice_Extensions(TWIF_IDchoice_Extensions),
}

#[derive(Debug, AperCodec)]
#[asn(type = "CHOICE", lb = "0", ub = "2", extensible = false)]
pub enum TargetID {
    #[asn(key = 0, extended = false)]
    targetRANNodeID(TargetRANNodeID),
    #[asn(key = 1, extended = false)]
    targeteNB_ID(TargeteNB_ID),
    #[asn(key = 2, extended = false)]
    choice_Extensions(TargetIDchoice_Extensions),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct TargetNGRANNode_ToSourceNGRANNode_FailureTransparentContainer {
    pub cell_cag_information: Cell_CAGInformation,
    #[asn(optional_idx = 0)]
    pub i_e_extensions:
        Option<TargetNGRANNode_ToSourceNGRANNode_FailureTransparentContaineriE_Extensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct TargetNGRANNode_ToSourceNGRANNode_TransparentContainer {
    pub r_rc_container: RRCContainer,
    #[asn(optional_idx = 0)]
    pub i_e_extensions: Option<TargetNGRANNode_ToSourceNGRANNode_TransparentContaineriE_Extensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct TargetRANNodeID {
    pub global_ran_node_id: GlobalRANNodeID,
    pub selected_tai: TAI,
    #[asn(optional_idx = 0)]
    pub i_e_extensions: Option<TargetRANNodeIDiE_Extensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 2)]
pub struct TargetRNC_ID {
    pub l_ai: LAI,
    pub r_nc_id: RNC_ID,
    #[asn(optional_idx = 0)]
    pub extended_rnc_id: Option<ExtendedRNC_ID>,
    #[asn(optional_idx = 1)]
    pub i_e_extensions: Option<TargetRNC_IDiE_Extensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "OCTET-STRING")]
pub struct TargetToSource_TransparentContainer(Vec<u8>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct TargeteNB_ID {
    pub global_enb_id: GlobalNgENB_ID,
    pub selected_eps_tai: EPS_TAI,
    #[asn(optional_idx = 0)]
    pub i_e_extensions: Option<TargeteNB_IDiE_Extensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "OCTET-STRING")]
pub struct TargettoSource_Failure_TransparentContainer(Vec<u8>);

#[derive(Debug, AperCodec)]
#[asn(type = "INTEGER", lb = "0", ub = "127")]
pub struct Threshold_RSRP(u8);

#[derive(Debug, AperCodec)]
#[asn(type = "INTEGER", lb = "0", ub = "127")]
pub struct Threshold_RSRQ(u8);

#[derive(Debug, AperCodec)]
#[asn(type = "INTEGER", lb = "0", ub = "127")]
pub struct Threshold_SINR(u8);

#[derive(Debug, AperCodec)]
#[asn(type = "OCTET-STRING", sz_extensible = false, sz_lb = "4", sz_ub = "4")]
pub struct TimeStamp(Vec<u8>);

#[derive(Debug, AperCodec)]
#[asn(type = "ENUMERATED", lb = "0", ub = "15")]
pub struct TimeToTrigger(u8);
impl TimeToTrigger {
    const MS0: u8 = 0u8;
    const MS40: u8 = 1u8;
    const MS64: u8 = 2u8;
    const MS80: u8 = 3u8;
    const MS100: u8 = 4u8;
    const MS128: u8 = 5u8;
    const MS160: u8 = 6u8;
    const MS256: u8 = 7u8;
    const MS320: u8 = 8u8;
    const MS480: u8 = 9u8;
    const MS512: u8 = 10u8;
    const MS640: u8 = 11u8;
    const MS1024: u8 = 12u8;
    const MS1280: u8 = 13u8;
    const MS2560: u8 = 14u8;
    const MS5120: u8 = 15u8;
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
#[asn(type = "INTEGER", lb = "0", ub = "4095")]
pub struct TimeUEStayedInCell(u16);

#[derive(Debug, AperCodec)]
#[asn(type = "INTEGER", lb = "0", ub = "40950")]
pub struct TimeUEStayedInCellEnhancedGranularity(u16);

#[derive(Debug, AperCodec)]
#[asn(type = "ENUMERATED", extensible = true, lb = "0", ub = "0")]
pub struct TimerApproachForGUAMIRemoval(u8);
impl TimerApproachForGUAMIRemoval {
    const APPLY_TIMER: u8 = 0u8;
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 2)]
pub struct TooearlyIntersystemHO {
    pub sourcecell_id: EUTRA_CGI,
    pub failurecell_id: NGRAN_CGI,
    #[asn(optional_idx = 0)]
    pub u_erlf_report_container: Option<UERLFReportContainer>,
    #[asn(optional_idx = 1)]
    pub i_e_extensions: Option<TooearlyIntersystemHOiE_Extensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct TraceActivation {
    pub n_gran_trace_id: NGRANTraceID,
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
#[asn(type = "ENUMERATED", extensible = true, lb = "0", ub = "1")]
pub struct TypeOfError(u8);
impl TypeOfError {
    const NOT_UNDERSTOOD: u8 = 0u8;
    const MISSING: u8 = 1u8;
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 7)]
pub struct UE_DifferentiationInfo {
    #[asn(optional_idx = 0)]
    pub periodic_communication_indicator: Option<ENUMERATED_83>,
    #[asn(optional_idx = 1)]
    pub periodic_time: Option<INTEGER_84>,
    #[asn(optional_idx = 2)]
    pub scheduled_communication_time: Option<ScheduledCommunicationTime>,
    #[asn(optional_idx = 3)]
    pub stationary_indication: Option<ENUMERATED_85>,
    #[asn(optional_idx = 4)]
    pub traffic_profile: Option<ENUMERATED_86>,
    #[asn(optional_idx = 5)]
    pub battery_indication: Option<ENUMERATED_87>,
    #[asn(optional_idx = 6)]
    pub i_e_extensions: Option<UE_DifferentiationInfoiE_Extensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct UE_NGAP_ID_pair {
    pub a_mf_ue_ngap_id: AMF_UE_NGAP_ID,
    pub r_an_ue_ngap_id: RAN_UE_NGAP_ID,
    #[asn(optional_idx = 0)]
    pub i_e_extensions: Option<UE_NGAP_ID_pairiE_Extensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "CHOICE", lb = "0", ub = "2", extensible = false)]
pub enum UE_NGAP_IDs {
    #[asn(key = 0, extended = false)]
    uE_NGAP_ID_pair(UE_NGAP_ID_pair),
    #[asn(key = 1, extended = false)]
    aMF_UE_NGAP_ID(AMF_UE_NGAP_ID),
    #[asn(key = 2, extended = false)]
    choice_Extensions(UE_NGAP_IDschoice_Extensions),
}

#[derive(Debug, AperCodec)]
#[asn(type = "ENUMERATED", extensible = true, lb = "0", ub = "0")]
pub struct UE_UP_CIoT_Support(u8);
impl UE_UP_CIoT_Support {
    const SUPPORTED: u8 = 0u8;
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 3)]
pub struct UE_associatedLogicalNG_connectionItem {
    #[asn(optional_idx = 0)]
    pub a_mf_ue_ngap_id: Option<AMF_UE_NGAP_ID>,
    #[asn(optional_idx = 1)]
    pub r_an_ue_ngap_id: Option<RAN_UE_NGAP_ID>,
    #[asn(optional_idx = 2)]
    pub i_e_extensions: Option<UE_associatedLogicalNG_connectionItemiE_Extensions>,
}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65536"
)]
pub struct UE_associatedLogicalNG_connectionList(Vec<UE_associatedLogicalNG_connectionItem>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct UEAggregateMaximumBitRate {
    pub u_e_aggregate_maximum_bit_rate_dl: BitRate,
    pub u_e_aggregate_maximum_bit_rate_ul: BitRate,
    #[asn(optional_idx = 0)]
    pub i_e_extensions: Option<UEAggregateMaximumBitRateiE_Extensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "ENUMERATED", extensible = true, lb = "0", ub = "0")]
pub struct UECapabilityInfoRequest(u8);
impl UECapabilityInfoRequest {
    const REQUESTED: u8 = 0u8;
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true)]
pub struct UEContextModificationFailure {
    pub protocol_i_es: UEContextModificationFailureprotocolIEs,
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
#[asn(type = "ENUMERATED", extensible = true, lb = "0", ub = "0")]
pub struct UEContextRequest(u8);
impl UEContextRequest {
    const REQUESTED: u8 = 0u8;
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
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 2)]
pub struct UEContextResumeRequestTransfer {
    #[asn(optional_idx = 0)]
    pub qos_flow_failed_to_resume_list: Option<QosFlowListWithCause>,
    #[asn(optional_idx = 1)]
    pub i_e_extensions: Option<UEContextResumeRequestTransferiE_Extensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true)]
pub struct UEContextResumeResponse {
    pub protocol_i_es: UEContextResumeResponseprotocolIEs,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 2)]
pub struct UEContextResumeResponseTransfer {
    #[asn(optional_idx = 0)]
    pub qos_flow_failed_to_resume_list: Option<QosFlowListWithCause>,
    #[asn(optional_idx = 1)]
    pub i_e_extensions: Option<UEContextResumeResponseTransferiE_Extensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true)]
pub struct UEContextSuspendFailure {
    pub protocol_i_es: UEContextSuspendFailureprotocolIEs,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true)]
pub struct UEContextSuspendRequest {
    pub protocol_i_es: UEContextSuspendRequestprotocolIEs,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 2)]
pub struct UEContextSuspendRequestTransfer {
    #[asn(optional_idx = 0)]
    pub suspend_indicator: Option<SuspendIndicator>,
    #[asn(optional_idx = 1)]
    pub i_e_extensions: Option<UEContextSuspendRequestTransferiE_Extensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true)]
pub struct UEContextSuspendResponse {
    pub protocol_i_es: UEContextSuspendResponseprotocolIEs,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "16")]
pub struct UEHistoryInformation(Vec<LastVisitedCellItem>);

#[derive(Debug, AperCodec)]
#[asn(type = "CHOICE", lb = "0", ub = "1", extensible = false)]
pub enum UEHistoryInformationFromTheUE {
    #[asn(key = 0, extended = false)]
    nR(NRMobilityHistoryReport),
    #[asn(key = 1, extended = false)]
    choice_Extensions(UEHistoryInformationFromTheUEchoice_Extensions),
}

#[derive(Debug, AperCodec)]
#[asn(type = "CHOICE", lb = "0", ub = "1", extensible = false)]
pub enum UEIdentityIndexValue {
    #[asn(key = 0, extended = false)]
    indexLength10(BIT_STRING_88),
    #[asn(key = 1, extended = false)]
    choice_Extensions(UEIdentityIndexValuechoice_Extensions),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true)]
pub struct UEInformationTransfer {
    pub protocol_i_es: UEInformationTransferprotocolIEs,
}

#[derive(Debug, AperCodec)]
#[asn(type = "CHOICE", lb = "0", ub = "1", extensible = false)]
pub enum UEPagingIdentity {
    #[asn(key = 0, extended = false)]
    fiveG_S_TMSI(FiveG_S_TMSI),
    #[asn(key = 1, extended = false)]
    choice_Extensions(UEPagingIdentitychoice_Extensions),
}

#[derive(Debug, AperCodec)]
#[asn(type = "ENUMERATED", extensible = true, lb = "0", ub = "2")]
pub struct UEPresence(u8);
impl UEPresence {
    const IN: u8 = 0u8;
    const OUT: u8 = 1u8;
    const UNKNOWN: u8 = 2u8;
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct UEPresenceInAreaOfInterestItem {
    pub location_reporting_reference_id: LocationReportingReferenceID,
    pub u_e_presence: UEPresence,
    #[asn(optional_idx = 0)]
    pub i_e_extensions: Option<UEPresenceInAreaOfInterestItemiE_Extensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "64")]
pub struct UEPresenceInAreaOfInterestList(Vec<UEPresenceInAreaOfInterestItem>);

#[derive(Debug, AperCodec)]
#[asn(type = "CHOICE", lb = "0", ub = "2", extensible = false)]
pub enum UERLFReportContainer {
    #[asn(key = 0, extended = false)]
    nR(NRUERLFReportContainer),
    #[asn(key = 1, extended = false)]
    lTE(LTEUERLFReportContainer),
    #[asn(key = 2, extended = false)]
    choice_Extensions(UERLFReportContainerchoice_Extensions),
}

#[derive(Debug, AperCodec)]
#[asn(type = "OCTET-STRING")]
pub struct UERadioCapability(Vec<u8>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true)]
pub struct UERadioCapabilityCheckRequest {
    pub protocol_i_es: UERadioCapabilityCheckRequestprotocolIEs,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true)]
pub struct UERadioCapabilityCheckResponse {
    pub protocol_i_es: UERadioCapabilityCheckResponseprotocolIEs,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 3)]
pub struct UERadioCapabilityForPaging {
    #[asn(optional_idx = 0)]
    pub u_e_radio_capability_for_paging_of_nr: Option<UERadioCapabilityForPagingOfNR>,
    #[asn(optional_idx = 1)]
    pub u_e_radio_capability_for_paging_of_eutra: Option<UERadioCapabilityForPagingOfEUTRA>,
    #[asn(optional_idx = 2)]
    pub i_e_extensions: Option<UERadioCapabilityForPagingiE_Extensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "OCTET-STRING")]
pub struct UERadioCapabilityForPagingOfEUTRA(Vec<u8>);

#[derive(Debug, AperCodec)]
#[asn(type = "OCTET-STRING")]
pub struct UERadioCapabilityForPagingOfNB_IoT(Vec<u8>);

#[derive(Debug, AperCodec)]
#[asn(type = "OCTET-STRING")]
pub struct UERadioCapabilityForPagingOfNR(Vec<u8>);

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
pub struct UERadioCapabilityInfoIndication {
    pub protocol_i_es: UERadioCapabilityInfoIndicationprotocolIEs,
}

#[derive(Debug, AperCodec)]
#[asn(type = "ENUMERATED", extensible = true, lb = "0", ub = "0")]
pub struct UERetentionInformation(u8);
impl UERetentionInformation {
    const UES_RETAINED: u8 = 0u8;
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct UESecurityCapabilities {
    pub n_rencryption_algorithms: NRencryptionAlgorithms,
    pub n_rintegrity_protection_algorithms: NRintegrityProtectionAlgorithms,
    pub e_utr_aencryption_algorithms: EUTRAencryptionAlgorithms,
    pub e_utr_aintegrity_protection_algorithms: EUTRAintegrityProtectionAlgorithms,
    #[asn(optional_idx = 0)]
    pub i_e_extensions: Option<UESecurityCapabilitiesiE_Extensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true)]
pub struct UETNLABindingReleaseRequest {
    pub protocol_i_es: UETNLABindingReleaseRequestprotocolIEs,
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
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct UL_NGU_UP_TNLModifyItem {
    pub u_l_ngu_up_tnl_information: UPTransportLayerInformation,
    pub d_l_ngu_up_tnl_information: UPTransportLayerInformation,
    #[asn(optional_idx = 0)]
    pub i_e_extensions: Option<UL_NGU_UP_TNLModifyItemiE_Extensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "4")]
pub struct UL_NGU_UP_TNLModifyList(Vec<UL_NGU_UP_TNLModifyItem>);

#[derive(Debug, AperCodec)]
#[asn(type = "ENUMERATED", extensible = true, lb = "0", ub = "0")]
pub struct ULForwarding(u8);
impl ULForwarding {
    const UL_FORWARDING_PROPOSED: u8 = 0u8;
}

#[derive(Debug, AperCodec)]
#[asn(type = "CHOICE", lb = "0", ub = "1", extensible = false)]
pub enum UPTransportLayerInformation {
    #[asn(key = 0, extended = false)]
    gTPTunnel(GTPTunnel),
    #[asn(key = 1, extended = false)]
    choice_Extensions(UPTransportLayerInformationchoice_Extensions),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct UPTransportLayerInformationItem {
    pub n_gu_up_tnl_information: UPTransportLayerInformation,
    #[asn(optional_idx = 0)]
    pub i_e_extensions: Option<UPTransportLayerInformationItemiE_Extensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "3")]
pub struct UPTransportLayerInformationList(Vec<UPTransportLayerInformationItem>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct UPTransportLayerInformationPairItem {
    pub u_l_ngu_up_tnl_information: UPTransportLayerInformation,
    pub d_l_ngu_up_tnl_information: UPTransportLayerInformation,
    #[asn(optional_idx = 0)]
    pub i_e_extensions: Option<UPTransportLayerInformationPairItemiE_Extensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "3")]
pub struct UPTransportLayerInformationPairList(Vec<UPTransportLayerInformationPairItem>);

#[derive(Debug, AperCodec)]
#[asn(type = "VisibleString")]
pub struct URI_address(String);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 3)]
pub struct UnavailableGUAMIItem {
    pub g_uami: GUAMI,
    #[asn(optional_idx = 0)]
    pub timer_approach_for_guami_removal: Option<TimerApproachForGUAMIRemoval>,
    #[asn(optional_idx = 1)]
    pub backup_amf_name: Option<AMFName>,
    #[asn(optional_idx = 2)]
    pub i_e_extensions: Option<UnavailableGUAMIItemiE_Extensions>,
}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "256"
)]
pub struct UnavailableGUAMIList(Vec<UnavailableGUAMIItem>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct UnsuccessfulOutcome {
    #[asn(key_field = true)]
    pub procedure_code: ProcedureCode,
    pub criticality: Criticality,
    pub value: UnsuccessfulOutcomevalue,
}

#[derive(Debug, AperCodec)]
#[asn(type = "BITSTRING", sz_extensible = true, sz_lb = "8", sz_ub = "8")]
pub struct UpdateFeedback(BitVec<Msb0, u8>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true)]
pub struct UplinkNASTransport {
    pub protocol_i_es: UplinkNASTransportprotocolIEs,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true)]
pub struct UplinkNonUEAssociatedNRPPaTransport {
    pub protocol_i_es: UplinkNonUEAssociatedNRPPaTransportprotocolIEs,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true)]
pub struct UplinkRANConfigurationTransfer {
    pub protocol_i_es: UplinkRANConfigurationTransferprotocolIEs,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true)]
pub struct UplinkRANEarlyStatusTransfer {
    pub protocol_i_es: UplinkRANEarlyStatusTransferprotocolIEs,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true)]
pub struct UplinkRANStatusTransfer {
    pub protocol_i_es: UplinkRANStatusTransferprotocolIEs,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true)]
pub struct UplinkRIMInformationTransfer {
    pub protocol_i_es: UplinkRIMInformationTransferprotocolIEs,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true)]
pub struct UplinkUEAssociatedNRPPaTransport {
    pub protocol_i_es: UplinkUEAssociatedNRPPaTransportprotocolIEs,
}

#[derive(Debug, AperCodec)]
#[asn(type = "CHOICE", lb = "0", ub = "3", extensible = false)]
pub enum UserLocationInformation {
    #[asn(key = 0, extended = false)]
    userLocationInformationEUTRA(UserLocationInformationEUTRA),
    #[asn(key = 1, extended = false)]
    userLocationInformationNR(UserLocationInformationNR),
    #[asn(key = 2, extended = false)]
    userLocationInformationN3IWF(UserLocationInformationN3IWF),
    #[asn(key = 3, extended = false)]
    choice_Extensions(UserLocationInformationchoice_Extensions),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 2)]
pub struct UserLocationInformationEUTRA {
    pub e_utra_cgi: EUTRA_CGI,
    pub t_ai: TAI,
    #[asn(optional_idx = 0)]
    pub time_stamp: Option<TimeStamp>,
    #[asn(optional_idx = 1)]
    pub i_e_extensions: Option<UserLocationInformationEUTRAiE_Extensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct UserLocationInformationN3IWF {
    pub i_p_address: TransportLayerAddress,
    pub port_number: PortNumber,
    #[asn(optional_idx = 0)]
    pub i_e_extensions: Option<UserLocationInformationN3IWFiE_Extensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 2)]
pub struct UserLocationInformationNR {
    pub n_r_cgi: NR_CGI,
    pub t_ai: TAI,
    #[asn(optional_idx = 0)]
    pub time_stamp: Option<TimeStamp>,
    #[asn(optional_idx = 1)]
    pub i_e_extensions: Option<UserLocationInformationNRiE_Extensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 2)]
pub struct UserLocationInformationTNGF {
    pub t_nap_id: TNAP_ID,
    pub i_p_address: TransportLayerAddress,
    #[asn(optional_idx = 0)]
    pub port_number: Option<PortNumber>,
    #[asn(optional_idx = 1)]
    pub i_e_extensions: Option<UserLocationInformationTNGFiE_Extensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 2)]
pub struct UserLocationInformationTWIF {
    pub t_wap_id: TWAP_ID,
    pub i_p_address: TransportLayerAddress,
    #[asn(optional_idx = 0)]
    pub port_number: Option<PortNumber>,
    #[asn(optional_idx = 1)]
    pub i_e_extensions: Option<UserLocationInformationTWIFiE_Extensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "CHOICE", lb = "0", ub = "2", extensible = false)]
pub enum UserLocationInformationW_AGF {
    #[asn(key = 0, extended = false)]
    globalLine_ID(GlobalLine_ID),
    #[asn(key = 1, extended = false)]
    hFCNode_ID(HFCNode_ID),
    #[asn(key = 2, extended = false)]
    choice_Extensions(UserLocationInformationW_AGFchoice_Extensions),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct UserPlaneSecurityInformation {
    pub security_result: SecurityResult,
    pub security_indication: SecurityIndication,
    #[asn(optional_idx = 0)]
    pub i_e_extensions: Option<UserPlaneSecurityInformationiE_Extensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "ENUMERATED", extensible = true, lb = "0", ub = "1")]
pub struct VehicleUE(u8);
impl VehicleUE {
    const AUTHORIZED: u8 = 0u8;
    const NOT_AUTHORIZED: u8 = 1u8;
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct VolumeTimedReport_Item {
    pub start_time_stamp: OCTET_STRING_89,
    pub end_time_stamp: OCTET_STRING_90,
    pub usage_count_ul: INTEGER_91,
    pub usage_count_dl: INTEGER_92,
    #[asn(optional_idx = 0)]
    pub i_e_extensions: Option<VolumeTimedReport_ItemiE_Extensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "2")]
pub struct VolumeTimedReportList(Vec<VolumeTimedReport_Item>);

#[derive(Debug, AperCodec)]
#[asn(type = "CHOICE", lb = "0", ub = "1", extensible = false)]
pub enum W_AGF_ID {
    #[asn(key = 0, extended = false)]
    w_AGF_ID(BIT_STRING_93),
    #[asn(key = 1, extended = false)]
    choice_Extensions(W_AGF_IDchoice_Extensions),
}

#[derive(Debug, AperCodec)]
#[asn(type = "ENUMERATED", extensible = true, lb = "0", ub = "0")]
pub struct WLANMeasConfig(u8);
impl WLANMeasConfig {
    const SETUP: u8 = 0u8;
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct WLANMeasConfigNameItem {
    pub w_lan_name: WLANName,
    #[asn(optional_idx = 0)]
    pub i_e_extensions: Option<WLANMeasConfigNameItemiE_Extensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "4")]
pub struct WLANMeasConfigNameList(Vec<WLANMeasConfigNameItem>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 4)]
pub struct WLANMeasurementConfiguration {
    pub wlan_meas_config: WLANMeasConfig,
    #[asn(optional_idx = 0)]
    pub wlan_meas_config_name_list: Option<WLANMeasConfigNameList>,
    #[asn(optional_idx = 1)]
    pub wlan_rssi: Option<ENUMERATED_94>,
    #[asn(optional_idx = 2)]
    pub wlan_rtt: Option<ENUMERATED_95>,
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
#[asn(type = "CHOICE", lb = "0", ub = "4", extensible = false)]
pub enum WarningAreaList {
    #[asn(key = 0, extended = false)]
    eUTRA_CGIListForWarning(EUTRA_CGIListForWarning),
    #[asn(key = 1, extended = false)]
    nR_CGIListForWarning(NR_CGIListForWarning),
    #[asn(key = 2, extended = false)]
    tAIListForWarning(TAIListForWarning),
    #[asn(key = 3, extended = false)]
    emergencyAreaIDList(EmergencyAreaIDList),
    #[asn(key = 4, extended = false)]
    choice_Extensions(WarningAreaListchoice_Extensions),
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
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 3)]
pub struct XnExtTLA_Item {
    #[asn(optional_idx = 0)]
    pub i_psec_tla: Option<TransportLayerAddress>,
    #[asn(optional_idx = 1)]
    pub g_tp_tl_as: Option<XnGTP_TLAs>,
    #[asn(optional_idx = 2)]
    pub i_e_extensions: Option<XnExtTLA_ItemiE_Extensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "16")]
pub struct XnExtTLAs(Vec<XnExtTLA_Item>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "16")]
pub struct XnGTP_TLAs(Vec<TransportLayerAddress>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "2")]
pub struct XnTLAs(Vec<TransportLayerAddress>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 2)]
pub struct XnTNLConfigurationInfo {
    pub xn_transport_layer_addresses: XnTLAs,
    #[asn(optional_idx = 0)]
    pub xn_extended_transport_layer_addresses: Option<XnExtTLAs>,
    #[asn(optional_idx = 1)]
    pub i_e_extensions: Option<XnTNLConfigurationInfoiE_Extensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct AMF_TNLAssociationSetupItemiE_Extensions_Item {}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct AMF_TNLAssociationSetupItemiE_Extensions(
    Vec<AMF_TNLAssociationSetupItemiE_Extensions_Item>,
);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct AMF_TNLAssociationToAddItemiE_Extensions_Item {}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct AMF_TNLAssociationToAddItemiE_Extensions(
    Vec<AMF_TNLAssociationToAddItemiE_Extensions_Item>,
);

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum AMF_TNLAssociationToRemoveItemiE_Extensions_ItemextensionValue {
    #[asn(key = 168)]
    CPTransportLayerInformation(CPTransportLayerInformation),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct AMF_TNLAssociationToRemoveItemiE_Extensions_Item {
    #[asn(key_field = true)]
    pub id: ProtocolExtensionID,
    pub criticality: Criticality,
    pub extension_value: AMF_TNLAssociationToRemoveItemiE_Extensions_ItemextensionValue,
}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct AMF_TNLAssociationToRemoveItemiE_Extensions(
    Vec<AMF_TNLAssociationToRemoveItemiE_Extensions_Item>,
);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct AMF_TNLAssociationToUpdateItemiE_Extensions_Item {}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct AMF_TNLAssociationToUpdateItemiE_Extensions(
    Vec<AMF_TNLAssociationToUpdateItemiE_Extensions_Item>,
);

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum AMFCPRelocationIndicationprotocolIEs_Itemvalue {
    #[asn(key = 10)]
    AMF_UE_NGAP_ID(AMF_UE_NGAP_ID),
    #[asn(key = 0)]
    AllowedNSSAI(AllowedNSSAI),
    #[asn(key = 85)]
    RAN_UE_NGAP_ID(RAN_UE_NGAP_ID),
    #[asn(key = 148)]
    S_NSSAI(S_NSSAI),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct AMFCPRelocationIndicationprotocolIEs_Item {
    #[asn(key_field = true)]
    pub id: ProtocolIE_ID,
    pub criticality: Criticality,
    pub value: AMFCPRelocationIndicationprotocolIEs_Itemvalue,
}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "0",
    sz_ub = "65535"
)]
pub struct AMFCPRelocationIndicationprotocolIEs(Vec<AMFCPRelocationIndicationprotocolIEs_Item>);

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum AMFConfigurationUpdateprotocolIEs_Itemvalue {
    #[asn(key = 6)]
    AMF_TNLAssociationToAddList(AMF_TNLAssociationToAddList),
    #[asn(key = 7)]
    AMF_TNLAssociationToRemoveList(AMF_TNLAssociationToRemoveList),
    #[asn(key = 8)]
    AMF_TNLAssociationToUpdateList(AMF_TNLAssociationToUpdateList),
    #[asn(key = 1)]
    AMFName(AMFName),
    #[asn(key = 274)]
    Extended_AMFName(Extended_AMFName),
    #[asn(key = 80)]
    PLMNSupportList(PLMNSupportList),
    #[asn(key = 86)]
    RelativeAMFCapacity(RelativeAMFCapacity),
    #[asn(key = 96)]
    ServedGUAMIList(ServedGUAMIList),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct AMFConfigurationUpdateprotocolIEs_Item {
    #[asn(key_field = true)]
    pub id: ProtocolIE_ID,
    pub criticality: Criticality,
    pub value: AMFConfigurationUpdateprotocolIEs_Itemvalue,
}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "0",
    sz_ub = "65535"
)]
pub struct AMFConfigurationUpdateprotocolIEs(Vec<AMFConfigurationUpdateprotocolIEs_Item>);

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum AMFConfigurationUpdateAcknowledgeprotocolIEs_Itemvalue {
    #[asn(key = 5)]
    AMF_TNLAssociationSetupList(AMF_TNLAssociationSetupList),
    #[asn(key = 19)]
    CriticalityDiagnostics(CriticalityDiagnostics),
    #[asn(key = 4)]
    TNLAssociationList(TNLAssociationList),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct AMFConfigurationUpdateAcknowledgeprotocolIEs_Item {
    #[asn(key_field = true)]
    pub id: ProtocolIE_ID,
    pub criticality: Criticality,
    pub value: AMFConfigurationUpdateAcknowledgeprotocolIEs_Itemvalue,
}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "0",
    sz_ub = "65535"
)]
pub struct AMFConfigurationUpdateAcknowledgeprotocolIEs(
    Vec<AMFConfigurationUpdateAcknowledgeprotocolIEs_Item>,
);

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum AMFConfigurationUpdateFailureprotocolIEs_Itemvalue {
    #[asn(key = 15)]
    Cause(Cause),
    #[asn(key = 19)]
    CriticalityDiagnostics(CriticalityDiagnostics),
    #[asn(key = 107)]
    TimeToWait(TimeToWait),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct AMFConfigurationUpdateFailureprotocolIEs_Item {
    #[asn(key_field = true)]
    pub id: ProtocolIE_ID,
    pub criticality: Criticality,
    pub value: AMFConfigurationUpdateFailureprotocolIEs_Itemvalue,
}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "0",
    sz_ub = "65535"
)]
pub struct AMFConfigurationUpdateFailureprotocolIEs(
    Vec<AMFConfigurationUpdateFailureprotocolIEs_Item>,
);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct AMFPagingTargetchoice_Extensions {}

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum AMFStatusIndicationprotocolIEs_Itemvalue {
    #[asn(key = 120)]
    UnavailableGUAMIList(UnavailableGUAMIList),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct AMFStatusIndicationprotocolIEs_Item {
    #[asn(key_field = true)]
    pub id: ProtocolIE_ID,
    pub criticality: Criticality,
    pub value: AMFStatusIndicationprotocolIEs_Itemvalue,
}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "0",
    sz_ub = "65535"
)]
pub struct AMFStatusIndicationprotocolIEs(Vec<AMFStatusIndicationprotocolIEs_Item>);

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum AdditionalDLUPTNLInformationForHOItemiE_Extensions_ItemextensionValue {
    #[asn(key = 183)]
    UPTransportLayerInformation(UPTransportLayerInformation),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct AdditionalDLUPTNLInformationForHOItemiE_Extensions_Item {
    #[asn(key_field = true)]
    pub id: ProtocolExtensionID,
    pub criticality: Criticality,
    pub extension_value: AdditionalDLUPTNLInformationForHOItemiE_Extensions_ItemextensionValue,
}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct AdditionalDLUPTNLInformationForHOItemiE_Extensions(
    Vec<AdditionalDLUPTNLInformationForHOItemiE_Extensions_Item>,
);

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
#[asn(type = "ENUMERATED", extensible = true, lb = "0", ub = "1")]
pub struct ENUMERATED_2(u8);
impl ENUMERATED_2 {
    const RESTRICTED: u8 = 0u8;
    const NOT_RESTRICTED: u8 = 1u8;
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct Allowed_PNI_NPN_ItemiE_Extensions_Item {}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct Allowed_PNI_NPN_ItemiE_Extensions(Vec<Allowed_PNI_NPN_ItemiE_Extensions_Item>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct AllowedNSSAI_ItemiE_Extensions_Item {}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct AllowedNSSAI_ItemiE_Extensions(Vec<AllowedNSSAI_ItemiE_Extensions_Item>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct AlternativeQoSParaSetItemiE_Extensions_Item {}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct AlternativeQoSParaSetItemiE_Extensions(Vec<AlternativeQoSParaSetItemiE_Extensions_Item>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct AreaOfInterestiE_Extensions_Item {}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct AreaOfInterestiE_Extensions(Vec<AreaOfInterestiE_Extensions_Item>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct AreaOfInterestCellItemiE_Extensions_Item {}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct AreaOfInterestCellItemiE_Extensions(Vec<AreaOfInterestCellItemiE_Extensions_Item>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct AreaOfInterestItemiE_Extensions_Item {}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct AreaOfInterestItemiE_Extensions(Vec<AreaOfInterestItemiE_Extensions_Item>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct AreaOfInterestRANNodeItemiE_Extensions_Item {}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct AreaOfInterestRANNodeItemiE_Extensions(Vec<AreaOfInterestRANNodeItemiE_Extensions_Item>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct AreaOfInterestTAIItemiE_Extensions_Item {}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct AreaOfInterestTAIItemiE_Extensions(Vec<AreaOfInterestTAIItemiE_Extensions_Item>);

#[derive(Debug, AperCodec)]
#[asn(type = "NULL")]
pub struct NULL_3;

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct AreaScopeOfMDT_EUTRAchoice_Extensions {}

#[derive(Debug, AperCodec)]
#[asn(type = "NULL")]
pub struct NULL_4;

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct AreaScopeOfMDT_NRchoice_Extensions {}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct AreaScopeOfNeighCellsItemiE_Extensions_Item {}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct AreaScopeOfNeighCellsItemiE_Extensions(Vec<AreaScopeOfNeighCellsItemiE_Extensions_Item>);

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum AssistanceDataForPagingiE_Extensions_ItemextensionValue {
    #[asn(key = 260)]
    NPN_PagingAssistanceInformation(NPN_PagingAssistanceInformation),
    #[asn(key = 207)]
    PagingAssisDataforCEcapabUE(PagingAssisDataforCEcapabUE),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct AssistanceDataForPagingiE_Extensions_Item {
    #[asn(key_field = true)]
    pub id: ProtocolExtensionID,
    pub criticality: Criticality,
    pub extension_value: AssistanceDataForPagingiE_Extensions_ItemextensionValue,
}

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
#[asn(type = "ENUMERATED", extensible = true, lb = "0", ub = "1")]
pub struct ENUMERATED_5(u8);
impl ENUMERATED_5 {
    const UL: u8 = 0u8;
    const DL: u8 = 1u8;
}

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum AssociatedQosFlowItemiE_Extensions_ItemextensionValue {
    #[asn(key = 221)]
    AlternativeQoSParaSetIndex(AlternativeQoSParaSetIndex),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct AssociatedQosFlowItemiE_Extensions_Item {
    #[asn(key_field = true)]
    pub id: ProtocolExtensionID,
    pub criticality: Criticality,
    pub extension_value: AssociatedQosFlowItemiE_Extensions_ItemextensionValue,
}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct AssociatedQosFlowItemiE_Extensions(Vec<AssociatedQosFlowItemiE_Extensions_Item>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct BluetoothMeasConfigNameItemiE_Extensions_Item {}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct BluetoothMeasConfigNameItemiE_Extensions(
    Vec<BluetoothMeasConfigNameItemiE_Extensions_Item>,
);

#[derive(Debug, AperCodec)]
#[asn(type = "ENUMERATED", extensible = true, lb = "0", ub = "0")]
pub struct ENUMERATED_6(u8);
impl ENUMERATED_6 {
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
pub struct BroadcastCancelledAreaListchoice_Extensions {}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct BroadcastCompletedAreaListchoice_Extensions {}

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum BroadcastPLMNItemiE_Extensions_ItemextensionValue {
    #[asn(key = 271)]
    ExtendedSliceSupportList(ExtendedSliceSupportList),
    #[asn(key = 258)]
    NPN_Support(NPN_Support),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct BroadcastPLMNItemiE_Extensions_Item {
    #[asn(key_field = true)]
    pub id: ProtocolExtensionID,
    pub criticality: Criticality,
    pub extension_value: BroadcastPLMNItemiE_Extensions_ItemextensionValue,
}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct BroadcastPLMNItemiE_Extensions(Vec<BroadcastPLMNItemiE_Extensions_Item>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct CNAssistedRANTuningiE_Extensions_Item {}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct CNAssistedRANTuningiE_Extensions(Vec<CNAssistedRANTuningiE_Extensions_Item>);

#[derive(Debug, AperCodec)]
#[asn(type = "ENUMERATED", extensible = true, lb = "0", ub = "1")]
pub struct ENUMERATED_7(u8);
impl ENUMERATED_7 {
    const EPC_FORBIDDEN: u8 = 0u8;
    const FIVE_GC_FORBIDDEN: u8 = 1u8;
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct CNTypeRestrictionsForEquivalentItemiE_Extensions_Item {}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct CNTypeRestrictionsForEquivalentItemiE_Extensions(
    Vec<CNTypeRestrictionsForEquivalentItemiE_Extensions_Item>,
);

#[derive(Debug, AperCodec)]
#[asn(type = "INTEGER", lb = "0", ub = "4095")]
pub struct INTEGER_8(u16);

#[derive(Debug, AperCodec)]
#[asn(type = "INTEGER", lb = "0", ub = "1048575")]
pub struct INTEGER_9(u32);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct COUNTValueForPDCP_SN12iE_Extensions_Item {}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct COUNTValueForPDCP_SN12iE_Extensions(Vec<COUNTValueForPDCP_SN12iE_Extensions_Item>);

#[derive(Debug, AperCodec)]
#[asn(type = "INTEGER", lb = "0", ub = "262143")]
pub struct INTEGER_10(u32);

#[derive(Debug, AperCodec)]
#[asn(type = "INTEGER", lb = "0", ub = "16383")]
pub struct INTEGER_11(u16);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct COUNTValueForPDCP_SN18iE_Extensions_Item {}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct COUNTValueForPDCP_SN18iE_Extensions(Vec<COUNTValueForPDCP_SN18iE_Extensions_Item>);

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum CPTransportLayerInformationchoice_Extensionsvalue {
    #[asn(key = 169)]
    EndpointIPAddressAndPort(EndpointIPAddressAndPort),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct CPTransportLayerInformationchoice_Extensions {
    #[asn(key_field = true)]
    pub id: ProtocolIE_ID,
    pub criticality: Criticality,
    pub value: CPTransportLayerInformationchoice_Extensionsvalue,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct CancelledCellsInEAI_EUTRA_ItemiE_Extensions_Item {}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct CancelledCellsInEAI_EUTRA_ItemiE_Extensions(
    Vec<CancelledCellsInEAI_EUTRA_ItemiE_Extensions_Item>,
);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct CancelledCellsInEAI_NR_ItemiE_Extensions_Item {}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct CancelledCellsInEAI_NR_ItemiE_Extensions(
    Vec<CancelledCellsInEAI_NR_ItemiE_Extensions_Item>,
);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct CancelledCellsInTAI_EUTRA_ItemiE_Extensions_Item {}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct CancelledCellsInTAI_EUTRA_ItemiE_Extensions(
    Vec<CancelledCellsInTAI_EUTRA_ItemiE_Extensions_Item>,
);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct CancelledCellsInTAI_NR_ItemiE_Extensions_Item {}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct CancelledCellsInTAI_NR_ItemiE_Extensions(
    Vec<CancelledCellsInTAI_NR_ItemiE_Extensions_Item>,
);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct CandidateCellchoice_Extensions {}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct CandidateCellIDiE_Extensions_Item {}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct CandidateCellIDiE_Extensions(Vec<CandidateCellIDiE_Extensions_Item>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct CandidateCellItemiE_Extensions_Item {}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct CandidateCellItemiE_Extensions(Vec<CandidateCellItemiE_Extensions_Item>);

#[derive(Debug, AperCodec)]
#[asn(type = "INTEGER", lb = "0", ub = "1007", extensible = true)]
pub struct INTEGER_12(u16);

#[derive(Debug, AperCodec)]
#[asn(type = "INTEGER", lb = "0", ub = "3279165")]
pub struct INTEGER_13(u32);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct CandidatePCIiE_Extensions_Item {}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct CandidatePCIiE_Extensions(Vec<CandidatePCIiE_Extensions_Item>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct Causechoice_Extensions {}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct Cell_CAGInformationiE_Extensions_Item {}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct Cell_CAGInformationiE_Extensions(Vec<Cell_CAGInformationiE_Extensions_Item>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct CellBasedMDT_EUTRAiE_Extensions_Item {}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct CellBasedMDT_EUTRAiE_Extensions(Vec<CellBasedMDT_EUTRAiE_Extensions_Item>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct CellBasedMDT_NRiE_Extensions_Item {}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct CellBasedMDT_NRiE_Extensions(Vec<CellBasedMDT_NRiE_Extensions_Item>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct CellIDBroadcastEUTRA_ItemiE_Extensions_Item {}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct CellIDBroadcastEUTRA_ItemiE_Extensions(Vec<CellIDBroadcastEUTRA_ItemiE_Extensions_Item>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct CellIDBroadcastNR_ItemiE_Extensions_Item {}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct CellIDBroadcastNR_ItemiE_Extensions(Vec<CellIDBroadcastNR_ItemiE_Extensions_Item>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct CellIDCancelledEUTRA_ItemiE_Extensions_Item {}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct CellIDCancelledEUTRA_ItemiE_Extensions(Vec<CellIDCancelledEUTRA_ItemiE_Extensions_Item>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct CellIDCancelledNR_ItemiE_Extensions_Item {}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct CellIDCancelledNR_ItemiE_Extensions(Vec<CellIDCancelledNR_ItemiE_Extensions_Item>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct CellIDListForRestartchoice_Extensions {}

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum CellTrafficTraceprotocolIEs_Itemvalue {
    #[asn(key = 10)]
    AMF_UE_NGAP_ID(AMF_UE_NGAP_ID),
    #[asn(key = 43)]
    NGRAN_CGI(NGRAN_CGI),
    #[asn(key = 44)]
    NGRANTraceID(NGRANTraceID),
    #[asn(key = 256)]
    PrivacyIndicator(PrivacyIndicator),
    #[asn(key = 85)]
    RAN_UE_NGAP_ID(RAN_UE_NGAP_ID),
    #[asn(key = 109)]
    TransportLayerAddress(TransportLayerAddress),
    #[asn(key = 257)]
    URI_address(URI_address),
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
#[asn(type = "SEQUENCE", extensible = false)]
pub struct CompletedCellsInEAI_EUTRA_ItemiE_Extensions_Item {}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct CompletedCellsInEAI_EUTRA_ItemiE_Extensions(
    Vec<CompletedCellsInEAI_EUTRA_ItemiE_Extensions_Item>,
);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct CompletedCellsInEAI_NR_ItemiE_Extensions_Item {}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct CompletedCellsInEAI_NR_ItemiE_Extensions(
    Vec<CompletedCellsInEAI_NR_ItemiE_Extensions_Item>,
);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct CompletedCellsInTAI_EUTRA_ItemiE_Extensions_Item {}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct CompletedCellsInTAI_EUTRA_ItemiE_Extensions(
    Vec<CompletedCellsInTAI_EUTRA_ItemiE_Extensions_Item>,
);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct CompletedCellsInTAI_NR_ItemiE_Extensions_Item {}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct CompletedCellsInTAI_NR_ItemiE_Extensions(
    Vec<CompletedCellsInTAI_NR_ItemiE_Extensions_Item>,
);

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum ConnectionEstablishmentIndicationprotocolIEs_Itemvalue {
    #[asn(key = 10)]
    AMF_UE_NGAP_ID(AMF_UE_NGAP_ID),
    #[asn(key = 0)]
    AllowedNSSAI(AllowedNSSAI),
    #[asn(key = 222)]
    CEmodeBrestricted(CEmodeBrestricted),
    #[asn(key = 212)]
    DL_CP_SecurityInformation(DL_CP_SecurityInformation),
    #[asn(key = 226)]
    EndIndication(EndIndication),
    #[asn(key = 205)]
    Enhanced_CoverageRestriction(Enhanced_CoverageRestriction),
    #[asn(key = 210)]
    NB_IoT_UEPriority(NB_IoT_UEPriority),
    #[asn(key = 85)]
    RAN_UE_NGAP_ID(RAN_UE_NGAP_ID),
    #[asn(key = 148)]
    S_NSSAI(S_NSSAI),
    #[asn(key = 209)]
    UE_DifferentiationInfo(UE_DifferentiationInfo),
    #[asn(key = 117)]
    UERadioCapability(UERadioCapability),
    #[asn(key = 264)]
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
#[asn(type = "OPEN")]
pub enum CoreNetworkAssistanceInformationForInactiveiE_Extensions_ItemextensionValue {
    #[asn(key = 280)]
    ExtendedUEIdentityIndexValue(ExtendedUEIdentityIndexValue),
    #[asn(key = 223)]
    PagingeDRXInformation(PagingeDRXInformation),
    #[asn(key = 118)]
    UERadioCapabilityForPaging(UERadioCapabilityForPaging),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct CoreNetworkAssistanceInformationForInactiveiE_Extensions_Item {
    #[asn(key_field = true)]
    pub id: ProtocolExtensionID,
    pub criticality: Criticality,
    pub extension_value:
        CoreNetworkAssistanceInformationForInactiveiE_Extensions_ItemextensionValue,
}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct CoreNetworkAssistanceInformationForInactiveiE_Extensions(
    Vec<CoreNetworkAssistanceInformationForInactiveiE_Extensions_Item>,
);

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
pub struct ENUMERATED_14(u8);
impl ENUMERATED_14 {
    const DAPS_HO_REQUIRED: u8 = 0u8;
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
pub struct ENUMERATED_15(u8);
impl ENUMERATED_15 {
    const DAPS_HO_ACCEPTED: u8 = 0u8;
    const DAPS_HO_NOT_ACCEPTED: u8 = 1u8;
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
pub struct DAPSResponseInfoItemiE_Extension_Item {}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct DAPSResponseInfoItemiE_Extension(Vec<DAPSResponseInfoItemiE_Extension_Item>);

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
#[asn(type = "SEQUENCE", extensible = false)]
pub struct DRBStatusDLchoice_Extensions {}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct DRBStatusDL12iE_Extension_Item {}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct DRBStatusDL12iE_Extension(Vec<DRBStatusDL12iE_Extension_Item>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct DRBStatusDL18iE_Extension_Item {}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct DRBStatusDL18iE_Extension(Vec<DRBStatusDL18iE_Extension_Item>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct DRBStatusULchoice_Extensions {}

#[derive(Debug, AperCodec)]
#[asn(type = "BITSTRING", sz_extensible = false, sz_lb = "1", sz_ub = "2048")]
pub struct BIT_STRING_16(BitVec<Msb0, u8>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct DRBStatusUL12iE_Extension_Item {}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct DRBStatusUL12iE_Extension(Vec<DRBStatusUL12iE_Extension_Item>);

#[derive(Debug, AperCodec)]
#[asn(
    type = "BITSTRING",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "131072"
)]
pub struct BIT_STRING_17(BitVec<Msb0, u8>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct DRBStatusUL18iE_Extension_Item {}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct DRBStatusUL18iE_Extension(Vec<DRBStatusUL18iE_Extension_Item>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct DRBsSubjectToEarlyStatusTransfer_ItemiE_Extension_Item {}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct DRBsSubjectToEarlyStatusTransfer_ItemiE_Extension(
    Vec<DRBsSubjectToEarlyStatusTransfer_ItemiE_Extension_Item>,
);

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum DRBsSubjectToStatusTransferItemiE_Extension_ItemextensionValue {
    #[asn(key = 159)]
    AssociatedQosFlowList(AssociatedQosFlowList),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct DRBsSubjectToStatusTransferItemiE_Extension_Item {
    #[asn(key_field = true)]
    pub id: ProtocolExtensionID,
    pub criticality: Criticality,
    pub extension_value: DRBsSubjectToStatusTransferItemiE_Extension_ItemextensionValue,
}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct DRBsSubjectToStatusTransferItemiE_Extension(
    Vec<DRBsSubjectToStatusTransferItemiE_Extension_Item>,
);

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum DRBsToQosFlowsMappingItemiE_Extensions_ItemextensionValue {
    #[asn(key = 266)]
    DAPSRequestInfo(DAPSRequestInfo),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct DRBsToQosFlowsMappingItemiE_Extensions_Item {
    #[asn(key_field = true)]
    pub id: ProtocolExtensionID,
    pub criticality: Criticality,
    pub extension_value: DRBsToQosFlowsMappingItemiE_Extensions_ItemextensionValue,
}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct DRBsToQosFlowsMappingItemiE_Extensions(Vec<DRBsToQosFlowsMappingItemiE_Extensions_Item>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct DataForwardingResponseDRBItemiE_Extensions_Item {}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct DataForwardingResponseDRBItemiE_Extensions(
    Vec<DataForwardingResponseDRBItemiE_Extensions_Item>,
);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct DataForwardingResponseERABListItemiE_Extensions_Item {}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct DataForwardingResponseERABListItemiE_Extensions(
    Vec<DataForwardingResponseERABListItemiE_Extensions_Item>,
);

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum DeactivateTraceprotocolIEs_Itemvalue {
    #[asn(key = 10)]
    AMF_UE_NGAP_ID(AMF_UE_NGAP_ID),
    #[asn(key = 44)]
    NGRANTraceID(NGRANTraceID),
    #[asn(key = 85)]
    RAN_UE_NGAP_ID(RAN_UE_NGAP_ID),
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
    #[asn(key = 10)]
    AMF_UE_NGAP_ID(AMF_UE_NGAP_ID),
    #[asn(key = 48)]
    AMFName(AMFName),
    #[asn(key = 0)]
    AllowedNSSAI(AllowedNSSAI),
    #[asn(key = 222)]
    CEmodeBrestricted(CEmodeBrestricted),
    #[asn(key = 226)]
    EndIndication(EndIndication),
    #[asn(key = 205)]
    Enhanced_CoverageRestriction(Enhanced_CoverageRestriction),
    #[asn(key = 206)]
    Extended_ConnectedTime(Extended_ConnectedTime),
    #[asn(key = 31)]
    IndexToRFSP(IndexToRFSP),
    #[asn(key = 36)]
    MobilityRestrictionList(MobilityRestrictionList),
    #[asn(key = 38)]
    NAS_PDU(NAS_PDU),
    #[asn(key = 85)]
    RAN_UE_NGAP_ID(RAN_UE_NGAP_ID),
    #[asn(key = 83)]
    RANPagingPriority(RANPagingPriority),
    #[asn(key = 177)]
    SRVCCOperationPossible(SRVCCOperationPossible),
    #[asn(key = 209)]
    UE_DifferentiationInfo(UE_DifferentiationInfo),
    #[asn(key = 110)]
    UEAggregateMaximumBitRate(UEAggregateMaximumBitRate),
    #[asn(key = 228)]
    UECapabilityInfoRequest(UECapabilityInfoRequest),
    #[asn(key = 117)]
    UERadioCapability(UERadioCapability),
    #[asn(key = 264)]
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
pub enum DownlinkNonUEAssociatedNRPPaTransportprotocolIEs_Itemvalue {
    #[asn(key = 46)]
    NRPPa_PDU(NRPPa_PDU),
    #[asn(key = 89)]
    RoutingID(RoutingID),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct DownlinkNonUEAssociatedNRPPaTransportprotocolIEs_Item {
    #[asn(key_field = true)]
    pub id: ProtocolIE_ID,
    pub criticality: Criticality,
    pub value: DownlinkNonUEAssociatedNRPPaTransportprotocolIEs_Itemvalue,
}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "0",
    sz_ub = "65535"
)]
pub struct DownlinkNonUEAssociatedNRPPaTransportprotocolIEs(
    Vec<DownlinkNonUEAssociatedNRPPaTransportprotocolIEs_Item>,
);

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum DownlinkRANConfigurationTransferprotocolIEs_Itemvalue {
    #[asn(key = 157)]
    EN_DCSONConfigurationTransfer(EN_DCSONConfigurationTransfer),
    #[asn(key = 250)]
    IntersystemSONConfigurationTransfer(IntersystemSONConfigurationTransfer),
    #[asn(key = 98)]
    SONConfigurationTransfer(SONConfigurationTransfer),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct DownlinkRANConfigurationTransferprotocolIEs_Item {
    #[asn(key_field = true)]
    pub id: ProtocolIE_ID,
    pub criticality: Criticality,
    pub value: DownlinkRANConfigurationTransferprotocolIEs_Itemvalue,
}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "0",
    sz_ub = "65535"
)]
pub struct DownlinkRANConfigurationTransferprotocolIEs(
    Vec<DownlinkRANConfigurationTransferprotocolIEs_Item>,
);

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum DownlinkRANEarlyStatusTransferprotocolIEs_Itemvalue {
    #[asn(key = 10)]
    AMF_UE_NGAP_ID(AMF_UE_NGAP_ID),
    #[asn(key = 268)]
    EarlyStatusTransfer_TransparentContainer(EarlyStatusTransfer_TransparentContainer),
    #[asn(key = 85)]
    RAN_UE_NGAP_ID(RAN_UE_NGAP_ID),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct DownlinkRANEarlyStatusTransferprotocolIEs_Item {
    #[asn(key_field = true)]
    pub id: ProtocolIE_ID,
    pub criticality: Criticality,
    pub value: DownlinkRANEarlyStatusTransferprotocolIEs_Itemvalue,
}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "0",
    sz_ub = "65535"
)]
pub struct DownlinkRANEarlyStatusTransferprotocolIEs(
    Vec<DownlinkRANEarlyStatusTransferprotocolIEs_Item>,
);

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum DownlinkRANStatusTransferprotocolIEs_Itemvalue {
    #[asn(key = 10)]
    AMF_UE_NGAP_ID(AMF_UE_NGAP_ID),
    #[asn(key = 85)]
    RAN_UE_NGAP_ID(RAN_UE_NGAP_ID),
    #[asn(key = 84)]
    RANStatusTransfer_TransparentContainer(RANStatusTransfer_TransparentContainer),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct DownlinkRANStatusTransferprotocolIEs_Item {
    #[asn(key_field = true)]
    pub id: ProtocolIE_ID,
    pub criticality: Criticality,
    pub value: DownlinkRANStatusTransferprotocolIEs_Itemvalue,
}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "0",
    sz_ub = "65535"
)]
pub struct DownlinkRANStatusTransferprotocolIEs(Vec<DownlinkRANStatusTransferprotocolIEs_Item>);

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum DownlinkRIMInformationTransferprotocolIEs_Itemvalue {
    #[asn(key = 175)]
    RIMInformationTransfer(RIMInformationTransfer),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct DownlinkRIMInformationTransferprotocolIEs_Item {
    #[asn(key_field = true)]
    pub id: ProtocolIE_ID,
    pub criticality: Criticality,
    pub value: DownlinkRIMInformationTransferprotocolIEs_Itemvalue,
}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "0",
    sz_ub = "65535"
)]
pub struct DownlinkRIMInformationTransferprotocolIEs(
    Vec<DownlinkRIMInformationTransferprotocolIEs_Item>,
);

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum DownlinkUEAssociatedNRPPaTransportprotocolIEs_Itemvalue {
    #[asn(key = 10)]
    AMF_UE_NGAP_ID(AMF_UE_NGAP_ID),
    #[asn(key = 46)]
    NRPPa_PDU(NRPPa_PDU),
    #[asn(key = 85)]
    RAN_UE_NGAP_ID(RAN_UE_NGAP_ID),
    #[asn(key = 89)]
    RoutingID(RoutingID),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct DownlinkUEAssociatedNRPPaTransportprotocolIEs_Item {
    #[asn(key_field = true)]
    pub id: ProtocolIE_ID,
    pub criticality: Criticality,
    pub value: DownlinkUEAssociatedNRPPaTransportprotocolIEs_Itemvalue,
}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "0",
    sz_ub = "65535"
)]
pub struct DownlinkUEAssociatedNRPPaTransportprotocolIEs(
    Vec<DownlinkUEAssociatedNRPPaTransportprotocolIEs_Item>,
);

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum Dynamic5QIDescriptoriE_Extensions_ItemextensionValue {
    #[asn(key = 188)]
    ExtendedPacketDelayBudget(ExtendedPacketDelayBudget),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct Dynamic5QIDescriptoriE_Extensions_Item {
    #[asn(key_field = true)]
    pub id: ProtocolExtensionID,
    pub criticality: Criticality,
    pub extension_value: Dynamic5QIDescriptoriE_Extensions_ItemextensionValue,
}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct Dynamic5QIDescriptoriE_Extensions(Vec<Dynamic5QIDescriptoriE_Extensions_Item>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct E_RABInformationItemiE_Extensions_Item {}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct E_RABInformationItemiE_Extensions(Vec<E_RABInformationItemiE_Extensions_Item>);

#[derive(Debug, AperCodec)]
#[asn(type = "BITSTRING", sz_extensible = false, sz_lb = "20", sz_ub = "20")]
pub struct BIT_STRING_18(BitVec<Msb0, u8>);

#[derive(Debug, AperCodec)]
#[asn(type = "BITSTRING", sz_extensible = false, sz_lb = "28", sz_ub = "28")]
pub struct BIT_STRING_19(BitVec<Msb0, u8>);

#[derive(Debug, AperCodec)]
#[asn(type = "BITSTRING", sz_extensible = false, sz_lb = "18", sz_ub = "18")]
pub struct BIT_STRING_20(BitVec<Msb0, u8>);

#[derive(Debug, AperCodec)]
#[asn(type = "BITSTRING", sz_extensible = false, sz_lb = "21", sz_ub = "21")]
pub struct BIT_STRING_21(BitVec<Msb0, u8>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct ENB_IDchoice_Extensions {}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct EPS_TAIiE_Extensions_Item {}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct EPS_TAIiE_Extensions(Vec<EPS_TAIiE_Extensions_Item>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct EUTRA_CGIiE_Extensions_Item {}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct EUTRA_CGIiE_Extensions(Vec<EUTRA_CGIiE_Extensions_Item>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct EarlyStatusTransfer_TransparentContaineriE_Extensions_Item {}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct EarlyStatusTransfer_TransparentContaineriE_Extensions(
    Vec<EarlyStatusTransfer_TransparentContaineriE_Extensions_Item>,
);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct EmergencyAreaIDBroadcastEUTRA_ItemiE_Extensions_Item {}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct EmergencyAreaIDBroadcastEUTRA_ItemiE_Extensions(
    Vec<EmergencyAreaIDBroadcastEUTRA_ItemiE_Extensions_Item>,
);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct EmergencyAreaIDBroadcastNR_ItemiE_Extensions_Item {}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct EmergencyAreaIDBroadcastNR_ItemiE_Extensions(
    Vec<EmergencyAreaIDBroadcastNR_ItemiE_Extensions_Item>,
);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct EmergencyAreaIDCancelledEUTRA_ItemiE_Extensions_Item {}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct EmergencyAreaIDCancelledEUTRA_ItemiE_Extensions(
    Vec<EmergencyAreaIDCancelledEUTRA_ItemiE_Extensions_Item>,
);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct EmergencyAreaIDCancelledNR_ItemiE_Extensions_Item {}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct EmergencyAreaIDCancelledNR_ItemiE_Extensions(
    Vec<EmergencyAreaIDCancelledNR_ItemiE_Extensions_Item>,
);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct EmergencyFallbackIndicatoriE_Extensions_Item {}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct EmergencyFallbackIndicatoriE_Extensions(
    Vec<EmergencyFallbackIndicatoriE_Extensions_Item>,
);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct EndpointIPAddressAndPortiE_Extensions_Item {}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct EndpointIPAddressAndPortiE_Extensions(Vec<EndpointIPAddressAndPortiE_Extensions_Item>);

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum ErrorIndicationprotocolIEs_Itemvalue {
    #[asn(key = 10)]
    AMF_UE_NGAP_ID(AMF_UE_NGAP_ID),
    #[asn(key = 15)]
    Cause(Cause),
    #[asn(key = 19)]
    CriticalityDiagnostics(CriticalityDiagnostics),
    #[asn(key = 26)]
    FiveG_S_TMSI(FiveG_S_TMSI),
    #[asn(key = 85)]
    RAN_UE_NGAP_ID(RAN_UE_NGAP_ID),
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
pub struct EventL1LoggedMDTConfigiE_Extensions_Item {}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct EventL1LoggedMDTConfigiE_Extensions(Vec<EventL1LoggedMDTConfigiE_Extensions_Item>);

#[derive(Debug, AperCodec)]
#[asn(type = "ENUMERATED", extensible = true, lb = "0", ub = "0")]
pub struct ENUMERATED_22(u8);
impl ENUMERATED_22 {
    const TRUE: u8 = 0u8;
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct EventTriggerchoice_Extensions {}

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
#[asn(type = "INTEGER", lb = "0", ub = "4095")]
pub struct INTEGER_23(u16);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct ExpectedUEMovingTrajectoryItemiE_Extensions_Item {}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct ExpectedUEMovingTrajectoryItemiE_Extensions(
    Vec<ExpectedUEMovingTrajectoryItemiE_Extensions_Item>,
);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct Extended_AMFNameiE_Extensions_Item {}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct Extended_AMFNameiE_Extensions(Vec<Extended_AMFNameiE_Extensions_Item>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct Extended_RANNodeNameiE_Extensions_Item {}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct Extended_RANNodeNameiE_Extensions(Vec<Extended_RANNodeNameiE_Extensions_Item>);

#[derive(Debug, AperCodec)]
#[asn(type = "BITSTRING", sz_extensible = true, sz_lb = "8", sz_ub = "8")]
pub struct BIT_STRING_24(BitVec<Msb0, u8>);

#[derive(Debug, AperCodec)]
#[asn(type = "BITSTRING", sz_extensible = true, sz_lb = "8", sz_ub = "8")]
pub struct BIT_STRING_25(BitVec<Msb0, u8>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct ExtendedRATRestrictionInformationiE_Extensions_Item {}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct ExtendedRATRestrictionInformationiE_Extensions(
    Vec<ExtendedRATRestrictionInformationiE_Extensions_Item>,
);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct FailureIndicationiE_Extensions_Item {}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct FailureIndicationiE_Extensions(Vec<FailureIndicationiE_Extensions_Item>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct FirstDLCountiE_Extension_Item {}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct FirstDLCountiE_Extension(Vec<FirstDLCountiE_Extension_Item>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct FiveG_S_TMSIiE_Extensions_Item {}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct FiveG_S_TMSIiE_Extensions(Vec<FiveG_S_TMSIiE_Extensions_Item>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct ForbiddenAreaInformation_ItemiE_Extensions_Item {}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct ForbiddenAreaInformation_ItemiE_Extensions(
    Vec<ForbiddenAreaInformation_ItemiE_Extensions_Item>,
);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct FromEUTRANtoNGRANiE_Extensions_Item {}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct FromEUTRANtoNGRANiE_Extensions(Vec<FromEUTRANtoNGRANiE_Extensions_Item>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct FromNGRANtoEUTRANiE_Extensions_Item {}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct FromNGRANtoEUTRANiE_Extensions(Vec<FromNGRANtoEUTRANiE_Extensions_Item>);

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum GBR_QosInformationiE_Extensions_ItemextensionValue {
    #[asn(key = 220)]
    AlternativeQoSParaSetList(AlternativeQoSParaSetList),
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
#[asn(type = "BITSTRING", sz_extensible = false, sz_lb = "22", sz_ub = "32")]
pub struct BIT_STRING_26(BitVec<Msb0, u8>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct GNB_IDchoice_Extensions {}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct GTPTunneliE_Extensions_Item {}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct GTPTunneliE_Extensions(Vec<GTPTunneliE_Extensions_Item>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct GUAMIiE_Extensions_Item {}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct GUAMIiE_Extensions(Vec<GUAMIiE_Extensions_Item>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct GlobalENB_IDiE_Extensions_Item {}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct GlobalENB_IDiE_Extensions(Vec<GlobalENB_IDiE_Extensions_Item>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct GlobalGNB_IDiE_Extensions_Item {}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct GlobalGNB_IDiE_Extensions(Vec<GlobalGNB_IDiE_Extensions_Item>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct GlobalLine_IDiE_Extensions_Item {}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct GlobalLine_IDiE_Extensions(Vec<GlobalLine_IDiE_Extensions_Item>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct GlobalN3IWF_IDiE_Extensions_Item {}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct GlobalN3IWF_IDiE_Extensions(Vec<GlobalN3IWF_IDiE_Extensions_Item>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct GlobalNgENB_IDiE_Extensions_Item {}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct GlobalNgENB_IDiE_Extensions(Vec<GlobalNgENB_IDiE_Extensions_Item>);

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum GlobalRANNodeIDchoice_Extensionsvalue {
    #[asn(key = 240)]
    GlobalTNGF_ID(GlobalTNGF_ID),
    #[asn(key = 241)]
    GlobalTWIF_ID(GlobalTWIF_ID),
    #[asn(key = 242)]
    GlobalW_AGF_ID(GlobalW_AGF_ID),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct GlobalRANNodeIDchoice_Extensions {
    #[asn(key_field = true)]
    pub id: ProtocolIE_ID,
    pub criticality: Criticality,
    pub value: GlobalRANNodeIDchoice_Extensionsvalue,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct GlobalTNGF_IDiE_Extensions_Item {}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct GlobalTNGF_IDiE_Extensions(Vec<GlobalTNGF_IDiE_Extensions_Item>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct GlobalTWIF_IDiE_Extensions_Item {}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct GlobalTWIF_IDiE_Extensions(Vec<GlobalTWIF_IDiE_Extensions_Item>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct GlobalW_AGF_IDiE_Extensions_Item {}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct GlobalW_AGF_IDiE_Extensions(Vec<GlobalW_AGF_IDiE_Extensions_Item>);

#[derive(Debug, AperCodec)]
#[asn(type = "ENUMERATED", extensible = true, lb = "0", ub = "2")]
pub struct ENUMERATED_27(u8);
impl ENUMERATED_27 {
    const HO_TOO_EARLY: u8 = 0u8;
    const HO_TO_WRONG_CELL: u8 = 1u8;
    const INTERSYSTEM_PING_PONG: u8 = 2u8;
}

#[derive(Debug, AperCodec)]
#[asn(type = "BITSTRING", sz_extensible = false, sz_lb = "16", sz_ub = "16")]
pub struct BIT_STRING_28(BitVec<Msb0, u8>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct HOReportiE_Extensions_Item {}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct HOReportiE_Extensions(Vec<HOReportiE_Extensions_Item>);

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum HandoverCancelprotocolIEs_Itemvalue {
    #[asn(key = 10)]
    AMF_UE_NGAP_ID(AMF_UE_NGAP_ID),
    #[asn(key = 15)]
    Cause(Cause),
    #[asn(key = 85)]
    RAN_UE_NGAP_ID(RAN_UE_NGAP_ID),
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
    #[asn(key = 10)]
    AMF_UE_NGAP_ID(AMF_UE_NGAP_ID),
    #[asn(key = 19)]
    CriticalityDiagnostics(CriticalityDiagnostics),
    #[asn(key = 85)]
    RAN_UE_NGAP_ID(RAN_UE_NGAP_ID),
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
    #[asn(key = 10)]
    AMF_UE_NGAP_ID(AMF_UE_NGAP_ID),
    #[asn(key = 19)]
    CriticalityDiagnostics(CriticalityDiagnostics),
    #[asn(key = 29)]
    HandoverType(HandoverType),
    #[asn(key = 39)]
    NASSecurityParametersFromNGRAN(NASSecurityParametersFromNGRAN),
    #[asn(key = 59)]
    PDUSessionResourceHandoverList(PDUSessionResourceHandoverList),
    #[asn(key = 78)]
    PDUSessionResourceToReleaseListHOCmd(PDUSessionResourceToReleaseListHOCmd),
    #[asn(key = 85)]
    RAN_UE_NGAP_ID(RAN_UE_NGAP_ID),
    #[asn(key = 106)]
    TargetToSource_TransparentContainer(TargetToSource_TransparentContainer),
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
pub enum HandoverCommandTransferiE_Extensions_ItemextensionValue {
    #[asn(key = 249)]
    DataForwardingResponseERABList(DataForwardingResponseERABList),
    #[asn(key = 152)]
    QosFlowPerTNLInformationList(QosFlowPerTNLInformationList),
    #[asn(key = 164)]
    UPTransportLayerInformation(UPTransportLayerInformation),
    #[asn(key = 172)]
    UPTransportLayerInformationList(UPTransportLayerInformationList),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct HandoverCommandTransferiE_Extensions_Item {
    #[asn(key_field = true)]
    pub id: ProtocolExtensionID,
    pub criticality: Criticality,
    pub extension_value: HandoverCommandTransferiE_Extensions_ItemextensionValue,
}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct HandoverCommandTransferiE_Extensions(Vec<HandoverCommandTransferiE_Extensions_Item>);

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum HandoverFailureprotocolIEs_Itemvalue {
    #[asn(key = 10)]
    AMF_UE_NGAP_ID(AMF_UE_NGAP_ID),
    #[asn(key = 15)]
    Cause(Cause),
    #[asn(key = 19)]
    CriticalityDiagnostics(CriticalityDiagnostics),
    #[asn(key = 262)]
    TargettoSource_Failure_TransparentContainer(TargettoSource_Failure_TransparentContainer),
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
    #[asn(key = 10)]
    AMF_UE_NGAP_ID(AMF_UE_NGAP_ID),
    #[asn(key = 269)]
    NotifySourceNGRANNode(NotifySourceNGRANNode),
    #[asn(key = 85)]
    RAN_UE_NGAP_ID(RAN_UE_NGAP_ID),
    #[asn(key = 121)]
    UserLocationInformation(UserLocationInformation),
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
    #[asn(key = 10)]
    AMF_UE_NGAP_ID(AMF_UE_NGAP_ID),
    #[asn(key = 15)]
    Cause(Cause),
    #[asn(key = 19)]
    CriticalityDiagnostics(CriticalityDiagnostics),
    #[asn(key = 85)]
    RAN_UE_NGAP_ID(RAN_UE_NGAP_ID),
    #[asn(key = 262)]
    TargettoSource_Failure_TransparentContainer(TargettoSource_Failure_TransparentContainer),
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
#[asn(type = "SEQUENCE", extensible = false)]
pub struct HandoverPreparationUnsuccessfulTransferiE_Extensions_Item {}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct HandoverPreparationUnsuccessfulTransferiE_Extensions(
    Vec<HandoverPreparationUnsuccessfulTransferiE_Extensions_Item>,
);

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum HandoverRequestprotocolIEs_Itemvalue {
    #[asn(key = 10)]
    AMF_UE_NGAP_ID(AMF_UE_NGAP_ID),
    #[asn(key = 0)]
    AllowedNSSAI(AllowedNSSAI),
    #[asn(key = 222)]
    CEmodeBrestricted(CEmodeBrestricted),
    #[asn(key = 165)]
    CNAssistedRANTuning(CNAssistedRANTuning),
    #[asn(key = 15)]
    Cause(Cause),
    #[asn(key = 18)]
    CoreNetworkAssistanceInformationForInactive(CoreNetworkAssistanceInformationForInactive),
    #[asn(key = 205)]
    Enhanced_CoverageRestriction(Enhanced_CoverageRestriction),
    #[asn(key = 206)]
    Extended_ConnectedTime(Extended_ConnectedTime),
    #[asn(key = 28)]
    GUAMI(GUAMI),
    #[asn(key = 29)]
    HandoverType(HandoverType),
    #[asn(key = 199)]
    IAB_Authorized(IAB_Authorized),
    #[asn(key = 217)]
    LTEUESidelinkAggregateMaximumBitrate(LTEUESidelinkAggregateMaximumBitrate),
    #[asn(key = 215)]
    LTEV2XServicesAuthorized(LTEV2XServicesAuthorized),
    #[asn(key = 33)]
    LocationReportingRequestType(LocationReportingRequestType),
    #[asn(key = 254)]
    MDTPLMNList(MDTPLMNList),
    #[asn(key = 34)]
    MaskedIMEISV(MaskedIMEISV),
    #[asn(key = 36)]
    MobilityRestrictionList(MobilityRestrictionList),
    #[asn(key = 37)]
    NAS_PDU(NAS_PDU),
    #[asn(key = 218)]
    NRUESidelinkAggregateMaximumBitrate(NRUESidelinkAggregateMaximumBitrate),
    #[asn(key = 216)]
    NRV2XServicesAuthorized(NRV2XServicesAuthorized),
    #[asn(key = 41)]
    NewSecurityContextInd(NewSecurityContextInd),
    #[asn(key = 219)]
    PC5QoSParameters(PC5QoSParameters),
    #[asn(key = 73)]
    PDUSessionResourceSetupListHOReq(PDUSessionResourceSetupListHOReq),
    #[asn(key = 91)]
    RRCInactiveTransitionReportRequest(RRCInactiveTransitionReportRequest),
    #[asn(key = 146)]
    RedirectionVoiceFallback(RedirectionVoiceFallback),
    #[asn(key = 177)]
    SRVCCOperationPossible(SRVCCOperationPossible),
    #[asn(key = 93)]
    SecurityContext(SecurityContext),
    #[asn(key = 101)]
    SourceToTarget_TransparentContainer(SourceToTarget_TransparentContainer),
    #[asn(key = 108)]
    TraceActivation(TraceActivation),
    #[asn(key = 209)]
    UE_DifferentiationInfo(UE_DifferentiationInfo),
    #[asn(key = 234)]
    UE_UP_CIoT_Support(UE_UP_CIoT_Support),
    #[asn(key = 110)]
    UEAggregateMaximumBitRate(UEAggregateMaximumBitRate),
    #[asn(key = 264)]
    UERadioCapabilityID(UERadioCapabilityID),
    #[asn(key = 119)]
    UESecurityCapabilities(UESecurityCapabilities),
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
    #[asn(key = 10)]
    AMF_UE_NGAP_ID(AMF_UE_NGAP_ID),
    #[asn(key = 19)]
    CriticalityDiagnostics(CriticalityDiagnostics),
    #[asn(key = 53)]
    PDUSessionResourceAdmittedList(PDUSessionResourceAdmittedList),
    #[asn(key = 56)]
    PDUSessionResourceFailedToSetupListHOAck(PDUSessionResourceFailedToSetupListHOAck),
    #[asn(key = 85)]
    RAN_UE_NGAP_ID(RAN_UE_NGAP_ID),
    #[asn(key = 106)]
    TargetToSource_TransparentContainer(TargetToSource_TransparentContainer),
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
pub enum HandoverRequestAcknowledgeTransferiE_Extensions_ItemextensionValue {
    #[asn(key = 153)]
    AdditionalDLUPTNLInformationForHOList(AdditionalDLUPTNLInformationForHOList),
    #[asn(key = 249)]
    DataForwardingResponseERABList(DataForwardingResponseERABList),
    #[asn(key = 27)]
    GlobalRANNodeID(GlobalRANNodeID),
    #[asn(key = 198)]
    RedundantPDUSessionInformation(RedundantPDUSessionInformation),
    #[asn(key = 164)]
    UPTransportLayerInformation(UPTransportLayerInformation),
    #[asn(key = 172)]
    UPTransportLayerInformationList(UPTransportLayerInformationList),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct HandoverRequestAcknowledgeTransferiE_Extensions_Item {
    #[asn(key_field = true)]
    pub id: ProtocolExtensionID,
    pub criticality: Criticality,
    pub extension_value: HandoverRequestAcknowledgeTransferiE_Extensions_ItemextensionValue,
}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct HandoverRequestAcknowledgeTransferiE_Extensions(
    Vec<HandoverRequestAcknowledgeTransferiE_Extensions_Item>,
);

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum HandoverRequiredprotocolIEs_Itemvalue {
    #[asn(key = 10)]
    AMF_UE_NGAP_ID(AMF_UE_NGAP_ID),
    #[asn(key = 15)]
    Cause(Cause),
    #[asn(key = 22)]
    DirectForwardingPathAvailability(DirectForwardingPathAvailability),
    #[asn(key = 29)]
    HandoverType(HandoverType),
    #[asn(key = 61)]
    PDUSessionResourceListHORqd(PDUSessionResourceListHORqd),
    #[asn(key = 85)]
    RAN_UE_NGAP_ID(RAN_UE_NGAP_ID),
    #[asn(key = 101)]
    SourceToTarget_TransparentContainer(SourceToTarget_TransparentContainer),
    #[asn(key = 105)]
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
#[asn(type = "SEQUENCE", extensible = false)]
pub struct HandoverRequiredTransferiE_Extensions_Item {}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct HandoverRequiredTransferiE_Extensions(Vec<HandoverRequiredTransferiE_Extensions_Item>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct HandoverResourceAllocationUnsuccessfulTransferiE_Extensions_Item {}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct HandoverResourceAllocationUnsuccessfulTransferiE_Extensions(
    Vec<HandoverResourceAllocationUnsuccessfulTransferiE_Extensions_Item>,
);

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum HandoverSuccessprotocolIEs_Itemvalue {
    #[asn(key = 10)]
    AMF_UE_NGAP_ID(AMF_UE_NGAP_ID),
    #[asn(key = 85)]
    RAN_UE_NGAP_ID(RAN_UE_NGAP_ID),
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
#[asn(type = "SEQUENCE", extensible = false)]
pub struct ImmediateMDTNriE_Extensions_Item {}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct ImmediateMDTNriE_Extensions(Vec<ImmediateMDTNriE_Extensions_Item>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct InfoOnRecommendedCellsAndRANNodesForPagingiE_Extensions_Item {}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct InfoOnRecommendedCellsAndRANNodesForPagingiE_Extensions(
    Vec<InfoOnRecommendedCellsAndRANNodesForPagingiE_Extensions_Item>,
);

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum InitialContextSetupFailureprotocolIEs_Itemvalue {
    #[asn(key = 10)]
    AMF_UE_NGAP_ID(AMF_UE_NGAP_ID),
    #[asn(key = 15)]
    Cause(Cause),
    #[asn(key = 19)]
    CriticalityDiagnostics(CriticalityDiagnostics),
    #[asn(key = 132)]
    PDUSessionResourceFailedToSetupListCxtFail(PDUSessionResourceFailedToSetupListCxtFail),
    #[asn(key = 85)]
    RAN_UE_NGAP_ID(RAN_UE_NGAP_ID),
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
    #[asn(key = 10)]
    AMF_UE_NGAP_ID(AMF_UE_NGAP_ID),
    #[asn(key = 48)]
    AMFName(AMFName),
    #[asn(key = 0)]
    AllowedNSSAI(AllowedNSSAI),
    #[asn(key = 222)]
    CEmodeBrestricted(CEmodeBrestricted),
    #[asn(key = 165)]
    CNAssistedRANTuning(CNAssistedRANTuning),
    #[asn(key = 18)]
    CoreNetworkAssistanceInformationForInactive(CoreNetworkAssistanceInformationForInactive),
    #[asn(key = 24)]
    EmergencyFallbackIndicator(EmergencyFallbackIndicator),
    #[asn(key = 205)]
    Enhanced_CoverageRestriction(Enhanced_CoverageRestriction),
    #[asn(key = 206)]
    Extended_ConnectedTime(Extended_ConnectedTime),
    #[asn(key = 28)]
    GUAMI(GUAMI),
    #[asn(key = 199)]
    IAB_Authorized(IAB_Authorized),
    #[asn(key = 31)]
    IndexToRFSP(IndexToRFSP),
    #[asn(key = 217)]
    LTEUESidelinkAggregateMaximumBitrate(LTEUESidelinkAggregateMaximumBitrate),
    #[asn(key = 215)]
    LTEV2XServicesAuthorized(LTEV2XServicesAuthorized),
    #[asn(key = 33)]
    LocationReportingRequestType(LocationReportingRequestType),
    #[asn(key = 254)]
    MDTPLMNList(MDTPLMNList),
    #[asn(key = 34)]
    MaskedIMEISV(MaskedIMEISV),
    #[asn(key = 36)]
    MobilityRestrictionList(MobilityRestrictionList),
    #[asn(key = 38)]
    NAS_PDU(NAS_PDU),
    #[asn(key = 218)]
    NRUESidelinkAggregateMaximumBitrate(NRUESidelinkAggregateMaximumBitrate),
    #[asn(key = 216)]
    NRV2XServicesAuthorized(NRV2XServicesAuthorized),
    #[asn(key = 219)]
    PC5QoSParameters(PC5QoSParameters),
    #[asn(key = 71)]
    PDUSessionResourceSetupListCxtReq(PDUSessionResourceSetupListCxtReq),
    #[asn(key = 85)]
    RAN_UE_NGAP_ID(RAN_UE_NGAP_ID),
    #[asn(key = 238)]
    RGLevelWirelineAccessCharacteristics(RGLevelWirelineAccessCharacteristics),
    #[asn(key = 91)]
    RRCInactiveTransitionReportRequest(RRCInactiveTransitionReportRequest),
    #[asn(key = 146)]
    RedirectionVoiceFallback(RedirectionVoiceFallback),
    #[asn(key = 177)]
    SRVCCOperationPossible(SRVCCOperationPossible),
    #[asn(key = 94)]
    SecurityKey(SecurityKey),
    #[asn(key = 108)]
    TraceActivation(TraceActivation),
    #[asn(key = 209)]
    UE_DifferentiationInfo(UE_DifferentiationInfo),
    #[asn(key = 234)]
    UE_UP_CIoT_Support(UE_UP_CIoT_Support),
    #[asn(key = 110)]
    UEAggregateMaximumBitRate(UEAggregateMaximumBitRate),
    #[asn(key = 117)]
    UERadioCapability(UERadioCapability),
    #[asn(key = 118)]
    UERadioCapabilityForPaging(UERadioCapabilityForPaging),
    #[asn(key = 264)]
    UERadioCapabilityID(UERadioCapabilityID),
    #[asn(key = 119)]
    UESecurityCapabilities(UESecurityCapabilities),
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
    #[asn(key = 10)]
    AMF_UE_NGAP_ID(AMF_UE_NGAP_ID),
    #[asn(key = 19)]
    CriticalityDiagnostics(CriticalityDiagnostics),
    #[asn(key = 55)]
    PDUSessionResourceFailedToSetupListCxtRes(PDUSessionResourceFailedToSetupListCxtRes),
    #[asn(key = 72)]
    PDUSessionResourceSetupListCxtRes(PDUSessionResourceSetupListCxtRes),
    #[asn(key = 85)]
    RAN_UE_NGAP_ID(RAN_UE_NGAP_ID),
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
    #[asn(key = 3)]
    AMFSetID(AMFSetID),
    #[asn(key = 0)]
    AllowedNSSAI(AllowedNSSAI),
    #[asn(key = 245)]
    AuthenticatedIndication(AuthenticatedIndication),
    #[asn(key = 224)]
    CEmodeBSupport_Indicator(CEmodeBSupport_Indicator),
    #[asn(key = 227)]
    EDT_Session(EDT_Session),
    #[asn(key = 26)]
    FiveG_S_TMSI(FiveG_S_TMSI),
    #[asn(key = 201)]
    IABNodeIndication(IABNodeIndication),
    #[asn(key = 225)]
    LTEM_Indication(LTEM_Indication),
    #[asn(key = 38)]
    NAS_PDU(NAS_PDU),
    #[asn(key = 259)]
    NPN_AccessInformation(NPN_AccessInformation),
    #[asn(key = 174)]
    PLMNIdentity(PLMNIdentity),
    #[asn(key = 85)]
    RAN_UE_NGAP_ID(RAN_UE_NGAP_ID),
    #[asn(key = 90)]
    RRCEstablishmentCause(RRCEstablishmentCause),
    #[asn(key = 171)]
    SourceToTarget_AMFInformationReroute(SourceToTarget_AMFInformationReroute),
    #[asn(key = 112)]
    UEContextRequest(UEContextRequest),
    #[asn(key = 121)]
    UserLocationInformation(UserLocationInformation),
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
    #[asn(key = 64)]
    AMFCPRelocationIndication(AMFCPRelocationIndication),
    #[asn(key = 0)]
    AMFConfigurationUpdate(AMFConfigurationUpdate),
    #[asn(key = 1)]
    AMFStatusIndication(AMFStatusIndication),
    #[asn(key = 2)]
    CellTrafficTrace(CellTrafficTrace),
    #[asn(key = 65)]
    ConnectionEstablishmentIndication(ConnectionEstablishmentIndication),
    #[asn(key = 3)]
    DeactivateTrace(DeactivateTrace),
    #[asn(key = 4)]
    DownlinkNASTransport(DownlinkNASTransport),
    #[asn(key = 5)]
    DownlinkNonUEAssociatedNRPPaTransport(DownlinkNonUEAssociatedNRPPaTransport),
    #[asn(key = 6)]
    DownlinkRANConfigurationTransfer(DownlinkRANConfigurationTransfer),
    #[asn(key = 63)]
    DownlinkRANEarlyStatusTransfer(DownlinkRANEarlyStatusTransfer),
    #[asn(key = 7)]
    DownlinkRANStatusTransfer(DownlinkRANStatusTransfer),
    #[asn(key = 54)]
    DownlinkRIMInformationTransfer(DownlinkRIMInformationTransfer),
    #[asn(key = 8)]
    DownlinkUEAssociatedNRPPaTransport(DownlinkUEAssociatedNRPPaTransport),
    #[asn(key = 9)]
    ErrorIndication(ErrorIndication),
    #[asn(key = 10)]
    HandoverCancel(HandoverCancel),
    #[asn(key = 11)]
    HandoverNotify(HandoverNotify),
    #[asn(key = 13)]
    HandoverRequest(HandoverRequest),
    #[asn(key = 12)]
    HandoverRequired(HandoverRequired),
    #[asn(key = 61)]
    HandoverSuccess(HandoverSuccess),
    #[asn(key = 14)]
    InitialContextSetupRequest(InitialContextSetupRequest),
    #[asn(key = 15)]
    InitialUEMessage(InitialUEMessage),
    #[asn(key = 18)]
    LocationReport(LocationReport),
    #[asn(key = 16)]
    LocationReportingControl(LocationReportingControl),
    #[asn(key = 17)]
    LocationReportingFailureIndication(LocationReportingFailureIndication),
    #[asn(key = 19)]
    NASNonDeliveryIndication(NASNonDeliveryIndication),
    #[asn(key = 20)]
    NGReset(NGReset),
    #[asn(key = 21)]
    NGSetupRequest(NGSetupRequest),
    #[asn(key = 22)]
    OverloadStart(OverloadStart),
    #[asn(key = 23)]
    OverloadStop(OverloadStop),
    #[asn(key = 27)]
    PDUSessionResourceModifyIndication(PDUSessionResourceModifyIndication),
    #[asn(key = 26)]
    PDUSessionResourceModifyRequest(PDUSessionResourceModifyRequest),
    #[asn(key = 30)]
    PDUSessionResourceNotify(PDUSessionResourceNotify),
    #[asn(key = 28)]
    PDUSessionResourceReleaseCommand(PDUSessionResourceReleaseCommand),
    #[asn(key = 29)]
    PDUSessionResourceSetupRequest(PDUSessionResourceSetupRequest),
    #[asn(key = 32)]
    PWSCancelRequest(PWSCancelRequest),
    #[asn(key = 33)]
    PWSFailureIndication(PWSFailureIndication),
    #[asn(key = 34)]
    PWSRestartIndication(PWSRestartIndication),
    #[asn(key = 24)]
    Paging(Paging),
    #[asn(key = 25)]
    PathSwitchRequest(PathSwitchRequest),
    #[asn(key = 31)]
    PrivateMessage(PrivateMessage),
    #[asn(key = 57)]
    RANCPRelocationIndication(RANCPRelocationIndication),
    #[asn(key = 35)]
    RANConfigurationUpdate(RANConfigurationUpdate),
    #[asn(key = 37)]
    RRCInactiveTransitionReport(RRCInactiveTransitionReport),
    #[asn(key = 36)]
    RerouteNASRequest(RerouteNASRequest),
    #[asn(key = 55)]
    RetrieveUEInformation(RetrieveUEInformation),
    #[asn(key = 52)]
    SecondaryRATDataUsageReport(SecondaryRATDataUsageReport),
    #[asn(key = 38)]
    TraceFailureIndication(TraceFailureIndication),
    #[asn(key = 39)]
    TraceStart(TraceStart),
    #[asn(key = 40)]
    UEContextModificationRequest(UEContextModificationRequest),
    #[asn(key = 41)]
    UEContextReleaseCommand(UEContextReleaseCommand),
    #[asn(key = 42)]
    UEContextReleaseRequest(UEContextReleaseRequest),
    #[asn(key = 58)]
    UEContextResumeRequest(UEContextResumeRequest),
    #[asn(key = 59)]
    UEContextSuspendRequest(UEContextSuspendRequest),
    #[asn(key = 56)]
    UEInformationTransfer(UEInformationTransfer),
    #[asn(key = 43)]
    UERadioCapabilityCheckRequest(UERadioCapabilityCheckRequest),
    #[asn(key = 60)]
    UERadioCapabilityIDMappingRequest(UERadioCapabilityIDMappingRequest),
    #[asn(key = 44)]
    UERadioCapabilityInfoIndication(UERadioCapabilityInfoIndication),
    #[asn(key = 45)]
    UETNLABindingReleaseRequest(UETNLABindingReleaseRequest),
    #[asn(key = 46)]
    UplinkNASTransport(UplinkNASTransport),
    #[asn(key = 47)]
    UplinkNonUEAssociatedNRPPaTransport(UplinkNonUEAssociatedNRPPaTransport),
    #[asn(key = 48)]
    UplinkRANConfigurationTransfer(UplinkRANConfigurationTransfer),
    #[asn(key = 62)]
    UplinkRANEarlyStatusTransfer(UplinkRANEarlyStatusTransfer),
    #[asn(key = 49)]
    UplinkRANStatusTransfer(UplinkRANStatusTransfer),
    #[asn(key = 53)]
    UplinkRIMInformationTransfer(UplinkRIMInformationTransfer),
    #[asn(key = 50)]
    UplinkUEAssociatedNRPPaTransport(UplinkUEAssociatedNRPPaTransport),
    #[asn(key = 51)]
    WriteReplaceWarningRequest(WriteReplaceWarningRequest),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct InterSystemFailureIndicationiE_Extensions_Item {}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct InterSystemFailureIndicationiE_Extensions(
    Vec<InterSystemFailureIndicationiE_Extensions_Item>,
);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct InterSystemHOReportiE_Extensions_Item {}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct InterSystemHOReportiE_Extensions(Vec<InterSystemHOReportiE_Extensions_Item>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct InterSystemHandoverReportTypechoice_Extensions {}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct IntersystemSONConfigurationTransferiE_Extensions_Item {}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct IntersystemSONConfigurationTransferiE_Extensions(
    Vec<IntersystemSONConfigurationTransferiE_Extensions_Item>,
);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct IntersystemSONInformationchoice_Extensions {}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct IntersystemSONInformationReportchoice_Extensions {}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct IntersystemSONNGRANnodeIDiE_Extensions_Item {}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct IntersystemSONNGRANnodeIDiE_Extensions(Vec<IntersystemSONNGRANnodeIDiE_Extensions_Item>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct IntersystemSONTransferTypechoice_Extensions {}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct IntersystemSONeNBIDiE_Extensions_Item {}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct IntersystemSONeNBIDiE_Extensions(Vec<IntersystemSONeNBIDiE_Extensions_Item>);

#[derive(Debug, AperCodec)]
#[asn(type = "ENUMERATED", extensible = true, lb = "0", ub = "1")]
pub struct ENUMERATED_29(u8);
impl ENUMERATED_29 {
    const TRUE: u8 = 0u8;
    const FALSE: u8 = 1u8;
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct IntersystemUnnecessaryHOiE_Extensions_Item {}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct IntersystemUnnecessaryHOiE_Extensions(Vec<IntersystemUnnecessaryHOiE_Extensions_Item>);

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
#[asn(type = "SEQUENCE", extensible = false)]
pub struct LTEUESidelinkAggregateMaximumBitrateiE_Extensions_Item {}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct LTEUESidelinkAggregateMaximumBitrateiE_Extensions(
    Vec<LTEUESidelinkAggregateMaximumBitrateiE_Extensions_Item>,
);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct LTEV2XServicesAuthorizediE_Extensions_Item {}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct LTEV2XServicesAuthorizediE_Extensions(Vec<LTEV2XServicesAuthorizediE_Extensions_Item>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct LastVisitedCellInformationchoice_Extensions {}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct LastVisitedCellItemiE_Extensions_Item {}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct LastVisitedCellItemiE_Extensions(Vec<LastVisitedCellItemiE_Extensions_Item>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct LastVisitedNGRANCellInformationiE_Extensions_Item {}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct LastVisitedNGRANCellInformationiE_Extensions(
    Vec<LastVisitedNGRANCellInformationiE_Extensions_Item>,
);

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum LocationReportprotocolIEs_Itemvalue {
    #[asn(key = 10)]
    AMF_UE_NGAP_ID(AMF_UE_NGAP_ID),
    #[asn(key = 33)]
    LocationReportingRequestType(LocationReportingRequestType),
    #[asn(key = 85)]
    RAN_UE_NGAP_ID(RAN_UE_NGAP_ID),
    #[asn(key = 116)]
    UEPresenceInAreaOfInterestList(UEPresenceInAreaOfInterestList),
    #[asn(key = 121)]
    UserLocationInformation(UserLocationInformation),
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
    #[asn(key = 10)]
    AMF_UE_NGAP_ID(AMF_UE_NGAP_ID),
    #[asn(key = 33)]
    LocationReportingRequestType(LocationReportingRequestType),
    #[asn(key = 85)]
    RAN_UE_NGAP_ID(RAN_UE_NGAP_ID),
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
    #[asn(key = 10)]
    AMF_UE_NGAP_ID(AMF_UE_NGAP_ID),
    #[asn(key = 15)]
    Cause(Cause),
    #[asn(key = 85)]
    RAN_UE_NGAP_ID(RAN_UE_NGAP_ID),
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
#[asn(type = "OPEN")]
pub enum LocationReportingRequestTypeiE_Extensions_ItemextensionValue {
    #[asn(key = 170)]
    LocationReportingAdditionalInfo(LocationReportingAdditionalInfo),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct LocationReportingRequestTypeiE_Extensions_Item {
    #[asn(key_field = true)]
    pub id: ProtocolExtensionID,
    pub criticality: Criticality,
    pub extension_value: LocationReportingRequestTypeiE_Extensions_ItemextensionValue,
}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct LocationReportingRequestTypeiE_Extensions(
    Vec<LocationReportingRequestTypeiE_Extensions_Item>,
);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct LoggedMDTNriE_Extensions_Item {}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct LoggedMDTNriE_Extensions(Vec<LoggedMDTNriE_Extensions_Item>);

#[derive(Debug, AperCodec)]
#[asn(type = "NULL")]
pub struct NULL_30;

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct LoggedMDTTriggerchoice_Extensions {}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct M1ConfigurationiE_Extensions_Item {}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct M1ConfigurationiE_Extensions(Vec<M1ConfigurationiE_Extensions_Item>);

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
pub struct M1ThresholdTypechoice_Extensions {}

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
#[asn(type = "SEQUENCE", extensible = false)]
pub struct MDT_ConfigurationiE_Extensions_Item {}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct MDT_ConfigurationiE_Extensions(Vec<MDT_ConfigurationiE_Extensions_Item>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct MDT_Configuration_EUTRAiE_Extensions_Item {}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct MDT_Configuration_EUTRAiE_Extensions(Vec<MDT_Configuration_EUTRAiE_Extensions_Item>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct MDT_Configuration_NRiE_Extensions_Item {}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct MDT_Configuration_NRiE_Extensions(Vec<MDT_Configuration_NRiE_Extensions_Item>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct MDT_Location_InfoiE_Extensions_Item {}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct MDT_Location_InfoiE_Extensions(Vec<MDT_Location_InfoiE_Extensions_Item>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct MDTModeNrchoice_Extensions {}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct MeasurementThresholdL1LoggedMDTchoice_Extensions {}

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum MobilityRestrictionListiE_Extensions_ItemextensionValue {
    #[asn(key = 160)]
    CNTypeRestrictionsForEquivalent(CNTypeRestrictionsForEquivalent),
    #[asn(key = 161)]
    CNTypeRestrictionsForServing(CNTypeRestrictionsForServing),
    #[asn(key = 261)]
    NPN_MobilityInformation(NPN_MobilityInformation),
    #[asn(key = 150)]
    PLMNIdentity(PLMNIdentity),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct MobilityRestrictionListiE_Extensions_Item {
    #[asn(key_field = true)]
    pub id: ProtocolExtensionID,
    pub criticality: Criticality,
    pub extension_value: MobilityRestrictionListiE_Extensions_ItemextensionValue,
}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct MobilityRestrictionListiE_Extensions(Vec<MobilityRestrictionListiE_Extensions_Item>);

#[derive(Debug, AperCodec)]
#[asn(type = "BITSTRING", sz_extensible = false, sz_lb = "16", sz_ub = "16")]
pub struct BIT_STRING_31(BitVec<Msb0, u8>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct N3IWF_IDchoice_Extensions {}

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum NASNonDeliveryIndicationprotocolIEs_Itemvalue {
    #[asn(key = 10)]
    AMF_UE_NGAP_ID(AMF_UE_NGAP_ID),
    #[asn(key = 15)]
    Cause(Cause),
    #[asn(key = 38)]
    NAS_PDU(NAS_PDU),
    #[asn(key = 85)]
    RAN_UE_NGAP_ID(RAN_UE_NGAP_ID),
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
pub struct NB_IoT_Paging_eDRXInfoiE_Extensions_Item {}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct NB_IoT_Paging_eDRXInfoiE_Extensions(Vec<NB_IoT_Paging_eDRXInfoiE_Extensions_Item>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct NGRAN_CGIchoice_Extensions {}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct NGRAN_TNLAssociationToRemoveItemiE_Extensions_Item {}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct NGRAN_TNLAssociationToRemoveItemiE_Extensions(
    Vec<NGRAN_TNLAssociationToRemoveItemiE_Extensions_Item>,
);

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum NGResetprotocolIEs_Itemvalue {
    #[asn(key = 15)]
    Cause(Cause),
    #[asn(key = 88)]
    ResetType(ResetType),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct NGResetprotocolIEs_Item {
    #[asn(key_field = true)]
    pub id: ProtocolIE_ID,
    pub criticality: Criticality,
    pub value: NGResetprotocolIEs_Itemvalue,
}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "0",
    sz_ub = "65535"
)]
pub struct NGResetprotocolIEs(Vec<NGResetprotocolIEs_Item>);

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum NGResetAcknowledgeprotocolIEs_Itemvalue {
    #[asn(key = 19)]
    CriticalityDiagnostics(CriticalityDiagnostics),
    #[asn(key = 111)]
    UE_associatedLogicalNG_connectionList(UE_associatedLogicalNG_connectionList),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct NGResetAcknowledgeprotocolIEs_Item {
    #[asn(key_field = true)]
    pub id: ProtocolIE_ID,
    pub criticality: Criticality,
    pub value: NGResetAcknowledgeprotocolIEs_Itemvalue,
}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "0",
    sz_ub = "65535"
)]
pub struct NGResetAcknowledgeprotocolIEs(Vec<NGResetAcknowledgeprotocolIEs_Item>);

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum NGSetupFailureprotocolIEs_Itemvalue {
    #[asn(key = 15)]
    Cause(Cause),
    #[asn(key = 19)]
    CriticalityDiagnostics(CriticalityDiagnostics),
    #[asn(key = 107)]
    TimeToWait(TimeToWait),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct NGSetupFailureprotocolIEs_Item {
    #[asn(key_field = true)]
    pub id: ProtocolIE_ID,
    pub criticality: Criticality,
    pub value: NGSetupFailureprotocolIEs_Itemvalue,
}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "0",
    sz_ub = "65535"
)]
pub struct NGSetupFailureprotocolIEs(Vec<NGSetupFailureprotocolIEs_Item>);

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum NGSetupRequestprotocolIEs_Itemvalue {
    #[asn(key = 273)]
    Extended_RANNodeName(Extended_RANNodeName),
    #[asn(key = 27)]
    GlobalRANNodeID(GlobalRANNodeID),
    #[asn(key = 204)]
    NB_IoT_DefaultPagingDRX(NB_IoT_DefaultPagingDRX),
    #[asn(key = 21)]
    PagingDRX(PagingDRX),
    #[asn(key = 82)]
    RANNodeName(RANNodeName),
    #[asn(key = 102)]
    SupportedTAList(SupportedTAList),
    #[asn(key = 147)]
    UERetentionInformation(UERetentionInformation),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct NGSetupRequestprotocolIEs_Item {
    #[asn(key_field = true)]
    pub id: ProtocolIE_ID,
    pub criticality: Criticality,
    pub value: NGSetupRequestprotocolIEs_Itemvalue,
}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "0",
    sz_ub = "65535"
)]
pub struct NGSetupRequestprotocolIEs(Vec<NGSetupRequestprotocolIEs_Item>);

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum NGSetupResponseprotocolIEs_Itemvalue {
    #[asn(key = 1)]
    AMFName(AMFName),
    #[asn(key = 19)]
    CriticalityDiagnostics(CriticalityDiagnostics),
    #[asn(key = 274)]
    Extended_AMFName(Extended_AMFName),
    #[asn(key = 200)]
    IAB_Supported(IAB_Supported),
    #[asn(key = 80)]
    PLMNSupportList(PLMNSupportList),
    #[asn(key = 86)]
    RelativeAMFCapacity(RelativeAMFCapacity),
    #[asn(key = 96)]
    ServedGUAMIList(ServedGUAMIList),
    #[asn(key = 147)]
    UERetentionInformation(UERetentionInformation),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct NGSetupResponseprotocolIEs_Item {
    #[asn(key_field = true)]
    pub id: ProtocolIE_ID,
    pub criticality: Criticality,
    pub value: NGSetupResponseprotocolIEs_Itemvalue,
}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "0",
    sz_ub = "65535"
)]
pub struct NGSetupResponseprotocolIEs(Vec<NGSetupResponseprotocolIEs_Item>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct NPN_AccessInformationchoice_Extensions {}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct NPN_MobilityInformationchoice_Extensions {}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct NPN_PagingAssistanceInformationchoice_Extensions {}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct NPN_Supportchoice_Extensions {}

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
pub struct NRFrequencyBandItemiE_Extension_Item {}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct NRFrequencyBandItemiE_Extension(Vec<NRFrequencyBandItemiE_Extension_Item>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct NRFrequencyInfoiE_Extension_Item {}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct NRFrequencyInfoiE_Extension(Vec<NRFrequencyInfoiE_Extension_Item>);

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
#[asn(type = "BITSTRING", sz_extensible = false, sz_lb = "20", sz_ub = "20")]
pub struct BIT_STRING_32(BitVec<Msb0, u8>);

#[derive(Debug, AperCodec)]
#[asn(type = "BITSTRING", sz_extensible = false, sz_lb = "18", sz_ub = "18")]
pub struct BIT_STRING_33(BitVec<Msb0, u8>);

#[derive(Debug, AperCodec)]
#[asn(type = "BITSTRING", sz_extensible = false, sz_lb = "21", sz_ub = "21")]
pub struct BIT_STRING_34(BitVec<Msb0, u8>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct NgENB_IDchoice_Extensions {}

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum NonDynamic5QIDescriptoriE_Extensions_ItemextensionValue {
    #[asn(key = 187)]
    ExtendedPacketDelayBudget(ExtendedPacketDelayBudget),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct NonDynamic5QIDescriptoriE_Extensions_Item {
    #[asn(key_field = true)]
    pub id: ProtocolExtensionID,
    pub criticality: Criticality,
    pub extension_value: NonDynamic5QIDescriptoriE_Extensions_ItemextensionValue,
}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct NonDynamic5QIDescriptoriE_Extensions(Vec<NonDynamic5QIDescriptoriE_Extensions_Item>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct OverloadResponsechoice_Extensions {}

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum OverloadStartprotocolIEs_Itemvalue {
    #[asn(key = 2)]
    OverloadResponse(OverloadResponse),
    #[asn(key = 49)]
    OverloadStartNSSAIList(OverloadStartNSSAIList),
    #[asn(key = 9)]
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
#[asn(type = "SEQUENCE", extensible = false)]
pub struct OverloadStartNSSAIItemiE_Extensions_Item {}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct OverloadStartNSSAIItemiE_Extensions(Vec<OverloadStartNSSAIItemiE_Extensions_Item>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct OverloadStopprotocolIEs_Item {}

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
pub struct PDUSessionAggregateMaximumBitRateiE_Extensions_Item {}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct PDUSessionAggregateMaximumBitRateiE_Extensions(
    Vec<PDUSessionAggregateMaximumBitRateiE_Extensions_Item>,
);

#[derive(Debug, AperCodec)]
#[asn(type = "OCTET-STRING")]
pub struct OCTET_STRING_35(Vec<u8>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct PDUSessionResourceAdmittedItemiE_Extensions_Item {}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct PDUSessionResourceAdmittedItemiE_Extensions(
    Vec<PDUSessionResourceAdmittedItemiE_Extensions_Item>,
);

#[derive(Debug, AperCodec)]
#[asn(type = "OCTET-STRING")]
pub struct OCTET_STRING_36(Vec<u8>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct PDUSessionResourceFailedToModifyItemModCfmiE_Extensions_Item {}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct PDUSessionResourceFailedToModifyItemModCfmiE_Extensions(
    Vec<PDUSessionResourceFailedToModifyItemModCfmiE_Extensions_Item>,
);

#[derive(Debug, AperCodec)]
#[asn(type = "OCTET-STRING")]
pub struct OCTET_STRING_37(Vec<u8>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct PDUSessionResourceFailedToModifyItemModResiE_Extensions_Item {}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct PDUSessionResourceFailedToModifyItemModResiE_Extensions(
    Vec<PDUSessionResourceFailedToModifyItemModResiE_Extensions_Item>,
);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct PDUSessionResourceFailedToResumeItemRESReqiE_Extensions_Item {}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct PDUSessionResourceFailedToResumeItemRESReqiE_Extensions(
    Vec<PDUSessionResourceFailedToResumeItemRESReqiE_Extensions_Item>,
);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct PDUSessionResourceFailedToResumeItemRESResiE_Extensions_Item {}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct PDUSessionResourceFailedToResumeItemRESResiE_Extensions(
    Vec<PDUSessionResourceFailedToResumeItemRESResiE_Extensions_Item>,
);

#[derive(Debug, AperCodec)]
#[asn(type = "OCTET-STRING")]
pub struct OCTET_STRING_38(Vec<u8>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct PDUSessionResourceFailedToSetupItemCxtFailiE_Extensions_Item {}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct PDUSessionResourceFailedToSetupItemCxtFailiE_Extensions(
    Vec<PDUSessionResourceFailedToSetupItemCxtFailiE_Extensions_Item>,
);

#[derive(Debug, AperCodec)]
#[asn(type = "OCTET-STRING")]
pub struct OCTET_STRING_39(Vec<u8>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct PDUSessionResourceFailedToSetupItemCxtResiE_Extensions_Item {}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct PDUSessionResourceFailedToSetupItemCxtResiE_Extensions(
    Vec<PDUSessionResourceFailedToSetupItemCxtResiE_Extensions_Item>,
);

#[derive(Debug, AperCodec)]
#[asn(type = "OCTET-STRING")]
pub struct OCTET_STRING_40(Vec<u8>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct PDUSessionResourceFailedToSetupItemHOAckiE_Extensions_Item {}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct PDUSessionResourceFailedToSetupItemHOAckiE_Extensions(
    Vec<PDUSessionResourceFailedToSetupItemHOAckiE_Extensions_Item>,
);

#[derive(Debug, AperCodec)]
#[asn(type = "OCTET-STRING")]
pub struct OCTET_STRING_41(Vec<u8>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct PDUSessionResourceFailedToSetupItemPSReqiE_Extensions_Item {}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct PDUSessionResourceFailedToSetupItemPSReqiE_Extensions(
    Vec<PDUSessionResourceFailedToSetupItemPSReqiE_Extensions_Item>,
);

#[derive(Debug, AperCodec)]
#[asn(type = "OCTET-STRING")]
pub struct OCTET_STRING_42(Vec<u8>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct PDUSessionResourceFailedToSetupItemSUResiE_Extensions_Item {}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct PDUSessionResourceFailedToSetupItemSUResiE_Extensions(
    Vec<PDUSessionResourceFailedToSetupItemSUResiE_Extensions_Item>,
);

#[derive(Debug, AperCodec)]
#[asn(type = "OCTET-STRING")]
pub struct OCTET_STRING_43(Vec<u8>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct PDUSessionResourceHandoverItemiE_Extensions_Item {}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct PDUSessionResourceHandoverItemiE_Extensions(
    Vec<PDUSessionResourceHandoverItemiE_Extensions_Item>,
);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct PDUSessionResourceInformationItemiE_Extensions_Item {}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct PDUSessionResourceInformationItemiE_Extensions(
    Vec<PDUSessionResourceInformationItemiE_Extensions_Item>,
);

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum PDUSessionResourceItemCxtRelCpliE_Extensions_ItemextensionValue {}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct PDUSessionResourceItemCxtRelCpliE_Extensions_Item {
    #[asn(key_field = true)]
    pub id: ProtocolExtensionID,
    pub criticality: Criticality,
    pub extension_value: PDUSessionResourceItemCxtRelCpliE_Extensions_ItemextensionValue,
}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct PDUSessionResourceItemCxtRelCpliE_Extensions(
    Vec<PDUSessionResourceItemCxtRelCpliE_Extensions_Item>,
);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct PDUSessionResourceItemCxtRelReqiE_Extensions_Item {}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct PDUSessionResourceItemCxtRelReqiE_Extensions(
    Vec<PDUSessionResourceItemCxtRelReqiE_Extensions_Item>,
);

#[derive(Debug, AperCodec)]
#[asn(type = "OCTET-STRING")]
pub struct OCTET_STRING_44(Vec<u8>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct PDUSessionResourceItemHORqdiE_Extensions_Item {}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct PDUSessionResourceItemHORqdiE_Extensions(
    Vec<PDUSessionResourceItemHORqdiE_Extensions_Item>,
);

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum PDUSessionResourceModifyConfirmprotocolIEs_Itemvalue {
    #[asn(key = 10)]
    AMF_UE_NGAP_ID(AMF_UE_NGAP_ID),
    #[asn(key = 19)]
    CriticalityDiagnostics(CriticalityDiagnostics),
    #[asn(key = 131)]
    PDUSessionResourceFailedToModifyListModCfm(PDUSessionResourceFailedToModifyListModCfm),
    #[asn(key = 62)]
    PDUSessionResourceModifyListModCfm(PDUSessionResourceModifyListModCfm),
    #[asn(key = 85)]
    RAN_UE_NGAP_ID(RAN_UE_NGAP_ID),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct PDUSessionResourceModifyConfirmprotocolIEs_Item {
    #[asn(key_field = true)]
    pub id: ProtocolIE_ID,
    pub criticality: Criticality,
    pub value: PDUSessionResourceModifyConfirmprotocolIEs_Itemvalue,
}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "0",
    sz_ub = "65535"
)]
pub struct PDUSessionResourceModifyConfirmprotocolIEs(
    Vec<PDUSessionResourceModifyConfirmprotocolIEs_Item>,
);

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum PDUSessionResourceModifyConfirmTransferiE_Extensions_ItemextensionValue {
    #[asn(key = 195)]
    UPTransportLayerInformation(UPTransportLayerInformation),
    #[asn(key = 185)]
    UPTransportLayerInformationPairList(UPTransportLayerInformationPairList),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct PDUSessionResourceModifyConfirmTransferiE_Extensions_Item {
    #[asn(key_field = true)]
    pub id: ProtocolExtensionID,
    pub criticality: Criticality,
    pub extension_value: PDUSessionResourceModifyConfirmTransferiE_Extensions_ItemextensionValue,
}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct PDUSessionResourceModifyConfirmTransferiE_Extensions(
    Vec<PDUSessionResourceModifyConfirmTransferiE_Extensions_Item>,
);

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum PDUSessionResourceModifyIndicationprotocolIEs_Itemvalue {
    #[asn(key = 10)]
    AMF_UE_NGAP_ID(AMF_UE_NGAP_ID),
    #[asn(key = 63)]
    PDUSessionResourceModifyListModInd(PDUSessionResourceModifyListModInd),
    #[asn(key = 85)]
    RAN_UE_NGAP_ID(RAN_UE_NGAP_ID),
    #[asn(key = 121)]
    UserLocationInformation(UserLocationInformation),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct PDUSessionResourceModifyIndicationprotocolIEs_Item {
    #[asn(key_field = true)]
    pub id: ProtocolIE_ID,
    pub criticality: Criticality,
    pub value: PDUSessionResourceModifyIndicationprotocolIEs_Itemvalue,
}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "0",
    sz_ub = "65535"
)]
pub struct PDUSessionResourceModifyIndicationprotocolIEs(
    Vec<PDUSessionResourceModifyIndicationprotocolIEs_Item>,
);

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum PDUSessionResourceModifyIndicationTransferiE_Extensions_ItemextensionValue {
    #[asn(key = 27)]
    GlobalRANNodeID(GlobalRANNodeID),
    #[asn(key = 193)]
    QosFlowPerTNLInformation(QosFlowPerTNLInformation),
    #[asn(key = 184)]
    QosFlowPerTNLInformationList(QosFlowPerTNLInformationList),
    #[asn(key = 144)]
    SecondaryRATUsageInformation(SecondaryRATUsageInformation),
    #[asn(key = 156)]
    SecurityResult(SecurityResult),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct PDUSessionResourceModifyIndicationTransferiE_Extensions_Item {
    #[asn(key_field = true)]
    pub id: ProtocolExtensionID,
    pub criticality: Criticality,
    pub extension_value: PDUSessionResourceModifyIndicationTransferiE_Extensions_ItemextensionValue,
}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct PDUSessionResourceModifyIndicationTransferiE_Extensions(
    Vec<PDUSessionResourceModifyIndicationTransferiE_Extensions_Item>,
);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct PDUSessionResourceModifyIndicationUnsuccessfulTransferiE_Extensions_Item {}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct PDUSessionResourceModifyIndicationUnsuccessfulTransferiE_Extensions(
    Vec<PDUSessionResourceModifyIndicationUnsuccessfulTransferiE_Extensions_Item>,
);

#[derive(Debug, AperCodec)]
#[asn(type = "OCTET-STRING")]
pub struct OCTET_STRING_45(Vec<u8>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct PDUSessionResourceModifyItemModCfmiE_Extensions_Item {}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct PDUSessionResourceModifyItemModCfmiE_Extensions(
    Vec<PDUSessionResourceModifyItemModCfmiE_Extensions_Item>,
);

#[derive(Debug, AperCodec)]
#[asn(type = "OCTET-STRING")]
pub struct OCTET_STRING_46(Vec<u8>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct PDUSessionResourceModifyItemModIndiE_Extensions_Item {}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct PDUSessionResourceModifyItemModIndiE_Extensions(
    Vec<PDUSessionResourceModifyItemModIndiE_Extensions_Item>,
);

#[derive(Debug, AperCodec)]
#[asn(type = "OCTET-STRING")]
pub struct OCTET_STRING_47(Vec<u8>);

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum PDUSessionResourceModifyItemModReqiE_Extensions_ItemextensionValue {
    #[asn(key = 148)]
    S_NSSAI(S_NSSAI),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct PDUSessionResourceModifyItemModReqiE_Extensions_Item {
    #[asn(key_field = true)]
    pub id: ProtocolExtensionID,
    pub criticality: Criticality,
    pub extension_value: PDUSessionResourceModifyItemModReqiE_Extensions_ItemextensionValue,
}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct PDUSessionResourceModifyItemModReqiE_Extensions(
    Vec<PDUSessionResourceModifyItemModReqiE_Extensions_Item>,
);

#[derive(Debug, AperCodec)]
#[asn(type = "OCTET-STRING")]
pub struct OCTET_STRING_48(Vec<u8>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct PDUSessionResourceModifyItemModResiE_Extensions_Item {}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct PDUSessionResourceModifyItemModResiE_Extensions(
    Vec<PDUSessionResourceModifyItemModResiE_Extensions_Item>,
);

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum PDUSessionResourceModifyRequestprotocolIEs_Itemvalue {
    #[asn(key = 10)]
    AMF_UE_NGAP_ID(AMF_UE_NGAP_ID),
    #[asn(key = 64)]
    PDUSessionResourceModifyListModReq(PDUSessionResourceModifyListModReq),
    #[asn(key = 85)]
    RAN_UE_NGAP_ID(RAN_UE_NGAP_ID),
    #[asn(key = 83)]
    RANPagingPriority(RANPagingPriority),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct PDUSessionResourceModifyRequestprotocolIEs_Item {
    #[asn(key_field = true)]
    pub id: ProtocolIE_ID,
    pub criticality: Criticality,
    pub value: PDUSessionResourceModifyRequestprotocolIEs_Itemvalue,
}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "0",
    sz_ub = "65535"
)]
pub struct PDUSessionResourceModifyRequestprotocolIEs(
    Vec<PDUSessionResourceModifyRequestprotocolIEs_Item>,
);

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum PDUSessionResourceModifyRequestTransferprotocolIEs_Itemvalue {
    #[asn(key = 166)]
    CommonNetworkInstance(CommonNetworkInstance),
    #[asn(key = 129)]
    NetworkInstance(NetworkInstance),
    #[asn(key = 130)]
    PDUSessionAggregateMaximumBitRate(PDUSessionAggregateMaximumBitRate),
    #[asn(key = 135)]
    QosFlowAddOrModifyRequestList(QosFlowAddOrModifyRequestList),
    #[asn(key = 137)]
    QosFlowListWithCause(QosFlowListWithCause),
    #[asn(key = 138)]
    SecurityIndication(SecurityIndication),
    #[asn(key = 140)]
    UL_NGU_UP_TNLModifyList(UL_NGU_UP_TNLModifyList),
    #[asn(key = 195)]
    UPTransportLayerInformation(UPTransportLayerInformation),
    #[asn(key = 126)]
    UPTransportLayerInformationList(UPTransportLayerInformationList),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct PDUSessionResourceModifyRequestTransferprotocolIEs_Item {
    #[asn(key_field = true)]
    pub id: ProtocolIE_ID,
    pub criticality: Criticality,
    pub value: PDUSessionResourceModifyRequestTransferprotocolIEs_Itemvalue,
}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "0",
    sz_ub = "65535"
)]
pub struct PDUSessionResourceModifyRequestTransferprotocolIEs(
    Vec<PDUSessionResourceModifyRequestTransferprotocolIEs_Item>,
);

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum PDUSessionResourceModifyResponseprotocolIEs_Itemvalue {
    #[asn(key = 10)]
    AMF_UE_NGAP_ID(AMF_UE_NGAP_ID),
    #[asn(key = 19)]
    CriticalityDiagnostics(CriticalityDiagnostics),
    #[asn(key = 54)]
    PDUSessionResourceFailedToModifyListModRes(PDUSessionResourceFailedToModifyListModRes),
    #[asn(key = 65)]
    PDUSessionResourceModifyListModRes(PDUSessionResourceModifyListModRes),
    #[asn(key = 85)]
    RAN_UE_NGAP_ID(RAN_UE_NGAP_ID),
    #[asn(key = 121)]
    UserLocationInformation(UserLocationInformation),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct PDUSessionResourceModifyResponseprotocolIEs_Item {
    #[asn(key_field = true)]
    pub id: ProtocolIE_ID,
    pub criticality: Criticality,
    pub value: PDUSessionResourceModifyResponseprotocolIEs_Itemvalue,
}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "0",
    sz_ub = "65535"
)]
pub struct PDUSessionResourceModifyResponseprotocolIEs(
    Vec<PDUSessionResourceModifyResponseprotocolIEs_Item>,
);

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum PDUSessionResourceModifyResponseTransferiE_Extensions_ItemextensionValue {
    #[asn(key = 184)]
    QosFlowPerTNLInformationList(QosFlowPerTNLInformationList),
    #[asn(key = 195)]
    UPTransportLayerInformation(UPTransportLayerInformation),
    #[asn(key = 185)]
    UPTransportLayerInformationPairList(UPTransportLayerInformationPairList),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct PDUSessionResourceModifyResponseTransferiE_Extensions_Item {
    #[asn(key_field = true)]
    pub id: ProtocolExtensionID,
    pub criticality: Criticality,
    pub extension_value: PDUSessionResourceModifyResponseTransferiE_Extensions_ItemextensionValue,
}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct PDUSessionResourceModifyResponseTransferiE_Extensions(
    Vec<PDUSessionResourceModifyResponseTransferiE_Extensions_Item>,
);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct PDUSessionResourceModifyUnsuccessfulTransferiE_Extensions_Item {}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct PDUSessionResourceModifyUnsuccessfulTransferiE_Extensions(
    Vec<PDUSessionResourceModifyUnsuccessfulTransferiE_Extensions_Item>,
);

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum PDUSessionResourceNotifyprotocolIEs_Itemvalue {
    #[asn(key = 10)]
    AMF_UE_NGAP_ID(AMF_UE_NGAP_ID),
    #[asn(key = 66)]
    PDUSessionResourceNotifyList(PDUSessionResourceNotifyList),
    #[asn(key = 67)]
    PDUSessionResourceReleasedListNot(PDUSessionResourceReleasedListNot),
    #[asn(key = 85)]
    RAN_UE_NGAP_ID(RAN_UE_NGAP_ID),
    #[asn(key = 121)]
    UserLocationInformation(UserLocationInformation),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct PDUSessionResourceNotifyprotocolIEs_Item {
    #[asn(key_field = true)]
    pub id: ProtocolIE_ID,
    pub criticality: Criticality,
    pub value: PDUSessionResourceNotifyprotocolIEs_Itemvalue,
}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "0",
    sz_ub = "65535"
)]
pub struct PDUSessionResourceNotifyprotocolIEs(Vec<PDUSessionResourceNotifyprotocolIEs_Item>);

#[derive(Debug, AperCodec)]
#[asn(type = "OCTET-STRING")]
pub struct OCTET_STRING_49(Vec<u8>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct PDUSessionResourceNotifyItemiE_Extensions_Item {}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct PDUSessionResourceNotifyItemiE_Extensions(
    Vec<PDUSessionResourceNotifyItemiE_Extensions_Item>,
);

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum PDUSessionResourceNotifyReleasedTransferiE_Extensions_ItemextensionValue {
    #[asn(key = 144)]
    SecondaryRATUsageInformation(SecondaryRATUsageInformation),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct PDUSessionResourceNotifyReleasedTransferiE_Extensions_Item {
    #[asn(key_field = true)]
    pub id: ProtocolExtensionID,
    pub criticality: Criticality,
    pub extension_value: PDUSessionResourceNotifyReleasedTransferiE_Extensions_ItemextensionValue,
}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct PDUSessionResourceNotifyReleasedTransferiE_Extensions(
    Vec<PDUSessionResourceNotifyReleasedTransferiE_Extensions_Item>,
);

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum PDUSessionResourceNotifyTransferiE_Extensions_ItemextensionValue {
    #[asn(key = 278)]
    QosFlowFeedbackList(QosFlowFeedbackList),
    #[asn(key = 144)]
    SecondaryRATUsageInformation(SecondaryRATUsageInformation),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct PDUSessionResourceNotifyTransferiE_Extensions_Item {
    #[asn(key_field = true)]
    pub id: ProtocolExtensionID,
    pub criticality: Criticality,
    pub extension_value: PDUSessionResourceNotifyTransferiE_Extensions_ItemextensionValue,
}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct PDUSessionResourceNotifyTransferiE_Extensions(
    Vec<PDUSessionResourceNotifyTransferiE_Extensions_Item>,
);

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum PDUSessionResourceReleaseCommandprotocolIEs_Itemvalue {
    #[asn(key = 10)]
    AMF_UE_NGAP_ID(AMF_UE_NGAP_ID),
    #[asn(key = 38)]
    NAS_PDU(NAS_PDU),
    #[asn(key = 79)]
    PDUSessionResourceToReleaseListRelCmd(PDUSessionResourceToReleaseListRelCmd),
    #[asn(key = 85)]
    RAN_UE_NGAP_ID(RAN_UE_NGAP_ID),
    #[asn(key = 83)]
    RANPagingPriority(RANPagingPriority),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct PDUSessionResourceReleaseCommandprotocolIEs_Item {
    #[asn(key_field = true)]
    pub id: ProtocolIE_ID,
    pub criticality: Criticality,
    pub value: PDUSessionResourceReleaseCommandprotocolIEs_Itemvalue,
}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "0",
    sz_ub = "65535"
)]
pub struct PDUSessionResourceReleaseCommandprotocolIEs(
    Vec<PDUSessionResourceReleaseCommandprotocolIEs_Item>,
);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct PDUSessionResourceReleaseCommandTransferiE_Extensions_Item {}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct PDUSessionResourceReleaseCommandTransferiE_Extensions(
    Vec<PDUSessionResourceReleaseCommandTransferiE_Extensions_Item>,
);

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum PDUSessionResourceReleaseResponseprotocolIEs_Itemvalue {
    #[asn(key = 10)]
    AMF_UE_NGAP_ID(AMF_UE_NGAP_ID),
    #[asn(key = 19)]
    CriticalityDiagnostics(CriticalityDiagnostics),
    #[asn(key = 70)]
    PDUSessionResourceReleasedListRelRes(PDUSessionResourceReleasedListRelRes),
    #[asn(key = 85)]
    RAN_UE_NGAP_ID(RAN_UE_NGAP_ID),
    #[asn(key = 121)]
    UserLocationInformation(UserLocationInformation),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct PDUSessionResourceReleaseResponseprotocolIEs_Item {
    #[asn(key_field = true)]
    pub id: ProtocolIE_ID,
    pub criticality: Criticality,
    pub value: PDUSessionResourceReleaseResponseprotocolIEs_Itemvalue,
}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "0",
    sz_ub = "65535"
)]
pub struct PDUSessionResourceReleaseResponseprotocolIEs(
    Vec<PDUSessionResourceReleaseResponseprotocolIEs_Item>,
);

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum PDUSessionResourceReleaseResponseTransferiE_Extensions_ItemextensionValue {
    #[asn(key = 144)]
    SecondaryRATUsageInformation(SecondaryRATUsageInformation),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct PDUSessionResourceReleaseResponseTransferiE_Extensions_Item {
    #[asn(key_field = true)]
    pub id: ProtocolExtensionID,
    pub criticality: Criticality,
    pub extension_value: PDUSessionResourceReleaseResponseTransferiE_Extensions_ItemextensionValue,
}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct PDUSessionResourceReleaseResponseTransferiE_Extensions(
    Vec<PDUSessionResourceReleaseResponseTransferiE_Extensions_Item>,
);

#[derive(Debug, AperCodec)]
#[asn(type = "OCTET-STRING")]
pub struct OCTET_STRING_50(Vec<u8>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct PDUSessionResourceReleasedItemNotiE_Extensions_Item {}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct PDUSessionResourceReleasedItemNotiE_Extensions(
    Vec<PDUSessionResourceReleasedItemNotiE_Extensions_Item>,
);

#[derive(Debug, AperCodec)]
#[asn(type = "OCTET-STRING")]
pub struct OCTET_STRING_51(Vec<u8>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct PDUSessionResourceReleasedItemPSAckiE_Extensions_Item {}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct PDUSessionResourceReleasedItemPSAckiE_Extensions(
    Vec<PDUSessionResourceReleasedItemPSAckiE_Extensions_Item>,
);

#[derive(Debug, AperCodec)]
#[asn(type = "OCTET-STRING")]
pub struct OCTET_STRING_52(Vec<u8>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct PDUSessionResourceReleasedItemPSFailiE_Extensions_Item {}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct PDUSessionResourceReleasedItemPSFailiE_Extensions(
    Vec<PDUSessionResourceReleasedItemPSFailiE_Extensions_Item>,
);

#[derive(Debug, AperCodec)]
#[asn(type = "OCTET-STRING")]
pub struct OCTET_STRING_53(Vec<u8>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct PDUSessionResourceReleasedItemRelResiE_Extensions_Item {}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct PDUSessionResourceReleasedItemRelResiE_Extensions(
    Vec<PDUSessionResourceReleasedItemRelResiE_Extensions_Item>,
);

#[derive(Debug, AperCodec)]
#[asn(type = "OCTET-STRING")]
pub struct OCTET_STRING_54(Vec<u8>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct PDUSessionResourceResumeItemRESReqiE_Extensions_Item {}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct PDUSessionResourceResumeItemRESReqiE_Extensions(
    Vec<PDUSessionResourceResumeItemRESReqiE_Extensions_Item>,
);

#[derive(Debug, AperCodec)]
#[asn(type = "OCTET-STRING")]
pub struct OCTET_STRING_55(Vec<u8>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct PDUSessionResourceResumeItemRESResiE_Extensions_Item {}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct PDUSessionResourceResumeItemRESResiE_Extensions(
    Vec<PDUSessionResourceResumeItemRESResiE_Extensions_Item>,
);

#[derive(Debug, AperCodec)]
#[asn(type = "OCTET-STRING")]
pub struct OCTET_STRING_56(Vec<u8>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct PDUSessionResourceSecondaryRATUsageItemiE_Extensions_Item {}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct PDUSessionResourceSecondaryRATUsageItemiE_Extensions(
    Vec<PDUSessionResourceSecondaryRATUsageItemiE_Extensions_Item>,
);

#[derive(Debug, AperCodec)]
#[asn(type = "OCTET-STRING")]
pub struct OCTET_STRING_57(Vec<u8>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct PDUSessionResourceSetupItemCxtReqiE_Extensions_Item {}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct PDUSessionResourceSetupItemCxtReqiE_Extensions(
    Vec<PDUSessionResourceSetupItemCxtReqiE_Extensions_Item>,
);

#[derive(Debug, AperCodec)]
#[asn(type = "OCTET-STRING")]
pub struct OCTET_STRING_58(Vec<u8>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct PDUSessionResourceSetupItemCxtResiE_Extensions_Item {}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct PDUSessionResourceSetupItemCxtResiE_Extensions(
    Vec<PDUSessionResourceSetupItemCxtResiE_Extensions_Item>,
);

#[derive(Debug, AperCodec)]
#[asn(type = "OCTET-STRING")]
pub struct OCTET_STRING_59(Vec<u8>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct PDUSessionResourceSetupItemHOReqiE_Extensions_Item {}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct PDUSessionResourceSetupItemHOReqiE_Extensions(
    Vec<PDUSessionResourceSetupItemHOReqiE_Extensions_Item>,
);

#[derive(Debug, AperCodec)]
#[asn(type = "OCTET-STRING")]
pub struct OCTET_STRING_60(Vec<u8>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct PDUSessionResourceSetupItemSUReqiE_Extensions_Item {}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct PDUSessionResourceSetupItemSUReqiE_Extensions(
    Vec<PDUSessionResourceSetupItemSUReqiE_Extensions_Item>,
);

#[derive(Debug, AperCodec)]
#[asn(type = "OCTET-STRING")]
pub struct OCTET_STRING_61(Vec<u8>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct PDUSessionResourceSetupItemSUResiE_Extensions_Item {}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct PDUSessionResourceSetupItemSUResiE_Extensions(
    Vec<PDUSessionResourceSetupItemSUResiE_Extensions_Item>,
);

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum PDUSessionResourceSetupRequestprotocolIEs_Itemvalue {
    #[asn(key = 10)]
    AMF_UE_NGAP_ID(AMF_UE_NGAP_ID),
    #[asn(key = 38)]
    NAS_PDU(NAS_PDU),
    #[asn(key = 74)]
    PDUSessionResourceSetupListSUReq(PDUSessionResourceSetupListSUReq),
    #[asn(key = 85)]
    RAN_UE_NGAP_ID(RAN_UE_NGAP_ID),
    #[asn(key = 83)]
    RANPagingPriority(RANPagingPriority),
    #[asn(key = 110)]
    UEAggregateMaximumBitRate(UEAggregateMaximumBitRate),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct PDUSessionResourceSetupRequestprotocolIEs_Item {
    #[asn(key_field = true)]
    pub id: ProtocolIE_ID,
    pub criticality: Criticality,
    pub value: PDUSessionResourceSetupRequestprotocolIEs_Itemvalue,
}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "0",
    sz_ub = "65535"
)]
pub struct PDUSessionResourceSetupRequestprotocolIEs(
    Vec<PDUSessionResourceSetupRequestprotocolIEs_Item>,
);

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum PDUSessionResourceSetupRequestTransferprotocolIEs_Itemvalue {
    #[asn(key = 166)]
    CommonNetworkInstance(CommonNetworkInstance),
    #[asn(key = 127)]
    DataForwardingNotPossible(DataForwardingNotPossible),
    #[asn(key = 22)]
    DirectForwardingPathAvailability(DirectForwardingPathAvailability),
    #[asn(key = 129)]
    NetworkInstance(NetworkInstance),
    #[asn(key = 130)]
    PDUSessionAggregateMaximumBitRate(PDUSessionAggregateMaximumBitRate),
    #[asn(key = 134)]
    PDUSessionType(PDUSessionType),
    #[asn(key = 136)]
    QosFlowSetupRequestList(QosFlowSetupRequestList),
    #[asn(key = 197)]
    RedundantPDUSessionInformation(RedundantPDUSessionInformation),
    #[asn(key = 138)]
    SecurityIndication(SecurityIndication),
    #[asn(key = 139)]
    UPTransportLayerInformation(UPTransportLayerInformation),
    #[asn(key = 186)]
    UPTransportLayerInformationList(UPTransportLayerInformationList),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct PDUSessionResourceSetupRequestTransferprotocolIEs_Item {
    #[asn(key_field = true)]
    pub id: ProtocolIE_ID,
    pub criticality: Criticality,
    pub value: PDUSessionResourceSetupRequestTransferprotocolIEs_Itemvalue,
}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "0",
    sz_ub = "65535"
)]
pub struct PDUSessionResourceSetupRequestTransferprotocolIEs(
    Vec<PDUSessionResourceSetupRequestTransferprotocolIEs_Item>,
);

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum PDUSessionResourceSetupResponseprotocolIEs_Itemvalue {
    #[asn(key = 10)]
    AMF_UE_NGAP_ID(AMF_UE_NGAP_ID),
    #[asn(key = 19)]
    CriticalityDiagnostics(CriticalityDiagnostics),
    #[asn(key = 58)]
    PDUSessionResourceFailedToSetupListSURes(PDUSessionResourceFailedToSetupListSURes),
    #[asn(key = 75)]
    PDUSessionResourceSetupListSURes(PDUSessionResourceSetupListSURes),
    #[asn(key = 85)]
    RAN_UE_NGAP_ID(RAN_UE_NGAP_ID),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct PDUSessionResourceSetupResponseprotocolIEs_Item {
    #[asn(key_field = true)]
    pub id: ProtocolIE_ID,
    pub criticality: Criticality,
    pub value: PDUSessionResourceSetupResponseprotocolIEs_Itemvalue,
}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "0",
    sz_ub = "65535"
)]
pub struct PDUSessionResourceSetupResponseprotocolIEs(
    Vec<PDUSessionResourceSetupResponseprotocolIEs_Item>,
);

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum PDUSessionResourceSetupResponseTransferiE_Extensions_ItemextensionValue {
    #[asn(key = 27)]
    GlobalRANNodeID(GlobalRANNodeID),
    #[asn(key = 193)]
    QosFlowPerTNLInformation(QosFlowPerTNLInformation),
    #[asn(key = 184)]
    QosFlowPerTNLInformationList(QosFlowPerTNLInformationList),
    #[asn(key = 198)]
    RedundantPDUSessionInformation(RedundantPDUSessionInformation),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct PDUSessionResourceSetupResponseTransferiE_Extensions_Item {
    #[asn(key_field = true)]
    pub id: ProtocolExtensionID,
    pub criticality: Criticality,
    pub extension_value: PDUSessionResourceSetupResponseTransferiE_Extensions_ItemextensionValue,
}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct PDUSessionResourceSetupResponseTransferiE_Extensions(
    Vec<PDUSessionResourceSetupResponseTransferiE_Extensions_Item>,
);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct PDUSessionResourceSetupUnsuccessfulTransferiE_Extensions_Item {}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct PDUSessionResourceSetupUnsuccessfulTransferiE_Extensions(
    Vec<PDUSessionResourceSetupUnsuccessfulTransferiE_Extensions_Item>,
);

#[derive(Debug, AperCodec)]
#[asn(type = "OCTET-STRING")]
pub struct OCTET_STRING_62(Vec<u8>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct PDUSessionResourceSuspendItemSUSReqiE_Extensions_Item {}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct PDUSessionResourceSuspendItemSUSReqiE_Extensions(
    Vec<PDUSessionResourceSuspendItemSUSReqiE_Extensions_Item>,
);

#[derive(Debug, AperCodec)]
#[asn(type = "OCTET-STRING")]
pub struct OCTET_STRING_63(Vec<u8>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct PDUSessionResourceSwitchedItemiE_Extensions_Item {}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct PDUSessionResourceSwitchedItemiE_Extensions(
    Vec<PDUSessionResourceSwitchedItemiE_Extensions_Item>,
);

#[derive(Debug, AperCodec)]
#[asn(type = "OCTET-STRING")]
pub struct OCTET_STRING_64(Vec<u8>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct PDUSessionResourceToBeSwitchedDLItemiE_Extensions_Item {}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct PDUSessionResourceToBeSwitchedDLItemiE_Extensions(
    Vec<PDUSessionResourceToBeSwitchedDLItemiE_Extensions_Item>,
);

#[derive(Debug, AperCodec)]
#[asn(type = "OCTET-STRING")]
pub struct OCTET_STRING_65(Vec<u8>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct PDUSessionResourceToReleaseItemHOCmdiE_Extensions_Item {}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct PDUSessionResourceToReleaseItemHOCmdiE_Extensions(
    Vec<PDUSessionResourceToReleaseItemHOCmdiE_Extensions_Item>,
);

#[derive(Debug, AperCodec)]
#[asn(type = "OCTET-STRING")]
pub struct OCTET_STRING_66(Vec<u8>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct PDUSessionResourceToReleaseItemRelCmdiE_Extensions_Item {}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct PDUSessionResourceToReleaseItemRelCmdiE_Extensions(
    Vec<PDUSessionResourceToReleaseItemRelCmdiE_Extensions_Item>,
);

#[derive(Debug, AperCodec)]
#[asn(type = "ENUMERATED", extensible = true, lb = "0", ub = "1")]
pub struct ENUMERATED_67(u8);
impl ENUMERATED_67 {
    const NR: u8 = 0u8;
    const EUTRA: u8 = 1u8;
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct PDUSessionUsageReportiE_Extensions_Item {}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct PDUSessionUsageReportiE_Extensions(Vec<PDUSessionUsageReportiE_Extensions_Item>);

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum PLMNSupportItemiE_Extensions_ItemextensionValue {
    #[asn(key = 270)]
    ExtendedSliceSupportList(ExtendedSliceSupportList),
    #[asn(key = 258)]
    NPN_Support(NPN_Support),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct PLMNSupportItemiE_Extensions_Item {
    #[asn(key_field = true)]
    pub id: ProtocolExtensionID,
    pub criticality: Criticality,
    pub extension_value: PLMNSupportItemiE_Extensions_ItemextensionValue,
}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct PLMNSupportItemiE_Extensions(Vec<PLMNSupportItemiE_Extensions_Item>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct PNI_NPN_MobilityInformationiE_Extensions_Item {}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct PNI_NPN_MobilityInformationiE_Extensions(
    Vec<PNI_NPN_MobilityInformationiE_Extensions_Item>,
);

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum PWSCancelRequestprotocolIEs_Itemvalue {
    #[asn(key = 14)]
    CancelAllWarningMessages(CancelAllWarningMessages),
    #[asn(key = 35)]
    MessageIdentifier(MessageIdentifier),
    #[asn(key = 95)]
    SerialNumber(SerialNumber),
    #[asn(key = 122)]
    WarningAreaList(WarningAreaList),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct PWSCancelRequestprotocolIEs_Item {
    #[asn(key_field = true)]
    pub id: ProtocolIE_ID,
    pub criticality: Criticality,
    pub value: PWSCancelRequestprotocolIEs_Itemvalue,
}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "0",
    sz_ub = "65535"
)]
pub struct PWSCancelRequestprotocolIEs(Vec<PWSCancelRequestprotocolIEs_Item>);

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum PWSCancelResponseprotocolIEs_Itemvalue {
    #[asn(key = 12)]
    BroadcastCancelledAreaList(BroadcastCancelledAreaList),
    #[asn(key = 19)]
    CriticalityDiagnostics(CriticalityDiagnostics),
    #[asn(key = 35)]
    MessageIdentifier(MessageIdentifier),
    #[asn(key = 95)]
    SerialNumber(SerialNumber),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct PWSCancelResponseprotocolIEs_Item {
    #[asn(key_field = true)]
    pub id: ProtocolIE_ID,
    pub criticality: Criticality,
    pub value: PWSCancelResponseprotocolIEs_Itemvalue,
}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "0",
    sz_ub = "65535"
)]
pub struct PWSCancelResponseprotocolIEs(Vec<PWSCancelResponseprotocolIEs_Item>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct PWSFailedCellIDListchoice_Extensions {}

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum PWSFailureIndicationprotocolIEs_Itemvalue {
    #[asn(key = 27)]
    GlobalRANNodeID(GlobalRANNodeID),
    #[asn(key = 81)]
    PWSFailedCellIDList(PWSFailedCellIDList),
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
    #[asn(key = 16)]
    CellIDListForRestart(CellIDListForRestart),
    #[asn(key = 23)]
    EmergencyAreaIDListForRestart(EmergencyAreaIDListForRestart),
    #[asn(key = 27)]
    GlobalRANNodeID(GlobalRANNodeID),
    #[asn(key = 104)]
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
#[asn(type = "INTEGER", lb = "0", ub = "9", extensible = true)]
pub struct INTEGER_68(u8);

#[derive(Debug, AperCodec)]
#[asn(type = "INTEGER", lb = "0", ub = "9", extensible = true)]
pub struct INTEGER_69(u8);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct PacketErrorRateiE_Extensions_Item {}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct PacketErrorRateiE_Extensions(Vec<PacketErrorRateiE_Extensions_Item>);

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum PagingprotocolIEs_Itemvalue {
    #[asn(key = 11)]
    AssistanceDataForPaging(AssistanceDataForPaging),
    #[asn(key = 222)]
    CEmodeBrestricted(CEmodeBrestricted),
    #[asn(key = 205)]
    Enhanced_CoverageRestriction(Enhanced_CoverageRestriction),
    #[asn(key = 203)]
    NB_IoT_Paging_eDRXInfo(NB_IoT_Paging_eDRXInfo),
    #[asn(key = 202)]
    NB_IoT_PagingDRX(NB_IoT_PagingDRX),
    #[asn(key = 50)]
    PagingDRX(PagingDRX),
    #[asn(key = 51)]
    PagingOrigin(PagingOrigin),
    #[asn(key = 52)]
    PagingPriority(PagingPriority),
    #[asn(key = 223)]
    PagingeDRXInformation(PagingeDRXInformation),
    #[asn(key = 103)]
    TAIListForPaging(TAIListForPaging),
    #[asn(key = 115)]
    UEPagingIdentity(UEPagingIdentity),
    #[asn(key = 118)]
    UERadioCapabilityForPaging(UERadioCapabilityForPaging),
    #[asn(key = 208)]
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
pub struct PagingAssisDataforCEcapabUEiE_Extensions_Item {}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct PagingAssisDataforCEcapabUEiE_Extensions(
    Vec<PagingAssisDataforCEcapabUEiE_Extensions_Item>,
);

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
#[asn(type = "SEQUENCE", extensible = false)]
pub struct PagingeDRXInformationiE_Extensions_Item {}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct PagingeDRXInformationiE_Extensions(Vec<PagingeDRXInformationiE_Extensions_Item>);

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum PathSwitchRequestprotocolIEs_Itemvalue {
    #[asn(key = 100)]
    AMF_UE_NGAP_ID(AMF_UE_NGAP_ID),
    #[asn(key = 57)]
    PDUSessionResourceFailedToSetupListPSReq(PDUSessionResourceFailedToSetupListPSReq),
    #[asn(key = 76)]
    PDUSessionResourceToBeSwitchedDLList(PDUSessionResourceToBeSwitchedDLList),
    #[asn(key = 85)]
    RAN_UE_NGAP_ID(RAN_UE_NGAP_ID),
    #[asn(key = 237)]
    RRCEstablishmentCause(RRCEstablishmentCause),
    #[asn(key = 119)]
    UESecurityCapabilities(UESecurityCapabilities),
    #[asn(key = 121)]
    UserLocationInformation(UserLocationInformation),
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
    #[asn(key = 10)]
    AMF_UE_NGAP_ID(AMF_UE_NGAP_ID),
    #[asn(key = 0)]
    AllowedNSSAI(AllowedNSSAI),
    #[asn(key = 222)]
    CEmodeBrestricted(CEmodeBrestricted),
    #[asn(key = 165)]
    CNAssistedRANTuning(CNAssistedRANTuning),
    #[asn(key = 18)]
    CoreNetworkAssistanceInformationForInactive(CoreNetworkAssistanceInformationForInactive),
    #[asn(key = 19)]
    CriticalityDiagnostics(CriticalityDiagnostics),
    #[asn(key = 205)]
    Enhanced_CoverageRestriction(Enhanced_CoverageRestriction),
    #[asn(key = 206)]
    Extended_ConnectedTime(Extended_ConnectedTime),
    #[asn(key = 217)]
    LTEUESidelinkAggregateMaximumBitrate(LTEUESidelinkAggregateMaximumBitrate),
    #[asn(key = 215)]
    LTEV2XServicesAuthorized(LTEV2XServicesAuthorized),
    #[asn(key = 218)]
    NRUESidelinkAggregateMaximumBitrate(NRUESidelinkAggregateMaximumBitrate),
    #[asn(key = 216)]
    NRV2XServicesAuthorized(NRV2XServicesAuthorized),
    #[asn(key = 41)]
    NewSecurityContextInd(NewSecurityContextInd),
    #[asn(key = 219)]
    PC5QoSParameters(PC5QoSParameters),
    #[asn(key = 68)]
    PDUSessionResourceReleasedListPSAck(PDUSessionResourceReleasedListPSAck),
    #[asn(key = 77)]
    PDUSessionResourceSwitchedList(PDUSessionResourceSwitchedList),
    #[asn(key = 85)]
    RAN_UE_NGAP_ID(RAN_UE_NGAP_ID),
    #[asn(key = 91)]
    RRCInactiveTransitionReportRequest(RRCInactiveTransitionReportRequest),
    #[asn(key = 146)]
    RedirectionVoiceFallback(RedirectionVoiceFallback),
    #[asn(key = 177)]
    SRVCCOperationPossible(SRVCCOperationPossible),
    #[asn(key = 93)]
    SecurityContext(SecurityContext),
    #[asn(key = 209)]
    UE_DifferentiationInfo(UE_DifferentiationInfo),
    #[asn(key = 234)]
    UE_UP_CIoT_Support(UE_UP_CIoT_Support),
    #[asn(key = 264)]
    UERadioCapabilityID(UERadioCapabilityID),
    #[asn(key = 119)]
    UESecurityCapabilities(UESecurityCapabilities),
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
pub enum PathSwitchRequestAcknowledgeTransferiE_Extensions_ItemextensionValue {
    #[asn(key = 277)]
    QosFlowParametersList(QosFlowParametersList),
    #[asn(key = 195)]
    UPTransportLayerInformation(UPTransportLayerInformation),
    #[asn(key = 185)]
    UPTransportLayerInformationPairList(UPTransportLayerInformationPairList),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct PathSwitchRequestAcknowledgeTransferiE_Extensions_Item {
    #[asn(key_field = true)]
    pub id: ProtocolExtensionID,
    pub criticality: Criticality,
    pub extension_value: PathSwitchRequestAcknowledgeTransferiE_Extensions_ItemextensionValue,
}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct PathSwitchRequestAcknowledgeTransferiE_Extensions(
    Vec<PathSwitchRequestAcknowledgeTransferiE_Extensions_Item>,
);

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum PathSwitchRequestFailureprotocolIEs_Itemvalue {
    #[asn(key = 10)]
    AMF_UE_NGAP_ID(AMF_UE_NGAP_ID),
    #[asn(key = 19)]
    CriticalityDiagnostics(CriticalityDiagnostics),
    #[asn(key = 69)]
    PDUSessionResourceReleasedListPSFail(PDUSessionResourceReleasedListPSFail),
    #[asn(key = 85)]
    RAN_UE_NGAP_ID(RAN_UE_NGAP_ID),
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
#[asn(type = "SEQUENCE", extensible = false)]
pub struct PathSwitchRequestSetupFailedTransferiE_Extensions_Item {}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct PathSwitchRequestSetupFailedTransferiE_Extensions(
    Vec<PathSwitchRequestSetupFailedTransferiE_Extensions_Item>,
);

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum PathSwitchRequestTransferiE_Extensions_ItemextensionValue {
    #[asn(key = 191)]
    DL_NGU_TNLInformationReused(DL_NGU_TNLInformationReused),
    #[asn(key = 27)]
    GlobalRANNodeID(GlobalRANNodeID),
    #[asn(key = 155)]
    QosFlowPerTNLInformationList(QosFlowPerTNLInformationList),
    #[asn(key = 198)]
    RedundantPDUSessionInformation(RedundantPDUSessionInformation),
    #[asn(key = 192)]
    UPTransportLayerInformation(UPTransportLayerInformation),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct PathSwitchRequestTransferiE_Extensions_Item {
    #[asn(key_field = true)]
    pub id: ProtocolExtensionID,
    pub criticality: Criticality,
    pub extension_value: PathSwitchRequestTransferiE_Extensions_ItemextensionValue,
}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct PathSwitchRequestTransferiE_Extensions(Vec<PathSwitchRequestTransferiE_Extensions_Item>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct PathSwitchRequestUnsuccessfulTransferiE_Extensions_Item {}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct PathSwitchRequestUnsuccessfulTransferiE_Extensions(
    Vec<PathSwitchRequestUnsuccessfulTransferiE_Extensions_Item>,
);

#[derive(Debug, AperCodec)]
#[asn(type = "INTEGER", lb = "0", ub = "65535")]
pub struct INTEGER_70(u16);

#[derive(Debug, AperCodec)]
#[asn(type = "OBJECT-IDENTIFIER")]
pub struct OBJECT_IDENTIFIER_71;

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
#[asn(type = "SEQUENCE", extensible = false)]
pub struct ProcedureStageChoicechoice_Extensions {}

#[derive(Debug, AperCodec)]
#[asn(type = "ENUMERATED", extensible = true, lb = "0", ub = "1")]
pub struct ENUMERATED_72(u8);
impl ENUMERATED_72 {
    const NR: u8 = 0u8;
    const EUTRA: u8 = 1u8;
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct QoSFlowsUsageReport_ItemiE_Extensions_Item {}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct QoSFlowsUsageReport_ItemiE_Extensions(Vec<QoSFlowsUsageReport_ItemiE_Extensions_Item>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct QosCharacteristicschoice_Extensions {}

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum QosFlowAcceptedItemiE_Extensions_ItemextensionValue {
    #[asn(key = 221)]
    AlternativeQoSParaSetIndex(AlternativeQoSParaSetIndex),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct QosFlowAcceptedItemiE_Extensions_Item {
    #[asn(key_field = true)]
    pub id: ProtocolExtensionID,
    pub criticality: Criticality,
    pub extension_value: QosFlowAcceptedItemiE_Extensions_ItemextensionValue,
}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct QosFlowAcceptedItemiE_Extensions(Vec<QosFlowAcceptedItemiE_Extensions_Item>);

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum QosFlowAddOrModifyRequestItemiE_Extensions_ItemextensionValue {
    #[asn(key = 194)]
    RedundantQosFlowIndicator(RedundantQosFlowIndicator),
    #[asn(key = 196)]
    TSCTrafficCharacteristics(TSCTrafficCharacteristics),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct QosFlowAddOrModifyRequestItemiE_Extensions_Item {
    #[asn(key_field = true)]
    pub id: ProtocolExtensionID,
    pub criticality: Criticality,
    pub extension_value: QosFlowAddOrModifyRequestItemiE_Extensions_ItemextensionValue,
}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct QosFlowAddOrModifyRequestItemiE_Extensions(
    Vec<QosFlowAddOrModifyRequestItemiE_Extensions_Item>,
);

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum QosFlowAddOrModifyResponseItemiE_Extensions_ItemextensionValue {
    #[asn(key = 221)]
    AlternativeQoSParaSetIndex(AlternativeQoSParaSetIndex),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct QosFlowAddOrModifyResponseItemiE_Extensions_Item {
    #[asn(key_field = true)]
    pub id: ProtocolExtensionID,
    pub criticality: Criticality,
    pub extension_value: QosFlowAddOrModifyResponseItemiE_Extensions_ItemextensionValue,
}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct QosFlowAddOrModifyResponseItemiE_Extensions(
    Vec<QosFlowAddOrModifyResponseItemiE_Extensions_Item>,
);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct QosFlowFeedbackItemiE_Extensions_Item {}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct QosFlowFeedbackItemiE_Extensions(Vec<QosFlowFeedbackItemiE_Extensions_Item>);

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum QosFlowInformationItemiE_Extensions_ItemextensionValue {
    #[asn(key = 163)]
    ULForwarding(ULForwarding),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct QosFlowInformationItemiE_Extensions_Item {
    #[asn(key_field = true)]
    pub id: ProtocolExtensionID,
    pub criticality: Criticality,
    pub extension_value: QosFlowInformationItemiE_Extensions_ItemextensionValue,
}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct QosFlowInformationItemiE_Extensions(Vec<QosFlowInformationItemiE_Extensions_Item>);

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum QosFlowItemWithDataForwardingiE_Extensions_ItemextensionValue {
    #[asn(key = 221)]
    AlternativeQoSParaSetIndex(AlternativeQoSParaSetIndex),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct QosFlowItemWithDataForwardingiE_Extensions_Item {
    #[asn(key_field = true)]
    pub id: ProtocolExtensionID,
    pub criticality: Criticality,
    pub extension_value: QosFlowItemWithDataForwardingiE_Extensions_ItemextensionValue,
}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct QosFlowItemWithDataForwardingiE_Extensions(
    Vec<QosFlowItemWithDataForwardingiE_Extensions_Item>,
);

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum QosFlowLevelQosParametersiE_Extensions_ItemextensionValue {
    #[asn(key = 276)]
    QosMonitoringReportingFrequency(QosMonitoringReportingFrequency),
    #[asn(key = 181)]
    QosMonitoringRequest(QosMonitoringRequest),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct QosFlowLevelQosParametersiE_Extensions_Item {
    #[asn(key_field = true)]
    pub id: ProtocolExtensionID,
    pub criticality: Criticality,
    pub extension_value: QosFlowLevelQosParametersiE_Extensions_ItemextensionValue,
}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct QosFlowLevelQosParametersiE_Extensions(Vec<QosFlowLevelQosParametersiE_Extensions_Item>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct QosFlowModifyConfirmItemiE_Extensions_Item {}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct QosFlowModifyConfirmItemiE_Extensions(Vec<QosFlowModifyConfirmItemiE_Extensions_Item>);

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum QosFlowNotifyItemiE_Extensions_ItemextensionValue {
    #[asn(key = 221)]
    AlternativeQoSParaSetNotifyIndex(AlternativeQoSParaSetNotifyIndex),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct QosFlowNotifyItemiE_Extensions_Item {
    #[asn(key_field = true)]
    pub id: ProtocolExtensionID,
    pub criticality: Criticality,
    pub extension_value: QosFlowNotifyItemiE_Extensions_ItemextensionValue,
}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct QosFlowNotifyItemiE_Extensions(Vec<QosFlowNotifyItemiE_Extensions_Item>);

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum QosFlowParametersItemiE_Extensions_ItemextensionValue {
    #[asn(key = 279)]
    BurstArrivalTime(BurstArrivalTime),
    #[asn(key = 188)]
    ExtendedPacketDelayBudget(ExtendedPacketDelayBudget),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct QosFlowParametersItemiE_Extensions_Item {
    #[asn(key_field = true)]
    pub id: ProtocolExtensionID,
    pub criticality: Criticality,
    pub extension_value: QosFlowParametersItemiE_Extensions_ItemextensionValue,
}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct QosFlowParametersItemiE_Extensions(Vec<QosFlowParametersItemiE_Extensions_Item>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct QosFlowPerTNLInformationiE_Extensions_Item {}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct QosFlowPerTNLInformationiE_Extensions(Vec<QosFlowPerTNLInformationiE_Extensions_Item>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct QosFlowPerTNLInformationItemiE_Extensions_Item {}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct QosFlowPerTNLInformationItemiE_Extensions(
    Vec<QosFlowPerTNLInformationItemiE_Extensions_Item>,
);

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum QosFlowSetupRequestItemiE_Extensions_ItemextensionValue {
    #[asn(key = 194)]
    RedundantQosFlowIndicator(RedundantQosFlowIndicator),
    #[asn(key = 196)]
    TSCTrafficCharacteristics(TSCTrafficCharacteristics),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct QosFlowSetupRequestItemiE_Extensions_Item {
    #[asn(key_field = true)]
    pub id: ProtocolExtensionID,
    pub criticality: Criticality,
    pub extension_value: QosFlowSetupRequestItemiE_Extensions_ItemextensionValue,
}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct QosFlowSetupRequestItemiE_Extensions(Vec<QosFlowSetupRequestItemiE_Extensions_Item>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct QosFlowToBeForwardedItemiE_Extensions_Item {}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct QosFlowToBeForwardedItemiE_Extensions(Vec<QosFlowToBeForwardedItemiE_Extensions_Item>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct QosFlowWithCauseItemiE_Extensions_Item {}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct QosFlowWithCauseItemiE_Extensions(Vec<QosFlowWithCauseItemiE_Extensions_Item>);

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum RANCPRelocationIndicationprotocolIEs_Itemvalue {
    #[asn(key = 25)]
    EUTRA_CGI(EUTRA_CGI),
    #[asn(key = 26)]
    FiveG_S_TMSI(FiveG_S_TMSI),
    #[asn(key = 85)]
    RAN_UE_NGAP_ID(RAN_UE_NGAP_ID),
    #[asn(key = 213)]
    TAI(TAI),
    #[asn(key = 211)]
    UL_CP_SecurityInformation(UL_CP_SecurityInformation),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct RANCPRelocationIndicationprotocolIEs_Item {
    #[asn(key_field = true)]
    pub id: ProtocolIE_ID,
    pub criticality: Criticality,
    pub value: RANCPRelocationIndicationprotocolIEs_Itemvalue,
}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "0",
    sz_ub = "65535"
)]
pub struct RANCPRelocationIndicationprotocolIEs(Vec<RANCPRelocationIndicationprotocolIEs_Item>);

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum RANConfigurationUpdateprotocolIEs_Itemvalue {
    #[asn(key = 273)]
    Extended_RANNodeName(Extended_RANNodeName),
    #[asn(key = 27)]
    GlobalRANNodeID(GlobalRANNodeID),
    #[asn(key = 204)]
    NB_IoT_DefaultPagingDRX(NB_IoT_DefaultPagingDRX),
    #[asn(key = 167)]
    NGRAN_TNLAssociationToRemoveList(NGRAN_TNLAssociationToRemoveList),
    #[asn(key = 21)]
    PagingDRX(PagingDRX),
    #[asn(key = 82)]
    RANNodeName(RANNodeName),
    #[asn(key = 102)]
    SupportedTAList(SupportedTAList),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct RANConfigurationUpdateprotocolIEs_Item {
    #[asn(key_field = true)]
    pub id: ProtocolIE_ID,
    pub criticality: Criticality,
    pub value: RANConfigurationUpdateprotocolIEs_Itemvalue,
}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "0",
    sz_ub = "65535"
)]
pub struct RANConfigurationUpdateprotocolIEs(Vec<RANConfigurationUpdateprotocolIEs_Item>);

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum RANConfigurationUpdateAcknowledgeprotocolIEs_Itemvalue {
    #[asn(key = 19)]
    CriticalityDiagnostics(CriticalityDiagnostics),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct RANConfigurationUpdateAcknowledgeprotocolIEs_Item {
    #[asn(key_field = true)]
    pub id: ProtocolIE_ID,
    pub criticality: Criticality,
    pub value: RANConfigurationUpdateAcknowledgeprotocolIEs_Itemvalue,
}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "0",
    sz_ub = "65535"
)]
pub struct RANConfigurationUpdateAcknowledgeprotocolIEs(
    Vec<RANConfigurationUpdateAcknowledgeprotocolIEs_Item>,
);

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum RANConfigurationUpdateFailureprotocolIEs_Itemvalue {
    #[asn(key = 15)]
    Cause(Cause),
    #[asn(key = 19)]
    CriticalityDiagnostics(CriticalityDiagnostics),
    #[asn(key = 107)]
    TimeToWait(TimeToWait),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct RANConfigurationUpdateFailureprotocolIEs_Item {
    #[asn(key_field = true)]
    pub id: ProtocolIE_ID,
    pub criticality: Criticality,
    pub value: RANConfigurationUpdateFailureprotocolIEs_Itemvalue,
}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "0",
    sz_ub = "65535"
)]
pub struct RANConfigurationUpdateFailureprotocolIEs(
    Vec<RANConfigurationUpdateFailureprotocolIEs_Item>,
);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct RANStatusTransfer_TransparentContaineriE_Extensions_Item {}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct RANStatusTransfer_TransparentContaineriE_Extensions(
    Vec<RANStatusTransfer_TransparentContaineriE_Extensions_Item>,
);

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum RATRestrictions_ItemiE_Extensions_ItemextensionValue {
    #[asn(key = 180)]
    ExtendedRATRestrictionInformation(ExtendedRATRestrictionInformation),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct RATRestrictions_ItemiE_Extensions_Item {
    #[asn(key_field = true)]
    pub id: ProtocolExtensionID,
    pub criticality: Criticality,
    pub extension_value: RATRestrictions_ItemiE_Extensions_ItemextensionValue,
}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct RATRestrictions_ItemiE_Extensions(Vec<RATRestrictions_ItemiE_Extensions_Item>);

#[derive(Debug, AperCodec)]
#[asn(type = "ENUMERATED", extensible = true, lb = "0", ub = "1")]
pub struct ENUMERATED_73(u8);
impl ENUMERATED_73 {
    const RS_DETECTED: u8 = 0u8;
    const RS_DISAPPEARED: u8 = 1u8;
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct RIMInformationiE_Extensions_Item {}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct RIMInformationiE_Extensions(Vec<RIMInformationiE_Extensions_Item>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct RIMInformationTransferiE_Extensions_Item {}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct RIMInformationTransferiE_Extensions(Vec<RIMInformationTransferiE_Extensions_Item>);

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum RRCInactiveTransitionReportprotocolIEs_Itemvalue {
    #[asn(key = 10)]
    AMF_UE_NGAP_ID(AMF_UE_NGAP_ID),
    #[asn(key = 85)]
    RAN_UE_NGAP_ID(RAN_UE_NGAP_ID),
    #[asn(key = 92)]
    RRCState(RRCState),
    #[asn(key = 121)]
    UserLocationInformation(UserLocationInformation),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct RRCInactiveTransitionReportprotocolIEs_Item {
    #[asn(key_field = true)]
    pub id: ProtocolIE_ID,
    pub criticality: Criticality,
    pub value: RRCInactiveTransitionReportprotocolIEs_Itemvalue,
}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "0",
    sz_ub = "65535"
)]
pub struct RRCInactiveTransitionReportprotocolIEs(Vec<RRCInactiveTransitionReportprotocolIEs_Item>);

#[derive(Debug, AperCodec)]
#[asn(type = "INTEGER", lb = "0", ub = "4095")]
pub struct INTEGER_74(u16);

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
pub struct RecommendedRANNodeItemiE_Extensions_Item {}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct RecommendedRANNodeItemiE_Extensions(Vec<RecommendedRANNodeItemiE_Extensions_Item>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct RecommendedRANNodesForPagingiE_Extensions_Item {}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct RecommendedRANNodesForPagingiE_Extensions(
    Vec<RecommendedRANNodesForPagingiE_Extensions_Item>,
);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct RedundantPDUSessionInformationiE_Extensions_Item {}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct RedundantPDUSessionInformationiE_Extensions(
    Vec<RedundantPDUSessionInformationiE_Extensions_Item>,
);

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum RerouteNASRequestprotocolIEs_Itemvalue {
    #[asn(key = 10)]
    AMF_UE_NGAP_ID(AMF_UE_NGAP_ID),
    #[asn(key = 3)]
    AMFSetID(AMFSetID),
    #[asn(key = 0)]
    AllowedNSSAI(AllowedNSSAI),
    #[asn(key = 85)]
    RAN_UE_NGAP_ID(RAN_UE_NGAP_ID),
    #[asn(key = 171)]
    SourceToTarget_AMFInformationReroute(SourceToTarget_AMFInformationReroute),
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
pub struct ResetTypechoice_Extensions {}

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum RetrieveUEInformationprotocolIEs_Itemvalue {
    #[asn(key = 26)]
    FiveG_S_TMSI(FiveG_S_TMSI),
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
pub struct S_NSSAIiE_Extensions_Item {}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct S_NSSAIiE_Extensions(Vec<S_NSSAIiE_Extensions_Item>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct SNPN_MobilityInformationiE_Extensions_Item {}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct SNPN_MobilityInformationiE_Extensions(Vec<SNPN_MobilityInformationiE_Extensions_Item>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct SONConfigurationTransferiE_Extensions_Item {}

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
pub enum SONInformationchoice_Extensionsvalue {
    #[asn(key = 252)]
    SONInformationReport(SONInformationReport),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct SONInformationchoice_Extensions {
    #[asn(key_field = true)]
    pub id: ProtocolIE_ID,
    pub criticality: Criticality,
    pub value: SONInformationchoice_Extensionsvalue,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct SONInformationReplyiE_Extensions_Item {}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct SONInformationReplyiE_Extensions(Vec<SONInformationReplyiE_Extensions_Item>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct SONInformationReportchoice_Extensions {}

#[derive(Debug, AperCodec)]
#[asn(type = "BITSTRING", sz_extensible = false, sz_lb = "7", sz_ub = "7")]
pub struct BIT_STRING_75(BitVec<Msb0, u8>);

#[derive(Debug, AperCodec)]
#[asn(type = "INTEGER", lb = "0", ub = "86399", extensible = true)]
pub struct INTEGER_76(u32);

#[derive(Debug, AperCodec)]
#[asn(type = "INTEGER", lb = "0", ub = "86399", extensible = true)]
pub struct INTEGER_77(u32);

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
    #[asn(key = 10)]
    AMF_UE_NGAP_ID(AMF_UE_NGAP_ID),
    #[asn(key = 143)]
    HandoverFlag(HandoverFlag),
    #[asn(key = 142)]
    PDUSessionResourceSecondaryRATUsageList(PDUSessionResourceSecondaryRATUsageList),
    #[asn(key = 85)]
    RAN_UE_NGAP_ID(RAN_UE_NGAP_ID),
    #[asn(key = 121)]
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
pub struct SecondaryRATDataUsageReportTransferiE_Extensions_Item {}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct SecondaryRATDataUsageReportTransferiE_Extensions(
    Vec<SecondaryRATDataUsageReportTransferiE_Extensions_Item>,
);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct SecondaryRATUsageInformationiE_Extension_Item {}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct SecondaryRATUsageInformationiE_Extension(
    Vec<SecondaryRATUsageInformationiE_Extension_Item>,
);

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
#[asn(type = "OPEN")]
pub enum SecurityIndicationiE_Extensions_ItemextensionValue {
    #[asn(key = 151)]
    MaximumIntegrityProtectedDataRate(MaximumIntegrityProtectedDataRate),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct SecurityIndicationiE_Extensions_Item {
    #[asn(key_field = true)]
    pub id: ProtocolExtensionID,
    pub criticality: Criticality,
    pub extension_value: SecurityIndicationiE_Extensions_ItemextensionValue,
}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct SecurityIndicationiE_Extensions(Vec<SecurityIndicationiE_Extensions_Item>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct SecurityResultiE_Extensions_Item {}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct SecurityResultiE_Extensions(Vec<SecurityResultiE_Extensions_Item>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct SensorMeasConfigNameItemiE_Extensions_Item {}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct SensorMeasConfigNameItemiE_Extensions(Vec<SensorMeasConfigNameItemiE_Extensions_Item>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct SensorMeasurementConfigurationiE_Extensions_Item {}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct SensorMeasurementConfigurationiE_Extensions(
    Vec<SensorMeasurementConfigurationiE_Extensions_Item>,
);

#[derive(Debug, AperCodec)]
#[asn(type = "ENUMERATED", extensible = true, lb = "0", ub = "0")]
pub struct ENUMERATED_78(u8);
impl ENUMERATED_78 {
    const TRUE: u8 = 0u8;
}

#[derive(Debug, AperCodec)]
#[asn(type = "ENUMERATED", extensible = true, lb = "0", ub = "0")]
pub struct ENUMERATED_79(u8);
impl ENUMERATED_79 {
    const TRUE: u8 = 0u8;
}

#[derive(Debug, AperCodec)]
#[asn(type = "ENUMERATED", extensible = true, lb = "0", ub = "0")]
pub struct ENUMERATED_80(u8);
impl ENUMERATED_80 {
    const TRUE: u8 = 0u8;
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct SensorNameConfigchoice_Extensions {}

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum ServedGUAMIItemiE_Extensions_ItemextensionValue {
    #[asn(key = 176)]
    GUAMIType(GUAMIType),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct ServedGUAMIItemiE_Extensions_Item {
    #[asn(key_field = true)]
    pub id: ProtocolExtensionID,
    pub criticality: Criticality,
    pub extension_value: ServedGUAMIItemiE_Extensions_ItemextensionValue,
}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct ServedGUAMIItemiE_Extensions(Vec<ServedGUAMIItemiE_Extensions_Item>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct ServiceAreaInformation_ItemiE_Extensions_Item {}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct ServiceAreaInformation_ItemiE_Extensions(
    Vec<ServiceAreaInformation_ItemiE_Extensions_Item>,
);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct SliceOverloadItemiE_Extensions_Item {}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct SliceOverloadItemiE_Extensions(Vec<SliceOverloadItemiE_Extensions_Item>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct SliceSupportItemiE_Extensions_Item {}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct SliceSupportItemiE_Extensions(Vec<SliceSupportItemiE_Extensions_Item>);

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum SourceNGRANNode_ToTargetNGRANNode_TransparentContaineriE_Extensions_ItemextensionValue {
    #[asn(key = 182)]
    SgNB_UE_X2AP_ID(SgNB_UE_X2AP_ID),
    #[asn(key = 253)]
    UEHistoryInformationFromTheUE(UEHistoryInformationFromTheUE),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct SourceNGRANNode_ToTargetNGRANNode_TransparentContaineriE_Extensions_Item {
    #[asn(key_field = true)]
    pub id: ProtocolExtensionID,
    pub criticality: Criticality,
    pub extension_value:
        SourceNGRANNode_ToTargetNGRANNode_TransparentContaineriE_Extensions_ItemextensionValue,
}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct SourceNGRANNode_ToTargetNGRANNode_TransparentContaineriE_Extensions(
    Vec<SourceNGRANNode_ToTargetNGRANNode_TransparentContaineriE_Extensions_Item>,
);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct SourceRANNodeIDiE_Extensions_Item {}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct SourceRANNodeIDiE_Extensions(Vec<SourceRANNodeIDiE_Extensions_Item>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct SourceToTarget_AMFInformationRerouteiE_Extensions_Item {}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct SourceToTarget_AMFInformationRerouteiE_Extensions(
    Vec<SourceToTarget_AMFInformationRerouteiE_Extensions_Item>,
);

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum SuccessfulOutcomevalue {
    #[asn(key = 0)]
    AMFConfigurationUpdateAcknowledge(AMFConfigurationUpdateAcknowledge),
    #[asn(key = 10)]
    HandoverCancelAcknowledge(HandoverCancelAcknowledge),
    #[asn(key = 12)]
    HandoverCommand(HandoverCommand),
    #[asn(key = 13)]
    HandoverRequestAcknowledge(HandoverRequestAcknowledge),
    #[asn(key = 14)]
    InitialContextSetupResponse(InitialContextSetupResponse),
    #[asn(key = 20)]
    NGResetAcknowledge(NGResetAcknowledge),
    #[asn(key = 21)]
    NGSetupResponse(NGSetupResponse),
    #[asn(key = 27)]
    PDUSessionResourceModifyConfirm(PDUSessionResourceModifyConfirm),
    #[asn(key = 26)]
    PDUSessionResourceModifyResponse(PDUSessionResourceModifyResponse),
    #[asn(key = 28)]
    PDUSessionResourceReleaseResponse(PDUSessionResourceReleaseResponse),
    #[asn(key = 29)]
    PDUSessionResourceSetupResponse(PDUSessionResourceSetupResponse),
    #[asn(key = 32)]
    PWSCancelResponse(PWSCancelResponse),
    #[asn(key = 25)]
    PathSwitchRequestAcknowledge(PathSwitchRequestAcknowledge),
    #[asn(key = 35)]
    RANConfigurationUpdateAcknowledge(RANConfigurationUpdateAcknowledge),
    #[asn(key = 40)]
    UEContextModificationResponse(UEContextModificationResponse),
    #[asn(key = 41)]
    UEContextReleaseComplete(UEContextReleaseComplete),
    #[asn(key = 58)]
    UEContextResumeResponse(UEContextResumeResponse),
    #[asn(key = 59)]
    UEContextSuspendResponse(UEContextSuspendResponse),
    #[asn(key = 43)]
    UERadioCapabilityCheckResponse(UERadioCapabilityCheckResponse),
    #[asn(key = 60)]
    UERadioCapabilityIDMappingResponse(UERadioCapabilityIDMappingResponse),
    #[asn(key = 51)]
    WriteReplaceWarningResponse(WriteReplaceWarningResponse),
}

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum SupportedTAItemiE_Extensions_ItemextensionValue {
    #[asn(key = 272)]
    ConfiguredTACIndication(ConfiguredTACIndication),
    #[asn(key = 179)]
    RAT_Information(RAT_Information),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct SupportedTAItemiE_Extensions_Item {
    #[asn(key_field = true)]
    pub id: ProtocolExtensionID,
    pub criticality: Criticality,
    pub extension_value: SupportedTAItemiE_Extensions_ItemextensionValue,
}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct SupportedTAItemiE_Extensions(Vec<SupportedTAItemiE_Extensions_Item>);

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
pub struct TAIBroadcastEUTRA_ItemiE_Extensions_Item {}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct TAIBroadcastEUTRA_ItemiE_Extensions(Vec<TAIBroadcastEUTRA_ItemiE_Extensions_Item>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct TAIBroadcastNR_ItemiE_Extensions_Item {}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct TAIBroadcastNR_ItemiE_Extensions(Vec<TAIBroadcastNR_ItemiE_Extensions_Item>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct TAICancelledEUTRA_ItemiE_Extensions_Item {}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct TAICancelledEUTRA_ItemiE_Extensions(Vec<TAICancelledEUTRA_ItemiE_Extensions_Item>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct TAICancelledNR_ItemiE_Extensions_Item {}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct TAICancelledNR_ItemiE_Extensions(Vec<TAICancelledNR_ItemiE_Extensions_Item>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct TAIListForInactiveItemiE_Extensions_Item {}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct TAIListForInactiveItemiE_Extensions(Vec<TAIListForInactiveItemiE_Extensions_Item>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct TAIListForPagingItemiE_Extensions_Item {}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct TAIListForPagingItemiE_Extensions(Vec<TAIListForPagingItemiE_Extensions_Item>);

#[derive(Debug, AperCodec)]
#[asn(type = "BITSTRING", sz_extensible = true, sz_lb = "32", sz_ub = "32")]
pub struct BIT_STRING_81(BitVec<Msb0, u8>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct TNGF_IDchoice_Extensions {}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct TNLAssociationItemiE_Extensions_Item {}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct TNLAssociationItemiE_Extensions(Vec<TNLAssociationItemiE_Extensions_Item>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct TSCAssistanceInformationiE_Extensions_Item {}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct TSCAssistanceInformationiE_Extensions(Vec<TSCAssistanceInformationiE_Extensions_Item>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct TSCTrafficCharacteristicsiE_Extensions_Item {}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct TSCTrafficCharacteristicsiE_Extensions(Vec<TSCTrafficCharacteristicsiE_Extensions_Item>);

#[derive(Debug, AperCodec)]
#[asn(type = "BITSTRING", sz_extensible = true, sz_lb = "32", sz_ub = "32")]
pub struct BIT_STRING_82(BitVec<Msb0, u8>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct TWIF_IDchoice_Extensions {}

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum TargetIDchoice_Extensionsvalue {
    #[asn(key = 178)]
    TargetRNC_ID(TargetRNC_ID),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct TargetIDchoice_Extensions {
    #[asn(key_field = true)]
    pub id: ProtocolIE_ID,
    pub criticality: Criticality,
    pub value: TargetIDchoice_Extensionsvalue,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct TargetNGRANNode_ToSourceNGRANNode_FailureTransparentContaineriE_Extensions_Item {}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct TargetNGRANNode_ToSourceNGRANNode_FailureTransparentContaineriE_Extensions(
    Vec<TargetNGRANNode_ToSourceNGRANNode_FailureTransparentContaineriE_Extensions_Item>,
);

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum TargetNGRANNode_ToSourceNGRANNode_TransparentContaineriE_Extensions_ItemextensionValue {
    #[asn(key = 267)]
    DAPSResponseInfoList(DAPSResponseInfoList),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct TargetNGRANNode_ToSourceNGRANNode_TransparentContaineriE_Extensions_Item {
    #[asn(key_field = true)]
    pub id: ProtocolExtensionID,
    pub criticality: Criticality,
    pub extension_value:
        TargetNGRANNode_ToSourceNGRANNode_TransparentContaineriE_Extensions_ItemextensionValue,
}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct TargetNGRANNode_ToSourceNGRANNode_TransparentContaineriE_Extensions(
    Vec<TargetNGRANNode_ToSourceNGRANNode_TransparentContaineriE_Extensions_Item>,
);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct TargetRANNodeIDiE_Extensions_Item {}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct TargetRANNodeIDiE_Extensions(Vec<TargetRANNodeIDiE_Extensions_Item>);

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
#[asn(type = "SEQUENCE", extensible = false)]
pub struct TooearlyIntersystemHOiE_Extensions_Item {}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct TooearlyIntersystemHOiE_Extensions(Vec<TooearlyIntersystemHOiE_Extensions_Item>);

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum TraceActivationiE_Extensions_ItemextensionValue {
    #[asn(key = 255)]
    MDT_Configuration(MDT_Configuration),
    #[asn(key = 257)]
    URI_address(URI_address),
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
    #[asn(key = 10)]
    AMF_UE_NGAP_ID(AMF_UE_NGAP_ID),
    #[asn(key = 15)]
    Cause(Cause),
    #[asn(key = 44)]
    NGRANTraceID(NGRANTraceID),
    #[asn(key = 85)]
    RAN_UE_NGAP_ID(RAN_UE_NGAP_ID),
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
    #[asn(key = 10)]
    AMF_UE_NGAP_ID(AMF_UE_NGAP_ID),
    #[asn(key = 85)]
    RAN_UE_NGAP_ID(RAN_UE_NGAP_ID),
    #[asn(key = 108)]
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
#[asn(type = "ENUMERATED", extensible = true, lb = "0", ub = "1")]
pub struct ENUMERATED_83(u8);
impl ENUMERATED_83 {
    const PERIODICALLY: u8 = 0u8;
    const ONDEMAND: u8 = 1u8;
}

#[derive(Debug, AperCodec)]
#[asn(type = "INTEGER", lb = "1", ub = "3600", extensible = true)]
pub struct INTEGER_84(u16);

#[derive(Debug, AperCodec)]
#[asn(type = "ENUMERATED", extensible = true, lb = "0", ub = "1")]
pub struct ENUMERATED_85(u8);
impl ENUMERATED_85 {
    const STATIONARY: u8 = 0u8;
    const MOBILE: u8 = 1u8;
}

#[derive(Debug, AperCodec)]
#[asn(type = "ENUMERATED", extensible = true, lb = "0", ub = "2")]
pub struct ENUMERATED_86(u8);
impl ENUMERATED_86 {
    const SINGLE_PACKET: u8 = 0u8;
    const DUAL_PACKETS: u8 = 1u8;
    const MULTIPLE_PACKETS: u8 = 2u8;
}

#[derive(Debug, AperCodec)]
#[asn(type = "ENUMERATED", extensible = true, lb = "0", ub = "2")]
pub struct ENUMERATED_87(u8);
impl ENUMERATED_87 {
    const BATTERY_POWERED: u8 = 0u8;
    const BATTERY_POWERED_NOT_RECHARGEABLE_OR_REPLACEABLE: u8 = 1u8;
    const NOT_BATTERY_POWERED: u8 = 2u8;
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct UE_DifferentiationInfoiE_Extensions_Item {}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct UE_DifferentiationInfoiE_Extensions(Vec<UE_DifferentiationInfoiE_Extensions_Item>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct UE_NGAP_ID_pairiE_Extensions_Item {}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct UE_NGAP_ID_pairiE_Extensions(Vec<UE_NGAP_ID_pairiE_Extensions_Item>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct UE_NGAP_IDschoice_Extensions {}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct UE_associatedLogicalNG_connectionItemiE_Extensions_Item {}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct UE_associatedLogicalNG_connectionItemiE_Extensions(
    Vec<UE_associatedLogicalNG_connectionItemiE_Extensions_Item>,
);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct UEAggregateMaximumBitRateiE_Extensions_Item {}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct UEAggregateMaximumBitRateiE_Extensions(Vec<UEAggregateMaximumBitRateiE_Extensions_Item>);

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum UEContextModificationFailureprotocolIEs_Itemvalue {
    #[asn(key = 10)]
    AMF_UE_NGAP_ID(AMF_UE_NGAP_ID),
    #[asn(key = 15)]
    Cause(Cause),
    #[asn(key = 19)]
    CriticalityDiagnostics(CriticalityDiagnostics),
    #[asn(key = 85)]
    RAN_UE_NGAP_ID(RAN_UE_NGAP_ID),
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
pub enum UEContextModificationRequestprotocolIEs_Itemvalue {
    #[asn(key = 10)]
    AMF_UE_NGAP_ID(AMF_UE_NGAP_ID),
    #[asn(key = 165)]
    CNAssistedRANTuning(CNAssistedRANTuning),
    #[asn(key = 18)]
    CoreNetworkAssistanceInformationForInactive(CoreNetworkAssistanceInformationForInactive),
    #[asn(key = 24)]
    EmergencyFallbackIndicator(EmergencyFallbackIndicator),
    #[asn(key = 162)]
    GUAMI(GUAMI),
    #[asn(key = 199)]
    IAB_Authorized(IAB_Authorized),
    #[asn(key = 31)]
    IndexToRFSP(IndexToRFSP),
    #[asn(key = 217)]
    LTEUESidelinkAggregateMaximumBitrate(LTEUESidelinkAggregateMaximumBitrate),
    #[asn(key = 215)]
    LTEV2XServicesAuthorized(LTEV2XServicesAuthorized),
    #[asn(key = 218)]
    NRUESidelinkAggregateMaximumBitrate(NRUESidelinkAggregateMaximumBitrate),
    #[asn(key = 216)]
    NRV2XServicesAuthorized(NRV2XServicesAuthorized),
    #[asn(key = 219)]
    PC5QoSParameters(PC5QoSParameters),
    #[asn(key = 85)]
    RAN_UE_NGAP_ID(RAN_UE_NGAP_ID),
    #[asn(key = 83)]
    RANPagingPriority(RANPagingPriority),
    #[asn(key = 238)]
    RGLevelWirelineAccessCharacteristics(RGLevelWirelineAccessCharacteristics),
    #[asn(key = 91)]
    RRCInactiveTransitionReportRequest(RRCInactiveTransitionReportRequest),
    #[asn(key = 177)]
    SRVCCOperationPossible(SRVCCOperationPossible),
    #[asn(key = 94)]
    SecurityKey(SecurityKey),
    #[asn(key = 110)]
    UEAggregateMaximumBitRate(UEAggregateMaximumBitRate),
    #[asn(key = 264)]
    UERadioCapabilityID(UERadioCapabilityID),
    #[asn(key = 119)]
    UESecurityCapabilities(UESecurityCapabilities),
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
    #[asn(key = 10)]
    AMF_UE_NGAP_ID(AMF_UE_NGAP_ID),
    #[asn(key = 19)]
    CriticalityDiagnostics(CriticalityDiagnostics),
    #[asn(key = 85)]
    RAN_UE_NGAP_ID(RAN_UE_NGAP_ID),
    #[asn(key = 92)]
    RRCState(RRCState),
    #[asn(key = 121)]
    UserLocationInformation(UserLocationInformation),
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
    #[asn(key = 15)]
    Cause(Cause),
    #[asn(key = 114)]
    UE_NGAP_IDs(UE_NGAP_IDs),
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
    #[asn(key = 10)]
    AMF_UE_NGAP_ID(AMF_UE_NGAP_ID),
    #[asn(key = 19)]
    CriticalityDiagnostics(CriticalityDiagnostics),
    #[asn(key = 32)]
    InfoOnRecommendedCellsAndRANNodesForPaging(InfoOnRecommendedCellsAndRANNodesForPaging),
    #[asn(key = 60)]
    PDUSessionResourceListCxtRelCpl(PDUSessionResourceListCxtRelCpl),
    #[asn(key = 207)]
    PagingAssisDataforCEcapabUE(PagingAssisDataforCEcapabUE),
    #[asn(key = 85)]
    RAN_UE_NGAP_ID(RAN_UE_NGAP_ID),
    #[asn(key = 121)]
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
    #[asn(key = 10)]
    AMF_UE_NGAP_ID(AMF_UE_NGAP_ID),
    #[asn(key = 15)]
    Cause(Cause),
    #[asn(key = 133)]
    PDUSessionResourceListCxtRelReq(PDUSessionResourceListCxtRelReq),
    #[asn(key = 85)]
    RAN_UE_NGAP_ID(RAN_UE_NGAP_ID),
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
    #[asn(key = 10)]
    AMF_UE_NGAP_ID(AMF_UE_NGAP_ID),
    #[asn(key = 15)]
    Cause(Cause),
    #[asn(key = 19)]
    CriticalityDiagnostics(CriticalityDiagnostics),
    #[asn(key = 85)]
    RAN_UE_NGAP_ID(RAN_UE_NGAP_ID),
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
    #[asn(key = 10)]
    AMF_UE_NGAP_ID(AMF_UE_NGAP_ID),
    #[asn(key = 32)]
    InfoOnRecommendedCellsAndRANNodesForPaging(InfoOnRecommendedCellsAndRANNodesForPaging),
    #[asn(key = 229)]
    PDUSessionResourceFailedToResumeListRESReq(PDUSessionResourceFailedToResumeListRESReq),
    #[asn(key = 232)]
    PDUSessionResourceResumeListRESReq(PDUSessionResourceResumeListRESReq),
    #[asn(key = 207)]
    PagingAssisDataforCEcapabUE(PagingAssisDataforCEcapabUE),
    #[asn(key = 85)]
    RAN_UE_NGAP_ID(RAN_UE_NGAP_ID),
    #[asn(key = 237)]
    RRCEstablishmentCause(RRCEstablishmentCause),
    #[asn(key = 235)]
    Suspend_Request_Indication(Suspend_Request_Indication),
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
#[asn(type = "SEQUENCE", extensible = false)]
pub struct UEContextResumeRequestTransferiE_Extensions_Item {}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct UEContextResumeRequestTransferiE_Extensions(
    Vec<UEContextResumeRequestTransferiE_Extensions_Item>,
);

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum UEContextResumeResponseprotocolIEs_Itemvalue {
    #[asn(key = 10)]
    AMF_UE_NGAP_ID(AMF_UE_NGAP_ID),
    #[asn(key = 19)]
    CriticalityDiagnostics(CriticalityDiagnostics),
    #[asn(key = 206)]
    Extended_ConnectedTime(Extended_ConnectedTime),
    #[asn(key = 230)]
    PDUSessionResourceFailedToResumeListRESRes(PDUSessionResourceFailedToResumeListRESRes),
    #[asn(key = 233)]
    PDUSessionResourceResumeListRESRes(PDUSessionResourceResumeListRESRes),
    #[asn(key = 85)]
    RAN_UE_NGAP_ID(RAN_UE_NGAP_ID),
    #[asn(key = 93)]
    SecurityContext(SecurityContext),
    #[asn(key = 236)]
    Suspend_Response_Indication(Suspend_Response_Indication),
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
#[asn(type = "SEQUENCE", extensible = false)]
pub struct UEContextResumeResponseTransferiE_Extensions_Item {}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct UEContextResumeResponseTransferiE_Extensions(
    Vec<UEContextResumeResponseTransferiE_Extensions_Item>,
);

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum UEContextSuspendFailureprotocolIEs_Itemvalue {
    #[asn(key = 10)]
    AMF_UE_NGAP_ID(AMF_UE_NGAP_ID),
    #[asn(key = 15)]
    Cause(Cause),
    #[asn(key = 19)]
    CriticalityDiagnostics(CriticalityDiagnostics),
    #[asn(key = 85)]
    RAN_UE_NGAP_ID(RAN_UE_NGAP_ID),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct UEContextSuspendFailureprotocolIEs_Item {
    #[asn(key_field = true)]
    pub id: ProtocolIE_ID,
    pub criticality: Criticality,
    pub value: UEContextSuspendFailureprotocolIEs_Itemvalue,
}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "0",
    sz_ub = "65535"
)]
pub struct UEContextSuspendFailureprotocolIEs(Vec<UEContextSuspendFailureprotocolIEs_Item>);

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum UEContextSuspendRequestprotocolIEs_Itemvalue {
    #[asn(key = 10)]
    AMF_UE_NGAP_ID(AMF_UE_NGAP_ID),
    #[asn(key = 32)]
    InfoOnRecommendedCellsAndRANNodesForPaging(InfoOnRecommendedCellsAndRANNodesForPaging),
    #[asn(key = 231)]
    PDUSessionResourceSuspendListSUSReq(PDUSessionResourceSuspendListSUSReq),
    #[asn(key = 207)]
    PagingAssisDataforCEcapabUE(PagingAssisDataforCEcapabUE),
    #[asn(key = 85)]
    RAN_UE_NGAP_ID(RAN_UE_NGAP_ID),
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
#[asn(type = "SEQUENCE", extensible = false)]
pub struct UEContextSuspendRequestTransferiE_Extensions_Item {}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct UEContextSuspendRequestTransferiE_Extensions(
    Vec<UEContextSuspendRequestTransferiE_Extensions_Item>,
);

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum UEContextSuspendResponseprotocolIEs_Itemvalue {
    #[asn(key = 10)]
    AMF_UE_NGAP_ID(AMF_UE_NGAP_ID),
    #[asn(key = 19)]
    CriticalityDiagnostics(CriticalityDiagnostics),
    #[asn(key = 85)]
    RAN_UE_NGAP_ID(RAN_UE_NGAP_ID),
    #[asn(key = 93)]
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
#[asn(type = "SEQUENCE", extensible = false)]
pub struct UEHistoryInformationFromTheUEchoice_Extensions {}

#[derive(Debug, AperCodec)]
#[asn(type = "BITSTRING", sz_extensible = false, sz_lb = "10", sz_ub = "10")]
pub struct BIT_STRING_88(BitVec<Msb0, u8>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct UEIdentityIndexValuechoice_Extensions {}

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum UEInformationTransferprotocolIEs_Itemvalue {
    #[asn(key = 0)]
    AllowedNSSAI(AllowedNSSAI),
    #[asn(key = 26)]
    FiveG_S_TMSI(FiveG_S_TMSI),
    #[asn(key = 210)]
    NB_IoT_UEPriority(NB_IoT_UEPriority),
    #[asn(key = 148)]
    S_NSSAI(S_NSSAI),
    #[asn(key = 209)]
    UE_DifferentiationInfo(UE_DifferentiationInfo),
    #[asn(key = 117)]
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
#[asn(type = "SEQUENCE", extensible = false)]
pub struct UEPagingIdentitychoice_Extensions {}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct UEPresenceInAreaOfInterestItemiE_Extensions_Item {}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct UEPresenceInAreaOfInterestItemiE_Extensions(
    Vec<UEPresenceInAreaOfInterestItemiE_Extensions_Item>,
);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct UERLFReportContainerchoice_Extensions {}

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum UERadioCapabilityCheckRequestprotocolIEs_Itemvalue {
    #[asn(key = 10)]
    AMF_UE_NGAP_ID(AMF_UE_NGAP_ID),
    #[asn(key = 85)]
    RAN_UE_NGAP_ID(RAN_UE_NGAP_ID),
    #[asn(key = 117)]
    UERadioCapability(UERadioCapability),
    #[asn(key = 264)]
    UERadioCapabilityID(UERadioCapabilityID),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct UERadioCapabilityCheckRequestprotocolIEs_Item {
    #[asn(key_field = true)]
    pub id: ProtocolIE_ID,
    pub criticality: Criticality,
    pub value: UERadioCapabilityCheckRequestprotocolIEs_Itemvalue,
}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "0",
    sz_ub = "65535"
)]
pub struct UERadioCapabilityCheckRequestprotocolIEs(
    Vec<UERadioCapabilityCheckRequestprotocolIEs_Item>,
);

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum UERadioCapabilityCheckResponseprotocolIEs_Itemvalue {
    #[asn(key = 10)]
    AMF_UE_NGAP_ID(AMF_UE_NGAP_ID),
    #[asn(key = 19)]
    CriticalityDiagnostics(CriticalityDiagnostics),
    #[asn(key = 30)]
    IMSVoiceSupportIndicator(IMSVoiceSupportIndicator),
    #[asn(key = 85)]
    RAN_UE_NGAP_ID(RAN_UE_NGAP_ID),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct UERadioCapabilityCheckResponseprotocolIEs_Item {
    #[asn(key_field = true)]
    pub id: ProtocolIE_ID,
    pub criticality: Criticality,
    pub value: UERadioCapabilityCheckResponseprotocolIEs_Itemvalue,
}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "0",
    sz_ub = "65535"
)]
pub struct UERadioCapabilityCheckResponseprotocolIEs(
    Vec<UERadioCapabilityCheckResponseprotocolIEs_Item>,
);

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum UERadioCapabilityForPagingiE_Extensions_ItemextensionValue {
    #[asn(key = 214)]
    UERadioCapabilityForPagingOfNB_IoT(UERadioCapabilityForPagingOfNB_IoT),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct UERadioCapabilityForPagingiE_Extensions_Item {
    #[asn(key_field = true)]
    pub id: ProtocolExtensionID,
    pub criticality: Criticality,
    pub extension_value: UERadioCapabilityForPagingiE_Extensions_ItemextensionValue,
}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct UERadioCapabilityForPagingiE_Extensions(
    Vec<UERadioCapabilityForPagingiE_Extensions_Item>,
);

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum UERadioCapabilityIDMappingRequestprotocolIEs_Itemvalue {
    #[asn(key = 264)]
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
    #[asn(key = 19)]
    CriticalityDiagnostics(CriticalityDiagnostics),
    #[asn(key = 117)]
    UERadioCapability(UERadioCapability),
    #[asn(key = 264)]
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
pub enum UERadioCapabilityInfoIndicationprotocolIEs_Itemvalue {
    #[asn(key = 10)]
    AMF_UE_NGAP_ID(AMF_UE_NGAP_ID),
    #[asn(key = 85)]
    RAN_UE_NGAP_ID(RAN_UE_NGAP_ID),
    #[asn(key = 117)]
    UERadioCapability(UERadioCapability),
    #[asn(key = 118)]
    UERadioCapabilityForPaging(UERadioCapabilityForPaging),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct UERadioCapabilityInfoIndicationprotocolIEs_Item {
    #[asn(key_field = true)]
    pub id: ProtocolIE_ID,
    pub criticality: Criticality,
    pub value: UERadioCapabilityInfoIndicationprotocolIEs_Itemvalue,
}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "0",
    sz_ub = "65535"
)]
pub struct UERadioCapabilityInfoIndicationprotocolIEs(
    Vec<UERadioCapabilityInfoIndicationprotocolIEs_Item>,
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
#[asn(type = "OPEN")]
pub enum UETNLABindingReleaseRequestprotocolIEs_Itemvalue {
    #[asn(key = 10)]
    AMF_UE_NGAP_ID(AMF_UE_NGAP_ID),
    #[asn(key = 85)]
    RAN_UE_NGAP_ID(RAN_UE_NGAP_ID),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct UETNLABindingReleaseRequestprotocolIEs_Item {
    #[asn(key_field = true)]
    pub id: ProtocolIE_ID,
    pub criticality: Criticality,
    pub value: UETNLABindingReleaseRequestprotocolIEs_Itemvalue,
}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "0",
    sz_ub = "65535"
)]
pub struct UETNLABindingReleaseRequestprotocolIEs(Vec<UETNLABindingReleaseRequestprotocolIEs_Item>);

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
pub enum UL_NGU_UP_TNLModifyItemiE_Extensions_ItemextensionValue {
    #[asn(key = 192)]
    UPTransportLayerInformation(UPTransportLayerInformation),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct UL_NGU_UP_TNLModifyItemiE_Extensions_Item {
    #[asn(key_field = true)]
    pub id: ProtocolExtensionID,
    pub criticality: Criticality,
    pub extension_value: UL_NGU_UP_TNLModifyItemiE_Extensions_ItemextensionValue,
}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct UL_NGU_UP_TNLModifyItemiE_Extensions(Vec<UL_NGU_UP_TNLModifyItemiE_Extensions_Item>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct UPTransportLayerInformationchoice_Extensions {}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct UPTransportLayerInformationItemiE_Extensions_Item {}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct UPTransportLayerInformationItemiE_Extensions(
    Vec<UPTransportLayerInformationItemiE_Extensions_Item>,
);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct UPTransportLayerInformationPairItemiE_Extensions_Item {}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct UPTransportLayerInformationPairItemiE_Extensions(
    Vec<UPTransportLayerInformationPairItemiE_Extensions_Item>,
);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct UnavailableGUAMIItemiE_Extensions_Item {}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct UnavailableGUAMIItemiE_Extensions(Vec<UnavailableGUAMIItemiE_Extensions_Item>);

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum UnsuccessfulOutcomevalue {
    #[asn(key = 0)]
    AMFConfigurationUpdateFailure(AMFConfigurationUpdateFailure),
    #[asn(key = 13)]
    HandoverFailure(HandoverFailure),
    #[asn(key = 12)]
    HandoverPreparationFailure(HandoverPreparationFailure),
    #[asn(key = 14)]
    InitialContextSetupFailure(InitialContextSetupFailure),
    #[asn(key = 21)]
    NGSetupFailure(NGSetupFailure),
    #[asn(key = 25)]
    PathSwitchRequestFailure(PathSwitchRequestFailure),
    #[asn(key = 35)]
    RANConfigurationUpdateFailure(RANConfigurationUpdateFailure),
    #[asn(key = 40)]
    UEContextModificationFailure(UEContextModificationFailure),
    #[asn(key = 58)]
    UEContextResumeFailure(UEContextResumeFailure),
    #[asn(key = 59)]
    UEContextSuspendFailure(UEContextSuspendFailure),
}

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum UplinkNASTransportprotocolIEs_Itemvalue {
    #[asn(key = 10)]
    AMF_UE_NGAP_ID(AMF_UE_NGAP_ID),
    #[asn(key = 38)]
    NAS_PDU(NAS_PDU),
    #[asn(key = 85)]
    RAN_UE_NGAP_ID(RAN_UE_NGAP_ID),
    #[asn(key = 121)]
    UserLocationInformation(UserLocationInformation),
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
pub enum UplinkNonUEAssociatedNRPPaTransportprotocolIEs_Itemvalue {
    #[asn(key = 46)]
    NRPPa_PDU(NRPPa_PDU),
    #[asn(key = 89)]
    RoutingID(RoutingID),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct UplinkNonUEAssociatedNRPPaTransportprotocolIEs_Item {
    #[asn(key_field = true)]
    pub id: ProtocolIE_ID,
    pub criticality: Criticality,
    pub value: UplinkNonUEAssociatedNRPPaTransportprotocolIEs_Itemvalue,
}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "0",
    sz_ub = "65535"
)]
pub struct UplinkNonUEAssociatedNRPPaTransportprotocolIEs(
    Vec<UplinkNonUEAssociatedNRPPaTransportprotocolIEs_Item>,
);

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum UplinkRANConfigurationTransferprotocolIEs_Itemvalue {
    #[asn(key = 158)]
    EN_DCSONConfigurationTransfer(EN_DCSONConfigurationTransfer),
    #[asn(key = 251)]
    IntersystemSONConfigurationTransfer(IntersystemSONConfigurationTransfer),
    #[asn(key = 99)]
    SONConfigurationTransfer(SONConfigurationTransfer),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct UplinkRANConfigurationTransferprotocolIEs_Item {
    #[asn(key_field = true)]
    pub id: ProtocolIE_ID,
    pub criticality: Criticality,
    pub value: UplinkRANConfigurationTransferprotocolIEs_Itemvalue,
}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "0",
    sz_ub = "65535"
)]
pub struct UplinkRANConfigurationTransferprotocolIEs(
    Vec<UplinkRANConfigurationTransferprotocolIEs_Item>,
);

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum UplinkRANEarlyStatusTransferprotocolIEs_Itemvalue {
    #[asn(key = 10)]
    AMF_UE_NGAP_ID(AMF_UE_NGAP_ID),
    #[asn(key = 268)]
    EarlyStatusTransfer_TransparentContainer(EarlyStatusTransfer_TransparentContainer),
    #[asn(key = 85)]
    RAN_UE_NGAP_ID(RAN_UE_NGAP_ID),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct UplinkRANEarlyStatusTransferprotocolIEs_Item {
    #[asn(key_field = true)]
    pub id: ProtocolIE_ID,
    pub criticality: Criticality,
    pub value: UplinkRANEarlyStatusTransferprotocolIEs_Itemvalue,
}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "0",
    sz_ub = "65535"
)]
pub struct UplinkRANEarlyStatusTransferprotocolIEs(
    Vec<UplinkRANEarlyStatusTransferprotocolIEs_Item>,
);

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum UplinkRANStatusTransferprotocolIEs_Itemvalue {
    #[asn(key = 10)]
    AMF_UE_NGAP_ID(AMF_UE_NGAP_ID),
    #[asn(key = 85)]
    RAN_UE_NGAP_ID(RAN_UE_NGAP_ID),
    #[asn(key = 84)]
    RANStatusTransfer_TransparentContainer(RANStatusTransfer_TransparentContainer),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct UplinkRANStatusTransferprotocolIEs_Item {
    #[asn(key_field = true)]
    pub id: ProtocolIE_ID,
    pub criticality: Criticality,
    pub value: UplinkRANStatusTransferprotocolIEs_Itemvalue,
}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "0",
    sz_ub = "65535"
)]
pub struct UplinkRANStatusTransferprotocolIEs(Vec<UplinkRANStatusTransferprotocolIEs_Item>);

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum UplinkRIMInformationTransferprotocolIEs_Itemvalue {
    #[asn(key = 175)]
    RIMInformationTransfer(RIMInformationTransfer),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct UplinkRIMInformationTransferprotocolIEs_Item {
    #[asn(key_field = true)]
    pub id: ProtocolIE_ID,
    pub criticality: Criticality,
    pub value: UplinkRIMInformationTransferprotocolIEs_Itemvalue,
}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "0",
    sz_ub = "65535"
)]
pub struct UplinkRIMInformationTransferprotocolIEs(
    Vec<UplinkRIMInformationTransferprotocolIEs_Item>,
);

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum UplinkUEAssociatedNRPPaTransportprotocolIEs_Itemvalue {
    #[asn(key = 10)]
    AMF_UE_NGAP_ID(AMF_UE_NGAP_ID),
    #[asn(key = 46)]
    NRPPa_PDU(NRPPa_PDU),
    #[asn(key = 85)]
    RAN_UE_NGAP_ID(RAN_UE_NGAP_ID),
    #[asn(key = 89)]
    RoutingID(RoutingID),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct UplinkUEAssociatedNRPPaTransportprotocolIEs_Item {
    #[asn(key_field = true)]
    pub id: ProtocolIE_ID,
    pub criticality: Criticality,
    pub value: UplinkUEAssociatedNRPPaTransportprotocolIEs_Itemvalue,
}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "0",
    sz_ub = "65535"
)]
pub struct UplinkUEAssociatedNRPPaTransportprotocolIEs(
    Vec<UplinkUEAssociatedNRPPaTransportprotocolIEs_Item>,
);

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum UserLocationInformationchoice_Extensionsvalue {
    #[asn(key = 244)]
    UserLocationInformationTNGF(UserLocationInformationTNGF),
    #[asn(key = 248)]
    UserLocationInformationTWIF(UserLocationInformationTWIF),
    #[asn(key = 243)]
    UserLocationInformationW_AGF(UserLocationInformationW_AGF),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct UserLocationInformationchoice_Extensions {
    #[asn(key_field = true)]
    pub id: ProtocolIE_ID,
    pub criticality: Criticality,
    pub value: UserLocationInformationchoice_Extensionsvalue,
}

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum UserLocationInformationEUTRAiE_Extensions_ItemextensionValue {
    #[asn(key = 149)]
    NGRAN_CGI(NGRAN_CGI),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct UserLocationInformationEUTRAiE_Extensions_Item {
    #[asn(key_field = true)]
    pub id: ProtocolExtensionID,
    pub criticality: Criticality,
    pub extension_value: UserLocationInformationEUTRAiE_Extensions_ItemextensionValue,
}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct UserLocationInformationEUTRAiE_Extensions(
    Vec<UserLocationInformationEUTRAiE_Extensions_Item>,
);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct UserLocationInformationN3IWFiE_Extensions_Item {}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct UserLocationInformationN3IWFiE_Extensions(
    Vec<UserLocationInformationN3IWFiE_Extensions_Item>,
);

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum UserLocationInformationNRiE_Extensions_ItemextensionValue {
    #[asn(key = 149)]
    NGRAN_CGI(NGRAN_CGI),
    #[asn(key = 263)]
    NID(NID),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct UserLocationInformationNRiE_Extensions_Item {
    #[asn(key_field = true)]
    pub id: ProtocolExtensionID,
    pub criticality: Criticality,
    pub extension_value: UserLocationInformationNRiE_Extensions_ItemextensionValue,
}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct UserLocationInformationNRiE_Extensions(Vec<UserLocationInformationNRiE_Extensions_Item>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct UserLocationInformationTNGFiE_Extensions_Item {}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct UserLocationInformationTNGFiE_Extensions(
    Vec<UserLocationInformationTNGFiE_Extensions_Item>,
);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct UserLocationInformationTWIFiE_Extensions_Item {}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct UserLocationInformationTWIFiE_Extensions(
    Vec<UserLocationInformationTWIFiE_Extensions_Item>,
);

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum UserLocationInformationW_AGFchoice_Extensionsvalue {
    #[asn(key = 275)]
    GlobalCable_ID(GlobalCable_ID),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct UserLocationInformationW_AGFchoice_Extensions {
    #[asn(key_field = true)]
    pub id: ProtocolIE_ID,
    pub criticality: Criticality,
    pub value: UserLocationInformationW_AGFchoice_Extensionsvalue,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct UserPlaneSecurityInformationiE_Extensions_Item {}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct UserPlaneSecurityInformationiE_Extensions(
    Vec<UserPlaneSecurityInformationiE_Extensions_Item>,
);

#[derive(Debug, AperCodec)]
#[asn(type = "OCTET-STRING", sz_extensible = false, sz_lb = "4", sz_ub = "4")]
pub struct OCTET_STRING_89(Vec<u8>);

#[derive(Debug, AperCodec)]
#[asn(type = "OCTET-STRING", sz_extensible = false, sz_lb = "4", sz_ub = "4")]
pub struct OCTET_STRING_90(Vec<u8>);

#[derive(Debug, AperCodec)]
#[asn(type = "INTEGER", lb = "0", ub = "18446744073709551615")]
pub struct INTEGER_91(u64);

#[derive(Debug, AperCodec)]
#[asn(type = "INTEGER", lb = "0", ub = "18446744073709551615")]
pub struct INTEGER_92(u64);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct VolumeTimedReport_ItemiE_Extensions_Item {}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct VolumeTimedReport_ItemiE_Extensions(Vec<VolumeTimedReport_ItemiE_Extensions_Item>);

#[derive(Debug, AperCodec)]
#[asn(type = "BITSTRING", sz_extensible = true, sz_lb = "16", sz_ub = "16")]
pub struct BIT_STRING_93(BitVec<Msb0, u8>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct W_AGF_IDchoice_Extensions {}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct WLANMeasConfigNameItemiE_Extensions_Item {}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct WLANMeasConfigNameItemiE_Extensions(Vec<WLANMeasConfigNameItemiE_Extensions_Item>);

#[derive(Debug, AperCodec)]
#[asn(type = "ENUMERATED", extensible = true, lb = "0", ub = "0")]
pub struct ENUMERATED_94(u8);
impl ENUMERATED_94 {
    const TRUE: u8 = 0u8;
}

#[derive(Debug, AperCodec)]
#[asn(type = "ENUMERATED", extensible = true, lb = "0", ub = "0")]
pub struct ENUMERATED_95(u8);
impl ENUMERATED_95 {
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
#[asn(type = "SEQUENCE", extensible = false)]
pub struct WarningAreaListchoice_Extensions {}

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum WriteReplaceWarningRequestprotocolIEs_Itemvalue {
    #[asn(key = 17)]
    ConcurrentWarningMessageInd(ConcurrentWarningMessageInd),
    #[asn(key = 20)]
    DataCodingScheme(DataCodingScheme),
    #[asn(key = 35)]
    MessageIdentifier(MessageIdentifier),
    #[asn(key = 47)]
    NumberOfBroadcastsRequested(NumberOfBroadcastsRequested),
    #[asn(key = 87)]
    RepetitionPeriod(RepetitionPeriod),
    #[asn(key = 95)]
    SerialNumber(SerialNumber),
    #[asn(key = 141)]
    WarningAreaCoordinates(WarningAreaCoordinates),
    #[asn(key = 122)]
    WarningAreaList(WarningAreaList),
    #[asn(key = 123)]
    WarningMessageContents(WarningMessageContents),
    #[asn(key = 124)]
    WarningSecurityInfo(WarningSecurityInfo),
    #[asn(key = 125)]
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
    #[asn(key = 13)]
    BroadcastCompletedAreaList(BroadcastCompletedAreaList),
    #[asn(key = 19)]
    CriticalityDiagnostics(CriticalityDiagnostics),
    #[asn(key = 35)]
    MessageIdentifier(MessageIdentifier),
    #[asn(key = 95)]
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
pub enum XnExtTLA_ItemiE_Extensions_ItemextensionValue {
    #[asn(key = 173)]
    SCTP_TLAs(SCTP_TLAs),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct XnExtTLA_ItemiE_Extensions_Item {
    #[asn(key_field = true)]
    pub id: ProtocolExtensionID,
    pub criticality: Criticality,
    pub extension_value: XnExtTLA_ItemiE_Extensions_ItemextensionValue,
}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct XnExtTLA_ItemiE_Extensions(Vec<XnExtTLA_ItemiE_Extensions_Item>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct XnTNLConfigurationInfoiE_Extensions_Item {}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct XnTNLConfigurationInfoiE_Extensions(Vec<XnTNLConfigurationInfoiE_Extensions_Item>);

fn main() {
    use asn_codecs::aper::*;
    eprintln!("NGAP");

    let ngap_data = hex::decode(
        "0015404a000004001b00084002f898000000000052400f06004d79206c6974746c6520674e420066001f01000000000002f8980001000800800000010002f8390001001881c00013880015400140",
    )
    .unwrap();
    let mut codec_data = AperCodecData::from_slice(&ngap_data);
    let ngap_pdu = NGAP_PDU::decode(&mut codec_data).unwrap();
    eprintln!("ngap_pdu: {:#?}", ngap_pdu);
}
