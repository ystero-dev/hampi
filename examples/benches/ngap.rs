#![allow(dead_code, unreachable_patterns, non_camel_case_types)]
use asn1_codecs_derive::AperCodec;
use bitvec::order::Msb0;
use bitvec::vec::BitVec;

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct AMF_TNLAssociationSetupItem {
    pub amf_tnl_association_address: CPTransportLayerInformation,
    #[asn(optional_idx = 0)]
    pub ie_extensions: Option<AMF_TNLAssociationSetupItemIE_Extensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "32")]
pub struct AMF_TNLAssociationSetupList(Vec<AMF_TNLAssociationSetupItem>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 2)]
pub struct AMF_TNLAssociationToAddItem {
    pub amf_tnl_association_address: CPTransportLayerInformation,
    #[asn(optional_idx = 0)]
    pub tnl_association_usage: Option<TNLAssociationUsage>,
    pub tnl_address_weight_factor: TNLAddressWeightFactor,
    #[asn(optional_idx = 1)]
    pub ie_extensions: Option<AMF_TNLAssociationToAddItemIE_Extensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "32")]
pub struct AMF_TNLAssociationToAddList(Vec<AMF_TNLAssociationToAddItem>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct AMF_TNLAssociationToRemoveItem {
    pub amf_tnl_association_address: CPTransportLayerInformation,
    #[asn(optional_idx = 0)]
    pub ie_extensions: Option<AMF_TNLAssociationToRemoveItemIE_Extensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "32")]
pub struct AMF_TNLAssociationToRemoveList(Vec<AMF_TNLAssociationToRemoveItem>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 3)]
pub struct AMF_TNLAssociationToUpdateItem {
    pub amf_tnl_association_address: CPTransportLayerInformation,
    #[asn(optional_idx = 0)]
    pub tnl_association_usage: Option<TNLAssociationUsage>,
    #[asn(optional_idx = 1)]
    pub tnl_address_weight_factor: Option<TNLAddressWeightFactor>,
    #[asn(optional_idx = 2)]
    pub ie_extensions: Option<AMF_TNLAssociationToUpdateItemIE_Extensions>,
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
    pub protocol_i_es: AMFCPRelocationIndicationProtocolIEs,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true)]
pub struct AMFConfigurationUpdate {
    pub protocol_i_es: AMFConfigurationUpdateProtocolIEs,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true)]
pub struct AMFConfigurationUpdateAcknowledge {
    pub protocol_i_es: AMFConfigurationUpdateAcknowledgeProtocolIEs,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true)]
pub struct AMFConfigurationUpdateFailure {
    pub protocol_i_es: AMFConfigurationUpdateFailureProtocolIEs,
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
    GlobalRANNodeID(GlobalRANNodeID),
    #[asn(key = 1, extended = false)]
    TAI(TAI),
    #[asn(key = 2, extended = false)]
    Choice_Extensions(AMFPagingTargetchoice_Extensions),
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
    pub protocol_i_es: AMFStatusIndicationProtocolIEs,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 2)]
pub struct AdditionalDLUPTNLInformationForHOItem {
    pub additional_dl_ngu_up_tnl_information: UPTransportLayerInformation,
    pub additional_qos_flow_setup_response_list: QosFlowListWithDataForwarding,
    #[asn(optional_idx = 0)]
    pub additional_dl_forwarding_uptnl_information: Option<UPTransportLayerInformation>,
    #[asn(optional_idx = 1)]
    pub ie_extensions: Option<AdditionalDLUPTNLInformationForHOItemIE_Extensions>,
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
    pub ie_extensions: Option<AllocationAndRetentionPriorityIE_Extensions>,
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
    pub plmn_identity: PLMNIdentity,
    pub pni_npn_restricted: ENUMERATED_2,
    pub allowed_cag_list_per_plmn: Allowed_CAG_List_per_PLMN,
    #[asn(optional_idx = 0)]
    pub ie_extensions: Option<Allowed_PNI_NPN_ItemIE_Extensions>,
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
    pub ie_extensions: Option<AllowedNSSAI_ItemIE_Extensions>,
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
    pub ie_extensions: Option<AlternativeQoSParaSetItemIE_Extensions>,
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
    pub ie_extensions: Option<AreaOfInterestIE_Extensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct AreaOfInterestCellItem {
    pub ngran_cgi: NGRAN_CGI,
    #[asn(optional_idx = 0)]
    pub ie_extensions: Option<AreaOfInterestCellItemIE_Extensions>,
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
    pub ie_extensions: Option<AreaOfInterestItemIE_Extensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "64")]
pub struct AreaOfInterestList(Vec<AreaOfInterestItem>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct AreaOfInterestRANNodeItem {
    pub global_ran_node_id: GlobalRANNodeID,
    #[asn(optional_idx = 0)]
    pub ie_extensions: Option<AreaOfInterestRANNodeItemIE_Extensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "64")]
pub struct AreaOfInterestRANNodeList(Vec<AreaOfInterestRANNodeItem>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct AreaOfInterestTAIItem {
    pub tai: TAI,
    #[asn(optional_idx = 0)]
    pub ie_extensions: Option<AreaOfInterestTAIItemIE_Extensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "16")]
pub struct AreaOfInterestTAIList(Vec<AreaOfInterestTAIItem>);

#[derive(Debug, AperCodec)]
#[asn(type = "CHOICE", lb = "0", ub = "4", extensible = false)]
pub enum AreaScopeOfMDT_EUTRA {
    #[asn(key = 0, extended = false)]
    CellBased(CellBasedMDT_EUTRA),
    #[asn(key = 1, extended = false)]
    TABased(TABasedMDT),
    #[asn(key = 2, extended = false)]
    PLMNWide(NULL_3),
    #[asn(key = 3, extended = false)]
    TAIBased(TAIBasedMDT),
    #[asn(key = 4, extended = false)]
    Choice_Extensions(AreaScopeOfMDT_EUTRAchoice_Extensions),
}

#[derive(Debug, AperCodec)]
#[asn(type = "CHOICE", lb = "0", ub = "4", extensible = false)]
pub enum AreaScopeOfMDT_NR {
    #[asn(key = 0, extended = false)]
    CellBased(CellBasedMDT_NR),
    #[asn(key = 1, extended = false)]
    TABased(TABasedMDT),
    #[asn(key = 2, extended = false)]
    PLMNWide(NULL_4),
    #[asn(key = 3, extended = false)]
    TAIBased(TAIBasedMDT),
    #[asn(key = 4, extended = false)]
    Choice_Extensions(AreaScopeOfMDT_NRchoice_Extensions),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 2)]
pub struct AreaScopeOfNeighCellsItem {
    pub nr_frequency_info: NRFrequencyInfo,
    #[asn(optional_idx = 0)]
    pub pci_list_for_mdt: Option<PCIListForMDT>,
    #[asn(optional_idx = 1)]
    pub ie_extensions: Option<AreaScopeOfNeighCellsItemIE_Extensions>,
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
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 2)]
pub struct AssociatedQosFlowItem {
    pub qos_flow_identifier: QosFlowIdentifier,
    #[asn(optional_idx = 0)]
    pub qos_flow_mapping_indication: Option<ENUMERATED_5>,
    #[asn(optional_idx = 1)]
    pub ie_extensions: Option<AssociatedQosFlowItemIE_Extensions>,
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
    pub ie_extensions: Option<BluetoothMeasConfigNameItemIE_Extensions>,
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
#[asn(type = "CHOICE", lb = "0", ub = "6", extensible = false)]
pub enum BroadcastCancelledAreaList {
    #[asn(key = 0, extended = false)]
    CellIDCancelledEUTRA(CellIDCancelledEUTRA),
    #[asn(key = 1, extended = false)]
    TAICancelledEUTRA(TAICancelledEUTRA),
    #[asn(key = 2, extended = false)]
    EmergencyAreaIDCancelledEUTRA(EmergencyAreaIDCancelledEUTRA),
    #[asn(key = 3, extended = false)]
    CellIDCancelledNR(CellIDCancelledNR),
    #[asn(key = 4, extended = false)]
    TAICancelledNR(TAICancelledNR),
    #[asn(key = 5, extended = false)]
    EmergencyAreaIDCancelledNR(EmergencyAreaIDCancelledNR),
    #[asn(key = 6, extended = false)]
    Choice_Extensions(BroadcastCancelledAreaListchoice_Extensions),
}

#[derive(Debug, AperCodec)]
#[asn(type = "CHOICE", lb = "0", ub = "6", extensible = false)]
pub enum BroadcastCompletedAreaList {
    #[asn(key = 0, extended = false)]
    CellIDBroadcastEUTRA(CellIDBroadcastEUTRA),
    #[asn(key = 1, extended = false)]
    TAIBroadcastEUTRA(TAIBroadcastEUTRA),
    #[asn(key = 2, extended = false)]
    EmergencyAreaIDBroadcastEUTRA(EmergencyAreaIDBroadcastEUTRA),
    #[asn(key = 3, extended = false)]
    CellIDBroadcastNR(CellIDBroadcastNR),
    #[asn(key = 4, extended = false)]
    TAIBroadcastNR(TAIBroadcastNR),
    #[asn(key = 5, extended = false)]
    EmergencyAreaIDBroadcastNR(EmergencyAreaIDBroadcastNR),
    #[asn(key = 6, extended = false)]
    Choice_Extensions(BroadcastCompletedAreaListchoice_Extensions),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct BroadcastPLMNItem {
    pub plmn_identity: PLMNIdentity,
    pub tai_slice_support_list: SliceSupportList,
    #[asn(optional_idx = 0)]
    pub ie_extensions: Option<BroadcastPLMNItemIE_Extensions>,
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
    pub ie_extensions: Option<CNAssistedRANTuningIE_Extensions>,
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
    pub ie_extensions: Option<CNTypeRestrictionsForEquivalentItemIE_Extensions>,
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
    pub pdcp_sn12: INTEGER_8,
    pub hfn_pdcp_sn12: INTEGER_9,
    #[asn(optional_idx = 0)]
    pub ie_extensions: Option<COUNTValueForPDCP_SN12IE_Extensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct COUNTValueForPDCP_SN18 {
    pub pdcp_sn18: INTEGER_10,
    pub hfn_pdcp_sn18: INTEGER_11,
    #[asn(optional_idx = 0)]
    pub ie_extensions: Option<COUNTValueForPDCP_SN18IE_Extensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "CHOICE", lb = "0", ub = "1", extensible = false)]
pub enum CPTransportLayerInformation {
    #[asn(key = 0, extended = false)]
    EndpointIPAddress(TransportLayerAddress),
    #[asn(key = 1, extended = false)]
    Choice_Extensions(CPTransportLayerInformationchoice_Extensions),
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
    pub eutra_cgi: EUTRA_CGI,
    pub number_of_broadcasts: NumberOfBroadcasts,
    #[asn(optional_idx = 0)]
    pub ie_extensions: Option<CancelledCellsInEAI_EUTRA_ItemIE_Extensions>,
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
    pub nr_cgi: NR_CGI,
    pub number_of_broadcasts: NumberOfBroadcasts,
    #[asn(optional_idx = 0)]
    pub ie_extensions: Option<CancelledCellsInEAI_NR_ItemIE_Extensions>,
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
    pub eutra_cgi: EUTRA_CGI,
    pub number_of_broadcasts: NumberOfBroadcasts,
    #[asn(optional_idx = 0)]
    pub ie_extensions: Option<CancelledCellsInTAI_EUTRA_ItemIE_Extensions>,
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
    pub nr_cgi: NR_CGI,
    pub number_of_broadcasts: NumberOfBroadcasts,
    #[asn(optional_idx = 0)]
    pub ie_extensions: Option<CancelledCellsInTAI_NR_ItemIE_Extensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "CHOICE", lb = "0", ub = "2", extensible = false)]
pub enum CandidateCell {
    #[asn(key = 0, extended = false)]
    CandidateCGI(CandidateCellID),
    #[asn(key = 1, extended = false)]
    CandidatePCI(CandidatePCI),
    #[asn(key = 2, extended = false)]
    Choice_Extensions(CandidateCellchoice_Extensions),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct CandidateCellID {
    pub candidate_cell_id: NR_CGI,
    #[asn(optional_idx = 0)]
    pub ie_extensions: Option<CandidateCellIDIE_Extensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct CandidateCellItem {
    pub candidate_cell: CandidateCell,
    #[asn(optional_idx = 0)]
    pub ie_extensions: Option<CandidateCellItemIE_Extensions>,
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
    pub ie_extensions: Option<CandidatePCIIE_Extensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "CHOICE", lb = "0", ub = "5", extensible = false)]
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
    #[asn(key = 5, extended = false)]
    Choice_Extensions(Causechoice_Extensions),
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
    pub ngran_cgi: NGRAN_CGI,
    pub cell_cag_list: CellCAGList,
    #[asn(optional_idx = 0)]
    pub ie_extensions: Option<Cell_CAGInformationIE_Extensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct CellBasedMDT_EUTRA {
    pub cell_id_listfor_mdt: CellIdListforMDT_EUTRA,
    #[asn(optional_idx = 0)]
    pub ie_extensions: Option<CellBasedMDT_EUTRAIE_Extensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct CellBasedMDT_NR {
    pub cell_id_listfor_mdt: CellIdListforMDT_NR,
    #[asn(optional_idx = 0)]
    pub ie_extensions: Option<CellBasedMDT_NRIE_Extensions>,
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
    pub eutra_cgi: EUTRA_CGI,
    #[asn(optional_idx = 0)]
    pub ie_extensions: Option<CellIDBroadcastEUTRA_ItemIE_Extensions>,
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
    pub nr_cgi: NR_CGI,
    #[asn(optional_idx = 0)]
    pub ie_extensions: Option<CellIDBroadcastNR_ItemIE_Extensions>,
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
    pub eutra_cgi: EUTRA_CGI,
    pub number_of_broadcasts: NumberOfBroadcasts,
    #[asn(optional_idx = 0)]
    pub ie_extensions: Option<CellIDCancelledEUTRA_ItemIE_Extensions>,
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
    pub nr_cgi: NR_CGI,
    pub number_of_broadcasts: NumberOfBroadcasts,
    #[asn(optional_idx = 0)]
    pub ie_extensions: Option<CellIDCancelledNR_ItemIE_Extensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "CHOICE", lb = "0", ub = "2", extensible = false)]
pub enum CellIDListForRestart {
    #[asn(key = 0, extended = false)]
    EUTRA_CGIListforRestart(EUTRA_CGIList),
    #[asn(key = 1, extended = false)]
    NR_CGIListforRestart(NR_CGIList),
    #[asn(key = 2, extended = false)]
    Choice_Extensions(CellIDListForRestartchoice_Extensions),
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
    pub protocol_i_es: CellTrafficTraceProtocolIEs,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct CellType {
    pub cell_size: CellSize,
    #[asn(optional_idx = 0)]
    pub ie_extensions: Option<CellTypeIE_Extensions>,
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
    pub eutra_cgi: EUTRA_CGI,
    #[asn(optional_idx = 0)]
    pub ie_extensions: Option<CompletedCellsInEAI_EUTRA_ItemIE_Extensions>,
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
    pub nr_cgi: NR_CGI,
    #[asn(optional_idx = 0)]
    pub ie_extensions: Option<CompletedCellsInEAI_NR_ItemIE_Extensions>,
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
    pub eutra_cgi: EUTRA_CGI,
    #[asn(optional_idx = 0)]
    pub ie_extensions: Option<CompletedCellsInTAI_EUTRA_ItemIE_Extensions>,
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
    pub nr_cgi: NR_CGI,
    #[asn(optional_idx = 0)]
    pub ie_extensions: Option<CompletedCellsInTAI_NR_ItemIE_Extensions>,
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
    pub protocol_i_es: ConnectionEstablishmentIndicationProtocolIEs,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 4)]
pub struct CoreNetworkAssistanceInformationForInactive {
    pub ue_identity_index_value: UEIdentityIndexValue,
    #[asn(optional_idx = 0)]
    pub ue_specific_drx: Option<PagingDRX>,
    pub periodic_registration_update_timer: PeriodicRegistrationUpdateTimer,
    #[asn(optional_idx = 1)]
    pub mico_mode_indication: Option<MICOModeIndication>,
    pub tai_list_for_inactive: TAIListForInactive,
    #[asn(optional_idx = 2)]
    pub expected_ue_behaviour: Option<ExpectedUEBehaviour>,
    #[asn(optional_idx = 3)]
    pub ie_extensions: Option<CoreNetworkAssistanceInformationForInactiveIE_Extensions>,
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
    pub daps_indicator: ENUMERATED_14,
    #[asn(optional_idx = 0)]
    pub ie_extensions: Option<DAPSRequestInfoIE_Extensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct DAPSResponseInfo {
    pub dapsresponseindicator: ENUMERATED_15,
    #[asn(optional_idx = 0)]
    pub ie_extensions: Option<DAPSResponseInfoIE_Extensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct DAPSResponseInfoItem {
    pub drb_id: DRB_ID,
    pub daps_response_info: DAPSResponseInfo,
    #[asn(optional_idx = 0)]
    pub ie_extension: Option<DAPSResponseInfoItemIE_Extension>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "32")]
pub struct DAPSResponseInfoList(Vec<DAPSResponseInfoItem>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct DL_CP_SecurityInformation {
    pub dl_nas_mac: DL_NAS_MAC,
    #[asn(optional_idx = 0)]
    pub ie_extensions: Option<DL_CP_SecurityInformationIE_Extensions>,
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
    DRBStatusDL12(DRBStatusDL12),
    #[asn(key = 1, extended = false)]
    DRBStatusDL18(DRBStatusDL18),
    #[asn(key = 2, extended = false)]
    Choice_Extensions(DRBStatusDLchoice_Extensions),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct DRBStatusDL12 {
    pub dl_count_value: COUNTValueForPDCP_SN12,
    #[asn(optional_idx = 0)]
    pub ie_extension: Option<DRBStatusDL12IE_Extension>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct DRBStatusDL18 {
    pub dl_count_value: COUNTValueForPDCP_SN18,
    #[asn(optional_idx = 0)]
    pub ie_extension: Option<DRBStatusDL18IE_Extension>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "CHOICE", lb = "0", ub = "2", extensible = false)]
pub enum DRBStatusUL {
    #[asn(key = 0, extended = false)]
    DRBStatusUL12(DRBStatusUL12),
    #[asn(key = 1, extended = false)]
    DRBStatusUL18(DRBStatusUL18),
    #[asn(key = 2, extended = false)]
    Choice_Extensions(DRBStatusULchoice_Extensions),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 2)]
pub struct DRBStatusUL12 {
    pub ul_count_value: COUNTValueForPDCP_SN12,
    #[asn(optional_idx = 0)]
    pub receive_status_of_ul_pdcp_sd_us: Option<BIT_STRING_16>,
    #[asn(optional_idx = 1)]
    pub ie_extension: Option<DRBStatusUL12IE_Extension>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 2)]
pub struct DRBStatusUL18 {
    pub ul_count_value: COUNTValueForPDCP_SN18,
    #[asn(optional_idx = 0)]
    pub receive_status_of_ul_pdcp_sd_us: Option<BIT_STRING_17>,
    #[asn(optional_idx = 1)]
    pub ie_extension: Option<DRBStatusUL18IE_Extension>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct DRBsSubjectToEarlyStatusTransfer_Item {
    pub drb_id: DRB_ID,
    pub first_dlcount: DRBStatusDL,
    #[asn(optional_idx = 0)]
    pub ie_extension: Option<DRBsSubjectToEarlyStatusTransfer_ItemIE_Extension>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "32")]
pub struct DRBsSubjectToEarlyStatusTransfer_List(Vec<DRBsSubjectToEarlyStatusTransfer_Item>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct DRBsSubjectToStatusTransferItem {
    pub drb_id: DRB_ID,
    pub drb_status_ul: DRBStatusUL,
    pub drb_status_dl: DRBStatusDL,
    #[asn(optional_idx = 0)]
    pub ie_extension: Option<DRBsSubjectToStatusTransferItemIE_Extension>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "32")]
pub struct DRBsSubjectToStatusTransferList(Vec<DRBsSubjectToStatusTransferItem>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct DRBsToQosFlowsMappingItem {
    pub drb_id: DRB_ID,
    pub associated_qos_flow_list: AssociatedQosFlowList,
    #[asn(optional_idx = 0)]
    pub ie_extensions: Option<DRBsToQosFlowsMappingItemIE_Extensions>,
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
    pub drb_id: DRB_ID,
    #[asn(optional_idx = 0)]
    pub dl_forwarding_up_tnl_information: Option<UPTransportLayerInformation>,
    #[asn(optional_idx = 1)]
    pub ul_forwarding_up_tnl_information: Option<UPTransportLayerInformation>,
    #[asn(optional_idx = 2)]
    pub ie_extensions: Option<DataForwardingResponseDRBItemIE_Extensions>,
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
    pub dl_forwarding_up_tnl_information: UPTransportLayerInformation,
    #[asn(optional_idx = 0)]
    pub ie_extensions: Option<DataForwardingResponseERABListItemIE_Extensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true)]
pub struct DeactivateTrace {
    pub protocol_i_es: DeactivateTraceProtocolIEs,
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
    pub protocol_i_es: DownlinkNASTransportProtocolIEs,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true)]
pub struct DownlinkNonUEAssociatedNRPPaTransport {
    pub protocol_i_es: DownlinkNonUEAssociatedNRPPaTransportProtocolIEs,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true)]
pub struct DownlinkRANConfigurationTransfer {
    pub protocol_i_es: DownlinkRANConfigurationTransferProtocolIEs,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true)]
pub struct DownlinkRANEarlyStatusTransfer {
    pub protocol_i_es: DownlinkRANEarlyStatusTransferProtocolIEs,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true)]
pub struct DownlinkRANStatusTransfer {
    pub protocol_i_es: DownlinkRANStatusTransferProtocolIEs,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true)]
pub struct DownlinkRIMInformationTransfer {
    pub protocol_i_es: DownlinkRIMInformationTransferProtocolIEs,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true)]
pub struct DownlinkUEAssociatedNRPPaTransport {
    pub protocol_i_es: DownlinkUEAssociatedNRPPaTransportProtocolIEs,
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
    pub ie_extensions: Option<Dynamic5QIDescriptorIE_Extensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "INTEGER", lb = "0", ub = "15", extensible = true)]
pub struct E_RAB_ID(u8);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 2)]
pub struct E_RABInformationItem {
    pub e_rab_id: E_RAB_ID,
    #[asn(optional_idx = 0)]
    pub dl_forwarding: Option<DLForwarding>,
    #[asn(optional_idx = 1)]
    pub ie_extensions: Option<E_RABInformationItemIE_Extensions>,
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
    MacroENB_ID(BIT_STRING_18),
    #[asn(key = 1, extended = false)]
    HomeENB_ID(BIT_STRING_19),
    #[asn(key = 2, extended = false)]
    Short_macroENB_ID(BIT_STRING_20),
    #[asn(key = 3, extended = false)]
    Long_macroENB_ID(BIT_STRING_21),
    #[asn(key = 4, extended = false)]
    Choice_Extensions(ENB_IDchoice_Extensions),
}

#[derive(Debug, AperCodec)]
#[asn(type = "OCTET-STRING", sz_extensible = false, sz_lb = "2", sz_ub = "2")]
pub struct EPS_TAC(Vec<u8>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct EPS_TAI {
    pub plmn_identity: PLMNIdentity,
    pub eps_tac: EPS_TAC,
    #[asn(optional_idx = 0)]
    pub ie_extensions: Option<EPS_TAIIE_Extensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct EUTRA_CGI {
    pub plmn_identity: PLMNIdentity,
    pub eutra_cell_identity: EUTRACellIdentity,
    #[asn(optional_idx = 0)]
    pub ie_extensions: Option<EUTRA_CGIIE_Extensions>,
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
    pub ie_extensions: Option<EarlyStatusTransfer_TransparentContainerIE_Extensions>,
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
    pub ie_extensions: Option<EmergencyAreaIDBroadcastEUTRA_ItemIE_Extensions>,
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
    pub ie_extensions: Option<EmergencyAreaIDBroadcastNR_ItemIE_Extensions>,
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
    pub ie_extensions: Option<EmergencyAreaIDCancelledEUTRA_ItemIE_Extensions>,
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
    pub ie_extensions: Option<EmergencyAreaIDCancelledNR_ItemIE_Extensions>,
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
    pub ie_extensions: Option<EmergencyFallbackIndicatorIE_Extensions>,
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
    pub ie_extensions: Option<EndpointIPAddressAndPortIE_Extensions>,
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
    pub protocol_i_es: ErrorIndicationProtocolIEs,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct EventL1LoggedMDTConfig {
    pub l1_threshold: MeasurementThresholdL1LoggedMDT,
    pub hysteresis: Hysteresis,
    pub time_to_trigger: TimeToTrigger,
    #[asn(optional_idx = 0)]
    pub ie_extensions: Option<EventL1LoggedMDTConfigIE_Extensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "CHOICE", lb = "0", ub = "2", extensible = false)]
pub enum EventTrigger {
    #[asn(key = 0, extended = false)]
    OutOfCoverage(ENUMERATED_22),
    #[asn(key = 1, extended = false)]
    EventL1LoggedMDTConfig(EventL1LoggedMDTConfig),
    #[asn(key = 2, extended = false)]
    Choice_Extensions(EventTriggerchoice_Extensions),
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
    pub ie_extensions: Option<ExpectedUEActivityBehaviourIE_Extensions>,
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
    pub ie_extensions: Option<ExpectedUEBehaviourIE_Extensions>,
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
    pub ngran_cgi: NGRAN_CGI,
    #[asn(optional_idx = 0)]
    pub time_stayed_in_cell: Option<INTEGER_23>,
    #[asn(optional_idx = 1)]
    pub ie_extensions: Option<ExpectedUEMovingTrajectoryItemIE_Extensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 3)]
pub struct Extended_AMFName {
    #[asn(optional_idx = 0)]
    pub amf_name_visible_string: Option<AMFNameVisibleString>,
    #[asn(optional_idx = 1)]
    pub amf_name_utf8_string: Option<AMFNameUTF8String>,
    #[asn(optional_idx = 2)]
    pub ie_extensions: Option<Extended_AMFNameIE_Extensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "INTEGER", lb = "0", ub = "255")]
pub struct Extended_ConnectedTime(u8);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 3)]
pub struct Extended_RANNodeName {
    #[asn(optional_idx = 0)]
    pub ran_node_name_visible_string: Option<RANNodeNameVisibleString>,
    #[asn(optional_idx = 1)]
    pub ran_node_name_utf8_string: Option<RANNodeNameUTF8String>,
    #[asn(optional_idx = 2)]
    pub ie_extensions: Option<Extended_RANNodeNameIE_Extensions>,
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
    pub ie_extensions: Option<ExtendedRATRestrictionInformationIE_Extensions>,
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
    pub uerlf_report_container: UERLFReportContainer,
    #[asn(optional_idx = 0)]
    pub ie_extensions: Option<FailureIndicationIE_Extensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct FirstDLCount {
    pub dr_bs_subject_to_early_status_transfer: DRBsSubjectToEarlyStatusTransfer_List,
    #[asn(optional_idx = 0)]
    pub ie_extension: Option<FirstDLCountIE_Extension>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct FiveG_S_TMSI {
    pub amf_set_id: AMFSetID,
    pub amf_pointer: AMFPointer,
    pub five_g_tmsi: FiveG_TMSI,
    #[asn(optional_idx = 0)]
    pub ie_extensions: Option<FiveG_S_TMSIIE_Extensions>,
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
    pub plmn_identity: PLMNIdentity,
    pub forbidden_ta_cs: ForbiddenTACs,
    #[asn(optional_idx = 0)]
    pub ie_extensions: Option<ForbiddenAreaInformation_ItemIE_Extensions>,
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
    pub ie_extensions: Option<FromEUTRANtoNGRANIE_Extensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false, optional_fields = 1)]
pub struct FromNGRANtoEUTRAN {
    pub source_ngra_nnode_id: IntersystemSONNGRANnodeID,
    pub targete_nbid: IntersystemSONeNBID,
    #[asn(optional_idx = 0)]
    pub ie_extensions: Option<FromNGRANtoEUTRANIE_Extensions>,
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
    pub ie_extensions: Option<GBR_QosInformationIE_Extensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "CHOICE", lb = "0", ub = "1", extensible = false)]
pub enum GNB_ID {
    #[asn(key = 0, extended = false)]
    GNB_ID(BIT_STRING_26),
    #[asn(key = 1, extended = false)]
    Choice_Extensions(GNB_IDchoice_Extensions),
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
    pub gtp_teid: GTP_TEID,
    #[asn(optional_idx = 0)]
    pub ie_extensions: Option<GTPTunnelIE_Extensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct GUAMI {
    pub plmn_identity: PLMNIdentity,
    pub amf_region_id: AMFRegionID,
    pub amf_set_id: AMFSetID,
    pub amf_pointer: AMFPointer,
    #[asn(optional_idx = 0)]
    pub ie_extensions: Option<GUAMIIE_Extensions>,
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
    pub plm_nidentity: PLMNIdentity,
    pub enb_id: ENB_ID,
    #[asn(optional_idx = 0)]
    pub ie_extensions: Option<GlobalENB_IDIE_Extensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct GlobalGNB_ID {
    pub plmn_identity: PLMNIdentity,
    pub gnb_id: GNB_ID,
    #[asn(optional_idx = 0)]
    pub ie_extensions: Option<GlobalGNB_IDIE_Extensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 2)]
pub struct GlobalLine_ID {
    pub global_line_identity: GlobalLineIdentity,
    #[asn(optional_idx = 0)]
    pub line_type: Option<LineType>,
    #[asn(optional_idx = 1)]
    pub ie_extensions: Option<GlobalLine_IDIE_Extensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "OCTET-STRING")]
pub struct GlobalLineIdentity(Vec<u8>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct GlobalN3IWF_ID {
    pub plmn_identity: PLMNIdentity,
    pub n3iwf_id: N3IWF_ID,
    #[asn(optional_idx = 0)]
    pub ie_extensions: Option<GlobalN3IWF_IDIE_Extensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct GlobalNgENB_ID {
    pub plmn_identity: PLMNIdentity,
    pub ng_enb_id: NgENB_ID,
    #[asn(optional_idx = 0)]
    pub ie_extensions: Option<GlobalNgENB_IDIE_Extensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "CHOICE", lb = "0", ub = "3", extensible = false)]
pub enum GlobalRANNodeID {
    #[asn(key = 0, extended = false)]
    GlobalGNB_ID(GlobalGNB_ID),
    #[asn(key = 1, extended = false)]
    GlobalNgENB_ID(GlobalNgENB_ID),
    #[asn(key = 2, extended = false)]
    GlobalN3IWF_ID(GlobalN3IWF_ID),
    #[asn(key = 3, extended = false)]
    Choice_Extensions(GlobalRANNodeIDchoice_Extensions),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct GlobalTNGF_ID {
    pub plmn_identity: PLMNIdentity,
    pub tngf_id: TNGF_ID,
    #[asn(optional_idx = 0)]
    pub ie_extensions: Option<GlobalTNGF_IDIE_Extensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct GlobalTWIF_ID {
    pub plmn_identity: PLMNIdentity,
    pub twif_id: TWIF_ID,
    #[asn(optional_idx = 0)]
    pub ie_extensions: Option<GlobalTWIF_IDIE_Extensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct GlobalW_AGF_ID {
    pub plmn_identity: PLMNIdentity,
    pub w_agf_id: W_AGF_ID,
    #[asn(optional_idx = 0)]
    pub ie_extensions: Option<GlobalW_AGF_IDIE_Extensions>,
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
    pub uerlf_report_container: Option<UERLFReportContainer>,
    #[asn(optional_idx = 5)]
    pub ie_extensions: Option<HOReportIE_Extensions>,
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
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 4)]
pub struct HandoverCommandTransfer {
    #[asn(optional_idx = 0)]
    pub dl_forwarding_up_tnl_information: Option<UPTransportLayerInformation>,
    #[asn(optional_idx = 1)]
    pub qos_flow_to_be_forwarded_list: Option<QosFlowToBeForwardedList>,
    #[asn(optional_idx = 2)]
    pub data_forwarding_response_drb_list: Option<DataForwardingResponseDRBList>,
    #[asn(optional_idx = 3)]
    pub ie_extensions: Option<HandoverCommandTransferIE_Extensions>,
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
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct HandoverPreparationUnsuccessfulTransfer {
    pub cause: Cause,
    #[asn(optional_idx = 0)]
    pub ie_extensions: Option<HandoverPreparationUnsuccessfulTransferIE_Extensions>,
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
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 5)]
pub struct HandoverRequestAcknowledgeTransfer {
    pub dl_ngu_up_tnl_information: UPTransportLayerInformation,
    #[asn(optional_idx = 0)]
    pub dl_forwarding_up_tnl_information: Option<UPTransportLayerInformation>,
    #[asn(optional_idx = 1)]
    pub security_result: Option<SecurityResult>,
    pub qos_flow_setup_response_list: QosFlowListWithDataForwarding,
    #[asn(optional_idx = 2)]
    pub qos_flow_failed_to_setup_list: Option<QosFlowListWithCause>,
    #[asn(optional_idx = 3)]
    pub data_forwarding_response_drb_list: Option<DataForwardingResponseDRBList>,
    #[asn(optional_idx = 4)]
    pub ie_extensions: Option<HandoverRequestAcknowledgeTransferIE_Extensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true)]
pub struct HandoverRequired {
    pub protocol_i_es: HandoverRequiredProtocolIEs,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 2)]
pub struct HandoverRequiredTransfer {
    #[asn(optional_idx = 0)]
    pub direct_forwarding_path_availability: Option<DirectForwardingPathAvailability>,
    #[asn(optional_idx = 1)]
    pub ie_extensions: Option<HandoverRequiredTransferIE_Extensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 2)]
pub struct HandoverResourceAllocationUnsuccessfulTransfer {
    pub cause: Cause,
    #[asn(optional_idx = 0)]
    pub criticality_diagnostics: Option<CriticalityDiagnostics>,
    #[asn(optional_idx = 1)]
    pub ie_extensions: Option<HandoverResourceAllocationUnsuccessfulTransferIE_Extensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true)]
pub struct HandoverSuccess {
    pub protocol_i_es: HandoverSuccessProtocolIEs,
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
    pub wlan_measurement_configuration: Option<WLANMeasurementConfiguration>,
    #[asn(optional_idx = 7)]
    pub mdt_location_info: Option<MDT_Location_Info>,
    #[asn(optional_idx = 8)]
    pub sensor_measurement_configuration: Option<SensorMeasurementConfiguration>,
    #[asn(optional_idx = 9)]
    pub ie_extensions: Option<ImmediateMDTNrIE_Extensions>,
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
    pub ie_extensions: Option<InfoOnRecommendedCellsAndRANNodesForPagingIE_Extensions>,
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
    pub uerlf_report_container: Option<UERLFReportContainer>,
    #[asn(optional_idx = 1)]
    pub ie_extensions: Option<InterSystemFailureIndicationIE_Extensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct InterSystemHOReport {
    pub handover_report_type: InterSystemHandoverReportType,
    #[asn(optional_idx = 0)]
    pub ie_extensions: Option<InterSystemHOReportIE_Extensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "CHOICE", lb = "0", ub = "2", extensible = false)]
pub enum InterSystemHandoverReportType {
    #[asn(key = 0, extended = false)]
    TooearlyIntersystemHO(TooearlyIntersystemHO),
    #[asn(key = 1, extended = false)]
    IntersystemUnnecessaryHO(IntersystemUnnecessaryHO),
    #[asn(key = 2, extended = false)]
    Choice_Extensions(InterSystemHandoverReportTypechoice_Extensions),
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
    pub ie_extensions: Option<IntersystemSONConfigurationTransferIE_Extensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "CHOICE", lb = "0", ub = "1", extensible = false)]
pub enum IntersystemSONInformation {
    #[asn(key = 0, extended = false)]
    IntersystemSONInformationReport(IntersystemSONInformationReport),
    #[asn(key = 1, extended = false)]
    Choice_Extensions(IntersystemSONInformationchoice_Extensions),
}

#[derive(Debug, AperCodec)]
#[asn(type = "CHOICE", lb = "0", ub = "2", extensible = false)]
pub enum IntersystemSONInformationReport {
    #[asn(key = 0, extended = false)]
    HOReportInformation(InterSystemHOReport),
    #[asn(key = 1, extended = false)]
    FailureIndicationInformation(InterSystemFailureIndication),
    #[asn(key = 2, extended = false)]
    Choice_Extensions(IntersystemSONInformationReportchoice_Extensions),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct IntersystemSONNGRANnodeID {
    pub global_ran_node_id: GlobalRANNodeID,
    pub selected_tai: TAI,
    #[asn(optional_idx = 0)]
    pub ie_extensions: Option<IntersystemSONNGRANnodeIDIE_Extensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "CHOICE", lb = "0", ub = "2", extensible = false)]
pub enum IntersystemSONTransferType {
    #[asn(key = 0, extended = false)]
    FromEUTRANtoNGRAN(FromEUTRANtoNGRAN),
    #[asn(key = 1, extended = false)]
    FromNGRANtoEUTRAN(FromNGRANtoEUTRAN),
    #[asn(key = 2, extended = false)]
    Choice_Extensions(IntersystemSONTransferTypechoice_Extensions),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct IntersystemSONeNBID {
    pub globale_nbid: GlobalENB_ID,
    pub selected_epstai: EPS_TAI,
    #[asn(optional_idx = 0)]
    pub ie_extensions: Option<IntersystemSONeNBIDIE_Extensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct IntersystemUnnecessaryHO {
    pub sourcecell_id: NGRAN_CGI,
    pub targetcell_id: EUTRA_CGI,
    pub early_iratho: ENUMERATED_29,
    pub candidate_cell_list: CandidateCellList,
    #[asn(optional_idx = 0)]
    pub ie_extensions: Option<IntersystemUnnecessaryHOIE_Extensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "OCTET-STRING", sz_extensible = false, sz_lb = "2", sz_ub = "2")]
pub struct LAC(Vec<u8>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct LAI {
    pub plm_nidentity: PLMNIdentity,
    pub lac: LAC,
    #[asn(optional_idx = 0)]
    pub ie_extensions: Option<LAIIE_Extensions>,
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
    pub ue_sidelink_aggregate_maximum_bit_rate: BitRate,
    #[asn(optional_idx = 0)]
    pub ie_extensions: Option<LTEUESidelinkAggregateMaximumBitrateIE_Extensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 3)]
pub struct LTEV2XServicesAuthorized {
    #[asn(optional_idx = 0)]
    pub vehicle_ue: Option<VehicleUE>,
    #[asn(optional_idx = 1)]
    pub pedestrian_ue: Option<PedestrianUE>,
    #[asn(optional_idx = 2)]
    pub ie_extensions: Option<LTEV2XServicesAuthorizedIE_Extensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "CHOICE", lb = "0", ub = "4", extensible = false)]
pub enum LastVisitedCellInformation {
    #[asn(key = 0, extended = false)]
    NGRANCell(LastVisitedNGRANCellInformation),
    #[asn(key = 1, extended = false)]
    EUTRANCell(LastVisitedEUTRANCellInformation),
    #[asn(key = 2, extended = false)]
    UTRANCell(LastVisitedUTRANCellInformation),
    #[asn(key = 3, extended = false)]
    GERANCell(LastVisitedGERANCellInformation),
    #[asn(key = 4, extended = false)]
    Choice_Extensions(LastVisitedCellInformationchoice_Extensions),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct LastVisitedCellItem {
    pub last_visited_cell_information: LastVisitedCellInformation,
    #[asn(optional_idx = 0)]
    pub ie_extensions: Option<LastVisitedCellItemIE_Extensions>,
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
    pub ho_cause_value: Option<Cause>,
    #[asn(optional_idx = 2)]
    pub ie_extensions: Option<LastVisitedNGRANCellInformationIE_Extensions>,
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
    pub protocol_i_es: LocationReportProtocolIEs,
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
    pub protocol_i_es: LocationReportingControlProtocolIEs,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true)]
pub struct LocationReportingFailureIndication {
    pub protocol_i_es: LocationReportingFailureIndicationProtocolIEs,
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
    pub ie_extensions: Option<LocationReportingRequestTypeIE_Extensions>,
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
    pub wlan_measurement_configuration: Option<WLANMeasurementConfiguration>,
    #[asn(optional_idx = 2)]
    pub sensor_measurement_configuration: Option<SensorMeasurementConfiguration>,
    #[asn(optional_idx = 3)]
    pub area_scope_of_neigh_cells_list: Option<AreaScopeOfNeighCellsList>,
    #[asn(optional_idx = 4)]
    pub ie_extensions: Option<LoggedMDTNrIE_Extensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "CHOICE", lb = "0", ub = "2", extensible = false)]
pub enum LoggedMDTTrigger {
    #[asn(key = 0, extended = false)]
    Periodical(NULL_30),
    #[asn(key = 1, extended = false)]
    EventTrigger(EventTrigger),
    #[asn(key = 2, extended = false)]
    Choice_Extensions(LoggedMDTTriggerchoice_Extensions),
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
    pub ie_extensions: Option<M1ConfigurationIE_Extensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct M1PeriodicReporting {
    pub report_interval: ReportIntervalMDT,
    pub report_amount: ReportAmountMDT,
    #[asn(optional_idx = 0)]
    pub ie_extensions: Option<M1PeriodicReportingIE_Extensions>,
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
    pub ie_extensions: Option<M1ThresholdEventA2IE_Extensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "CHOICE", lb = "0", ub = "3", extensible = false)]
pub enum M1ThresholdType {
    #[asn(key = 0, extended = false)]
    Threshold_RSRP(Threshold_RSRP),
    #[asn(key = 1, extended = false)]
    Threshold_RSRQ(Threshold_RSRQ),
    #[asn(key = 2, extended = false)]
    Threshold_SINR(Threshold_SINR),
    #[asn(key = 3, extended = false)]
    Choice_Extensions(M1ThresholdTypechoice_Extensions),
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
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct M6Configuration {
    pub m6report_interval: M6report_Interval,
    pub m6_links_to_log: Links_to_log,
    #[asn(optional_idx = 0)]
    pub ie_extensions: Option<M6ConfigurationIE_Extensions>,
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
    pub ie_extensions: Option<M7ConfigurationIE_Extensions>,
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
    pub ie_extensions: Option<MDT_ConfigurationIE_Extensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 2)]
pub struct MDT_Configuration_EUTRA {
    pub mdt_activation: MDT_Activation,
    pub area_scope_of_mdt: AreaScopeOfMDT_EUTRA,
    pub mdt_mode: MDTModeEutra,
    #[asn(optional_idx = 0)]
    pub signalling_based_mdtplmn_list: Option<MDTPLMNList>,
    #[asn(optional_idx = 1)]
    pub ie_extensions: Option<MDT_Configuration_EUTRAIE_Extensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 2)]
pub struct MDT_Configuration_NR {
    pub mdt_activation: MDT_Activation,
    pub area_scope_of_mdt: AreaScopeOfMDT_NR,
    pub mdt_mode_nr: MDTModeNr,
    #[asn(optional_idx = 0)]
    pub signalling_based_mdtplmn_list: Option<MDTPLMNList>,
    #[asn(optional_idx = 1)]
    pub ie_extensions: Option<MDT_Configuration_NRIE_Extensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct MDT_Location_Info {
    pub mdt_location_information: MDT_Location_Information,
    #[asn(optional_idx = 0)]
    pub ie_extensions: Option<MDT_Location_InfoIE_Extensions>,
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
    ImmediateMDTNr(ImmediateMDTNr),
    #[asn(key = 1, extended = false)]
    LoggedMDTNr(LoggedMDTNr),
    #[asn(key = 2, extended = false)]
    Choice_Extensions(MDTModeNrchoice_Extensions),
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
    Threshold_RSRP(Threshold_RSRP),
    #[asn(key = 1, extended = false)]
    Threshold_RSRQ(Threshold_RSRQ),
    #[asn(key = 2, extended = false)]
    Choice_Extensions(MeasurementThresholdL1LoggedMDTchoice_Extensions),
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
    pub rat_restrictions: Option<RATRestrictions>,
    #[asn(optional_idx = 2)]
    pub forbidden_area_information: Option<ForbiddenAreaInformation>,
    #[asn(optional_idx = 3)]
    pub service_area_information: Option<ServiceAreaInformation>,
    #[asn(optional_idx = 4)]
    pub ie_extensions: Option<MobilityRestrictionListIE_Extensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "CHOICE", lb = "0", ub = "1", extensible = false)]
pub enum N3IWF_ID {
    #[asn(key = 0, extended = false)]
    N3IWF_ID(BIT_STRING_31),
    #[asn(key = 1, extended = false)]
    Choice_Extensions(N3IWF_IDchoice_Extensions),
}

#[derive(Debug, AperCodec)]
#[asn(type = "OCTET-STRING")]
pub struct NAS_PDU(Vec<u8>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true)]
pub struct NASNonDeliveryIndication {
    pub protocol_i_es: NASNonDeliveryIndicationProtocolIEs,
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
    pub nb_io_t_paging_e_drx_cycle: NB_IoT_Paging_eDRXCycle,
    #[asn(optional_idx = 0)]
    pub nb_io_t_paging_time_window: Option<NB_IoT_Paging_TimeWindow>,
    #[asn(optional_idx = 1)]
    pub ie_extensions: Option<NB_IoT_Paging_eDRXInfoIE_Extensions>,
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
    InitiatingMessage(InitiatingMessage),
    #[asn(key = 1, extended = false)]
    SuccessfulOutcome(SuccessfulOutcome),
    #[asn(key = 2, extended = false)]
    UnsuccessfulOutcome(UnsuccessfulOutcome),
}

#[derive(Debug, AperCodec)]
#[asn(type = "CHOICE", lb = "0", ub = "2", extensible = false)]
pub enum NGRAN_CGI {
    #[asn(key = 0, extended = false)]
    NR_CGI(NR_CGI),
    #[asn(key = 1, extended = false)]
    EUTRA_CGI(EUTRA_CGI),
    #[asn(key = 2, extended = false)]
    Choice_Extensions(NGRAN_CGIchoice_Extensions),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false, optional_fields = 2)]
pub struct NGRAN_TNLAssociationToRemoveItem {
    pub tnl_association_transport_layer_address: CPTransportLayerInformation,
    #[asn(optional_idx = 0)]
    pub tnl_association_transport_layer_address_amf: Option<CPTransportLayerInformation>,
    #[asn(optional_idx = 1)]
    pub ie_extensions: Option<NGRAN_TNLAssociationToRemoveItemIE_Extensions>,
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
    pub protocol_i_es: NGResetProtocolIEs,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true)]
pub struct NGResetAcknowledge {
    pub protocol_i_es: NGResetAcknowledgeProtocolIEs,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true)]
pub struct NGSetupFailure {
    pub protocol_i_es: NGSetupFailureProtocolIEs,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true)]
pub struct NGSetupRequest {
    pub protocol_i_es: NGSetupRequestProtocolIEs,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true)]
pub struct NGSetupResponse {
    pub protocol_i_es: NGSetupResponseProtocolIEs,
}

#[derive(Debug, AperCodec)]
#[asn(type = "BITSTRING", sz_extensible = false, sz_lb = "44", sz_ub = "44")]
pub struct NID(BitVec<Msb0, u8>);

#[derive(Debug, AperCodec)]
#[asn(type = "CHOICE", lb = "0", ub = "1", extensible = false)]
pub enum NPN_AccessInformation {
    #[asn(key = 0, extended = false)]
    PNI_NPN_Access_Information(CellCAGList),
    #[asn(key = 1, extended = false)]
    Choice_Extensions(NPN_AccessInformationchoice_Extensions),
}

#[derive(Debug, AperCodec)]
#[asn(type = "CHOICE", lb = "0", ub = "2", extensible = false)]
pub enum NPN_MobilityInformation {
    #[asn(key = 0, extended = false)]
    SNPN_MobilityInformation(SNPN_MobilityInformation),
    #[asn(key = 1, extended = false)]
    PNI_NPN_MobilityInformation(PNI_NPN_MobilityInformation),
    #[asn(key = 2, extended = false)]
    Choice_Extensions(NPN_MobilityInformationchoice_Extensions),
}

#[derive(Debug, AperCodec)]
#[asn(type = "CHOICE", lb = "0", ub = "1", extensible = false)]
pub enum NPN_PagingAssistanceInformation {
    #[asn(key = 0, extended = false)]
    PNI_NPN_PagingAssistance(Allowed_PNI_NPN_List),
    #[asn(key = 1, extended = false)]
    Choice_Extensions(NPN_PagingAssistanceInformationchoice_Extensions),
}

#[derive(Debug, AperCodec)]
#[asn(type = "CHOICE", lb = "0", ub = "1", extensible = false)]
pub enum NPN_Support {
    #[asn(key = 0, extended = false)]
    SNPN(NID),
    #[asn(key = 1, extended = false)]
    Choice_Extensions(NPN_Supportchoice_Extensions),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct NR_CGI {
    pub plmn_identity: PLMNIdentity,
    pub nr_cell_identity: NRCellIdentity,
    #[asn(optional_idx = 0)]
    pub ie_extensions: Option<NR_CGIIE_Extensions>,
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
    pub ie_extension: Option<NRFrequencyBandItemIE_Extension>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct NRFrequencyInfo {
    pub nr_arfcn: NRARFCN,
    pub frequency_band_list: NRFrequencyBand_List,
    #[asn(optional_idx = 0)]
    pub ie_extension: Option<NRFrequencyInfoIE_Extension>,
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
    pub ue_sidelink_aggregate_maximum_bit_rate: BitRate,
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
    MacroNgENB_ID(BIT_STRING_32),
    #[asn(key = 1, extended = false)]
    ShortMacroNgENB_ID(BIT_STRING_33),
    #[asn(key = 2, extended = false)]
    LongMacroNgENB_ID(BIT_STRING_34),
    #[asn(key = 3, extended = false)]
    Choice_Extensions(NgENB_IDchoice_Extensions),
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
    pub ie_extensions: Option<NonDynamic5QIDescriptorIE_Extensions>,
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
    OverloadAction(OverloadAction),
    #[asn(key = 1, extended = false)]
    Choice_Extensions(OverloadResponsechoice_Extensions),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true)]
pub struct OverloadStart {
    pub protocol_i_es: OverloadStartProtocolIEs,
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
    pub ie_extensions: Option<OverloadStartNSSAIItemIE_Extensions>,
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
    pub pc5_link_aggregate_bit_rates: Option<BitRate>,
    #[asn(optional_idx = 1)]
    pub ie_extensions: Option<PC5QoSParametersIE_Extensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "32")]
pub struct PCIListForMDT(Vec<NR_PCI>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct PDUSessionAggregateMaximumBitRate {
    pub pdu_session_aggregate_maximum_bit_rate_dl: BitRate,
    pub pdu_session_aggregate_maximum_bit_rate_ul: BitRate,
    #[asn(optional_idx = 0)]
    pub ie_extensions: Option<PDUSessionAggregateMaximumBitRateIE_Extensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "INTEGER", lb = "0", ub = "255")]
pub struct PDUSessionID(u8);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct PDUSessionResourceAdmittedItem {
    pub pdu_session_id: PDUSessionID,
    pub handover_request_acknowledge_transfer: OCTET_STRING_35,
    #[asn(optional_idx = 0)]
    pub ie_extensions: Option<PDUSessionResourceAdmittedItemIE_Extensions>,
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
    pub pdu_session_id: PDUSessionID,
    pub pdu_session_resource_modify_indication_unsuccessful_transfer: OCTET_STRING_36,
    #[asn(optional_idx = 0)]
    pub ie_extensions: Option<PDUSessionResourceFailedToModifyItemModCfmIE_Extensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct PDUSessionResourceFailedToModifyItemModRes {
    pub pdu_session_id: PDUSessionID,
    pub pdu_session_resource_modify_unsuccessful_transfer: OCTET_STRING_37,
    #[asn(optional_idx = 0)]
    pub ie_extensions: Option<PDUSessionResourceFailedToModifyItemModResIE_Extensions>,
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
    pub pdu_session_id: PDUSessionID,
    pub cause: Cause,
    #[asn(optional_idx = 0)]
    pub ie_extensions: Option<PDUSessionResourceFailedToResumeItemRESReqIE_Extensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct PDUSessionResourceFailedToResumeItemRESRes {
    pub pdu_session_id: PDUSessionID,
    pub cause: Cause,
    #[asn(optional_idx = 0)]
    pub ie_extensions: Option<PDUSessionResourceFailedToResumeItemRESResIE_Extensions>,
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
    pub pdu_session_id: PDUSessionID,
    pub pdu_session_resource_setup_unsuccessful_transfer: OCTET_STRING_38,
    #[asn(optional_idx = 0)]
    pub ie_extensions: Option<PDUSessionResourceFailedToSetupItemCxtFailIE_Extensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct PDUSessionResourceFailedToSetupItemCxtRes {
    pub pdu_session_id: PDUSessionID,
    pub pdu_session_resource_setup_unsuccessful_transfer: OCTET_STRING_39,
    #[asn(optional_idx = 0)]
    pub ie_extensions: Option<PDUSessionResourceFailedToSetupItemCxtResIE_Extensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct PDUSessionResourceFailedToSetupItemHOAck {
    pub pdu_session_id: PDUSessionID,
    pub handover_resource_allocation_unsuccessful_transfer: OCTET_STRING_40,
    #[asn(optional_idx = 0)]
    pub ie_extensions: Option<PDUSessionResourceFailedToSetupItemHOAckIE_Extensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct PDUSessionResourceFailedToSetupItemPSReq {
    pub pdu_session_id: PDUSessionID,
    pub path_switch_request_setup_failed_transfer: OCTET_STRING_41,
    #[asn(optional_idx = 0)]
    pub ie_extensions: Option<PDUSessionResourceFailedToSetupItemPSReqIE_Extensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct PDUSessionResourceFailedToSetupItemSURes {
    pub pdu_session_id: PDUSessionID,
    pub pdu_session_resource_setup_unsuccessful_transfer: OCTET_STRING_42,
    #[asn(optional_idx = 0)]
    pub ie_extensions: Option<PDUSessionResourceFailedToSetupItemSUResIE_Extensions>,
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
    pub pdu_session_id: PDUSessionID,
    pub handover_command_transfer: OCTET_STRING_43,
    #[asn(optional_idx = 0)]
    pub ie_extensions: Option<PDUSessionResourceHandoverItemIE_Extensions>,
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
    pub pdu_session_id: PDUSessionID,
    pub qos_flow_information_list: QosFlowInformationList,
    #[asn(optional_idx = 0)]
    pub dr_bs_to_qos_flows_mapping_list: Option<DRBsToQosFlowsMappingList>,
    #[asn(optional_idx = 1)]
    pub ie_extensions: Option<PDUSessionResourceInformationItemIE_Extensions>,
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
    pub pdu_session_id: PDUSessionID,
    #[asn(optional_idx = 0)]
    pub ie_extensions: Option<PDUSessionResourceItemCxtRelCplIE_Extensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct PDUSessionResourceItemCxtRelReq {
    pub pdu_session_id: PDUSessionID,
    #[asn(optional_idx = 0)]
    pub ie_extensions: Option<PDUSessionResourceItemCxtRelReqIE_Extensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct PDUSessionResourceItemHORqd {
    pub pdu_session_id: PDUSessionID,
    pub handover_required_transfer: OCTET_STRING_44,
    #[asn(optional_idx = 0)]
    pub ie_extensions: Option<PDUSessionResourceItemHORqdIE_Extensions>,
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
    pub protocol_i_es: PDUSessionResourceModifyConfirmProtocolIEs,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 3)]
pub struct PDUSessionResourceModifyConfirmTransfer {
    pub qos_flow_modify_confirm_list: QosFlowModifyConfirmList,
    pub ulngu_up_tnl_information: UPTransportLayerInformation,
    #[asn(optional_idx = 0)]
    pub additional_ng_uuptnl_information: Option<UPTransportLayerInformationPairList>,
    #[asn(optional_idx = 1)]
    pub qos_flow_failed_to_modify_list: Option<QosFlowListWithCause>,
    #[asn(optional_idx = 2)]
    pub ie_extensions: Option<PDUSessionResourceModifyConfirmTransferIE_Extensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true)]
pub struct PDUSessionResourceModifyIndication {
    pub protocol_i_es: PDUSessionResourceModifyIndicationProtocolIEs,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 2)]
pub struct PDUSessionResourceModifyIndicationTransfer {
    pub dl_qos_flow_per_tnl_information: QosFlowPerTNLInformation,
    #[asn(optional_idx = 0)]
    pub additional_dl_qos_flow_per_tnl_information: Option<QosFlowPerTNLInformationList>,
    #[asn(optional_idx = 1)]
    pub ie_extensions: Option<PDUSessionResourceModifyIndicationTransferIE_Extensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct PDUSessionResourceModifyIndicationUnsuccessfulTransfer {
    pub cause: Cause,
    #[asn(optional_idx = 0)]
    pub ie_extensions: Option<PDUSessionResourceModifyIndicationUnsuccessfulTransferIE_Extensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct PDUSessionResourceModifyItemModCfm {
    pub pdu_session_id: PDUSessionID,
    pub pdu_session_resource_modify_confirm_transfer: OCTET_STRING_45,
    #[asn(optional_idx = 0)]
    pub ie_extensions: Option<PDUSessionResourceModifyItemModCfmIE_Extensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct PDUSessionResourceModifyItemModInd {
    pub pdu_session_id: PDUSessionID,
    pub pdu_session_resource_modify_indication_transfer: OCTET_STRING_46,
    #[asn(optional_idx = 0)]
    pub ie_extensions: Option<PDUSessionResourceModifyItemModIndIE_Extensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 2)]
pub struct PDUSessionResourceModifyItemModReq {
    pub pdu_session_id: PDUSessionID,
    #[asn(optional_idx = 0)]
    pub nas_pdu: Option<NAS_PDU>,
    pub pdu_session_resource_modify_request_transfer: OCTET_STRING_47,
    #[asn(optional_idx = 1)]
    pub ie_extensions: Option<PDUSessionResourceModifyItemModReqIE_Extensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct PDUSessionResourceModifyItemModRes {
    pub pdu_session_id: PDUSessionID,
    pub pdu_session_resource_modify_response_transfer: OCTET_STRING_48,
    #[asn(optional_idx = 0)]
    pub ie_extensions: Option<PDUSessionResourceModifyItemModResIE_Extensions>,
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
    pub protocol_i_es: PDUSessionResourceModifyRequestProtocolIEs,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true)]
pub struct PDUSessionResourceModifyRequestTransfer {
    pub protocol_i_es: PDUSessionResourceModifyRequestTransferProtocolIEs,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true)]
pub struct PDUSessionResourceModifyResponse {
    pub protocol_i_es: PDUSessionResourceModifyResponseProtocolIEs,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 6)]
pub struct PDUSessionResourceModifyResponseTransfer {
    #[asn(optional_idx = 0)]
    pub dl_ngu_up_tnl_information: Option<UPTransportLayerInformation>,
    #[asn(optional_idx = 1)]
    pub ul_ngu_up_tnl_information: Option<UPTransportLayerInformation>,
    #[asn(optional_idx = 2)]
    pub qos_flow_add_or_modify_response_list: Option<QosFlowAddOrModifyResponseList>,
    #[asn(optional_idx = 3)]
    pub additional_dl_qos_flow_per_tnl_information: Option<QosFlowPerTNLInformationList>,
    #[asn(optional_idx = 4)]
    pub qos_flow_failed_to_add_or_modify_list: Option<QosFlowListWithCause>,
    #[asn(optional_idx = 5)]
    pub ie_extensions: Option<PDUSessionResourceModifyResponseTransferIE_Extensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 2)]
pub struct PDUSessionResourceModifyUnsuccessfulTransfer {
    pub cause: Cause,
    #[asn(optional_idx = 0)]
    pub criticality_diagnostics: Option<CriticalityDiagnostics>,
    #[asn(optional_idx = 1)]
    pub ie_extensions: Option<PDUSessionResourceModifyUnsuccessfulTransferIE_Extensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true)]
pub struct PDUSessionResourceNotify {
    pub protocol_i_es: PDUSessionResourceNotifyProtocolIEs,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct PDUSessionResourceNotifyItem {
    pub pdu_session_id: PDUSessionID,
    pub pdu_session_resource_notify_transfer: OCTET_STRING_49,
    #[asn(optional_idx = 0)]
    pub ie_extensions: Option<PDUSessionResourceNotifyItemIE_Extensions>,
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
    pub ie_extensions: Option<PDUSessionResourceNotifyReleasedTransferIE_Extensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 3)]
pub struct PDUSessionResourceNotifyTransfer {
    #[asn(optional_idx = 0)]
    pub qos_flow_notify_list: Option<QosFlowNotifyList>,
    #[asn(optional_idx = 1)]
    pub qos_flow_released_list: Option<QosFlowListWithCause>,
    #[asn(optional_idx = 2)]
    pub ie_extensions: Option<PDUSessionResourceNotifyTransferIE_Extensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true)]
pub struct PDUSessionResourceReleaseCommand {
    pub protocol_i_es: PDUSessionResourceReleaseCommandProtocolIEs,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct PDUSessionResourceReleaseCommandTransfer {
    pub cause: Cause,
    #[asn(optional_idx = 0)]
    pub ie_extensions: Option<PDUSessionResourceReleaseCommandTransferIE_Extensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true)]
pub struct PDUSessionResourceReleaseResponse {
    pub protocol_i_es: PDUSessionResourceReleaseResponseProtocolIEs,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct PDUSessionResourceReleaseResponseTransfer {
    #[asn(optional_idx = 0)]
    pub ie_extensions: Option<PDUSessionResourceReleaseResponseTransferIE_Extensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct PDUSessionResourceReleasedItemNot {
    pub pdu_session_id: PDUSessionID,
    pub pdu_session_resource_notify_released_transfer: OCTET_STRING_50,
    #[asn(optional_idx = 0)]
    pub ie_extensions: Option<PDUSessionResourceReleasedItemNotIE_Extensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct PDUSessionResourceReleasedItemPSAck {
    pub pdu_session_id: PDUSessionID,
    pub path_switch_request_unsuccessful_transfer: OCTET_STRING_51,
    #[asn(optional_idx = 0)]
    pub ie_extensions: Option<PDUSessionResourceReleasedItemPSAckIE_Extensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct PDUSessionResourceReleasedItemPSFail {
    pub pdu_session_id: PDUSessionID,
    pub path_switch_request_unsuccessful_transfer: OCTET_STRING_52,
    #[asn(optional_idx = 0)]
    pub ie_extensions: Option<PDUSessionResourceReleasedItemPSFailIE_Extensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct PDUSessionResourceReleasedItemRelRes {
    pub pdu_session_id: PDUSessionID,
    pub pdu_session_resource_release_response_transfer: OCTET_STRING_53,
    #[asn(optional_idx = 0)]
    pub ie_extensions: Option<PDUSessionResourceReleasedItemRelResIE_Extensions>,
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
    pub pdu_session_id: PDUSessionID,
    pub ue_context_resume_request_transfer: OCTET_STRING_54,
    #[asn(optional_idx = 0)]
    pub ie_extensions: Option<PDUSessionResourceResumeItemRESReqIE_Extensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct PDUSessionResourceResumeItemRESRes {
    pub pdu_session_id: PDUSessionID,
    pub ue_context_resume_response_transfer: OCTET_STRING_55,
    #[asn(optional_idx = 0)]
    pub ie_extensions: Option<PDUSessionResourceResumeItemRESResIE_Extensions>,
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
    pub pdu_session_id: PDUSessionID,
    pub secondary_rat_data_usage_report_transfer: OCTET_STRING_56,
    #[asn(optional_idx = 0)]
    pub ie_extensions: Option<PDUSessionResourceSecondaryRATUsageItemIE_Extensions>,
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
    pub pdu_session_id: PDUSessionID,
    #[asn(optional_idx = 0)]
    pub nas_pdu: Option<NAS_PDU>,
    pub s_nssai: S_NSSAI,
    pub pdu_session_resource_setup_request_transfer: OCTET_STRING_57,
    #[asn(optional_idx = 1)]
    pub ie_extensions: Option<PDUSessionResourceSetupItemCxtReqIE_Extensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct PDUSessionResourceSetupItemCxtRes {
    pub pdu_session_id: PDUSessionID,
    pub pdu_session_resource_setup_response_transfer: OCTET_STRING_58,
    #[asn(optional_idx = 0)]
    pub ie_extensions: Option<PDUSessionResourceSetupItemCxtResIE_Extensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct PDUSessionResourceSetupItemHOReq {
    pub pdu_session_id: PDUSessionID,
    pub s_nssai: S_NSSAI,
    pub handover_request_transfer: OCTET_STRING_59,
    #[asn(optional_idx = 0)]
    pub ie_extensions: Option<PDUSessionResourceSetupItemHOReqIE_Extensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 2)]
pub struct PDUSessionResourceSetupItemSUReq {
    pub pdu_session_id: PDUSessionID,
    #[asn(optional_idx = 0)]
    pub pdu_session_nas_pdu: Option<NAS_PDU>,
    pub s_nssai: S_NSSAI,
    pub pdu_session_resource_setup_request_transfer: OCTET_STRING_60,
    #[asn(optional_idx = 1)]
    pub ie_extensions: Option<PDUSessionResourceSetupItemSUReqIE_Extensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct PDUSessionResourceSetupItemSURes {
    pub pdu_session_id: PDUSessionID,
    pub pdu_session_resource_setup_response_transfer: OCTET_STRING_61,
    #[asn(optional_idx = 0)]
    pub ie_extensions: Option<PDUSessionResourceSetupItemSUResIE_Extensions>,
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
    pub protocol_i_es: PDUSessionResourceSetupRequestProtocolIEs,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true)]
pub struct PDUSessionResourceSetupRequestTransfer {
    pub protocol_i_es: PDUSessionResourceSetupRequestTransferProtocolIEs,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true)]
pub struct PDUSessionResourceSetupResponse {
    pub protocol_i_es: PDUSessionResourceSetupResponseProtocolIEs,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 4)]
pub struct PDUSessionResourceSetupResponseTransfer {
    pub dl_qos_flow_per_tnl_information: QosFlowPerTNLInformation,
    #[asn(optional_idx = 0)]
    pub additional_dl_qos_flow_per_tnl_information: Option<QosFlowPerTNLInformationList>,
    #[asn(optional_idx = 1)]
    pub security_result: Option<SecurityResult>,
    #[asn(optional_idx = 2)]
    pub qos_flow_failed_to_setup_list: Option<QosFlowListWithCause>,
    #[asn(optional_idx = 3)]
    pub ie_extensions: Option<PDUSessionResourceSetupResponseTransferIE_Extensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 2)]
pub struct PDUSessionResourceSetupUnsuccessfulTransfer {
    pub cause: Cause,
    #[asn(optional_idx = 0)]
    pub criticality_diagnostics: Option<CriticalityDiagnostics>,
    #[asn(optional_idx = 1)]
    pub ie_extensions: Option<PDUSessionResourceSetupUnsuccessfulTransferIE_Extensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct PDUSessionResourceSuspendItemSUSReq {
    pub pdu_session_id: PDUSessionID,
    pub ue_context_suspend_request_transfer: OCTET_STRING_62,
    #[asn(optional_idx = 0)]
    pub ie_extensions: Option<PDUSessionResourceSuspendItemSUSReqIE_Extensions>,
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
    pub pdu_session_id: PDUSessionID,
    pub path_switch_request_acknowledge_transfer: OCTET_STRING_63,
    #[asn(optional_idx = 0)]
    pub ie_extensions: Option<PDUSessionResourceSwitchedItemIE_Extensions>,
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
    pub pdu_session_id: PDUSessionID,
    pub path_switch_request_transfer: OCTET_STRING_64,
    #[asn(optional_idx = 0)]
    pub ie_extensions: Option<PDUSessionResourceToBeSwitchedDLItemIE_Extensions>,
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
    pub pdu_session_id: PDUSessionID,
    pub handover_preparation_unsuccessful_transfer: OCTET_STRING_65,
    #[asn(optional_idx = 0)]
    pub ie_extensions: Option<PDUSessionResourceToReleaseItemHOCmdIE_Extensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct PDUSessionResourceToReleaseItemRelCmd {
    pub pdu_session_id: PDUSessionID,
    pub pdu_session_resource_release_command_transfer: OCTET_STRING_66,
    #[asn(optional_idx = 0)]
    pub ie_extensions: Option<PDUSessionResourceToReleaseItemRelCmdIE_Extensions>,
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
    pub rat_type: ENUMERATED_67,
    pub pdu_session_timed_report_list: VolumeTimedReportList,
    #[asn(optional_idx = 0)]
    pub ie_extensions: Option<PDUSessionUsageReportIE_Extensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "OCTET-STRING", sz_extensible = false, sz_lb = "3", sz_ub = "3")]
pub struct PLMNIdentity(Vec<u8>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct PLMNSupportItem {
    pub plmn_identity: PLMNIdentity,
    pub slice_support_list: SliceSupportList,
    #[asn(optional_idx = 0)]
    pub ie_extensions: Option<PLMNSupportItemIE_Extensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "12")]
pub struct PLMNSupportList(Vec<PLMNSupportItem>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct PNI_NPN_MobilityInformation {
    pub allowed_pni_npi_list: Allowed_PNI_NPN_List,
    #[asn(optional_idx = 0)]
    pub ie_extensions: Option<PNI_NPN_MobilityInformationIE_Extensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true)]
pub struct PWSCancelRequest {
    pub protocol_i_es: PWSCancelRequestProtocolIEs,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true)]
pub struct PWSCancelResponse {
    pub protocol_i_es: PWSCancelResponseProtocolIEs,
}

#[derive(Debug, AperCodec)]
#[asn(type = "CHOICE", lb = "0", ub = "2", extensible = false)]
pub enum PWSFailedCellIDList {
    #[asn(key = 0, extended = false)]
    EUTRA_CGI_PWSFailedList(EUTRA_CGIList),
    #[asn(key = 1, extended = false)]
    NR_CGI_PWSFailedList(NR_CGIList),
    #[asn(key = 2, extended = false)]
    Choice_Extensions(PWSFailedCellIDListchoice_Extensions),
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
#[asn(type = "INTEGER", lb = "0", ub = "1023", extensible = true)]
pub struct PacketDelayBudget(u16);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct PacketErrorRate {
    pub per_scalar: INTEGER_68,
    pub per_exponent: INTEGER_69,
    #[asn(optional_idx = 0)]
    pub ie_extensions: Option<PacketErrorRateIE_Extensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "INTEGER", lb = "0", ub = "1000", extensible = true)]
pub struct PacketLossRate(u16);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true)]
pub struct Paging {
    pub protocol_i_es: PagingProtocolIEs,
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
    pub eutra_cgi: EUTRA_CGI,
    pub coverage_enhancement_level: CoverageEnhancementLevel,
    #[asn(optional_idx = 0)]
    pub ie_extensions: Option<PagingAssisDataforCEcapabUEIE_Extensions>,
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
    pub ie_extensions: Option<PagingeDRXInformationIE_Extensions>,
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
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 3)]
pub struct PathSwitchRequestAcknowledgeTransfer {
    #[asn(optional_idx = 0)]
    pub ul_ngu_up_tnl_information: Option<UPTransportLayerInformation>,
    #[asn(optional_idx = 1)]
    pub security_indication: Option<SecurityIndication>,
    #[asn(optional_idx = 2)]
    pub ie_extensions: Option<PathSwitchRequestAcknowledgeTransferIE_Extensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true)]
pub struct PathSwitchRequestFailure {
    pub protocol_i_es: PathSwitchRequestFailureProtocolIEs,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct PathSwitchRequestSetupFailedTransfer {
    pub cause: Cause,
    #[asn(optional_idx = 0)]
    pub ie_extensions: Option<PathSwitchRequestSetupFailedTransferIE_Extensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 3)]
pub struct PathSwitchRequestTransfer {
    pub dl_ngu_up_tnl_information: UPTransportLayerInformation,
    #[asn(optional_idx = 0)]
    pub dl_ngu_tnl_information_reused: Option<DL_NGU_TNLInformationReused>,
    #[asn(optional_idx = 1)]
    pub user_plane_security_information: Option<UserPlaneSecurityInformation>,
    pub qos_flow_accepted_list: QosFlowAcceptedList,
    #[asn(optional_idx = 2)]
    pub ie_extensions: Option<PathSwitchRequestTransferIE_Extensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct PathSwitchRequestUnsuccessfulTransfer {
    pub cause: Cause,
    #[asn(optional_idx = 0)]
    pub ie_extensions: Option<PathSwitchRequestUnsuccessfulTransferIE_Extensions>,
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
    Local(INTEGER_70),
    #[asn(key = 1, extended = false)]
    Global(OBJECT_IDENTIFIER_71),
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
#[asn(type = "CHOICE", lb = "0", ub = "1", extensible = false)]
pub enum ProcedureStageChoice {
    #[asn(key = 0, extended = false)]
    First_dl_count(FirstDLCount),
    #[asn(key = 1, extended = false)]
    Choice_Extensions(ProcedureStageChoicechoice_Extensions),
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
    pub rat_type: ENUMERATED_72,
    pub qo_s_flows_timed_report_list: VolumeTimedReportList,
    #[asn(optional_idx = 0)]
    pub ie_extensions: Option<QoSFlowsUsageReport_ItemIE_Extensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "64")]
pub struct QoSFlowsUsageReportList(Vec<QoSFlowsUsageReport_Item>);

#[derive(Debug, AperCodec)]
#[asn(type = "CHOICE", lb = "0", ub = "2", extensible = false)]
pub enum QosCharacteristics {
    #[asn(key = 0, extended = false)]
    NonDynamic5QI(NonDynamic5QIDescriptor),
    #[asn(key = 1, extended = false)]
    Dynamic5QI(Dynamic5QIDescriptor),
    #[asn(key = 2, extended = false)]
    Choice_Extensions(QosCharacteristicschoice_Extensions),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct QosFlowAcceptedItem {
    pub qos_flow_identifier: QosFlowIdentifier,
    #[asn(optional_idx = 0)]
    pub ie_extensions: Option<QosFlowAcceptedItemIE_Extensions>,
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
    pub ie_extensions: Option<QosFlowAddOrModifyRequestItemIE_Extensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "64")]
pub struct QosFlowAddOrModifyRequestList(Vec<QosFlowAddOrModifyRequestItem>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct QosFlowAddOrModifyResponseItem {
    pub qos_flow_identifier: QosFlowIdentifier,
    #[asn(optional_idx = 0)]
    pub ie_extensions: Option<QosFlowAddOrModifyResponseItemIE_Extensions>,
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
    pub ie_extensions: Option<QosFlowFeedbackItemIE_Extensions>,
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
    pub dl_forwarding: Option<DLForwarding>,
    #[asn(optional_idx = 1)]
    pub ie_extensions: Option<QosFlowInformationItemIE_Extensions>,
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
    pub ie_extensions: Option<QosFlowItemWithDataForwardingIE_Extensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 4)]
pub struct QosFlowLevelQosParameters {
    pub qos_characteristics: QosCharacteristics,
    pub allocation_and_retention_priority: AllocationAndRetentionPriority,
    #[asn(optional_idx = 0)]
    pub gbr_qos_information: Option<GBR_QosInformation>,
    #[asn(optional_idx = 1)]
    pub reflective_qos_attribute: Option<ReflectiveQosAttribute>,
    #[asn(optional_idx = 2)]
    pub additional_qos_flow_information: Option<AdditionalQosFlowInformation>,
    #[asn(optional_idx = 3)]
    pub ie_extensions: Option<QosFlowLevelQosParametersIE_Extensions>,
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
    pub ie_extensions: Option<QosFlowModifyConfirmItemIE_Extensions>,
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
    pub ie_extensions: Option<QosFlowNotifyItemIE_Extensions>,
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
    pub ie_extensions: Option<QosFlowParametersItemIE_Extensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "64")]
pub struct QosFlowParametersList(Vec<QosFlowParametersItem>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct QosFlowPerTNLInformation {
    pub up_transport_layer_information: UPTransportLayerInformation,
    pub associated_qos_flow_list: AssociatedQosFlowList,
    #[asn(optional_idx = 0)]
    pub ie_extensions: Option<QosFlowPerTNLInformationIE_Extensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct QosFlowPerTNLInformationItem {
    pub qos_flow_per_tnl_information: QosFlowPerTNLInformation,
    #[asn(optional_idx = 0)]
    pub ie_extensions: Option<QosFlowPerTNLInformationItemIE_Extensions>,
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
    pub ie_extensions: Option<QosFlowSetupRequestItemIE_Extensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "64")]
pub struct QosFlowSetupRequestList(Vec<QosFlowSetupRequestItem>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct QosFlowToBeForwardedItem {
    pub qos_flow_identifier: QosFlowIdentifier,
    #[asn(optional_idx = 0)]
    pub ie_extensions: Option<QosFlowToBeForwardedItemIE_Extensions>,
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
    pub ie_extensions: Option<QosFlowWithCauseItemIE_Extensions>,
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
    pub protocol_i_es: RANCPRelocationIndicationProtocolIEs,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true)]
pub struct RANConfigurationUpdate {
    pub protocol_i_es: RANConfigurationUpdateProtocolIEs,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true)]
pub struct RANConfigurationUpdateAcknowledge {
    pub protocol_i_es: RANConfigurationUpdateAcknowledgeProtocolIEs,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true)]
pub struct RANConfigurationUpdateFailure {
    pub protocol_i_es: RANConfigurationUpdateFailureProtocolIEs,
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
    pub dr_bs_subject_to_status_transfer_list: DRBsSubjectToStatusTransferList,
    #[asn(optional_idx = 0)]
    pub ie_extensions: Option<RANStatusTransfer_TransparentContainerIE_Extensions>,
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
    pub plmn_identity: PLMNIdentity,
    pub rat_restriction_information: RATRestrictionInformation,
    #[asn(optional_idx = 0)]
    pub ie_extensions: Option<RATRestrictions_ItemIE_Extensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "OCTET-STRING")]
pub struct RGLevelWirelineAccessCharacteristics(Vec<u8>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct RIMInformation {
    pub targetg_nb_set_id: GNBSetID,
    pub rim_rs_detection: ENUMERATED_73,
    #[asn(optional_idx = 0)]
    pub ie_extensions: Option<RIMInformationIE_Extensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct RIMInformationTransfer {
    pub target_ran_node_id: TargetRANNodeID,
    pub source_ran_node_id: SourceRANNodeID,
    pub rim_information: RIMInformation,
    #[asn(optional_idx = 0)]
    pub ie_extensions: Option<RIMInformationTransferIE_Extensions>,
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
    pub protocol_i_es: RRCInactiveTransitionReportProtocolIEs,
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
    pub ngran_cgi: NGRAN_CGI,
    #[asn(optional_idx = 0)]
    pub time_stayed_in_cell: Option<INTEGER_74>,
    #[asn(optional_idx = 1)]
    pub ie_extensions: Option<RecommendedCellItemIE_Extensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "16")]
pub struct RecommendedCellList(Vec<RecommendedCellItem>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct RecommendedCellsForPaging {
    pub recommended_cell_list: RecommendedCellList,
    #[asn(optional_idx = 0)]
    pub ie_extensions: Option<RecommendedCellsForPagingIE_Extensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct RecommendedRANNodeItem {
    pub amf_paging_target: AMFPagingTarget,
    #[asn(optional_idx = 0)]
    pub ie_extensions: Option<RecommendedRANNodeItemIE_Extensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "16")]
pub struct RecommendedRANNodeList(Vec<RecommendedRANNodeItem>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct RecommendedRANNodesForPaging {
    pub recommended_ran_node_list: RecommendedRANNodeList,
    #[asn(optional_idx = 0)]
    pub ie_extensions: Option<RecommendedRANNodesForPagingIE_Extensions>,
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
    pub rsn: RSN,
    #[asn(optional_idx = 0)]
    pub ie_extensions: Option<RedundantPDUSessionInformationIE_Extensions>,
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
    pub protocol_i_es: RerouteNASRequestProtocolIEs,
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
    NG_Interface(ResetAll),
    #[asn(key = 1, extended = false)]
    PartOfNG_Interface(UE_associatedLogicalNG_connectionList),
    #[asn(key = 2, extended = false)]
    Choice_Extensions(ResetTypechoice_Extensions),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true)]
pub struct RetrieveUEInformation {
    pub protocol_i_es: RetrieveUEInformationProtocolIEs,
}

#[derive(Debug, AperCodec)]
#[asn(type = "OCTET-STRING")]
pub struct RoutingID(Vec<u8>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 2)]
pub struct S_NSSAI {
    pub sst: SST,
    #[asn(optional_idx = 0)]
    pub sd: Option<SD>,
    #[asn(optional_idx = 1)]
    pub ie_extensions: Option<S_NSSAIIE_Extensions>,
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
    pub ie_extensions: Option<SNPN_MobilityInformationIE_Extensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 2)]
pub struct SONConfigurationTransfer {
    pub target_ran_node_id: TargetRANNodeID,
    pub source_ran_node_id: SourceRANNodeID,
    pub son_information: SONInformation,
    #[asn(optional_idx = 0)]
    pub xn_tnl_configuration_info: Option<XnTNLConfigurationInfo>,
    #[asn(optional_idx = 1)]
    pub ie_extensions: Option<SONConfigurationTransferIE_Extensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "CHOICE", lb = "0", ub = "2", extensible = false)]
pub enum SONInformation {
    #[asn(key = 0, extended = false)]
    SONInformationRequest(SONInformationRequest),
    #[asn(key = 1, extended = false)]
    SONInformationReply(SONInformationReply),
    #[asn(key = 2, extended = false)]
    Choice_Extensions(SONInformationchoice_Extensions),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 2)]
pub struct SONInformationReply {
    #[asn(optional_idx = 0)]
    pub xn_tnl_configuration_info: Option<XnTNLConfigurationInfo>,
    #[asn(optional_idx = 1)]
    pub ie_extensions: Option<SONInformationReplyIE_Extensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "CHOICE", lb = "0", ub = "2", extensible = false)]
pub enum SONInformationReport {
    #[asn(key = 0, extended = false)]
    FailureIndicationInformation(FailureIndication),
    #[asn(key = 1, extended = false)]
    HOReportInformation(HOReport),
    #[asn(key = 2, extended = false)]
    Choice_Extensions(SONInformationReportchoice_Extensions),
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
    pub ie_extensions: Option<ScheduledCommunicationTimeIE_Extensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true)]
pub struct SecondaryRATDataUsageReport {
    pub protocol_i_es: SecondaryRATDataUsageReportProtocolIEs,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 2)]
pub struct SecondaryRATDataUsageReportTransfer {
    #[asn(optional_idx = 0)]
    pub secondary_rat_usage_information: Option<SecondaryRATUsageInformation>,
    #[asn(optional_idx = 1)]
    pub ie_extensions: Option<SecondaryRATDataUsageReportTransferIE_Extensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 3)]
pub struct SecondaryRATUsageInformation {
    #[asn(optional_idx = 0)]
    pub pdu_session_usage_report: Option<PDUSessionUsageReport>,
    #[asn(optional_idx = 1)]
    pub qos_flows_usage_report_list: Option<QoSFlowsUsageReportList>,
    #[asn(optional_idx = 2)]
    pub ie_extension: Option<SecondaryRATUsageInformationIE_Extension>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct SecurityContext {
    pub next_hop_chaining_count: NextHopChainingCount,
    pub next_hop_nh: SecurityKey,
    #[asn(optional_idx = 0)]
    pub ie_extensions: Option<SecurityContextIE_Extensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 2)]
pub struct SecurityIndication {
    pub integrity_protection_indication: IntegrityProtectionIndication,
    pub confidentiality_protection_indication: ConfidentialityProtectionIndication,
    #[asn(optional_idx = 0)]
    pub maximum_integrity_protected_data_rate_ul: Option<MaximumIntegrityProtectedDataRate>,
    #[asn(optional_idx = 1)]
    pub ie_extensions: Option<SecurityIndicationIE_Extensions>,
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
    pub ie_extensions: Option<SecurityResultIE_Extensions>,
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
    pub ie_extensions: Option<SensorMeasConfigNameItemIE_Extensions>,
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
    pub ie_extensions: Option<SensorMeasurementConfigurationIE_Extensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "CHOICE", lb = "0", ub = "3", extensible = false)]
pub enum SensorNameConfig {
    #[asn(key = 0, extended = false)]
    UncompensatedBarometricConfig(ENUMERATED_78),
    #[asn(key = 1, extended = false)]
    UeSpeedConfig(ENUMERATED_79),
    #[asn(key = 2, extended = false)]
    UeOrientationConfig(ENUMERATED_80),
    #[asn(key = 3, extended = false)]
    Choice_Extensions(SensorNameConfigchoice_Extensions),
}

#[derive(Debug, AperCodec)]
#[asn(type = "BITSTRING", sz_extensible = false, sz_lb = "16", sz_ub = "16")]
pub struct SerialNumber(BitVec<Msb0, u8>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 2)]
pub struct ServedGUAMIItem {
    pub guami: GUAMI,
    #[asn(optional_idx = 0)]
    pub backup_amf_name: Option<AMFName>,
    #[asn(optional_idx = 1)]
    pub ie_extensions: Option<ServedGUAMIItemIE_Extensions>,
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
    pub plmn_identity: PLMNIdentity,
    #[asn(optional_idx = 0)]
    pub allowed_ta_cs: Option<AllowedTACs>,
    #[asn(optional_idx = 1)]
    pub not_allowed_ta_cs: Option<NotAllowedTACs>,
    #[asn(optional_idx = 2)]
    pub ie_extensions: Option<ServiceAreaInformation_ItemIE_Extensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "INTEGER", lb = "0", ub = "4294967295")]
pub struct SgNB_UE_X2AP_ID(u32);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct SliceOverloadItem {
    pub s_nssai: S_NSSAI,
    #[asn(optional_idx = 0)]
    pub ie_extensions: Option<SliceOverloadItemIE_Extensions>,
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
    pub ie_extensions: Option<SliceSupportItemIE_Extensions>,
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
    pub rrc_container: RRCContainer,
    #[asn(optional_idx = 0)]
    pub pdu_session_resource_information_list: Option<PDUSessionResourceInformationList>,
    #[asn(optional_idx = 1)]
    pub e_rab_information_list: Option<E_RABInformationList>,
    pub target_cell_id: NGRAN_CGI,
    #[asn(optional_idx = 2)]
    pub index_to_rfsp: Option<IndexToRFSP>,
    pub ue_history_information: UEHistoryInformation,
    #[asn(optional_idx = 3)]
    pub ie_extensions: Option<SourceNGRANNode_ToTargetNGRANNode_TransparentContainerIE_Extensions>,
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
    pub ie_extensions: Option<SourceRANNodeIDIE_Extensions>,
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
    pub ie_extensions: Option<SourceToTarget_AMFInformationRerouteIE_Extensions>,
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
    pub value: SuccessfulOutcomeValue,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct SupportedTAItem {
    pub tac: TAC,
    pub broadcast_plmn_list: BroadcastPLMNList,
    #[asn(optional_idx = 0)]
    pub ie_extensions: Option<SupportedTAItemIE_Extensions>,
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
    pub ta_listfor_mdt: TAListforMDT,
    #[asn(optional_idx = 0)]
    pub ie_extensions: Option<TABasedMDTIE_Extensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "OCTET-STRING", sz_extensible = false, sz_lb = "3", sz_ub = "3")]
pub struct TAC(Vec<u8>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct TAI {
    pub plmn_identity: PLMNIdentity,
    pub tac: TAC,
    #[asn(optional_idx = 0)]
    pub ie_extensions: Option<TAIIE_Extensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct TAIBasedMDT {
    pub tai_listfor_mdt: TAIListforMDT,
    #[asn(optional_idx = 0)]
    pub ie_extensions: Option<TAIBasedMDTIE_Extensions>,
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
    pub tai: TAI,
    pub completed_cells_in_tai_eutra: CompletedCellsInTAI_EUTRA,
    #[asn(optional_idx = 0)]
    pub ie_extensions: Option<TAIBroadcastEUTRA_ItemIE_Extensions>,
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
    pub tai: TAI,
    pub completed_cells_in_tai_nr: CompletedCellsInTAI_NR,
    #[asn(optional_idx = 0)]
    pub ie_extensions: Option<TAIBroadcastNR_ItemIE_Extensions>,
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
    pub tai: TAI,
    pub cancelled_cells_in_tai_eutra: CancelledCellsInTAI_EUTRA,
    #[asn(optional_idx = 0)]
    pub ie_extensions: Option<TAICancelledEUTRA_ItemIE_Extensions>,
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
    pub tai: TAI,
    pub cancelled_cells_in_tai_nr: CancelledCellsInTAI_NR,
    #[asn(optional_idx = 0)]
    pub ie_extensions: Option<TAICancelledNR_ItemIE_Extensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "16")]
pub struct TAIListForInactive(Vec<TAIListForInactiveItem>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct TAIListForInactiveItem {
    pub tai: TAI,
    #[asn(optional_idx = 0)]
    pub ie_extensions: Option<TAIListForInactiveItemIE_Extensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "16")]
pub struct TAIListForPaging(Vec<TAIListForPagingItem>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct TAIListForPagingItem {
    pub tai: TAI,
    #[asn(optional_idx = 0)]
    pub ie_extensions: Option<TAIListForPagingItemIE_Extensions>,
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
    TNGF_ID(BIT_STRING_81),
    #[asn(key = 1, extended = false)]
    Choice_Extensions(TNGF_IDchoice_Extensions),
}

#[derive(Debug, AperCodec)]
#[asn(type = "INTEGER", lb = "0", ub = "255")]
pub struct TNLAddressWeightFactor(u8);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct TNLAssociationItem {
    pub tnl_association_address: CPTransportLayerInformation,
    pub cause: Cause,
    #[asn(optional_idx = 0)]
    pub ie_extensions: Option<TNLAssociationItemIE_Extensions>,
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
    pub ie_extensions: Option<TSCAssistanceInformationIE_Extensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 3)]
pub struct TSCTrafficCharacteristics {
    #[asn(optional_idx = 0)]
    pub tsc_assistance_information_dl: Option<TSCAssistanceInformation>,
    #[asn(optional_idx = 1)]
    pub tsc_assistance_information_ul: Option<TSCAssistanceInformation>,
    #[asn(optional_idx = 2)]
    pub ie_extensions: Option<TSCTrafficCharacteristicsIE_Extensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "OCTET-STRING")]
pub struct TWAP_ID(Vec<u8>);

#[derive(Debug, AperCodec)]
#[asn(type = "CHOICE", lb = "0", ub = "1", extensible = false)]
pub enum TWIF_ID {
    #[asn(key = 0, extended = false)]
    TWIF_ID(BIT_STRING_82),
    #[asn(key = 1, extended = false)]
    Choice_Extensions(TWIF_IDchoice_Extensions),
}

#[derive(Debug, AperCodec)]
#[asn(type = "CHOICE", lb = "0", ub = "2", extensible = false)]
pub enum TargetID {
    #[asn(key = 0, extended = false)]
    TargetRANNodeID(TargetRANNodeID),
    #[asn(key = 1, extended = false)]
    TargeteNB_ID(TargeteNB_ID),
    #[asn(key = 2, extended = false)]
    Choice_Extensions(TargetIDchoice_Extensions),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct TargetNGRANNode_ToSourceNGRANNode_FailureTransparentContainer {
    pub cell_cag_information: Cell_CAGInformation,
    #[asn(optional_idx = 0)]
    pub ie_extensions:
        Option<TargetNGRANNode_ToSourceNGRANNode_FailureTransparentContainerIE_Extensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct TargetNGRANNode_ToSourceNGRANNode_TransparentContainer {
    pub rrc_container: RRCContainer,
    #[asn(optional_idx = 0)]
    pub ie_extensions: Option<TargetNGRANNode_ToSourceNGRANNode_TransparentContainerIE_Extensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct TargetRANNodeID {
    pub global_ran_node_id: GlobalRANNodeID,
    pub selected_tai: TAI,
    #[asn(optional_idx = 0)]
    pub ie_extensions: Option<TargetRANNodeIDIE_Extensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 2)]
pub struct TargetRNC_ID {
    pub lai: LAI,
    pub rnc_id: RNC_ID,
    #[asn(optional_idx = 0)]
    pub extended_rnc_id: Option<ExtendedRNC_ID>,
    #[asn(optional_idx = 1)]
    pub ie_extensions: Option<TargetRNC_IDIE_Extensions>,
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
    pub ie_extensions: Option<TargeteNB_IDIE_Extensions>,
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
    pub uerlf_report_container: Option<UERLFReportContainer>,
    #[asn(optional_idx = 1)]
    pub ie_extensions: Option<TooearlyIntersystemHOIE_Extensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct TraceActivation {
    pub ngran_trace_id: NGRANTraceID,
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
    pub ie_extensions: Option<UE_DifferentiationInfoIE_Extensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct UE_NGAP_ID_pair {
    pub amf_ue_ngap_id: AMF_UE_NGAP_ID,
    pub ran_ue_ngap_id: RAN_UE_NGAP_ID,
    #[asn(optional_idx = 0)]
    pub ie_extensions: Option<UE_NGAP_ID_pairIE_Extensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "CHOICE", lb = "0", ub = "2", extensible = false)]
pub enum UE_NGAP_IDs {
    #[asn(key = 0, extended = false)]
    UE_NGAP_ID_pair(UE_NGAP_ID_pair),
    #[asn(key = 1, extended = false)]
    AMF_UE_NGAP_ID(AMF_UE_NGAP_ID),
    #[asn(key = 2, extended = false)]
    Choice_Extensions(UE_NGAP_IDschoice_Extensions),
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
    pub amf_ue_ngap_id: Option<AMF_UE_NGAP_ID>,
    #[asn(optional_idx = 1)]
    pub ran_ue_ngap_id: Option<RAN_UE_NGAP_ID>,
    #[asn(optional_idx = 2)]
    pub ie_extensions: Option<UE_associatedLogicalNG_connectionItemIE_Extensions>,
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
    pub ue_aggregate_maximum_bit_rate_dl: BitRate,
    pub ue_aggregate_maximum_bit_rate_ul: BitRate,
    #[asn(optional_idx = 0)]
    pub ie_extensions: Option<UEAggregateMaximumBitRateIE_Extensions>,
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
    pub protocol_i_es: UEContextModificationFailureProtocolIEs,
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
#[asn(type = "ENUMERATED", extensible = true, lb = "0", ub = "0")]
pub struct UEContextRequest(u8);
impl UEContextRequest {
    const REQUESTED: u8 = 0u8;
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
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 2)]
pub struct UEContextResumeRequestTransfer {
    #[asn(optional_idx = 0)]
    pub qos_flow_failed_to_resume_list: Option<QosFlowListWithCause>,
    #[asn(optional_idx = 1)]
    pub ie_extensions: Option<UEContextResumeRequestTransferIE_Extensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true)]
pub struct UEContextResumeResponse {
    pub protocol_i_es: UEContextResumeResponseProtocolIEs,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 2)]
pub struct UEContextResumeResponseTransfer {
    #[asn(optional_idx = 0)]
    pub qos_flow_failed_to_resume_list: Option<QosFlowListWithCause>,
    #[asn(optional_idx = 1)]
    pub ie_extensions: Option<UEContextResumeResponseTransferIE_Extensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true)]
pub struct UEContextSuspendFailure {
    pub protocol_i_es: UEContextSuspendFailureProtocolIEs,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true)]
pub struct UEContextSuspendRequest {
    pub protocol_i_es: UEContextSuspendRequestProtocolIEs,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 2)]
pub struct UEContextSuspendRequestTransfer {
    #[asn(optional_idx = 0)]
    pub suspend_indicator: Option<SuspendIndicator>,
    #[asn(optional_idx = 1)]
    pub ie_extensions: Option<UEContextSuspendRequestTransferIE_Extensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true)]
pub struct UEContextSuspendResponse {
    pub protocol_i_es: UEContextSuspendResponseProtocolIEs,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "16")]
pub struct UEHistoryInformation(Vec<LastVisitedCellItem>);

#[derive(Debug, AperCodec)]
#[asn(type = "CHOICE", lb = "0", ub = "1", extensible = false)]
pub enum UEHistoryInformationFromTheUE {
    #[asn(key = 0, extended = false)]
    NR(NRMobilityHistoryReport),
    #[asn(key = 1, extended = false)]
    Choice_Extensions(UEHistoryInformationFromTheUEchoice_Extensions),
}

#[derive(Debug, AperCodec)]
#[asn(type = "CHOICE", lb = "0", ub = "1", extensible = false)]
pub enum UEIdentityIndexValue {
    #[asn(key = 0, extended = false)]
    IndexLength10(BIT_STRING_88),
    #[asn(key = 1, extended = false)]
    Choice_Extensions(UEIdentityIndexValuechoice_Extensions),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true)]
pub struct UEInformationTransfer {
    pub protocol_i_es: UEInformationTransferProtocolIEs,
}

#[derive(Debug, AperCodec)]
#[asn(type = "CHOICE", lb = "0", ub = "1", extensible = false)]
pub enum UEPagingIdentity {
    #[asn(key = 0, extended = false)]
    FiveG_S_TMSI(FiveG_S_TMSI),
    #[asn(key = 1, extended = false)]
    Choice_Extensions(UEPagingIdentitychoice_Extensions),
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
    pub ue_presence: UEPresence,
    #[asn(optional_idx = 0)]
    pub ie_extensions: Option<UEPresenceInAreaOfInterestItemIE_Extensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "64")]
pub struct UEPresenceInAreaOfInterestList(Vec<UEPresenceInAreaOfInterestItem>);

#[derive(Debug, AperCodec)]
#[asn(type = "CHOICE", lb = "0", ub = "2", extensible = false)]
pub enum UERLFReportContainer {
    #[asn(key = 0, extended = false)]
    NR(NRUERLFReportContainer),
    #[asn(key = 1, extended = false)]
    LTE(LTEUERLFReportContainer),
    #[asn(key = 2, extended = false)]
    Choice_Extensions(UERLFReportContainerchoice_Extensions),
}

#[derive(Debug, AperCodec)]
#[asn(type = "OCTET-STRING")]
pub struct UERadioCapability(Vec<u8>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true)]
pub struct UERadioCapabilityCheckRequest {
    pub protocol_i_es: UERadioCapabilityCheckRequestProtocolIEs,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true)]
pub struct UERadioCapabilityCheckResponse {
    pub protocol_i_es: UERadioCapabilityCheckResponseProtocolIEs,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 3)]
pub struct UERadioCapabilityForPaging {
    #[asn(optional_idx = 0)]
    pub ue_radio_capability_for_paging_of_nr: Option<UERadioCapabilityForPagingOfNR>,
    #[asn(optional_idx = 1)]
    pub ue_radio_capability_for_paging_of_eutra: Option<UERadioCapabilityForPagingOfEUTRA>,
    #[asn(optional_idx = 2)]
    pub ie_extensions: Option<UERadioCapabilityForPagingIE_Extensions>,
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
    pub protocol_i_es: UERadioCapabilityIDMappingRequestProtocolIEs,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true)]
pub struct UERadioCapabilityIDMappingResponse {
    pub protocol_i_es: UERadioCapabilityIDMappingResponseProtocolIEs,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true)]
pub struct UERadioCapabilityInfoIndication {
    pub protocol_i_es: UERadioCapabilityInfoIndicationProtocolIEs,
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
    pub eutr_aencryption_algorithms: EUTRAencryptionAlgorithms,
    pub eutr_aintegrity_protection_algorithms: EUTRAintegrityProtectionAlgorithms,
    #[asn(optional_idx = 0)]
    pub ie_extensions: Option<UESecurityCapabilitiesIE_Extensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true)]
pub struct UETNLABindingReleaseRequest {
    pub protocol_i_es: UETNLABindingReleaseRequestProtocolIEs,
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
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct UL_NGU_UP_TNLModifyItem {
    pub ul_ngu_up_tnl_information: UPTransportLayerInformation,
    pub dl_ngu_up_tnl_information: UPTransportLayerInformation,
    #[asn(optional_idx = 0)]
    pub ie_extensions: Option<UL_NGU_UP_TNLModifyItemIE_Extensions>,
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
    GTPTunnel(GTPTunnel),
    #[asn(key = 1, extended = false)]
    Choice_Extensions(UPTransportLayerInformationchoice_Extensions),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct UPTransportLayerInformationItem {
    pub ngu_up_tnl_information: UPTransportLayerInformation,
    #[asn(optional_idx = 0)]
    pub ie_extensions: Option<UPTransportLayerInformationItemIE_Extensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "3")]
pub struct UPTransportLayerInformationList(Vec<UPTransportLayerInformationItem>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct UPTransportLayerInformationPairItem {
    pub ul_ngu_up_tnl_information: UPTransportLayerInformation,
    pub dl_ngu_up_tnl_information: UPTransportLayerInformation,
    #[asn(optional_idx = 0)]
    pub ie_extensions: Option<UPTransportLayerInformationPairItemIE_Extensions>,
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
    pub guami: GUAMI,
    #[asn(optional_idx = 0)]
    pub timer_approach_for_guami_removal: Option<TimerApproachForGUAMIRemoval>,
    #[asn(optional_idx = 1)]
    pub backup_amf_name: Option<AMFName>,
    #[asn(optional_idx = 2)]
    pub ie_extensions: Option<UnavailableGUAMIItemIE_Extensions>,
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
    pub value: UnsuccessfulOutcomeValue,
}

#[derive(Debug, AperCodec)]
#[asn(type = "BITSTRING", sz_extensible = true, sz_lb = "8", sz_ub = "8")]
pub struct UpdateFeedback(BitVec<Msb0, u8>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true)]
pub struct UplinkNASTransport {
    pub protocol_i_es: UplinkNASTransportProtocolIEs,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true)]
pub struct UplinkNonUEAssociatedNRPPaTransport {
    pub protocol_i_es: UplinkNonUEAssociatedNRPPaTransportProtocolIEs,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true)]
pub struct UplinkRANConfigurationTransfer {
    pub protocol_i_es: UplinkRANConfigurationTransferProtocolIEs,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true)]
pub struct UplinkRANEarlyStatusTransfer {
    pub protocol_i_es: UplinkRANEarlyStatusTransferProtocolIEs,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true)]
pub struct UplinkRANStatusTransfer {
    pub protocol_i_es: UplinkRANStatusTransferProtocolIEs,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true)]
pub struct UplinkRIMInformationTransfer {
    pub protocol_i_es: UplinkRIMInformationTransferProtocolIEs,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true)]
pub struct UplinkUEAssociatedNRPPaTransport {
    pub protocol_i_es: UplinkUEAssociatedNRPPaTransportProtocolIEs,
}

#[derive(Debug, AperCodec)]
#[asn(type = "CHOICE", lb = "0", ub = "3", extensible = false)]
pub enum UserLocationInformation {
    #[asn(key = 0, extended = false)]
    UserLocationInformationEUTRA(UserLocationInformationEUTRA),
    #[asn(key = 1, extended = false)]
    UserLocationInformationNR(UserLocationInformationNR),
    #[asn(key = 2, extended = false)]
    UserLocationInformationN3IWF(UserLocationInformationN3IWF),
    #[asn(key = 3, extended = false)]
    Choice_Extensions(UserLocationInformationchoice_Extensions),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 2)]
pub struct UserLocationInformationEUTRA {
    pub eutra_cgi: EUTRA_CGI,
    pub tai: TAI,
    #[asn(optional_idx = 0)]
    pub time_stamp: Option<TimeStamp>,
    #[asn(optional_idx = 1)]
    pub ie_extensions: Option<UserLocationInformationEUTRAIE_Extensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct UserLocationInformationN3IWF {
    pub ip_address: TransportLayerAddress,
    pub port_number: PortNumber,
    #[asn(optional_idx = 0)]
    pub ie_extensions: Option<UserLocationInformationN3IWFIE_Extensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 2)]
pub struct UserLocationInformationNR {
    pub nr_cgi: NR_CGI,
    pub tai: TAI,
    #[asn(optional_idx = 0)]
    pub time_stamp: Option<TimeStamp>,
    #[asn(optional_idx = 1)]
    pub ie_extensions: Option<UserLocationInformationNRIE_Extensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 2)]
pub struct UserLocationInformationTNGF {
    pub tnap_id: TNAP_ID,
    pub ip_address: TransportLayerAddress,
    #[asn(optional_idx = 0)]
    pub port_number: Option<PortNumber>,
    #[asn(optional_idx = 1)]
    pub ie_extensions: Option<UserLocationInformationTNGFIE_Extensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 2)]
pub struct UserLocationInformationTWIF {
    pub twap_id: TWAP_ID,
    pub ip_address: TransportLayerAddress,
    #[asn(optional_idx = 0)]
    pub port_number: Option<PortNumber>,
    #[asn(optional_idx = 1)]
    pub ie_extensions: Option<UserLocationInformationTWIFIE_Extensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "CHOICE", lb = "0", ub = "2", extensible = false)]
pub enum UserLocationInformationW_AGF {
    #[asn(key = 0, extended = false)]
    GlobalLine_ID(GlobalLine_ID),
    #[asn(key = 1, extended = false)]
    HFCNode_ID(HFCNode_ID),
    #[asn(key = 2, extended = false)]
    Choice_Extensions(UserLocationInformationW_AGFchoice_Extensions),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct UserPlaneSecurityInformation {
    pub security_result: SecurityResult,
    pub security_indication: SecurityIndication,
    #[asn(optional_idx = 0)]
    pub ie_extensions: Option<UserPlaneSecurityInformationIE_Extensions>,
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
    pub ie_extensions: Option<VolumeTimedReport_ItemIE_Extensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "2")]
pub struct VolumeTimedReportList(Vec<VolumeTimedReport_Item>);

#[derive(Debug, AperCodec)]
#[asn(type = "CHOICE", lb = "0", ub = "1", extensible = false)]
pub enum W_AGF_ID {
    #[asn(key = 0, extended = false)]
    W_AGF_ID(BIT_STRING_93),
    #[asn(key = 1, extended = false)]
    Choice_Extensions(W_AGF_IDchoice_Extensions),
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
    pub wlan_name: WLANName,
    #[asn(optional_idx = 0)]
    pub ie_extensions: Option<WLANMeasConfigNameItemIE_Extensions>,
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
#[asn(type = "CHOICE", lb = "0", ub = "4", extensible = false)]
pub enum WarningAreaList {
    #[asn(key = 0, extended = false)]
    EUTRA_CGIListForWarning(EUTRA_CGIListForWarning),
    #[asn(key = 1, extended = false)]
    NR_CGIListForWarning(NR_CGIListForWarning),
    #[asn(key = 2, extended = false)]
    TAIListForWarning(TAIListForWarning),
    #[asn(key = 3, extended = false)]
    EmergencyAreaIDList(EmergencyAreaIDList),
    #[asn(key = 4, extended = false)]
    Choice_Extensions(WarningAreaListchoice_Extensions),
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
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 3)]
pub struct XnExtTLA_Item {
    #[asn(optional_idx = 0)]
    pub i_psec_tla: Option<TransportLayerAddress>,
    #[asn(optional_idx = 1)]
    pub gtp_tl_as: Option<XnGTP_TLAs>,
    #[asn(optional_idx = 2)]
    pub ie_extensions: Option<XnExtTLA_ItemIE_Extensions>,
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
    pub ie_extensions: Option<XnTNLConfigurationInfoIE_Extensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct AMF_TNLAssociationSetupItemIE_Extensions_Entry {}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct AMF_TNLAssociationSetupItemIE_Extensions(
    Vec<AMF_TNLAssociationSetupItemIE_Extensions_Entry>,
);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct AMF_TNLAssociationToAddItemIE_Extensions_Entry {}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct AMF_TNLAssociationToAddItemIE_Extensions(
    Vec<AMF_TNLAssociationToAddItemIE_Extensions_Entry>,
);

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum AMF_TNLAssociationToRemoveItemIE_Extensions_EntryExtensionValue {
    #[asn(key = 168)]
    Id_TNLAssociationTransportLayerAddressNGRAN(CPTransportLayerInformation),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct AMF_TNLAssociationToRemoveItemIE_Extensions_Entry {
    #[asn(key_field = true)]
    pub id: ProtocolExtensionID,
    pub criticality: Criticality,
    pub extension_value: AMF_TNLAssociationToRemoveItemIE_Extensions_EntryExtensionValue,
}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct AMF_TNLAssociationToRemoveItemIE_Extensions(
    Vec<AMF_TNLAssociationToRemoveItemIE_Extensions_Entry>,
);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct AMF_TNLAssociationToUpdateItemIE_Extensions_Entry {}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct AMF_TNLAssociationToUpdateItemIE_Extensions(
    Vec<AMF_TNLAssociationToUpdateItemIE_Extensions_Entry>,
);

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum AMFCPRelocationIndicationProtocolIEs_EntryValue {
    #[asn(key = 10)]
    Id_AMF_UE_NGAP_ID(AMF_UE_NGAP_ID),
    #[asn(key = 0)]
    Id_AllowedNSSAI(AllowedNSSAI),
    #[asn(key = 85)]
    Id_RAN_UE_NGAP_ID(RAN_UE_NGAP_ID),
    #[asn(key = 148)]
    Id_S_NSSAI(S_NSSAI),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct AMFCPRelocationIndicationProtocolIEs_Entry {
    #[asn(key_field = true)]
    pub id: ProtocolIE_ID,
    pub criticality: Criticality,
    pub value: AMFCPRelocationIndicationProtocolIEs_EntryValue,
}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "0",
    sz_ub = "65535"
)]
pub struct AMFCPRelocationIndicationProtocolIEs(Vec<AMFCPRelocationIndicationProtocolIEs_Entry>);

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum AMFConfigurationUpdateProtocolIEs_EntryValue {
    #[asn(key = 6)]
    Id_AMF_TNLAssociationToAddList(AMF_TNLAssociationToAddList),
    #[asn(key = 7)]
    Id_AMF_TNLAssociationToRemoveList(AMF_TNLAssociationToRemoveList),
    #[asn(key = 8)]
    Id_AMF_TNLAssociationToUpdateList(AMF_TNLAssociationToUpdateList),
    #[asn(key = 1)]
    Id_AMFName(AMFName),
    #[asn(key = 274)]
    Id_Extended_AMFName(Extended_AMFName),
    #[asn(key = 80)]
    Id_PLMNSupportList(PLMNSupportList),
    #[asn(key = 86)]
    Id_RelativeAMFCapacity(RelativeAMFCapacity),
    #[asn(key = 96)]
    Id_ServedGUAMIList(ServedGUAMIList),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct AMFConfigurationUpdateProtocolIEs_Entry {
    #[asn(key_field = true)]
    pub id: ProtocolIE_ID,
    pub criticality: Criticality,
    pub value: AMFConfigurationUpdateProtocolIEs_EntryValue,
}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "0",
    sz_ub = "65535"
)]
pub struct AMFConfigurationUpdateProtocolIEs(Vec<AMFConfigurationUpdateProtocolIEs_Entry>);

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum AMFConfigurationUpdateAcknowledgeProtocolIEs_EntryValue {
    #[asn(key = 4)]
    Id_AMF_TNLAssociationFailedToSetupList(TNLAssociationList),
    #[asn(key = 5)]
    Id_AMF_TNLAssociationSetupList(AMF_TNLAssociationSetupList),
    #[asn(key = 19)]
    Id_CriticalityDiagnostics(CriticalityDiagnostics),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct AMFConfigurationUpdateAcknowledgeProtocolIEs_Entry {
    #[asn(key_field = true)]
    pub id: ProtocolIE_ID,
    pub criticality: Criticality,
    pub value: AMFConfigurationUpdateAcknowledgeProtocolIEs_EntryValue,
}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "0",
    sz_ub = "65535"
)]
pub struct AMFConfigurationUpdateAcknowledgeProtocolIEs(
    Vec<AMFConfigurationUpdateAcknowledgeProtocolIEs_Entry>,
);

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum AMFConfigurationUpdateFailureProtocolIEs_EntryValue {
    #[asn(key = 15)]
    Id_Cause(Cause),
    #[asn(key = 19)]
    Id_CriticalityDiagnostics(CriticalityDiagnostics),
    #[asn(key = 107)]
    Id_TimeToWait(TimeToWait),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct AMFConfigurationUpdateFailureProtocolIEs_Entry {
    #[asn(key_field = true)]
    pub id: ProtocolIE_ID,
    pub criticality: Criticality,
    pub value: AMFConfigurationUpdateFailureProtocolIEs_EntryValue,
}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "0",
    sz_ub = "65535"
)]
pub struct AMFConfigurationUpdateFailureProtocolIEs(
    Vec<AMFConfigurationUpdateFailureProtocolIEs_Entry>,
);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct AMFPagingTargetchoice_Extensions {}

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum AMFStatusIndicationProtocolIEs_EntryValue {
    #[asn(key = 120)]
    Id_UnavailableGUAMIList(UnavailableGUAMIList),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct AMFStatusIndicationProtocolIEs_Entry {
    #[asn(key_field = true)]
    pub id: ProtocolIE_ID,
    pub criticality: Criticality,
    pub value: AMFStatusIndicationProtocolIEs_EntryValue,
}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "0",
    sz_ub = "65535"
)]
pub struct AMFStatusIndicationProtocolIEs(Vec<AMFStatusIndicationProtocolIEs_Entry>);

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum AdditionalDLUPTNLInformationForHOItemIE_Extensions_EntryExtensionValue {
    #[asn(key = 183)]
    Id_AdditionalRedundantDL_NGU_UP_TNLInformation(UPTransportLayerInformation),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct AdditionalDLUPTNLInformationForHOItemIE_Extensions_Entry {
    #[asn(key_field = true)]
    pub id: ProtocolExtensionID,
    pub criticality: Criticality,
    pub extension_value: AdditionalDLUPTNLInformationForHOItemIE_Extensions_EntryExtensionValue,
}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct AdditionalDLUPTNLInformationForHOItemIE_Extensions(
    Vec<AdditionalDLUPTNLInformationForHOItemIE_Extensions_Entry>,
);

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
#[asn(type = "ENUMERATED", extensible = true, lb = "0", ub = "1")]
pub struct ENUMERATED_2(u8);
impl ENUMERATED_2 {
    const RESTRICTED: u8 = 0u8;
    const NOT_RESTRICTED: u8 = 1u8;
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct Allowed_PNI_NPN_ItemIE_Extensions_Entry {}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct Allowed_PNI_NPN_ItemIE_Extensions(Vec<Allowed_PNI_NPN_ItemIE_Extensions_Entry>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct AllowedNSSAI_ItemIE_Extensions_Entry {}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct AllowedNSSAI_ItemIE_Extensions(Vec<AllowedNSSAI_ItemIE_Extensions_Entry>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct AlternativeQoSParaSetItemIE_Extensions_Entry {}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct AlternativeQoSParaSetItemIE_Extensions(
    Vec<AlternativeQoSParaSetItemIE_Extensions_Entry>,
);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct AreaOfInterestIE_Extensions_Entry {}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct AreaOfInterestIE_Extensions(Vec<AreaOfInterestIE_Extensions_Entry>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct AreaOfInterestCellItemIE_Extensions_Entry {}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct AreaOfInterestCellItemIE_Extensions(Vec<AreaOfInterestCellItemIE_Extensions_Entry>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct AreaOfInterestItemIE_Extensions_Entry {}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct AreaOfInterestItemIE_Extensions(Vec<AreaOfInterestItemIE_Extensions_Entry>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct AreaOfInterestRANNodeItemIE_Extensions_Entry {}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct AreaOfInterestRANNodeItemIE_Extensions(
    Vec<AreaOfInterestRANNodeItemIE_Extensions_Entry>,
);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct AreaOfInterestTAIItemIE_Extensions_Entry {}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct AreaOfInterestTAIItemIE_Extensions(Vec<AreaOfInterestTAIItemIE_Extensions_Entry>);

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
pub struct AreaScopeOfNeighCellsItemIE_Extensions_Entry {}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct AreaScopeOfNeighCellsItemIE_Extensions(
    Vec<AreaScopeOfNeighCellsItemIE_Extensions_Entry>,
);

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum AssistanceDataForPagingIE_Extensions_EntryExtensionValue {
    #[asn(key = 260)]
    Id_NPN_PagingAssistanceInformation(NPN_PagingAssistanceInformation),
    #[asn(key = 207)]
    Id_PagingAssisDataforCEcapabUE(PagingAssisDataforCEcapabUE),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct AssistanceDataForPagingIE_Extensions_Entry {
    #[asn(key_field = true)]
    pub id: ProtocolExtensionID,
    pub criticality: Criticality,
    pub extension_value: AssistanceDataForPagingIE_Extensions_EntryExtensionValue,
}

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
#[asn(type = "ENUMERATED", extensible = true, lb = "0", ub = "1")]
pub struct ENUMERATED_5(u8);
impl ENUMERATED_5 {
    const UL: u8 = 0u8;
    const DL: u8 = 1u8;
}

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum AssociatedQosFlowItemIE_Extensions_EntryExtensionValue {
    #[asn(key = 221)]
    Id_CurrentQoSParaSetIndex(AlternativeQoSParaSetIndex),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct AssociatedQosFlowItemIE_Extensions_Entry {
    #[asn(key_field = true)]
    pub id: ProtocolExtensionID,
    pub criticality: Criticality,
    pub extension_value: AssociatedQosFlowItemIE_Extensions_EntryExtensionValue,
}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct AssociatedQosFlowItemIE_Extensions(Vec<AssociatedQosFlowItemIE_Extensions_Entry>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct BluetoothMeasConfigNameItemIE_Extensions_Entry {}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct BluetoothMeasConfigNameItemIE_Extensions(
    Vec<BluetoothMeasConfigNameItemIE_Extensions_Entry>,
);

#[derive(Debug, AperCodec)]
#[asn(type = "ENUMERATED", extensible = true, lb = "0", ub = "0")]
pub struct ENUMERATED_6(u8);
impl ENUMERATED_6 {
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
pub struct BroadcastCancelledAreaListchoice_Extensions {}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct BroadcastCompletedAreaListchoice_Extensions {}

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum BroadcastPLMNItemIE_Extensions_EntryExtensionValue {
    #[asn(key = 271)]
    Id_ExtendedTAISliceSupportList(ExtendedSliceSupportList),
    #[asn(key = 258)]
    Id_NPN_Support(NPN_Support),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct BroadcastPLMNItemIE_Extensions_Entry {
    #[asn(key_field = true)]
    pub id: ProtocolExtensionID,
    pub criticality: Criticality,
    pub extension_value: BroadcastPLMNItemIE_Extensions_EntryExtensionValue,
}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct BroadcastPLMNItemIE_Extensions(Vec<BroadcastPLMNItemIE_Extensions_Entry>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct CNAssistedRANTuningIE_Extensions_Entry {}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct CNAssistedRANTuningIE_Extensions(Vec<CNAssistedRANTuningIE_Extensions_Entry>);

#[derive(Debug, AperCodec)]
#[asn(type = "ENUMERATED", extensible = true, lb = "0", ub = "1")]
pub struct ENUMERATED_7(u8);
impl ENUMERATED_7 {
    const EPC_FORBIDDEN: u8 = 0u8;
    const FIVE_GC_FORBIDDEN: u8 = 1u8;
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct CNTypeRestrictionsForEquivalentItemIE_Extensions_Entry {}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct CNTypeRestrictionsForEquivalentItemIE_Extensions(
    Vec<CNTypeRestrictionsForEquivalentItemIE_Extensions_Entry>,
);

#[derive(Debug, AperCodec)]
#[asn(type = "INTEGER", lb = "0", ub = "4095")]
pub struct INTEGER_8(u16);

#[derive(Debug, AperCodec)]
#[asn(type = "INTEGER", lb = "0", ub = "1048575")]
pub struct INTEGER_9(u32);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct COUNTValueForPDCP_SN12IE_Extensions_Entry {}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct COUNTValueForPDCP_SN12IE_Extensions(Vec<COUNTValueForPDCP_SN12IE_Extensions_Entry>);

#[derive(Debug, AperCodec)]
#[asn(type = "INTEGER", lb = "0", ub = "262143")]
pub struct INTEGER_10(u32);

#[derive(Debug, AperCodec)]
#[asn(type = "INTEGER", lb = "0", ub = "16383")]
pub struct INTEGER_11(u16);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct COUNTValueForPDCP_SN18IE_Extensions_Entry {}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct COUNTValueForPDCP_SN18IE_Extensions(Vec<COUNTValueForPDCP_SN18IE_Extensions_Entry>);

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum CPTransportLayerInformationchoice_ExtensionsValue {
    #[asn(key = 169)]
    Id_EndpointIPAddressAndPort(EndpointIPAddressAndPort),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct CPTransportLayerInformationchoice_Extensions {
    #[asn(key_field = true)]
    pub id: ProtocolIE_ID,
    pub criticality: Criticality,
    pub value: CPTransportLayerInformationchoice_ExtensionsValue,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct CancelledCellsInEAI_EUTRA_ItemIE_Extensions_Entry {}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct CancelledCellsInEAI_EUTRA_ItemIE_Extensions(
    Vec<CancelledCellsInEAI_EUTRA_ItemIE_Extensions_Entry>,
);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct CancelledCellsInEAI_NR_ItemIE_Extensions_Entry {}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct CancelledCellsInEAI_NR_ItemIE_Extensions(
    Vec<CancelledCellsInEAI_NR_ItemIE_Extensions_Entry>,
);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct CancelledCellsInTAI_EUTRA_ItemIE_Extensions_Entry {}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct CancelledCellsInTAI_EUTRA_ItemIE_Extensions(
    Vec<CancelledCellsInTAI_EUTRA_ItemIE_Extensions_Entry>,
);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct CancelledCellsInTAI_NR_ItemIE_Extensions_Entry {}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct CancelledCellsInTAI_NR_ItemIE_Extensions(
    Vec<CancelledCellsInTAI_NR_ItemIE_Extensions_Entry>,
);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct CandidateCellchoice_Extensions {}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct CandidateCellIDIE_Extensions_Entry {}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct CandidateCellIDIE_Extensions(Vec<CandidateCellIDIE_Extensions_Entry>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct CandidateCellItemIE_Extensions_Entry {}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct CandidateCellItemIE_Extensions(Vec<CandidateCellItemIE_Extensions_Entry>);

#[derive(Debug, AperCodec)]
#[asn(type = "INTEGER", lb = "0", ub = "1007", extensible = true)]
pub struct INTEGER_12(u16);

#[derive(Debug, AperCodec)]
#[asn(type = "INTEGER", lb = "0", ub = "3279165")]
pub struct INTEGER_13(u32);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct CandidatePCIIE_Extensions_Entry {}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct CandidatePCIIE_Extensions(Vec<CandidatePCIIE_Extensions_Entry>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct Causechoice_Extensions {}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct Cell_CAGInformationIE_Extensions_Entry {}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct Cell_CAGInformationIE_Extensions(Vec<Cell_CAGInformationIE_Extensions_Entry>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct CellBasedMDT_EUTRAIE_Extensions_Entry {}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct CellBasedMDT_EUTRAIE_Extensions(Vec<CellBasedMDT_EUTRAIE_Extensions_Entry>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct CellBasedMDT_NRIE_Extensions_Entry {}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct CellBasedMDT_NRIE_Extensions(Vec<CellBasedMDT_NRIE_Extensions_Entry>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct CellIDBroadcastEUTRA_ItemIE_Extensions_Entry {}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct CellIDBroadcastEUTRA_ItemIE_Extensions(
    Vec<CellIDBroadcastEUTRA_ItemIE_Extensions_Entry>,
);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct CellIDBroadcastNR_ItemIE_Extensions_Entry {}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct CellIDBroadcastNR_ItemIE_Extensions(Vec<CellIDBroadcastNR_ItemIE_Extensions_Entry>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct CellIDCancelledEUTRA_ItemIE_Extensions_Entry {}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct CellIDCancelledEUTRA_ItemIE_Extensions(
    Vec<CellIDCancelledEUTRA_ItemIE_Extensions_Entry>,
);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct CellIDCancelledNR_ItemIE_Extensions_Entry {}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct CellIDCancelledNR_ItemIE_Extensions(Vec<CellIDCancelledNR_ItemIE_Extensions_Entry>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct CellIDListForRestartchoice_Extensions {}

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum CellTrafficTraceProtocolIEs_EntryValue {
    #[asn(key = 10)]
    Id_AMF_UE_NGAP_ID(AMF_UE_NGAP_ID),
    #[asn(key = 43)]
    Id_NGRAN_CGI(NGRAN_CGI),
    #[asn(key = 44)]
    Id_NGRANTraceID(NGRANTraceID),
    #[asn(key = 256)]
    Id_PrivacyIndicator(PrivacyIndicator),
    #[asn(key = 85)]
    Id_RAN_UE_NGAP_ID(RAN_UE_NGAP_ID),
    #[asn(key = 109)]
    Id_TraceCollectionEntityIPAddress(TransportLayerAddress),
    #[asn(key = 257)]
    Id_TraceCollectionEntityURI(URI_address),
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
#[asn(type = "SEQUENCE", extensible = false)]
pub struct CompletedCellsInEAI_EUTRA_ItemIE_Extensions_Entry {}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct CompletedCellsInEAI_EUTRA_ItemIE_Extensions(
    Vec<CompletedCellsInEAI_EUTRA_ItemIE_Extensions_Entry>,
);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct CompletedCellsInEAI_NR_ItemIE_Extensions_Entry {}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct CompletedCellsInEAI_NR_ItemIE_Extensions(
    Vec<CompletedCellsInEAI_NR_ItemIE_Extensions_Entry>,
);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct CompletedCellsInTAI_EUTRA_ItemIE_Extensions_Entry {}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct CompletedCellsInTAI_EUTRA_ItemIE_Extensions(
    Vec<CompletedCellsInTAI_EUTRA_ItemIE_Extensions_Entry>,
);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct CompletedCellsInTAI_NR_ItemIE_Extensions_Entry {}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct CompletedCellsInTAI_NR_ItemIE_Extensions(
    Vec<CompletedCellsInTAI_NR_ItemIE_Extensions_Entry>,
);

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum ConnectionEstablishmentIndicationProtocolIEs_EntryValue {
    #[asn(key = 10)]
    Id_AMF_UE_NGAP_ID(AMF_UE_NGAP_ID),
    #[asn(key = 0)]
    Id_AllowedNSSAI(AllowedNSSAI),
    #[asn(key = 222)]
    Id_CEmodeBrestricted(CEmodeBrestricted),
    #[asn(key = 212)]
    Id_DL_CP_SecurityInformation(DL_CP_SecurityInformation),
    #[asn(key = 226)]
    Id_EndIndication(EndIndication),
    #[asn(key = 205)]
    Id_Enhanced_CoverageRestriction(Enhanced_CoverageRestriction),
    #[asn(key = 210)]
    Id_NB_IoT_UEPriority(NB_IoT_UEPriority),
    #[asn(key = 85)]
    Id_RAN_UE_NGAP_ID(RAN_UE_NGAP_ID),
    #[asn(key = 148)]
    Id_S_NSSAI(S_NSSAI),
    #[asn(key = 209)]
    Id_UE_DifferentiationInfo(UE_DifferentiationInfo),
    #[asn(key = 117)]
    Id_UERadioCapability(UERadioCapability),
    #[asn(key = 264)]
    Id_UERadioCapabilityID(UERadioCapabilityID),
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
#[asn(type = "OPEN")]
pub enum CoreNetworkAssistanceInformationForInactiveIE_Extensions_EntryExtensionValue {
    #[asn(key = 280)]
    Id_ExtendedUEIdentityIndexValue(ExtendedUEIdentityIndexValue),
    #[asn(key = 223)]
    Id_PagingeDRXInformation(PagingeDRXInformation),
    #[asn(key = 118)]
    Id_UERadioCapabilityForPaging(UERadioCapabilityForPaging),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct CoreNetworkAssistanceInformationForInactiveIE_Extensions_Entry {
    #[asn(key_field = true)]
    pub id: ProtocolExtensionID,
    pub criticality: Criticality,
    pub extension_value:
        CoreNetworkAssistanceInformationForInactiveIE_Extensions_EntryExtensionValue,
}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct CoreNetworkAssistanceInformationForInactiveIE_Extensions(
    Vec<CoreNetworkAssistanceInformationForInactiveIE_Extensions_Entry>,
);

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
pub struct ENUMERATED_14(u8);
impl ENUMERATED_14 {
    const DAPS_HO_REQUIRED: u8 = 0u8;
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
pub struct ENUMERATED_15(u8);
impl ENUMERATED_15 {
    const DAPS_HO_ACCEPTED: u8 = 0u8;
    const DAPS_HO_NOT_ACCEPTED: u8 = 1u8;
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
pub struct DAPSResponseInfoItemIE_Extension_Entry {}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct DAPSResponseInfoItemIE_Extension(Vec<DAPSResponseInfoItemIE_Extension_Entry>);

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
#[asn(type = "SEQUENCE", extensible = false)]
pub struct DRBStatusDLchoice_Extensions {}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct DRBStatusDL12IE_Extension_Entry {}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct DRBStatusDL12IE_Extension(Vec<DRBStatusDL12IE_Extension_Entry>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct DRBStatusDL18IE_Extension_Entry {}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct DRBStatusDL18IE_Extension(Vec<DRBStatusDL18IE_Extension_Entry>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct DRBStatusULchoice_Extensions {}

#[derive(Debug, AperCodec)]
#[asn(type = "BITSTRING", sz_extensible = false, sz_lb = "1", sz_ub = "2048")]
pub struct BIT_STRING_16(BitVec<Msb0, u8>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct DRBStatusUL12IE_Extension_Entry {}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct DRBStatusUL12IE_Extension(Vec<DRBStatusUL12IE_Extension_Entry>);

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
pub struct DRBStatusUL18IE_Extension_Entry {}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct DRBStatusUL18IE_Extension(Vec<DRBStatusUL18IE_Extension_Entry>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct DRBsSubjectToEarlyStatusTransfer_ItemIE_Extension_Entry {}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct DRBsSubjectToEarlyStatusTransfer_ItemIE_Extension(
    Vec<DRBsSubjectToEarlyStatusTransfer_ItemIE_Extension_Entry>,
);

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum DRBsSubjectToStatusTransferItemIE_Extension_EntryExtensionValue {
    #[asn(key = 159)]
    Id_OldAssociatedQosFlowList_ULendmarkerexpected(AssociatedQosFlowList),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct DRBsSubjectToStatusTransferItemIE_Extension_Entry {
    #[asn(key_field = true)]
    pub id: ProtocolExtensionID,
    pub criticality: Criticality,
    pub extension_value: DRBsSubjectToStatusTransferItemIE_Extension_EntryExtensionValue,
}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct DRBsSubjectToStatusTransferItemIE_Extension(
    Vec<DRBsSubjectToStatusTransferItemIE_Extension_Entry>,
);

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum DRBsToQosFlowsMappingItemIE_Extensions_EntryExtensionValue {
    #[asn(key = 266)]
    Id_DAPSRequestInfo(DAPSRequestInfo),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct DRBsToQosFlowsMappingItemIE_Extensions_Entry {
    #[asn(key_field = true)]
    pub id: ProtocolExtensionID,
    pub criticality: Criticality,
    pub extension_value: DRBsToQosFlowsMappingItemIE_Extensions_EntryExtensionValue,
}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct DRBsToQosFlowsMappingItemIE_Extensions(
    Vec<DRBsToQosFlowsMappingItemIE_Extensions_Entry>,
);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct DataForwardingResponseDRBItemIE_Extensions_Entry {}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct DataForwardingResponseDRBItemIE_Extensions(
    Vec<DataForwardingResponseDRBItemIE_Extensions_Entry>,
);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct DataForwardingResponseERABListItemIE_Extensions_Entry {}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct DataForwardingResponseERABListItemIE_Extensions(
    Vec<DataForwardingResponseERABListItemIE_Extensions_Entry>,
);

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum DeactivateTraceProtocolIEs_EntryValue {
    #[asn(key = 10)]
    Id_AMF_UE_NGAP_ID(AMF_UE_NGAP_ID),
    #[asn(key = 44)]
    Id_NGRANTraceID(NGRANTraceID),
    #[asn(key = 85)]
    Id_RAN_UE_NGAP_ID(RAN_UE_NGAP_ID),
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
    #[asn(key = 10)]
    Id_AMF_UE_NGAP_ID(AMF_UE_NGAP_ID),
    #[asn(key = 0)]
    Id_AllowedNSSAI(AllowedNSSAI),
    #[asn(key = 222)]
    Id_CEmodeBrestricted(CEmodeBrestricted),
    #[asn(key = 226)]
    Id_EndIndication(EndIndication),
    #[asn(key = 205)]
    Id_Enhanced_CoverageRestriction(Enhanced_CoverageRestriction),
    #[asn(key = 206)]
    Id_Extended_ConnectedTime(Extended_ConnectedTime),
    #[asn(key = 31)]
    Id_IndexToRFSP(IndexToRFSP),
    #[asn(key = 36)]
    Id_MobilityRestrictionList(MobilityRestrictionList),
    #[asn(key = 38)]
    Id_NAS_PDU(NAS_PDU),
    #[asn(key = 48)]
    Id_OldAMF(AMFName),
    #[asn(key = 85)]
    Id_RAN_UE_NGAP_ID(RAN_UE_NGAP_ID),
    #[asn(key = 83)]
    Id_RANPagingPriority(RANPagingPriority),
    #[asn(key = 177)]
    Id_SRVCCOperationPossible(SRVCCOperationPossible),
    #[asn(key = 209)]
    Id_UE_DifferentiationInfo(UE_DifferentiationInfo),
    #[asn(key = 110)]
    Id_UEAggregateMaximumBitRate(UEAggregateMaximumBitRate),
    #[asn(key = 228)]
    Id_UECapabilityInfoRequest(UECapabilityInfoRequest),
    #[asn(key = 117)]
    Id_UERadioCapability(UERadioCapability),
    #[asn(key = 264)]
    Id_UERadioCapabilityID(UERadioCapabilityID),
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
pub enum DownlinkNonUEAssociatedNRPPaTransportProtocolIEs_EntryValue {
    #[asn(key = 46)]
    Id_NRPPa_PDU(NRPPa_PDU),
    #[asn(key = 89)]
    Id_RoutingID(RoutingID),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct DownlinkNonUEAssociatedNRPPaTransportProtocolIEs_Entry {
    #[asn(key_field = true)]
    pub id: ProtocolIE_ID,
    pub criticality: Criticality,
    pub value: DownlinkNonUEAssociatedNRPPaTransportProtocolIEs_EntryValue,
}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "0",
    sz_ub = "65535"
)]
pub struct DownlinkNonUEAssociatedNRPPaTransportProtocolIEs(
    Vec<DownlinkNonUEAssociatedNRPPaTransportProtocolIEs_Entry>,
);

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum DownlinkRANConfigurationTransferProtocolIEs_EntryValue {
    #[asn(key = 157)]
    Id_ENDC_SONConfigurationTransferDL(EN_DCSONConfigurationTransfer),
    #[asn(key = 250)]
    Id_IntersystemSONConfigurationTransferDL(IntersystemSONConfigurationTransfer),
    #[asn(key = 98)]
    Id_SONConfigurationTransferDL(SONConfigurationTransfer),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct DownlinkRANConfigurationTransferProtocolIEs_Entry {
    #[asn(key_field = true)]
    pub id: ProtocolIE_ID,
    pub criticality: Criticality,
    pub value: DownlinkRANConfigurationTransferProtocolIEs_EntryValue,
}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "0",
    sz_ub = "65535"
)]
pub struct DownlinkRANConfigurationTransferProtocolIEs(
    Vec<DownlinkRANConfigurationTransferProtocolIEs_Entry>,
);

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum DownlinkRANEarlyStatusTransferProtocolIEs_EntryValue {
    #[asn(key = 10)]
    Id_AMF_UE_NGAP_ID(AMF_UE_NGAP_ID),
    #[asn(key = 268)]
    Id_EarlyStatusTransfer_TransparentContainer(EarlyStatusTransfer_TransparentContainer),
    #[asn(key = 85)]
    Id_RAN_UE_NGAP_ID(RAN_UE_NGAP_ID),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct DownlinkRANEarlyStatusTransferProtocolIEs_Entry {
    #[asn(key_field = true)]
    pub id: ProtocolIE_ID,
    pub criticality: Criticality,
    pub value: DownlinkRANEarlyStatusTransferProtocolIEs_EntryValue,
}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "0",
    sz_ub = "65535"
)]
pub struct DownlinkRANEarlyStatusTransferProtocolIEs(
    Vec<DownlinkRANEarlyStatusTransferProtocolIEs_Entry>,
);

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum DownlinkRANStatusTransferProtocolIEs_EntryValue {
    #[asn(key = 10)]
    Id_AMF_UE_NGAP_ID(AMF_UE_NGAP_ID),
    #[asn(key = 85)]
    Id_RAN_UE_NGAP_ID(RAN_UE_NGAP_ID),
    #[asn(key = 84)]
    Id_RANStatusTransfer_TransparentContainer(RANStatusTransfer_TransparentContainer),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct DownlinkRANStatusTransferProtocolIEs_Entry {
    #[asn(key_field = true)]
    pub id: ProtocolIE_ID,
    pub criticality: Criticality,
    pub value: DownlinkRANStatusTransferProtocolIEs_EntryValue,
}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "0",
    sz_ub = "65535"
)]
pub struct DownlinkRANStatusTransferProtocolIEs(Vec<DownlinkRANStatusTransferProtocolIEs_Entry>);

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum DownlinkRIMInformationTransferProtocolIEs_EntryValue {
    #[asn(key = 175)]
    Id_RIMInformationTransfer(RIMInformationTransfer),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct DownlinkRIMInformationTransferProtocolIEs_Entry {
    #[asn(key_field = true)]
    pub id: ProtocolIE_ID,
    pub criticality: Criticality,
    pub value: DownlinkRIMInformationTransferProtocolIEs_EntryValue,
}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "0",
    sz_ub = "65535"
)]
pub struct DownlinkRIMInformationTransferProtocolIEs(
    Vec<DownlinkRIMInformationTransferProtocolIEs_Entry>,
);

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum DownlinkUEAssociatedNRPPaTransportProtocolIEs_EntryValue {
    #[asn(key = 10)]
    Id_AMF_UE_NGAP_ID(AMF_UE_NGAP_ID),
    #[asn(key = 46)]
    Id_NRPPa_PDU(NRPPa_PDU),
    #[asn(key = 85)]
    Id_RAN_UE_NGAP_ID(RAN_UE_NGAP_ID),
    #[asn(key = 89)]
    Id_RoutingID(RoutingID),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct DownlinkUEAssociatedNRPPaTransportProtocolIEs_Entry {
    #[asn(key_field = true)]
    pub id: ProtocolIE_ID,
    pub criticality: Criticality,
    pub value: DownlinkUEAssociatedNRPPaTransportProtocolIEs_EntryValue,
}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "0",
    sz_ub = "65535"
)]
pub struct DownlinkUEAssociatedNRPPaTransportProtocolIEs(
    Vec<DownlinkUEAssociatedNRPPaTransportProtocolIEs_Entry>,
);

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum Dynamic5QIDescriptorIE_Extensions_EntryExtensionValue {
    #[asn(key = 187)]
    Id_CNPacketDelayBudgetDL(ExtendedPacketDelayBudget),
    #[asn(key = 188)]
    Id_CNPacketDelayBudgetUL(ExtendedPacketDelayBudget),
    #[asn(key = 189)]
    Id_ExtendedPacketDelayBudget(ExtendedPacketDelayBudget),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct Dynamic5QIDescriptorIE_Extensions_Entry {
    #[asn(key_field = true)]
    pub id: ProtocolExtensionID,
    pub criticality: Criticality,
    pub extension_value: Dynamic5QIDescriptorIE_Extensions_EntryExtensionValue,
}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct Dynamic5QIDescriptorIE_Extensions(Vec<Dynamic5QIDescriptorIE_Extensions_Entry>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct E_RABInformationItemIE_Extensions_Entry {}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct E_RABInformationItemIE_Extensions(Vec<E_RABInformationItemIE_Extensions_Entry>);

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
pub struct EPS_TAIIE_Extensions_Entry {}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct EPS_TAIIE_Extensions(Vec<EPS_TAIIE_Extensions_Entry>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct EUTRA_CGIIE_Extensions_Entry {}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct EUTRA_CGIIE_Extensions(Vec<EUTRA_CGIIE_Extensions_Entry>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct EarlyStatusTransfer_TransparentContainerIE_Extensions_Entry {}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct EarlyStatusTransfer_TransparentContainerIE_Extensions(
    Vec<EarlyStatusTransfer_TransparentContainerIE_Extensions_Entry>,
);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct EmergencyAreaIDBroadcastEUTRA_ItemIE_Extensions_Entry {}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct EmergencyAreaIDBroadcastEUTRA_ItemIE_Extensions(
    Vec<EmergencyAreaIDBroadcastEUTRA_ItemIE_Extensions_Entry>,
);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct EmergencyAreaIDBroadcastNR_ItemIE_Extensions_Entry {}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct EmergencyAreaIDBroadcastNR_ItemIE_Extensions(
    Vec<EmergencyAreaIDBroadcastNR_ItemIE_Extensions_Entry>,
);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct EmergencyAreaIDCancelledEUTRA_ItemIE_Extensions_Entry {}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct EmergencyAreaIDCancelledEUTRA_ItemIE_Extensions(
    Vec<EmergencyAreaIDCancelledEUTRA_ItemIE_Extensions_Entry>,
);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct EmergencyAreaIDCancelledNR_ItemIE_Extensions_Entry {}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct EmergencyAreaIDCancelledNR_ItemIE_Extensions(
    Vec<EmergencyAreaIDCancelledNR_ItemIE_Extensions_Entry>,
);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct EmergencyFallbackIndicatorIE_Extensions_Entry {}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct EmergencyFallbackIndicatorIE_Extensions(
    Vec<EmergencyFallbackIndicatorIE_Extensions_Entry>,
);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct EndpointIPAddressAndPortIE_Extensions_Entry {}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct EndpointIPAddressAndPortIE_Extensions(Vec<EndpointIPAddressAndPortIE_Extensions_Entry>);

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum ErrorIndicationProtocolIEs_EntryValue {
    #[asn(key = 10)]
    Id_AMF_UE_NGAP_ID(AMF_UE_NGAP_ID),
    #[asn(key = 15)]
    Id_Cause(Cause),
    #[asn(key = 19)]
    Id_CriticalityDiagnostics(CriticalityDiagnostics),
    #[asn(key = 26)]
    Id_FiveG_S_TMSI(FiveG_S_TMSI),
    #[asn(key = 85)]
    Id_RAN_UE_NGAP_ID(RAN_UE_NGAP_ID),
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
pub struct EventL1LoggedMDTConfigIE_Extensions_Entry {}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct EventL1LoggedMDTConfigIE_Extensions(Vec<EventL1LoggedMDTConfigIE_Extensions_Entry>);

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
#[asn(type = "INTEGER", lb = "0", ub = "4095")]
pub struct INTEGER_23(u16);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct ExpectedUEMovingTrajectoryItemIE_Extensions_Entry {}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct ExpectedUEMovingTrajectoryItemIE_Extensions(
    Vec<ExpectedUEMovingTrajectoryItemIE_Extensions_Entry>,
);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct Extended_AMFNameIE_Extensions_Entry {}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct Extended_AMFNameIE_Extensions(Vec<Extended_AMFNameIE_Extensions_Entry>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct Extended_RANNodeNameIE_Extensions_Entry {}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct Extended_RANNodeNameIE_Extensions(Vec<Extended_RANNodeNameIE_Extensions_Entry>);

#[derive(Debug, AperCodec)]
#[asn(type = "BITSTRING", sz_extensible = true, sz_lb = "8", sz_ub = "8")]
pub struct BIT_STRING_24(BitVec<Msb0, u8>);

#[derive(Debug, AperCodec)]
#[asn(type = "BITSTRING", sz_extensible = true, sz_lb = "8", sz_ub = "8")]
pub struct BIT_STRING_25(BitVec<Msb0, u8>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct ExtendedRATRestrictionInformationIE_Extensions_Entry {}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct ExtendedRATRestrictionInformationIE_Extensions(
    Vec<ExtendedRATRestrictionInformationIE_Extensions_Entry>,
);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct FailureIndicationIE_Extensions_Entry {}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct FailureIndicationIE_Extensions(Vec<FailureIndicationIE_Extensions_Entry>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct FirstDLCountIE_Extension_Entry {}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct FirstDLCountIE_Extension(Vec<FirstDLCountIE_Extension_Entry>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct FiveG_S_TMSIIE_Extensions_Entry {}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct FiveG_S_TMSIIE_Extensions(Vec<FiveG_S_TMSIIE_Extensions_Entry>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct ForbiddenAreaInformation_ItemIE_Extensions_Entry {}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct ForbiddenAreaInformation_ItemIE_Extensions(
    Vec<ForbiddenAreaInformation_ItemIE_Extensions_Entry>,
);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct FromEUTRANtoNGRANIE_Extensions_Entry {}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct FromEUTRANtoNGRANIE_Extensions(Vec<FromEUTRANtoNGRANIE_Extensions_Entry>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct FromNGRANtoEUTRANIE_Extensions_Entry {}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct FromNGRANtoEUTRANIE_Extensions(Vec<FromNGRANtoEUTRANIE_Extensions_Entry>);

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum GBR_QosInformationIE_Extensions_EntryExtensionValue {
    #[asn(key = 220)]
    Id_AlternativeQoSParaSetList(AlternativeQoSParaSetList),
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
#[asn(type = "BITSTRING", sz_extensible = false, sz_lb = "22", sz_ub = "32")]
pub struct BIT_STRING_26(BitVec<Msb0, u8>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct GNB_IDchoice_Extensions {}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct GTPTunnelIE_Extensions_Entry {}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct GTPTunnelIE_Extensions(Vec<GTPTunnelIE_Extensions_Entry>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct GUAMIIE_Extensions_Entry {}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct GUAMIIE_Extensions(Vec<GUAMIIE_Extensions_Entry>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct GlobalENB_IDIE_Extensions_Entry {}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct GlobalENB_IDIE_Extensions(Vec<GlobalENB_IDIE_Extensions_Entry>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct GlobalGNB_IDIE_Extensions_Entry {}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct GlobalGNB_IDIE_Extensions(Vec<GlobalGNB_IDIE_Extensions_Entry>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct GlobalLine_IDIE_Extensions_Entry {}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct GlobalLine_IDIE_Extensions(Vec<GlobalLine_IDIE_Extensions_Entry>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct GlobalN3IWF_IDIE_Extensions_Entry {}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct GlobalN3IWF_IDIE_Extensions(Vec<GlobalN3IWF_IDIE_Extensions_Entry>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct GlobalNgENB_IDIE_Extensions_Entry {}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct GlobalNgENB_IDIE_Extensions(Vec<GlobalNgENB_IDIE_Extensions_Entry>);

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum GlobalRANNodeIDchoice_ExtensionsValue {
    #[asn(key = 240)]
    Id_GlobalTNGF_ID(GlobalTNGF_ID),
    #[asn(key = 241)]
    Id_GlobalTWIF_ID(GlobalTWIF_ID),
    #[asn(key = 242)]
    Id_GlobalW_AGF_ID(GlobalW_AGF_ID),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct GlobalRANNodeIDchoice_Extensions {
    #[asn(key_field = true)]
    pub id: ProtocolIE_ID,
    pub criticality: Criticality,
    pub value: GlobalRANNodeIDchoice_ExtensionsValue,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct GlobalTNGF_IDIE_Extensions_Entry {}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct GlobalTNGF_IDIE_Extensions(Vec<GlobalTNGF_IDIE_Extensions_Entry>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct GlobalTWIF_IDIE_Extensions_Entry {}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct GlobalTWIF_IDIE_Extensions(Vec<GlobalTWIF_IDIE_Extensions_Entry>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct GlobalW_AGF_IDIE_Extensions_Entry {}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct GlobalW_AGF_IDIE_Extensions(Vec<GlobalW_AGF_IDIE_Extensions_Entry>);

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
pub struct HOReportIE_Extensions_Entry {}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct HOReportIE_Extensions(Vec<HOReportIE_Extensions_Entry>);

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum HandoverCancelProtocolIEs_EntryValue {
    #[asn(key = 10)]
    Id_AMF_UE_NGAP_ID(AMF_UE_NGAP_ID),
    #[asn(key = 15)]
    Id_Cause(Cause),
    #[asn(key = 85)]
    Id_RAN_UE_NGAP_ID(RAN_UE_NGAP_ID),
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
    #[asn(key = 10)]
    Id_AMF_UE_NGAP_ID(AMF_UE_NGAP_ID),
    #[asn(key = 19)]
    Id_CriticalityDiagnostics(CriticalityDiagnostics),
    #[asn(key = 85)]
    Id_RAN_UE_NGAP_ID(RAN_UE_NGAP_ID),
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
    #[asn(key = 10)]
    Id_AMF_UE_NGAP_ID(AMF_UE_NGAP_ID),
    #[asn(key = 19)]
    Id_CriticalityDiagnostics(CriticalityDiagnostics),
    #[asn(key = 29)]
    Id_HandoverType(HandoverType),
    #[asn(key = 39)]
    Id_NASSecurityParametersFromNGRAN(NASSecurityParametersFromNGRAN),
    #[asn(key = 59)]
    Id_PDUSessionResourceHandoverList(PDUSessionResourceHandoverList),
    #[asn(key = 78)]
    Id_PDUSessionResourceToReleaseListHOCmd(PDUSessionResourceToReleaseListHOCmd),
    #[asn(key = 85)]
    Id_RAN_UE_NGAP_ID(RAN_UE_NGAP_ID),
    #[asn(key = 106)]
    Id_TargetToSource_TransparentContainer(TargetToSource_TransparentContainer),
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
pub enum HandoverCommandTransferIE_Extensions_EntryExtensionValue {
    #[asn(key = 152)]
    Id_AdditionalDLForwardingUPTNLInformation(QosFlowPerTNLInformationList),
    #[asn(key = 172)]
    Id_AdditionalULForwardingUPTNLInformation(UPTransportLayerInformationList),
    #[asn(key = 249)]
    Id_DataForwardingResponseERABList(DataForwardingResponseERABList),
    #[asn(key = 164)]
    Id_ULForwardingUP_TNLInformation(UPTransportLayerInformation),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct HandoverCommandTransferIE_Extensions_Entry {
    #[asn(key_field = true)]
    pub id: ProtocolExtensionID,
    pub criticality: Criticality,
    pub extension_value: HandoverCommandTransferIE_Extensions_EntryExtensionValue,
}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct HandoverCommandTransferIE_Extensions(Vec<HandoverCommandTransferIE_Extensions_Entry>);

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum HandoverFailureProtocolIEs_EntryValue {
    #[asn(key = 10)]
    Id_AMF_UE_NGAP_ID(AMF_UE_NGAP_ID),
    #[asn(key = 15)]
    Id_Cause(Cause),
    #[asn(key = 19)]
    Id_CriticalityDiagnostics(CriticalityDiagnostics),
    #[asn(key = 262)]
    Id_TargettoSource_Failure_TransparentContainer(TargettoSource_Failure_TransparentContainer),
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
    #[asn(key = 10)]
    Id_AMF_UE_NGAP_ID(AMF_UE_NGAP_ID),
    #[asn(key = 269)]
    Id_NotifySourceNGRANNode(NotifySourceNGRANNode),
    #[asn(key = 85)]
    Id_RAN_UE_NGAP_ID(RAN_UE_NGAP_ID),
    #[asn(key = 121)]
    Id_UserLocationInformation(UserLocationInformation),
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
    #[asn(key = 10)]
    Id_AMF_UE_NGAP_ID(AMF_UE_NGAP_ID),
    #[asn(key = 15)]
    Id_Cause(Cause),
    #[asn(key = 19)]
    Id_CriticalityDiagnostics(CriticalityDiagnostics),
    #[asn(key = 85)]
    Id_RAN_UE_NGAP_ID(RAN_UE_NGAP_ID),
    #[asn(key = 262)]
    Id_TargettoSource_Failure_TransparentContainer(TargettoSource_Failure_TransparentContainer),
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
#[asn(type = "SEQUENCE", extensible = false)]
pub struct HandoverPreparationUnsuccessfulTransferIE_Extensions_Entry {}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct HandoverPreparationUnsuccessfulTransferIE_Extensions(
    Vec<HandoverPreparationUnsuccessfulTransferIE_Extensions_Entry>,
);

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum HandoverRequestProtocolIEs_EntryValue {
    #[asn(key = 10)]
    Id_AMF_UE_NGAP_ID(AMF_UE_NGAP_ID),
    #[asn(key = 0)]
    Id_AllowedNSSAI(AllowedNSSAI),
    #[asn(key = 222)]
    Id_CEmodeBrestricted(CEmodeBrestricted),
    #[asn(key = 165)]
    Id_CNAssistedRANTuning(CNAssistedRANTuning),
    #[asn(key = 15)]
    Id_Cause(Cause),
    #[asn(key = 18)]
    Id_CoreNetworkAssistanceInformationForInactive(CoreNetworkAssistanceInformationForInactive),
    #[asn(key = 205)]
    Id_Enhanced_CoverageRestriction(Enhanced_CoverageRestriction),
    #[asn(key = 206)]
    Id_Extended_ConnectedTime(Extended_ConnectedTime),
    #[asn(key = 28)]
    Id_GUAMI(GUAMI),
    #[asn(key = 29)]
    Id_HandoverType(HandoverType),
    #[asn(key = 199)]
    Id_IAB_Authorized(IAB_Authorized),
    #[asn(key = 217)]
    Id_LTEUESidelinkAggregateMaximumBitrate(LTEUESidelinkAggregateMaximumBitrate),
    #[asn(key = 215)]
    Id_LTEV2XServicesAuthorized(LTEV2XServicesAuthorized),
    #[asn(key = 33)]
    Id_LocationReportingRequestType(LocationReportingRequestType),
    #[asn(key = 254)]
    Id_ManagementBasedMDTPLMNList(MDTPLMNList),
    #[asn(key = 34)]
    Id_MaskedIMEISV(MaskedIMEISV),
    #[asn(key = 36)]
    Id_MobilityRestrictionList(MobilityRestrictionList),
    #[asn(key = 37)]
    Id_NASC(NAS_PDU),
    #[asn(key = 218)]
    Id_NRUESidelinkAggregateMaximumBitrate(NRUESidelinkAggregateMaximumBitrate),
    #[asn(key = 216)]
    Id_NRV2XServicesAuthorized(NRV2XServicesAuthorized),
    #[asn(key = 41)]
    Id_NewSecurityContextInd(NewSecurityContextInd),
    #[asn(key = 219)]
    Id_PC5QoSParameters(PC5QoSParameters),
    #[asn(key = 73)]
    Id_PDUSessionResourceSetupListHOReq(PDUSessionResourceSetupListHOReq),
    #[asn(key = 91)]
    Id_RRCInactiveTransitionReportRequest(RRCInactiveTransitionReportRequest),
    #[asn(key = 146)]
    Id_RedirectionVoiceFallback(RedirectionVoiceFallback),
    #[asn(key = 177)]
    Id_SRVCCOperationPossible(SRVCCOperationPossible),
    #[asn(key = 93)]
    Id_SecurityContext(SecurityContext),
    #[asn(key = 101)]
    Id_SourceToTarget_TransparentContainer(SourceToTarget_TransparentContainer),
    #[asn(key = 108)]
    Id_TraceActivation(TraceActivation),
    #[asn(key = 209)]
    Id_UE_DifferentiationInfo(UE_DifferentiationInfo),
    #[asn(key = 234)]
    Id_UE_UP_CIoT_Support(UE_UP_CIoT_Support),
    #[asn(key = 110)]
    Id_UEAggregateMaximumBitRate(UEAggregateMaximumBitRate),
    #[asn(key = 264)]
    Id_UERadioCapabilityID(UERadioCapabilityID),
    #[asn(key = 119)]
    Id_UESecurityCapabilities(UESecurityCapabilities),
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
    #[asn(key = 10)]
    Id_AMF_UE_NGAP_ID(AMF_UE_NGAP_ID),
    #[asn(key = 19)]
    Id_CriticalityDiagnostics(CriticalityDiagnostics),
    #[asn(key = 53)]
    Id_PDUSessionResourceAdmittedList(PDUSessionResourceAdmittedList),
    #[asn(key = 56)]
    Id_PDUSessionResourceFailedToSetupListHOAck(PDUSessionResourceFailedToSetupListHOAck),
    #[asn(key = 85)]
    Id_RAN_UE_NGAP_ID(RAN_UE_NGAP_ID),
    #[asn(key = 106)]
    Id_TargetToSource_TransparentContainer(TargetToSource_TransparentContainer),
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
pub enum HandoverRequestAcknowledgeTransferIE_Extensions_EntryExtensionValue {
    #[asn(key = 153)]
    Id_AdditionalDLUPTNLInformationForHOList(AdditionalDLUPTNLInformationForHOList),
    #[asn(key = 172)]
    Id_AdditionalULForwardingUPTNLInformation(UPTransportLayerInformationList),
    #[asn(key = 249)]
    Id_DataForwardingResponseERABList(DataForwardingResponseERABList),
    #[asn(key = 27)]
    Id_GlobalRANNodeID(GlobalRANNodeID),
    #[asn(key = 192)]
    Id_RedundantDL_NGU_UP_TNLInformation(UPTransportLayerInformation),
    #[asn(key = 164)]
    Id_ULForwardingUP_TNLInformation(UPTransportLayerInformation),
    #[asn(key = 198)]
    Id_UsedRSNInformation(RedundantPDUSessionInformation),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct HandoverRequestAcknowledgeTransferIE_Extensions_Entry {
    #[asn(key_field = true)]
    pub id: ProtocolExtensionID,
    pub criticality: Criticality,
    pub extension_value: HandoverRequestAcknowledgeTransferIE_Extensions_EntryExtensionValue,
}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct HandoverRequestAcknowledgeTransferIE_Extensions(
    Vec<HandoverRequestAcknowledgeTransferIE_Extensions_Entry>,
);

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum HandoverRequiredProtocolIEs_EntryValue {
    #[asn(key = 10)]
    Id_AMF_UE_NGAP_ID(AMF_UE_NGAP_ID),
    #[asn(key = 15)]
    Id_Cause(Cause),
    #[asn(key = 22)]
    Id_DirectForwardingPathAvailability(DirectForwardingPathAvailability),
    #[asn(key = 29)]
    Id_HandoverType(HandoverType),
    #[asn(key = 61)]
    Id_PDUSessionResourceListHORqd(PDUSessionResourceListHORqd),
    #[asn(key = 85)]
    Id_RAN_UE_NGAP_ID(RAN_UE_NGAP_ID),
    #[asn(key = 101)]
    Id_SourceToTarget_TransparentContainer(SourceToTarget_TransparentContainer),
    #[asn(key = 105)]
    Id_TargetID(TargetID),
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
#[asn(type = "SEQUENCE", extensible = false)]
pub struct HandoverRequiredTransferIE_Extensions_Entry {}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct HandoverRequiredTransferIE_Extensions(Vec<HandoverRequiredTransferIE_Extensions_Entry>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct HandoverResourceAllocationUnsuccessfulTransferIE_Extensions_Entry {}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct HandoverResourceAllocationUnsuccessfulTransferIE_Extensions(
    Vec<HandoverResourceAllocationUnsuccessfulTransferIE_Extensions_Entry>,
);

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum HandoverSuccessProtocolIEs_EntryValue {
    #[asn(key = 10)]
    Id_AMF_UE_NGAP_ID(AMF_UE_NGAP_ID),
    #[asn(key = 85)]
    Id_RAN_UE_NGAP_ID(RAN_UE_NGAP_ID),
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
#[asn(type = "SEQUENCE", extensible = false)]
pub struct ImmediateMDTNrIE_Extensions_Entry {}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct ImmediateMDTNrIE_Extensions(Vec<ImmediateMDTNrIE_Extensions_Entry>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct InfoOnRecommendedCellsAndRANNodesForPagingIE_Extensions_Entry {}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct InfoOnRecommendedCellsAndRANNodesForPagingIE_Extensions(
    Vec<InfoOnRecommendedCellsAndRANNodesForPagingIE_Extensions_Entry>,
);

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum InitialContextSetupFailureProtocolIEs_EntryValue {
    #[asn(key = 10)]
    Id_AMF_UE_NGAP_ID(AMF_UE_NGAP_ID),
    #[asn(key = 15)]
    Id_Cause(Cause),
    #[asn(key = 19)]
    Id_CriticalityDiagnostics(CriticalityDiagnostics),
    #[asn(key = 132)]
    Id_PDUSessionResourceFailedToSetupListCxtFail(PDUSessionResourceFailedToSetupListCxtFail),
    #[asn(key = 85)]
    Id_RAN_UE_NGAP_ID(RAN_UE_NGAP_ID),
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
    #[asn(key = 10)]
    Id_AMF_UE_NGAP_ID(AMF_UE_NGAP_ID),
    #[asn(key = 0)]
    Id_AllowedNSSAI(AllowedNSSAI),
    #[asn(key = 222)]
    Id_CEmodeBrestricted(CEmodeBrestricted),
    #[asn(key = 165)]
    Id_CNAssistedRANTuning(CNAssistedRANTuning),
    #[asn(key = 18)]
    Id_CoreNetworkAssistanceInformationForInactive(CoreNetworkAssistanceInformationForInactive),
    #[asn(key = 24)]
    Id_EmergencyFallbackIndicator(EmergencyFallbackIndicator),
    #[asn(key = 205)]
    Id_Enhanced_CoverageRestriction(Enhanced_CoverageRestriction),
    #[asn(key = 206)]
    Id_Extended_ConnectedTime(Extended_ConnectedTime),
    #[asn(key = 28)]
    Id_GUAMI(GUAMI),
    #[asn(key = 199)]
    Id_IAB_Authorized(IAB_Authorized),
    #[asn(key = 31)]
    Id_IndexToRFSP(IndexToRFSP),
    #[asn(key = 217)]
    Id_LTEUESidelinkAggregateMaximumBitrate(LTEUESidelinkAggregateMaximumBitrate),
    #[asn(key = 215)]
    Id_LTEV2XServicesAuthorized(LTEV2XServicesAuthorized),
    #[asn(key = 33)]
    Id_LocationReportingRequestType(LocationReportingRequestType),
    #[asn(key = 254)]
    Id_ManagementBasedMDTPLMNList(MDTPLMNList),
    #[asn(key = 34)]
    Id_MaskedIMEISV(MaskedIMEISV),
    #[asn(key = 36)]
    Id_MobilityRestrictionList(MobilityRestrictionList),
    #[asn(key = 38)]
    Id_NAS_PDU(NAS_PDU),
    #[asn(key = 218)]
    Id_NRUESidelinkAggregateMaximumBitrate(NRUESidelinkAggregateMaximumBitrate),
    #[asn(key = 216)]
    Id_NRV2XServicesAuthorized(NRV2XServicesAuthorized),
    #[asn(key = 48)]
    Id_OldAMF(AMFName),
    #[asn(key = 219)]
    Id_PC5QoSParameters(PC5QoSParameters),
    #[asn(key = 71)]
    Id_PDUSessionResourceSetupListCxtReq(PDUSessionResourceSetupListCxtReq),
    #[asn(key = 85)]
    Id_RAN_UE_NGAP_ID(RAN_UE_NGAP_ID),
    #[asn(key = 238)]
    Id_RGLevelWirelineAccessCharacteristics(RGLevelWirelineAccessCharacteristics),
    #[asn(key = 91)]
    Id_RRCInactiveTransitionReportRequest(RRCInactiveTransitionReportRequest),
    #[asn(key = 146)]
    Id_RedirectionVoiceFallback(RedirectionVoiceFallback),
    #[asn(key = 177)]
    Id_SRVCCOperationPossible(SRVCCOperationPossible),
    #[asn(key = 94)]
    Id_SecurityKey(SecurityKey),
    #[asn(key = 108)]
    Id_TraceActivation(TraceActivation),
    #[asn(key = 209)]
    Id_UE_DifferentiationInfo(UE_DifferentiationInfo),
    #[asn(key = 234)]
    Id_UE_UP_CIoT_Support(UE_UP_CIoT_Support),
    #[asn(key = 110)]
    Id_UEAggregateMaximumBitRate(UEAggregateMaximumBitRate),
    #[asn(key = 117)]
    Id_UERadioCapability(UERadioCapability),
    #[asn(key = 118)]
    Id_UERadioCapabilityForPaging(UERadioCapabilityForPaging),
    #[asn(key = 264)]
    Id_UERadioCapabilityID(UERadioCapabilityID),
    #[asn(key = 119)]
    Id_UESecurityCapabilities(UESecurityCapabilities),
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
    #[asn(key = 10)]
    Id_AMF_UE_NGAP_ID(AMF_UE_NGAP_ID),
    #[asn(key = 19)]
    Id_CriticalityDiagnostics(CriticalityDiagnostics),
    #[asn(key = 55)]
    Id_PDUSessionResourceFailedToSetupListCxtRes(PDUSessionResourceFailedToSetupListCxtRes),
    #[asn(key = 72)]
    Id_PDUSessionResourceSetupListCxtRes(PDUSessionResourceSetupListCxtRes),
    #[asn(key = 85)]
    Id_RAN_UE_NGAP_ID(RAN_UE_NGAP_ID),
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
    #[asn(key = 3)]
    Id_AMFSetID(AMFSetID),
    #[asn(key = 0)]
    Id_AllowedNSSAI(AllowedNSSAI),
    #[asn(key = 245)]
    Id_AuthenticatedIndication(AuthenticatedIndication),
    #[asn(key = 224)]
    Id_CEmodeBSupport_Indicator(CEmodeBSupport_Indicator),
    #[asn(key = 227)]
    Id_EDT_Session(EDT_Session),
    #[asn(key = 26)]
    Id_FiveG_S_TMSI(FiveG_S_TMSI),
    #[asn(key = 201)]
    Id_IABNodeIndication(IABNodeIndication),
    #[asn(key = 225)]
    Id_LTEM_Indication(LTEM_Indication),
    #[asn(key = 38)]
    Id_NAS_PDU(NAS_PDU),
    #[asn(key = 259)]
    Id_NPN_AccessInformation(NPN_AccessInformation),
    #[asn(key = 85)]
    Id_RAN_UE_NGAP_ID(RAN_UE_NGAP_ID),
    #[asn(key = 90)]
    Id_RRCEstablishmentCause(RRCEstablishmentCause),
    #[asn(key = 174)]
    Id_SelectedPLMNIdentity(PLMNIdentity),
    #[asn(key = 171)]
    Id_SourceToTarget_AMFInformationReroute(SourceToTarget_AMFInformationReroute),
    #[asn(key = 112)]
    Id_UEContextRequest(UEContextRequest),
    #[asn(key = 121)]
    Id_UserLocationInformation(UserLocationInformation),
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
    #[asn(key = 64)]
    Id_AMFCPRelocationIndication(AMFCPRelocationIndication),
    #[asn(key = 0)]
    Id_AMFConfigurationUpdate(AMFConfigurationUpdate),
    #[asn(key = 1)]
    Id_AMFStatusIndication(AMFStatusIndication),
    #[asn(key = 2)]
    Id_CellTrafficTrace(CellTrafficTrace),
    #[asn(key = 65)]
    Id_ConnectionEstablishmentIndication(ConnectionEstablishmentIndication),
    #[asn(key = 3)]
    Id_DeactivateTrace(DeactivateTrace),
    #[asn(key = 4)]
    Id_DownlinkNASTransport(DownlinkNASTransport),
    #[asn(key = 5)]
    Id_DownlinkNonUEAssociatedNRPPaTransport(DownlinkNonUEAssociatedNRPPaTransport),
    #[asn(key = 6)]
    Id_DownlinkRANConfigurationTransfer(DownlinkRANConfigurationTransfer),
    #[asn(key = 63)]
    Id_DownlinkRANEarlyStatusTransfer(DownlinkRANEarlyStatusTransfer),
    #[asn(key = 7)]
    Id_DownlinkRANStatusTransfer(DownlinkRANStatusTransfer),
    #[asn(key = 54)]
    Id_DownlinkRIMInformationTransfer(DownlinkRIMInformationTransfer),
    #[asn(key = 8)]
    Id_DownlinkUEAssociatedNRPPaTransport(DownlinkUEAssociatedNRPPaTransport),
    #[asn(key = 9)]
    Id_ErrorIndication(ErrorIndication),
    #[asn(key = 10)]
    Id_HandoverCancel(HandoverCancel),
    #[asn(key = 11)]
    Id_HandoverNotification(HandoverNotify),
    #[asn(key = 12)]
    Id_HandoverPreparation(HandoverRequired),
    #[asn(key = 13)]
    Id_HandoverResourceAllocation(HandoverRequest),
    #[asn(key = 61)]
    Id_HandoverSuccess(HandoverSuccess),
    #[asn(key = 14)]
    Id_InitialContextSetup(InitialContextSetupRequest),
    #[asn(key = 15)]
    Id_InitialUEMessage(InitialUEMessage),
    #[asn(key = 18)]
    Id_LocationReport(LocationReport),
    #[asn(key = 16)]
    Id_LocationReportingControl(LocationReportingControl),
    #[asn(key = 17)]
    Id_LocationReportingFailureIndication(LocationReportingFailureIndication),
    #[asn(key = 19)]
    Id_NASNonDeliveryIndication(NASNonDeliveryIndication),
    #[asn(key = 20)]
    Id_NGReset(NGReset),
    #[asn(key = 21)]
    Id_NGSetup(NGSetupRequest),
    #[asn(key = 22)]
    Id_OverloadStart(OverloadStart),
    #[asn(key = 23)]
    Id_OverloadStop(OverloadStop),
    #[asn(key = 26)]
    Id_PDUSessionResourceModify(PDUSessionResourceModifyRequest),
    #[asn(key = 27)]
    Id_PDUSessionResourceModifyIndication(PDUSessionResourceModifyIndication),
    #[asn(key = 30)]
    Id_PDUSessionResourceNotify(PDUSessionResourceNotify),
    #[asn(key = 28)]
    Id_PDUSessionResourceRelease(PDUSessionResourceReleaseCommand),
    #[asn(key = 29)]
    Id_PDUSessionResourceSetup(PDUSessionResourceSetupRequest),
    #[asn(key = 32)]
    Id_PWSCancel(PWSCancelRequest),
    #[asn(key = 33)]
    Id_PWSFailureIndication(PWSFailureIndication),
    #[asn(key = 34)]
    Id_PWSRestartIndication(PWSRestartIndication),
    #[asn(key = 24)]
    Id_Paging(Paging),
    #[asn(key = 25)]
    Id_PathSwitchRequest(PathSwitchRequest),
    #[asn(key = 31)]
    Id_PrivateMessage(PrivateMessage),
    #[asn(key = 57)]
    Id_RANCPRelocationIndication(RANCPRelocationIndication),
    #[asn(key = 35)]
    Id_RANConfigurationUpdate(RANConfigurationUpdate),
    #[asn(key = 37)]
    Id_RRCInactiveTransitionReport(RRCInactiveTransitionReport),
    #[asn(key = 36)]
    Id_RerouteNASRequest(RerouteNASRequest),
    #[asn(key = 55)]
    Id_RetrieveUEInformation(RetrieveUEInformation),
    #[asn(key = 52)]
    Id_SecondaryRATDataUsageReport(SecondaryRATDataUsageReport),
    #[asn(key = 38)]
    Id_TraceFailureIndication(TraceFailureIndication),
    #[asn(key = 39)]
    Id_TraceStart(TraceStart),
    #[asn(key = 40)]
    Id_UEContextModification(UEContextModificationRequest),
    #[asn(key = 41)]
    Id_UEContextRelease(UEContextReleaseCommand),
    #[asn(key = 42)]
    Id_UEContextReleaseRequest(UEContextReleaseRequest),
    #[asn(key = 58)]
    Id_UEContextResume(UEContextResumeRequest),
    #[asn(key = 59)]
    Id_UEContextSuspend(UEContextSuspendRequest),
    #[asn(key = 56)]
    Id_UEInformationTransfer(UEInformationTransfer),
    #[asn(key = 43)]
    Id_UERadioCapabilityCheck(UERadioCapabilityCheckRequest),
    #[asn(key = 60)]
    Id_UERadioCapabilityIDMapping(UERadioCapabilityIDMappingRequest),
    #[asn(key = 44)]
    Id_UERadioCapabilityInfoIndication(UERadioCapabilityInfoIndication),
    #[asn(key = 45)]
    Id_UETNLABindingRelease(UETNLABindingReleaseRequest),
    #[asn(key = 46)]
    Id_UplinkNASTransport(UplinkNASTransport),
    #[asn(key = 47)]
    Id_UplinkNonUEAssociatedNRPPaTransport(UplinkNonUEAssociatedNRPPaTransport),
    #[asn(key = 48)]
    Id_UplinkRANConfigurationTransfer(UplinkRANConfigurationTransfer),
    #[asn(key = 62)]
    Id_UplinkRANEarlyStatusTransfer(UplinkRANEarlyStatusTransfer),
    #[asn(key = 49)]
    Id_UplinkRANStatusTransfer(UplinkRANStatusTransfer),
    #[asn(key = 53)]
    Id_UplinkRIMInformationTransfer(UplinkRIMInformationTransfer),
    #[asn(key = 50)]
    Id_UplinkUEAssociatedNRPPaTransport(UplinkUEAssociatedNRPPaTransport),
    #[asn(key = 51)]
    Id_WriteReplaceWarning(WriteReplaceWarningRequest),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct InterSystemFailureIndicationIE_Extensions_Entry {}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct InterSystemFailureIndicationIE_Extensions(
    Vec<InterSystemFailureIndicationIE_Extensions_Entry>,
);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct InterSystemHOReportIE_Extensions_Entry {}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct InterSystemHOReportIE_Extensions(Vec<InterSystemHOReportIE_Extensions_Entry>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct InterSystemHandoverReportTypechoice_Extensions {}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct IntersystemSONConfigurationTransferIE_Extensions_Entry {}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct IntersystemSONConfigurationTransferIE_Extensions(
    Vec<IntersystemSONConfigurationTransferIE_Extensions_Entry>,
);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct IntersystemSONInformationchoice_Extensions {}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct IntersystemSONInformationReportchoice_Extensions {}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct IntersystemSONNGRANnodeIDIE_Extensions_Entry {}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct IntersystemSONNGRANnodeIDIE_Extensions(
    Vec<IntersystemSONNGRANnodeIDIE_Extensions_Entry>,
);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct IntersystemSONTransferTypechoice_Extensions {}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct IntersystemSONeNBIDIE_Extensions_Entry {}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct IntersystemSONeNBIDIE_Extensions(Vec<IntersystemSONeNBIDIE_Extensions_Entry>);

#[derive(Debug, AperCodec)]
#[asn(type = "ENUMERATED", extensible = true, lb = "0", ub = "1")]
pub struct ENUMERATED_29(u8);
impl ENUMERATED_29 {
    const TRUE: u8 = 0u8;
    const FALSE: u8 = 1u8;
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct IntersystemUnnecessaryHOIE_Extensions_Entry {}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct IntersystemUnnecessaryHOIE_Extensions(Vec<IntersystemUnnecessaryHOIE_Extensions_Entry>);

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
#[asn(type = "SEQUENCE", extensible = false)]
pub struct LTEUESidelinkAggregateMaximumBitrateIE_Extensions_Entry {}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct LTEUESidelinkAggregateMaximumBitrateIE_Extensions(
    Vec<LTEUESidelinkAggregateMaximumBitrateIE_Extensions_Entry>,
);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct LTEV2XServicesAuthorizedIE_Extensions_Entry {}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct LTEV2XServicesAuthorizedIE_Extensions(Vec<LTEV2XServicesAuthorizedIE_Extensions_Entry>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct LastVisitedCellInformationchoice_Extensions {}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct LastVisitedCellItemIE_Extensions_Entry {}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct LastVisitedCellItemIE_Extensions(Vec<LastVisitedCellItemIE_Extensions_Entry>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct LastVisitedNGRANCellInformationIE_Extensions_Entry {}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct LastVisitedNGRANCellInformationIE_Extensions(
    Vec<LastVisitedNGRANCellInformationIE_Extensions_Entry>,
);

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum LocationReportProtocolIEs_EntryValue {
    #[asn(key = 10)]
    Id_AMF_UE_NGAP_ID(AMF_UE_NGAP_ID),
    #[asn(key = 33)]
    Id_LocationReportingRequestType(LocationReportingRequestType),
    #[asn(key = 85)]
    Id_RAN_UE_NGAP_ID(RAN_UE_NGAP_ID),
    #[asn(key = 116)]
    Id_UEPresenceInAreaOfInterestList(UEPresenceInAreaOfInterestList),
    #[asn(key = 121)]
    Id_UserLocationInformation(UserLocationInformation),
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
    #[asn(key = 10)]
    Id_AMF_UE_NGAP_ID(AMF_UE_NGAP_ID),
    #[asn(key = 33)]
    Id_LocationReportingRequestType(LocationReportingRequestType),
    #[asn(key = 85)]
    Id_RAN_UE_NGAP_ID(RAN_UE_NGAP_ID),
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
    #[asn(key = 10)]
    Id_AMF_UE_NGAP_ID(AMF_UE_NGAP_ID),
    #[asn(key = 15)]
    Id_Cause(Cause),
    #[asn(key = 85)]
    Id_RAN_UE_NGAP_ID(RAN_UE_NGAP_ID),
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
#[asn(type = "OPEN")]
pub enum LocationReportingRequestTypeIE_Extensions_EntryExtensionValue {
    #[asn(key = 170)]
    Id_LocationReportingAdditionalInfo(LocationReportingAdditionalInfo),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct LocationReportingRequestTypeIE_Extensions_Entry {
    #[asn(key_field = true)]
    pub id: ProtocolExtensionID,
    pub criticality: Criticality,
    pub extension_value: LocationReportingRequestTypeIE_Extensions_EntryExtensionValue,
}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct LocationReportingRequestTypeIE_Extensions(
    Vec<LocationReportingRequestTypeIE_Extensions_Entry>,
);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct LoggedMDTNrIE_Extensions_Entry {}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct LoggedMDTNrIE_Extensions(Vec<LoggedMDTNrIE_Extensions_Entry>);

#[derive(Debug, AperCodec)]
#[asn(type = "NULL")]
pub struct NULL_30;

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct LoggedMDTTriggerchoice_Extensions {}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct M1ConfigurationIE_Extensions_Entry {}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct M1ConfigurationIE_Extensions(Vec<M1ConfigurationIE_Extensions_Entry>);

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
pub struct M1ThresholdTypechoice_Extensions {}

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
#[asn(type = "SEQUENCE", extensible = false)]
pub struct MDT_ConfigurationIE_Extensions_Entry {}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct MDT_ConfigurationIE_Extensions(Vec<MDT_ConfigurationIE_Extensions_Entry>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct MDT_Configuration_EUTRAIE_Extensions_Entry {}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct MDT_Configuration_EUTRAIE_Extensions(Vec<MDT_Configuration_EUTRAIE_Extensions_Entry>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct MDT_Configuration_NRIE_Extensions_Entry {}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct MDT_Configuration_NRIE_Extensions(Vec<MDT_Configuration_NRIE_Extensions_Entry>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct MDT_Location_InfoIE_Extensions_Entry {}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct MDT_Location_InfoIE_Extensions(Vec<MDT_Location_InfoIE_Extensions_Entry>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct MDTModeNrchoice_Extensions {}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct MeasurementThresholdL1LoggedMDTchoice_Extensions {}

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum MobilityRestrictionListIE_Extensions_EntryExtensionValue {
    #[asn(key = 160)]
    Id_CNTypeRestrictionsForEquivalent(CNTypeRestrictionsForEquivalent),
    #[asn(key = 161)]
    Id_CNTypeRestrictionsForServing(CNTypeRestrictionsForServing),
    #[asn(key = 150)]
    Id_LastEUTRAN_PLMNIdentity(PLMNIdentity),
    #[asn(key = 261)]
    Id_NPN_MobilityInformation(NPN_MobilityInformation),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct MobilityRestrictionListIE_Extensions_Entry {
    #[asn(key_field = true)]
    pub id: ProtocolExtensionID,
    pub criticality: Criticality,
    pub extension_value: MobilityRestrictionListIE_Extensions_EntryExtensionValue,
}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct MobilityRestrictionListIE_Extensions(Vec<MobilityRestrictionListIE_Extensions_Entry>);

#[derive(Debug, AperCodec)]
#[asn(type = "BITSTRING", sz_extensible = false, sz_lb = "16", sz_ub = "16")]
pub struct BIT_STRING_31(BitVec<Msb0, u8>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct N3IWF_IDchoice_Extensions {}

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum NASNonDeliveryIndicationProtocolIEs_EntryValue {
    #[asn(key = 10)]
    Id_AMF_UE_NGAP_ID(AMF_UE_NGAP_ID),
    #[asn(key = 15)]
    Id_Cause(Cause),
    #[asn(key = 38)]
    Id_NAS_PDU(NAS_PDU),
    #[asn(key = 85)]
    Id_RAN_UE_NGAP_ID(RAN_UE_NGAP_ID),
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
pub struct NB_IoT_Paging_eDRXInfoIE_Extensions_Entry {}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct NB_IoT_Paging_eDRXInfoIE_Extensions(Vec<NB_IoT_Paging_eDRXInfoIE_Extensions_Entry>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct NGRAN_CGIchoice_Extensions {}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct NGRAN_TNLAssociationToRemoveItemIE_Extensions_Entry {}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct NGRAN_TNLAssociationToRemoveItemIE_Extensions(
    Vec<NGRAN_TNLAssociationToRemoveItemIE_Extensions_Entry>,
);

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum NGResetProtocolIEs_EntryValue {
    #[asn(key = 15)]
    Id_Cause(Cause),
    #[asn(key = 88)]
    Id_ResetType(ResetType),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct NGResetProtocolIEs_Entry {
    #[asn(key_field = true)]
    pub id: ProtocolIE_ID,
    pub criticality: Criticality,
    pub value: NGResetProtocolIEs_EntryValue,
}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "0",
    sz_ub = "65535"
)]
pub struct NGResetProtocolIEs(Vec<NGResetProtocolIEs_Entry>);

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum NGResetAcknowledgeProtocolIEs_EntryValue {
    #[asn(key = 19)]
    Id_CriticalityDiagnostics(CriticalityDiagnostics),
    #[asn(key = 111)]
    Id_UE_associatedLogicalNG_connectionList(UE_associatedLogicalNG_connectionList),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct NGResetAcknowledgeProtocolIEs_Entry {
    #[asn(key_field = true)]
    pub id: ProtocolIE_ID,
    pub criticality: Criticality,
    pub value: NGResetAcknowledgeProtocolIEs_EntryValue,
}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "0",
    sz_ub = "65535"
)]
pub struct NGResetAcknowledgeProtocolIEs(Vec<NGResetAcknowledgeProtocolIEs_Entry>);

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum NGSetupFailureProtocolIEs_EntryValue {
    #[asn(key = 15)]
    Id_Cause(Cause),
    #[asn(key = 19)]
    Id_CriticalityDiagnostics(CriticalityDiagnostics),
    #[asn(key = 107)]
    Id_TimeToWait(TimeToWait),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct NGSetupFailureProtocolIEs_Entry {
    #[asn(key_field = true)]
    pub id: ProtocolIE_ID,
    pub criticality: Criticality,
    pub value: NGSetupFailureProtocolIEs_EntryValue,
}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "0",
    sz_ub = "65535"
)]
pub struct NGSetupFailureProtocolIEs(Vec<NGSetupFailureProtocolIEs_Entry>);

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum NGSetupRequestProtocolIEs_EntryValue {
    #[asn(key = 21)]
    Id_DefaultPagingDRX(PagingDRX),
    #[asn(key = 273)]
    Id_Extended_RANNodeName(Extended_RANNodeName),
    #[asn(key = 27)]
    Id_GlobalRANNodeID(GlobalRANNodeID),
    #[asn(key = 204)]
    Id_NB_IoT_DefaultPagingDRX(NB_IoT_DefaultPagingDRX),
    #[asn(key = 82)]
    Id_RANNodeName(RANNodeName),
    #[asn(key = 102)]
    Id_SupportedTAList(SupportedTAList),
    #[asn(key = 147)]
    Id_UERetentionInformation(UERetentionInformation),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct NGSetupRequestProtocolIEs_Entry {
    #[asn(key_field = true)]
    pub id: ProtocolIE_ID,
    pub criticality: Criticality,
    pub value: NGSetupRequestProtocolIEs_EntryValue,
}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "0",
    sz_ub = "65535"
)]
pub struct NGSetupRequestProtocolIEs(Vec<NGSetupRequestProtocolIEs_Entry>);

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum NGSetupResponseProtocolIEs_EntryValue {
    #[asn(key = 1)]
    Id_AMFName(AMFName),
    #[asn(key = 19)]
    Id_CriticalityDiagnostics(CriticalityDiagnostics),
    #[asn(key = 274)]
    Id_Extended_AMFName(Extended_AMFName),
    #[asn(key = 200)]
    Id_IAB_Supported(IAB_Supported),
    #[asn(key = 80)]
    Id_PLMNSupportList(PLMNSupportList),
    #[asn(key = 86)]
    Id_RelativeAMFCapacity(RelativeAMFCapacity),
    #[asn(key = 96)]
    Id_ServedGUAMIList(ServedGUAMIList),
    #[asn(key = 147)]
    Id_UERetentionInformation(UERetentionInformation),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct NGSetupResponseProtocolIEs_Entry {
    #[asn(key_field = true)]
    pub id: ProtocolIE_ID,
    pub criticality: Criticality,
    pub value: NGSetupResponseProtocolIEs_EntryValue,
}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "0",
    sz_ub = "65535"
)]
pub struct NGSetupResponseProtocolIEs(Vec<NGSetupResponseProtocolIEs_Entry>);

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
pub struct NRFrequencyBandItemIE_Extension_Entry {}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct NRFrequencyBandItemIE_Extension(Vec<NRFrequencyBandItemIE_Extension_Entry>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct NRFrequencyInfoIE_Extension_Entry {}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct NRFrequencyInfoIE_Extension(Vec<NRFrequencyInfoIE_Extension_Entry>);

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
pub enum NonDynamic5QIDescriptorIE_Extensions_EntryExtensionValue {
    #[asn(key = 187)]
    Id_CNPacketDelayBudgetDL(ExtendedPacketDelayBudget),
    #[asn(key = 188)]
    Id_CNPacketDelayBudgetUL(ExtendedPacketDelayBudget),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct NonDynamic5QIDescriptorIE_Extensions_Entry {
    #[asn(key_field = true)]
    pub id: ProtocolExtensionID,
    pub criticality: Criticality,
    pub extension_value: NonDynamic5QIDescriptorIE_Extensions_EntryExtensionValue,
}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct NonDynamic5QIDescriptorIE_Extensions(Vec<NonDynamic5QIDescriptorIE_Extensions_Entry>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct OverloadResponsechoice_Extensions {}

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum OverloadStartProtocolIEs_EntryValue {
    #[asn(key = 2)]
    Id_AMFOverloadResponse(OverloadResponse),
    #[asn(key = 9)]
    Id_AMFTrafficLoadReductionIndication(TrafficLoadReductionIndication),
    #[asn(key = 49)]
    Id_OverloadStartNSSAIList(OverloadStartNSSAIList),
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
#[asn(type = "SEQUENCE", extensible = false)]
pub struct OverloadStartNSSAIItemIE_Extensions_Entry {}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct OverloadStartNSSAIItemIE_Extensions(Vec<OverloadStartNSSAIItemIE_Extensions_Entry>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct OverloadStopProtocolIEs_Entry {}

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
pub struct PDUSessionAggregateMaximumBitRateIE_Extensions_Entry {}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct PDUSessionAggregateMaximumBitRateIE_Extensions(
    Vec<PDUSessionAggregateMaximumBitRateIE_Extensions_Entry>,
);

#[derive(Debug, AperCodec)]
#[asn(type = "OCTET-STRING")]
pub struct OCTET_STRING_35(Vec<u8>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct PDUSessionResourceAdmittedItemIE_Extensions_Entry {}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct PDUSessionResourceAdmittedItemIE_Extensions(
    Vec<PDUSessionResourceAdmittedItemIE_Extensions_Entry>,
);

#[derive(Debug, AperCodec)]
#[asn(type = "OCTET-STRING")]
pub struct OCTET_STRING_36(Vec<u8>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct PDUSessionResourceFailedToModifyItemModCfmIE_Extensions_Entry {}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct PDUSessionResourceFailedToModifyItemModCfmIE_Extensions(
    Vec<PDUSessionResourceFailedToModifyItemModCfmIE_Extensions_Entry>,
);

#[derive(Debug, AperCodec)]
#[asn(type = "OCTET-STRING")]
pub struct OCTET_STRING_37(Vec<u8>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct PDUSessionResourceFailedToModifyItemModResIE_Extensions_Entry {}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct PDUSessionResourceFailedToModifyItemModResIE_Extensions(
    Vec<PDUSessionResourceFailedToModifyItemModResIE_Extensions_Entry>,
);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct PDUSessionResourceFailedToResumeItemRESReqIE_Extensions_Entry {}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct PDUSessionResourceFailedToResumeItemRESReqIE_Extensions(
    Vec<PDUSessionResourceFailedToResumeItemRESReqIE_Extensions_Entry>,
);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct PDUSessionResourceFailedToResumeItemRESResIE_Extensions_Entry {}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct PDUSessionResourceFailedToResumeItemRESResIE_Extensions(
    Vec<PDUSessionResourceFailedToResumeItemRESResIE_Extensions_Entry>,
);

#[derive(Debug, AperCodec)]
#[asn(type = "OCTET-STRING")]
pub struct OCTET_STRING_38(Vec<u8>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct PDUSessionResourceFailedToSetupItemCxtFailIE_Extensions_Entry {}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct PDUSessionResourceFailedToSetupItemCxtFailIE_Extensions(
    Vec<PDUSessionResourceFailedToSetupItemCxtFailIE_Extensions_Entry>,
);

#[derive(Debug, AperCodec)]
#[asn(type = "OCTET-STRING")]
pub struct OCTET_STRING_39(Vec<u8>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct PDUSessionResourceFailedToSetupItemCxtResIE_Extensions_Entry {}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct PDUSessionResourceFailedToSetupItemCxtResIE_Extensions(
    Vec<PDUSessionResourceFailedToSetupItemCxtResIE_Extensions_Entry>,
);

#[derive(Debug, AperCodec)]
#[asn(type = "OCTET-STRING")]
pub struct OCTET_STRING_40(Vec<u8>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct PDUSessionResourceFailedToSetupItemHOAckIE_Extensions_Entry {}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct PDUSessionResourceFailedToSetupItemHOAckIE_Extensions(
    Vec<PDUSessionResourceFailedToSetupItemHOAckIE_Extensions_Entry>,
);

#[derive(Debug, AperCodec)]
#[asn(type = "OCTET-STRING")]
pub struct OCTET_STRING_41(Vec<u8>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct PDUSessionResourceFailedToSetupItemPSReqIE_Extensions_Entry {}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct PDUSessionResourceFailedToSetupItemPSReqIE_Extensions(
    Vec<PDUSessionResourceFailedToSetupItemPSReqIE_Extensions_Entry>,
);

#[derive(Debug, AperCodec)]
#[asn(type = "OCTET-STRING")]
pub struct OCTET_STRING_42(Vec<u8>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct PDUSessionResourceFailedToSetupItemSUResIE_Extensions_Entry {}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct PDUSessionResourceFailedToSetupItemSUResIE_Extensions(
    Vec<PDUSessionResourceFailedToSetupItemSUResIE_Extensions_Entry>,
);

#[derive(Debug, AperCodec)]
#[asn(type = "OCTET-STRING")]
pub struct OCTET_STRING_43(Vec<u8>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct PDUSessionResourceHandoverItemIE_Extensions_Entry {}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct PDUSessionResourceHandoverItemIE_Extensions(
    Vec<PDUSessionResourceHandoverItemIE_Extensions_Entry>,
);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct PDUSessionResourceInformationItemIE_Extensions_Entry {}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct PDUSessionResourceInformationItemIE_Extensions(
    Vec<PDUSessionResourceInformationItemIE_Extensions_Entry>,
);

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum PDUSessionResourceItemCxtRelCplIE_Extensions_EntryExtensionValue {}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct PDUSessionResourceItemCxtRelCplIE_Extensions_Entry {
    #[asn(key_field = true)]
    pub id: ProtocolExtensionID,
    pub criticality: Criticality,
    pub extension_value: PDUSessionResourceItemCxtRelCplIE_Extensions_EntryExtensionValue,
}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct PDUSessionResourceItemCxtRelCplIE_Extensions(
    Vec<PDUSessionResourceItemCxtRelCplIE_Extensions_Entry>,
);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct PDUSessionResourceItemCxtRelReqIE_Extensions_Entry {}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct PDUSessionResourceItemCxtRelReqIE_Extensions(
    Vec<PDUSessionResourceItemCxtRelReqIE_Extensions_Entry>,
);

#[derive(Debug, AperCodec)]
#[asn(type = "OCTET-STRING")]
pub struct OCTET_STRING_44(Vec<u8>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct PDUSessionResourceItemHORqdIE_Extensions_Entry {}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct PDUSessionResourceItemHORqdIE_Extensions(
    Vec<PDUSessionResourceItemHORqdIE_Extensions_Entry>,
);

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum PDUSessionResourceModifyConfirmProtocolIEs_EntryValue {
    #[asn(key = 10)]
    Id_AMF_UE_NGAP_ID(AMF_UE_NGAP_ID),
    #[asn(key = 19)]
    Id_CriticalityDiagnostics(CriticalityDiagnostics),
    #[asn(key = 131)]
    Id_PDUSessionResourceFailedToModifyListModCfm(PDUSessionResourceFailedToModifyListModCfm),
    #[asn(key = 62)]
    Id_PDUSessionResourceModifyListModCfm(PDUSessionResourceModifyListModCfm),
    #[asn(key = 85)]
    Id_RAN_UE_NGAP_ID(RAN_UE_NGAP_ID),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct PDUSessionResourceModifyConfirmProtocolIEs_Entry {
    #[asn(key_field = true)]
    pub id: ProtocolIE_ID,
    pub criticality: Criticality,
    pub value: PDUSessionResourceModifyConfirmProtocolIEs_EntryValue,
}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "0",
    sz_ub = "65535"
)]
pub struct PDUSessionResourceModifyConfirmProtocolIEs(
    Vec<PDUSessionResourceModifyConfirmProtocolIEs_Entry>,
);

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum PDUSessionResourceModifyConfirmTransferIE_Extensions_EntryExtensionValue {
    #[asn(key = 185)]
    Id_AdditionalRedundantNGU_UP_TNLInformation(UPTransportLayerInformationPairList),
    #[asn(key = 195)]
    Id_RedundantUL_NGU_UP_TNLInformation(UPTransportLayerInformation),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct PDUSessionResourceModifyConfirmTransferIE_Extensions_Entry {
    #[asn(key_field = true)]
    pub id: ProtocolExtensionID,
    pub criticality: Criticality,
    pub extension_value: PDUSessionResourceModifyConfirmTransferIE_Extensions_EntryExtensionValue,
}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct PDUSessionResourceModifyConfirmTransferIE_Extensions(
    Vec<PDUSessionResourceModifyConfirmTransferIE_Extensions_Entry>,
);

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum PDUSessionResourceModifyIndicationProtocolIEs_EntryValue {
    #[asn(key = 10)]
    Id_AMF_UE_NGAP_ID(AMF_UE_NGAP_ID),
    #[asn(key = 63)]
    Id_PDUSessionResourceModifyListModInd(PDUSessionResourceModifyListModInd),
    #[asn(key = 85)]
    Id_RAN_UE_NGAP_ID(RAN_UE_NGAP_ID),
    #[asn(key = 121)]
    Id_UserLocationInformation(UserLocationInformation),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct PDUSessionResourceModifyIndicationProtocolIEs_Entry {
    #[asn(key_field = true)]
    pub id: ProtocolIE_ID,
    pub criticality: Criticality,
    pub value: PDUSessionResourceModifyIndicationProtocolIEs_EntryValue,
}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "0",
    sz_ub = "65535"
)]
pub struct PDUSessionResourceModifyIndicationProtocolIEs(
    Vec<PDUSessionResourceModifyIndicationProtocolIEs_Entry>,
);

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum PDUSessionResourceModifyIndicationTransferIE_Extensions_EntryExtensionValue {
    #[asn(key = 184)]
    Id_AdditionalRedundantDLQosFlowPerTNLInformation(QosFlowPerTNLInformationList),
    #[asn(key = 27)]
    Id_GlobalRANNodeID(GlobalRANNodeID),
    #[asn(key = 193)]
    Id_RedundantDLQosFlowPerTNLInformation(QosFlowPerTNLInformation),
    #[asn(key = 144)]
    Id_SecondaryRATUsageInformation(SecondaryRATUsageInformation),
    #[asn(key = 156)]
    Id_SecurityResult(SecurityResult),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct PDUSessionResourceModifyIndicationTransferIE_Extensions_Entry {
    #[asn(key_field = true)]
    pub id: ProtocolExtensionID,
    pub criticality: Criticality,
    pub extension_value:
        PDUSessionResourceModifyIndicationTransferIE_Extensions_EntryExtensionValue,
}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct PDUSessionResourceModifyIndicationTransferIE_Extensions(
    Vec<PDUSessionResourceModifyIndicationTransferIE_Extensions_Entry>,
);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct PDUSessionResourceModifyIndicationUnsuccessfulTransferIE_Extensions_Entry {}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct PDUSessionResourceModifyIndicationUnsuccessfulTransferIE_Extensions(
    Vec<PDUSessionResourceModifyIndicationUnsuccessfulTransferIE_Extensions_Entry>,
);

#[derive(Debug, AperCodec)]
#[asn(type = "OCTET-STRING")]
pub struct OCTET_STRING_45(Vec<u8>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct PDUSessionResourceModifyItemModCfmIE_Extensions_Entry {}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct PDUSessionResourceModifyItemModCfmIE_Extensions(
    Vec<PDUSessionResourceModifyItemModCfmIE_Extensions_Entry>,
);

#[derive(Debug, AperCodec)]
#[asn(type = "OCTET-STRING")]
pub struct OCTET_STRING_46(Vec<u8>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct PDUSessionResourceModifyItemModIndIE_Extensions_Entry {}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct PDUSessionResourceModifyItemModIndIE_Extensions(
    Vec<PDUSessionResourceModifyItemModIndIE_Extensions_Entry>,
);

#[derive(Debug, AperCodec)]
#[asn(type = "OCTET-STRING")]
pub struct OCTET_STRING_47(Vec<u8>);

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum PDUSessionResourceModifyItemModReqIE_Extensions_EntryExtensionValue {
    #[asn(key = 148)]
    Id_S_NSSAI(S_NSSAI),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct PDUSessionResourceModifyItemModReqIE_Extensions_Entry {
    #[asn(key_field = true)]
    pub id: ProtocolExtensionID,
    pub criticality: Criticality,
    pub extension_value: PDUSessionResourceModifyItemModReqIE_Extensions_EntryExtensionValue,
}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct PDUSessionResourceModifyItemModReqIE_Extensions(
    Vec<PDUSessionResourceModifyItemModReqIE_Extensions_Entry>,
);

#[derive(Debug, AperCodec)]
#[asn(type = "OCTET-STRING")]
pub struct OCTET_STRING_48(Vec<u8>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct PDUSessionResourceModifyItemModResIE_Extensions_Entry {}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct PDUSessionResourceModifyItemModResIE_Extensions(
    Vec<PDUSessionResourceModifyItemModResIE_Extensions_Entry>,
);

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum PDUSessionResourceModifyRequestProtocolIEs_EntryValue {
    #[asn(key = 10)]
    Id_AMF_UE_NGAP_ID(AMF_UE_NGAP_ID),
    #[asn(key = 64)]
    Id_PDUSessionResourceModifyListModReq(PDUSessionResourceModifyListModReq),
    #[asn(key = 85)]
    Id_RAN_UE_NGAP_ID(RAN_UE_NGAP_ID),
    #[asn(key = 83)]
    Id_RANPagingPriority(RANPagingPriority),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct PDUSessionResourceModifyRequestProtocolIEs_Entry {
    #[asn(key_field = true)]
    pub id: ProtocolIE_ID,
    pub criticality: Criticality,
    pub value: PDUSessionResourceModifyRequestProtocolIEs_EntryValue,
}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "0",
    sz_ub = "65535"
)]
pub struct PDUSessionResourceModifyRequestProtocolIEs(
    Vec<PDUSessionResourceModifyRequestProtocolIEs_Entry>,
);

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum PDUSessionResourceModifyRequestTransferProtocolIEs_EntryValue {
    #[asn(key = 186)]
    Id_AdditionalRedundantUL_NGU_UP_TNLInformation(UPTransportLayerInformationList),
    #[asn(key = 126)]
    Id_AdditionalUL_NGU_UP_TNLInformation(UPTransportLayerInformationList),
    #[asn(key = 166)]
    Id_CommonNetworkInstance(CommonNetworkInstance),
    #[asn(key = 129)]
    Id_NetworkInstance(NetworkInstance),
    #[asn(key = 130)]
    Id_PDUSessionAggregateMaximumBitRate(PDUSessionAggregateMaximumBitRate),
    #[asn(key = 135)]
    Id_QosFlowAddOrModifyRequestList(QosFlowAddOrModifyRequestList),
    #[asn(key = 137)]
    Id_QosFlowToReleaseList(QosFlowListWithCause),
    #[asn(key = 190)]
    Id_RedundantCommonNetworkInstance(CommonNetworkInstance),
    #[asn(key = 195)]
    Id_RedundantUL_NGU_UP_TNLInformation(UPTransportLayerInformation),
    #[asn(key = 138)]
    Id_SecurityIndication(SecurityIndication),
    #[asn(key = 140)]
    Id_UL_NGU_UP_TNLModifyList(UL_NGU_UP_TNLModifyList),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct PDUSessionResourceModifyRequestTransferProtocolIEs_Entry {
    #[asn(key_field = true)]
    pub id: ProtocolIE_ID,
    pub criticality: Criticality,
    pub value: PDUSessionResourceModifyRequestTransferProtocolIEs_EntryValue,
}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "0",
    sz_ub = "65535"
)]
pub struct PDUSessionResourceModifyRequestTransferProtocolIEs(
    Vec<PDUSessionResourceModifyRequestTransferProtocolIEs_Entry>,
);

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum PDUSessionResourceModifyResponseProtocolIEs_EntryValue {
    #[asn(key = 10)]
    Id_AMF_UE_NGAP_ID(AMF_UE_NGAP_ID),
    #[asn(key = 19)]
    Id_CriticalityDiagnostics(CriticalityDiagnostics),
    #[asn(key = 54)]
    Id_PDUSessionResourceFailedToModifyListModRes(PDUSessionResourceFailedToModifyListModRes),
    #[asn(key = 65)]
    Id_PDUSessionResourceModifyListModRes(PDUSessionResourceModifyListModRes),
    #[asn(key = 85)]
    Id_RAN_UE_NGAP_ID(RAN_UE_NGAP_ID),
    #[asn(key = 121)]
    Id_UserLocationInformation(UserLocationInformation),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct PDUSessionResourceModifyResponseProtocolIEs_Entry {
    #[asn(key_field = true)]
    pub id: ProtocolIE_ID,
    pub criticality: Criticality,
    pub value: PDUSessionResourceModifyResponseProtocolIEs_EntryValue,
}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "0",
    sz_ub = "65535"
)]
pub struct PDUSessionResourceModifyResponseProtocolIEs(
    Vec<PDUSessionResourceModifyResponseProtocolIEs_Entry>,
);

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum PDUSessionResourceModifyResponseTransferIE_Extensions_EntryExtensionValue {
    #[asn(key = 154)]
    Id_AdditionalNGU_UP_TNLInformation(UPTransportLayerInformationPairList),
    #[asn(key = 184)]
    Id_AdditionalRedundantDLQosFlowPerTNLInformation(QosFlowPerTNLInformationList),
    #[asn(key = 185)]
    Id_AdditionalRedundantNGU_UP_TNLInformation(UPTransportLayerInformationPairList),
    #[asn(key = 192)]
    Id_RedundantDL_NGU_UP_TNLInformation(UPTransportLayerInformation),
    #[asn(key = 195)]
    Id_RedundantUL_NGU_UP_TNLInformation(UPTransportLayerInformation),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct PDUSessionResourceModifyResponseTransferIE_Extensions_Entry {
    #[asn(key_field = true)]
    pub id: ProtocolExtensionID,
    pub criticality: Criticality,
    pub extension_value: PDUSessionResourceModifyResponseTransferIE_Extensions_EntryExtensionValue,
}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct PDUSessionResourceModifyResponseTransferIE_Extensions(
    Vec<PDUSessionResourceModifyResponseTransferIE_Extensions_Entry>,
);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct PDUSessionResourceModifyUnsuccessfulTransferIE_Extensions_Entry {}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct PDUSessionResourceModifyUnsuccessfulTransferIE_Extensions(
    Vec<PDUSessionResourceModifyUnsuccessfulTransferIE_Extensions_Entry>,
);

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum PDUSessionResourceNotifyProtocolIEs_EntryValue {
    #[asn(key = 10)]
    Id_AMF_UE_NGAP_ID(AMF_UE_NGAP_ID),
    #[asn(key = 66)]
    Id_PDUSessionResourceNotifyList(PDUSessionResourceNotifyList),
    #[asn(key = 67)]
    Id_PDUSessionResourceReleasedListNot(PDUSessionResourceReleasedListNot),
    #[asn(key = 85)]
    Id_RAN_UE_NGAP_ID(RAN_UE_NGAP_ID),
    #[asn(key = 121)]
    Id_UserLocationInformation(UserLocationInformation),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct PDUSessionResourceNotifyProtocolIEs_Entry {
    #[asn(key_field = true)]
    pub id: ProtocolIE_ID,
    pub criticality: Criticality,
    pub value: PDUSessionResourceNotifyProtocolIEs_EntryValue,
}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "0",
    sz_ub = "65535"
)]
pub struct PDUSessionResourceNotifyProtocolIEs(Vec<PDUSessionResourceNotifyProtocolIEs_Entry>);

#[derive(Debug, AperCodec)]
#[asn(type = "OCTET-STRING")]
pub struct OCTET_STRING_49(Vec<u8>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct PDUSessionResourceNotifyItemIE_Extensions_Entry {}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct PDUSessionResourceNotifyItemIE_Extensions(
    Vec<PDUSessionResourceNotifyItemIE_Extensions_Entry>,
);

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum PDUSessionResourceNotifyReleasedTransferIE_Extensions_EntryExtensionValue {
    #[asn(key = 144)]
    Id_SecondaryRATUsageInformation(SecondaryRATUsageInformation),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct PDUSessionResourceNotifyReleasedTransferIE_Extensions_Entry {
    #[asn(key_field = true)]
    pub id: ProtocolExtensionID,
    pub criticality: Criticality,
    pub extension_value: PDUSessionResourceNotifyReleasedTransferIE_Extensions_EntryExtensionValue,
}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct PDUSessionResourceNotifyReleasedTransferIE_Extensions(
    Vec<PDUSessionResourceNotifyReleasedTransferIE_Extensions_Entry>,
);

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum PDUSessionResourceNotifyTransferIE_Extensions_EntryExtensionValue {
    #[asn(key = 278)]
    Id_QosFlowFeedbackList(QosFlowFeedbackList),
    #[asn(key = 144)]
    Id_SecondaryRATUsageInformation(SecondaryRATUsageInformation),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct PDUSessionResourceNotifyTransferIE_Extensions_Entry {
    #[asn(key_field = true)]
    pub id: ProtocolExtensionID,
    pub criticality: Criticality,
    pub extension_value: PDUSessionResourceNotifyTransferIE_Extensions_EntryExtensionValue,
}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct PDUSessionResourceNotifyTransferIE_Extensions(
    Vec<PDUSessionResourceNotifyTransferIE_Extensions_Entry>,
);

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum PDUSessionResourceReleaseCommandProtocolIEs_EntryValue {
    #[asn(key = 10)]
    Id_AMF_UE_NGAP_ID(AMF_UE_NGAP_ID),
    #[asn(key = 38)]
    Id_NAS_PDU(NAS_PDU),
    #[asn(key = 79)]
    Id_PDUSessionResourceToReleaseListRelCmd(PDUSessionResourceToReleaseListRelCmd),
    #[asn(key = 85)]
    Id_RAN_UE_NGAP_ID(RAN_UE_NGAP_ID),
    #[asn(key = 83)]
    Id_RANPagingPriority(RANPagingPriority),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct PDUSessionResourceReleaseCommandProtocolIEs_Entry {
    #[asn(key_field = true)]
    pub id: ProtocolIE_ID,
    pub criticality: Criticality,
    pub value: PDUSessionResourceReleaseCommandProtocolIEs_EntryValue,
}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "0",
    sz_ub = "65535"
)]
pub struct PDUSessionResourceReleaseCommandProtocolIEs(
    Vec<PDUSessionResourceReleaseCommandProtocolIEs_Entry>,
);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct PDUSessionResourceReleaseCommandTransferIE_Extensions_Entry {}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct PDUSessionResourceReleaseCommandTransferIE_Extensions(
    Vec<PDUSessionResourceReleaseCommandTransferIE_Extensions_Entry>,
);

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum PDUSessionResourceReleaseResponseProtocolIEs_EntryValue {
    #[asn(key = 10)]
    Id_AMF_UE_NGAP_ID(AMF_UE_NGAP_ID),
    #[asn(key = 19)]
    Id_CriticalityDiagnostics(CriticalityDiagnostics),
    #[asn(key = 70)]
    Id_PDUSessionResourceReleasedListRelRes(PDUSessionResourceReleasedListRelRes),
    #[asn(key = 85)]
    Id_RAN_UE_NGAP_ID(RAN_UE_NGAP_ID),
    #[asn(key = 121)]
    Id_UserLocationInformation(UserLocationInformation),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct PDUSessionResourceReleaseResponseProtocolIEs_Entry {
    #[asn(key_field = true)]
    pub id: ProtocolIE_ID,
    pub criticality: Criticality,
    pub value: PDUSessionResourceReleaseResponseProtocolIEs_EntryValue,
}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "0",
    sz_ub = "65535"
)]
pub struct PDUSessionResourceReleaseResponseProtocolIEs(
    Vec<PDUSessionResourceReleaseResponseProtocolIEs_Entry>,
);

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum PDUSessionResourceReleaseResponseTransferIE_Extensions_EntryExtensionValue {
    #[asn(key = 144)]
    Id_SecondaryRATUsageInformation(SecondaryRATUsageInformation),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct PDUSessionResourceReleaseResponseTransferIE_Extensions_Entry {
    #[asn(key_field = true)]
    pub id: ProtocolExtensionID,
    pub criticality: Criticality,
    pub extension_value: PDUSessionResourceReleaseResponseTransferIE_Extensions_EntryExtensionValue,
}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct PDUSessionResourceReleaseResponseTransferIE_Extensions(
    Vec<PDUSessionResourceReleaseResponseTransferIE_Extensions_Entry>,
);

#[derive(Debug, AperCodec)]
#[asn(type = "OCTET-STRING")]
pub struct OCTET_STRING_50(Vec<u8>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct PDUSessionResourceReleasedItemNotIE_Extensions_Entry {}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct PDUSessionResourceReleasedItemNotIE_Extensions(
    Vec<PDUSessionResourceReleasedItemNotIE_Extensions_Entry>,
);

#[derive(Debug, AperCodec)]
#[asn(type = "OCTET-STRING")]
pub struct OCTET_STRING_51(Vec<u8>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct PDUSessionResourceReleasedItemPSAckIE_Extensions_Entry {}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct PDUSessionResourceReleasedItemPSAckIE_Extensions(
    Vec<PDUSessionResourceReleasedItemPSAckIE_Extensions_Entry>,
);

#[derive(Debug, AperCodec)]
#[asn(type = "OCTET-STRING")]
pub struct OCTET_STRING_52(Vec<u8>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct PDUSessionResourceReleasedItemPSFailIE_Extensions_Entry {}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct PDUSessionResourceReleasedItemPSFailIE_Extensions(
    Vec<PDUSessionResourceReleasedItemPSFailIE_Extensions_Entry>,
);

#[derive(Debug, AperCodec)]
#[asn(type = "OCTET-STRING")]
pub struct OCTET_STRING_53(Vec<u8>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct PDUSessionResourceReleasedItemRelResIE_Extensions_Entry {}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct PDUSessionResourceReleasedItemRelResIE_Extensions(
    Vec<PDUSessionResourceReleasedItemRelResIE_Extensions_Entry>,
);

#[derive(Debug, AperCodec)]
#[asn(type = "OCTET-STRING")]
pub struct OCTET_STRING_54(Vec<u8>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct PDUSessionResourceResumeItemRESReqIE_Extensions_Entry {}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct PDUSessionResourceResumeItemRESReqIE_Extensions(
    Vec<PDUSessionResourceResumeItemRESReqIE_Extensions_Entry>,
);

#[derive(Debug, AperCodec)]
#[asn(type = "OCTET-STRING")]
pub struct OCTET_STRING_55(Vec<u8>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct PDUSessionResourceResumeItemRESResIE_Extensions_Entry {}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct PDUSessionResourceResumeItemRESResIE_Extensions(
    Vec<PDUSessionResourceResumeItemRESResIE_Extensions_Entry>,
);

#[derive(Debug, AperCodec)]
#[asn(type = "OCTET-STRING")]
pub struct OCTET_STRING_56(Vec<u8>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct PDUSessionResourceSecondaryRATUsageItemIE_Extensions_Entry {}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct PDUSessionResourceSecondaryRATUsageItemIE_Extensions(
    Vec<PDUSessionResourceSecondaryRATUsageItemIE_Extensions_Entry>,
);

#[derive(Debug, AperCodec)]
#[asn(type = "OCTET-STRING")]
pub struct OCTET_STRING_57(Vec<u8>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct PDUSessionResourceSetupItemCxtReqIE_Extensions_Entry {}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct PDUSessionResourceSetupItemCxtReqIE_Extensions(
    Vec<PDUSessionResourceSetupItemCxtReqIE_Extensions_Entry>,
);

#[derive(Debug, AperCodec)]
#[asn(type = "OCTET-STRING")]
pub struct OCTET_STRING_58(Vec<u8>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct PDUSessionResourceSetupItemCxtResIE_Extensions_Entry {}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct PDUSessionResourceSetupItemCxtResIE_Extensions(
    Vec<PDUSessionResourceSetupItemCxtResIE_Extensions_Entry>,
);

#[derive(Debug, AperCodec)]
#[asn(type = "OCTET-STRING")]
pub struct OCTET_STRING_59(Vec<u8>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct PDUSessionResourceSetupItemHOReqIE_Extensions_Entry {}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct PDUSessionResourceSetupItemHOReqIE_Extensions(
    Vec<PDUSessionResourceSetupItemHOReqIE_Extensions_Entry>,
);

#[derive(Debug, AperCodec)]
#[asn(type = "OCTET-STRING")]
pub struct OCTET_STRING_60(Vec<u8>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct PDUSessionResourceSetupItemSUReqIE_Extensions_Entry {}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct PDUSessionResourceSetupItemSUReqIE_Extensions(
    Vec<PDUSessionResourceSetupItemSUReqIE_Extensions_Entry>,
);

#[derive(Debug, AperCodec)]
#[asn(type = "OCTET-STRING")]
pub struct OCTET_STRING_61(Vec<u8>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct PDUSessionResourceSetupItemSUResIE_Extensions_Entry {}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct PDUSessionResourceSetupItemSUResIE_Extensions(
    Vec<PDUSessionResourceSetupItemSUResIE_Extensions_Entry>,
);

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum PDUSessionResourceSetupRequestProtocolIEs_EntryValue {
    #[asn(key = 10)]
    Id_AMF_UE_NGAP_ID(AMF_UE_NGAP_ID),
    #[asn(key = 38)]
    Id_NAS_PDU(NAS_PDU),
    #[asn(key = 74)]
    Id_PDUSessionResourceSetupListSUReq(PDUSessionResourceSetupListSUReq),
    #[asn(key = 85)]
    Id_RAN_UE_NGAP_ID(RAN_UE_NGAP_ID),
    #[asn(key = 83)]
    Id_RANPagingPriority(RANPagingPriority),
    #[asn(key = 110)]
    Id_UEAggregateMaximumBitRate(UEAggregateMaximumBitRate),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct PDUSessionResourceSetupRequestProtocolIEs_Entry {
    #[asn(key_field = true)]
    pub id: ProtocolIE_ID,
    pub criticality: Criticality,
    pub value: PDUSessionResourceSetupRequestProtocolIEs_EntryValue,
}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "0",
    sz_ub = "65535"
)]
pub struct PDUSessionResourceSetupRequestProtocolIEs(
    Vec<PDUSessionResourceSetupRequestProtocolIEs_Entry>,
);

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum PDUSessionResourceSetupRequestTransferProtocolIEs_EntryValue {
    #[asn(key = 186)]
    Id_AdditionalRedundantUL_NGU_UP_TNLInformation(UPTransportLayerInformationList),
    #[asn(key = 126)]
    Id_AdditionalUL_NGU_UP_TNLInformation(UPTransportLayerInformationList),
    #[asn(key = 166)]
    Id_CommonNetworkInstance(CommonNetworkInstance),
    #[asn(key = 127)]
    Id_DataForwardingNotPossible(DataForwardingNotPossible),
    #[asn(key = 22)]
    Id_DirectForwardingPathAvailability(DirectForwardingPathAvailability),
    #[asn(key = 129)]
    Id_NetworkInstance(NetworkInstance),
    #[asn(key = 130)]
    Id_PDUSessionAggregateMaximumBitRate(PDUSessionAggregateMaximumBitRate),
    #[asn(key = 134)]
    Id_PDUSessionType(PDUSessionType),
    #[asn(key = 136)]
    Id_QosFlowSetupRequestList(QosFlowSetupRequestList),
    #[asn(key = 190)]
    Id_RedundantCommonNetworkInstance(CommonNetworkInstance),
    #[asn(key = 197)]
    Id_RedundantPDUSessionInformation(RedundantPDUSessionInformation),
    #[asn(key = 195)]
    Id_RedundantUL_NGU_UP_TNLInformation(UPTransportLayerInformation),
    #[asn(key = 138)]
    Id_SecurityIndication(SecurityIndication),
    #[asn(key = 139)]
    Id_UL_NGU_UP_TNLInformation(UPTransportLayerInformation),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct PDUSessionResourceSetupRequestTransferProtocolIEs_Entry {
    #[asn(key_field = true)]
    pub id: ProtocolIE_ID,
    pub criticality: Criticality,
    pub value: PDUSessionResourceSetupRequestTransferProtocolIEs_EntryValue,
}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "0",
    sz_ub = "65535"
)]
pub struct PDUSessionResourceSetupRequestTransferProtocolIEs(
    Vec<PDUSessionResourceSetupRequestTransferProtocolIEs_Entry>,
);

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum PDUSessionResourceSetupResponseProtocolIEs_EntryValue {
    #[asn(key = 10)]
    Id_AMF_UE_NGAP_ID(AMF_UE_NGAP_ID),
    #[asn(key = 19)]
    Id_CriticalityDiagnostics(CriticalityDiagnostics),
    #[asn(key = 58)]
    Id_PDUSessionResourceFailedToSetupListSURes(PDUSessionResourceFailedToSetupListSURes),
    #[asn(key = 75)]
    Id_PDUSessionResourceSetupListSURes(PDUSessionResourceSetupListSURes),
    #[asn(key = 85)]
    Id_RAN_UE_NGAP_ID(RAN_UE_NGAP_ID),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct PDUSessionResourceSetupResponseProtocolIEs_Entry {
    #[asn(key_field = true)]
    pub id: ProtocolIE_ID,
    pub criticality: Criticality,
    pub value: PDUSessionResourceSetupResponseProtocolIEs_EntryValue,
}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "0",
    sz_ub = "65535"
)]
pub struct PDUSessionResourceSetupResponseProtocolIEs(
    Vec<PDUSessionResourceSetupResponseProtocolIEs_Entry>,
);

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum PDUSessionResourceSetupResponseTransferIE_Extensions_EntryExtensionValue {
    #[asn(key = 184)]
    Id_AdditionalRedundantDLQosFlowPerTNLInformation(QosFlowPerTNLInformationList),
    #[asn(key = 27)]
    Id_GlobalRANNodeID(GlobalRANNodeID),
    #[asn(key = 193)]
    Id_RedundantDLQosFlowPerTNLInformation(QosFlowPerTNLInformation),
    #[asn(key = 198)]
    Id_UsedRSNInformation(RedundantPDUSessionInformation),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct PDUSessionResourceSetupResponseTransferIE_Extensions_Entry {
    #[asn(key_field = true)]
    pub id: ProtocolExtensionID,
    pub criticality: Criticality,
    pub extension_value: PDUSessionResourceSetupResponseTransferIE_Extensions_EntryExtensionValue,
}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct PDUSessionResourceSetupResponseTransferIE_Extensions(
    Vec<PDUSessionResourceSetupResponseTransferIE_Extensions_Entry>,
);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct PDUSessionResourceSetupUnsuccessfulTransferIE_Extensions_Entry {}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct PDUSessionResourceSetupUnsuccessfulTransferIE_Extensions(
    Vec<PDUSessionResourceSetupUnsuccessfulTransferIE_Extensions_Entry>,
);

#[derive(Debug, AperCodec)]
#[asn(type = "OCTET-STRING")]
pub struct OCTET_STRING_62(Vec<u8>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct PDUSessionResourceSuspendItemSUSReqIE_Extensions_Entry {}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct PDUSessionResourceSuspendItemSUSReqIE_Extensions(
    Vec<PDUSessionResourceSuspendItemSUSReqIE_Extensions_Entry>,
);

#[derive(Debug, AperCodec)]
#[asn(type = "OCTET-STRING")]
pub struct OCTET_STRING_63(Vec<u8>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct PDUSessionResourceSwitchedItemIE_Extensions_Entry {}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct PDUSessionResourceSwitchedItemIE_Extensions(
    Vec<PDUSessionResourceSwitchedItemIE_Extensions_Entry>,
);

#[derive(Debug, AperCodec)]
#[asn(type = "OCTET-STRING")]
pub struct OCTET_STRING_64(Vec<u8>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct PDUSessionResourceToBeSwitchedDLItemIE_Extensions_Entry {}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct PDUSessionResourceToBeSwitchedDLItemIE_Extensions(
    Vec<PDUSessionResourceToBeSwitchedDLItemIE_Extensions_Entry>,
);

#[derive(Debug, AperCodec)]
#[asn(type = "OCTET-STRING")]
pub struct OCTET_STRING_65(Vec<u8>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct PDUSessionResourceToReleaseItemHOCmdIE_Extensions_Entry {}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct PDUSessionResourceToReleaseItemHOCmdIE_Extensions(
    Vec<PDUSessionResourceToReleaseItemHOCmdIE_Extensions_Entry>,
);

#[derive(Debug, AperCodec)]
#[asn(type = "OCTET-STRING")]
pub struct OCTET_STRING_66(Vec<u8>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct PDUSessionResourceToReleaseItemRelCmdIE_Extensions_Entry {}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct PDUSessionResourceToReleaseItemRelCmdIE_Extensions(
    Vec<PDUSessionResourceToReleaseItemRelCmdIE_Extensions_Entry>,
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
pub struct PDUSessionUsageReportIE_Extensions_Entry {}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct PDUSessionUsageReportIE_Extensions(Vec<PDUSessionUsageReportIE_Extensions_Entry>);

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum PLMNSupportItemIE_Extensions_EntryExtensionValue {
    #[asn(key = 270)]
    Id_ExtendedSliceSupportList(ExtendedSliceSupportList),
    #[asn(key = 258)]
    Id_NPN_Support(NPN_Support),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct PLMNSupportItemIE_Extensions_Entry {
    #[asn(key_field = true)]
    pub id: ProtocolExtensionID,
    pub criticality: Criticality,
    pub extension_value: PLMNSupportItemIE_Extensions_EntryExtensionValue,
}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct PLMNSupportItemIE_Extensions(Vec<PLMNSupportItemIE_Extensions_Entry>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct PNI_NPN_MobilityInformationIE_Extensions_Entry {}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct PNI_NPN_MobilityInformationIE_Extensions(
    Vec<PNI_NPN_MobilityInformationIE_Extensions_Entry>,
);

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum PWSCancelRequestProtocolIEs_EntryValue {
    #[asn(key = 14)]
    Id_CancelAllWarningMessages(CancelAllWarningMessages),
    #[asn(key = 35)]
    Id_MessageIdentifier(MessageIdentifier),
    #[asn(key = 95)]
    Id_SerialNumber(SerialNumber),
    #[asn(key = 122)]
    Id_WarningAreaList(WarningAreaList),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct PWSCancelRequestProtocolIEs_Entry {
    #[asn(key_field = true)]
    pub id: ProtocolIE_ID,
    pub criticality: Criticality,
    pub value: PWSCancelRequestProtocolIEs_EntryValue,
}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "0",
    sz_ub = "65535"
)]
pub struct PWSCancelRequestProtocolIEs(Vec<PWSCancelRequestProtocolIEs_Entry>);

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum PWSCancelResponseProtocolIEs_EntryValue {
    #[asn(key = 12)]
    Id_BroadcastCancelledAreaList(BroadcastCancelledAreaList),
    #[asn(key = 19)]
    Id_CriticalityDiagnostics(CriticalityDiagnostics),
    #[asn(key = 35)]
    Id_MessageIdentifier(MessageIdentifier),
    #[asn(key = 95)]
    Id_SerialNumber(SerialNumber),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct PWSCancelResponseProtocolIEs_Entry {
    #[asn(key_field = true)]
    pub id: ProtocolIE_ID,
    pub criticality: Criticality,
    pub value: PWSCancelResponseProtocolIEs_EntryValue,
}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "0",
    sz_ub = "65535"
)]
pub struct PWSCancelResponseProtocolIEs(Vec<PWSCancelResponseProtocolIEs_Entry>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct PWSFailedCellIDListchoice_Extensions {}

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum PWSFailureIndicationProtocolIEs_EntryValue {
    #[asn(key = 27)]
    Id_GlobalRANNodeID(GlobalRANNodeID),
    #[asn(key = 81)]
    Id_PWSFailedCellIDList(PWSFailedCellIDList),
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
    #[asn(key = 16)]
    Id_CellIDListForRestart(CellIDListForRestart),
    #[asn(key = 23)]
    Id_EmergencyAreaIDListForRestart(EmergencyAreaIDListForRestart),
    #[asn(key = 27)]
    Id_GlobalRANNodeID(GlobalRANNodeID),
    #[asn(key = 104)]
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
#[asn(type = "INTEGER", lb = "0", ub = "9", extensible = true)]
pub struct INTEGER_68(u8);

#[derive(Debug, AperCodec)]
#[asn(type = "INTEGER", lb = "0", ub = "9", extensible = true)]
pub struct INTEGER_69(u8);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct PacketErrorRateIE_Extensions_Entry {}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct PacketErrorRateIE_Extensions(Vec<PacketErrorRateIE_Extensions_Entry>);

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum PagingProtocolIEs_EntryValue {
    #[asn(key = 11)]
    Id_AssistanceDataForPaging(AssistanceDataForPaging),
    #[asn(key = 222)]
    Id_CEmodeBrestricted(CEmodeBrestricted),
    #[asn(key = 205)]
    Id_Enhanced_CoverageRestriction(Enhanced_CoverageRestriction),
    #[asn(key = 203)]
    Id_NB_IoT_Paging_eDRXInfo(NB_IoT_Paging_eDRXInfo),
    #[asn(key = 202)]
    Id_NB_IoT_PagingDRX(NB_IoT_PagingDRX),
    #[asn(key = 50)]
    Id_PagingDRX(PagingDRX),
    #[asn(key = 51)]
    Id_PagingOrigin(PagingOrigin),
    #[asn(key = 52)]
    Id_PagingPriority(PagingPriority),
    #[asn(key = 223)]
    Id_PagingeDRXInformation(PagingeDRXInformation),
    #[asn(key = 103)]
    Id_TAIListForPaging(TAIListForPaging),
    #[asn(key = 115)]
    Id_UEPagingIdentity(UEPagingIdentity),
    #[asn(key = 118)]
    Id_UERadioCapabilityForPaging(UERadioCapabilityForPaging),
    #[asn(key = 208)]
    Id_WUS_Assistance_Information(WUS_Assistance_Information),
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
pub struct PagingAssisDataforCEcapabUEIE_Extensions_Entry {}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct PagingAssisDataforCEcapabUEIE_Extensions(
    Vec<PagingAssisDataforCEcapabUEIE_Extensions_Entry>,
);

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
#[asn(type = "SEQUENCE", extensible = false)]
pub struct PagingeDRXInformationIE_Extensions_Entry {}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct PagingeDRXInformationIE_Extensions(Vec<PagingeDRXInformationIE_Extensions_Entry>);

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum PathSwitchRequestProtocolIEs_EntryValue {
    #[asn(key = 57)]
    Id_PDUSessionResourceFailedToSetupListPSReq(PDUSessionResourceFailedToSetupListPSReq),
    #[asn(key = 76)]
    Id_PDUSessionResourceToBeSwitchedDLList(PDUSessionResourceToBeSwitchedDLList),
    #[asn(key = 85)]
    Id_RAN_UE_NGAP_ID(RAN_UE_NGAP_ID),
    #[asn(key = 237)]
    Id_RRC_Resume_Cause(RRCEstablishmentCause),
    #[asn(key = 100)]
    Id_SourceAMF_UE_NGAP_ID(AMF_UE_NGAP_ID),
    #[asn(key = 119)]
    Id_UESecurityCapabilities(UESecurityCapabilities),
    #[asn(key = 121)]
    Id_UserLocationInformation(UserLocationInformation),
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
    #[asn(key = 10)]
    Id_AMF_UE_NGAP_ID(AMF_UE_NGAP_ID),
    #[asn(key = 0)]
    Id_AllowedNSSAI(AllowedNSSAI),
    #[asn(key = 222)]
    Id_CEmodeBrestricted(CEmodeBrestricted),
    #[asn(key = 165)]
    Id_CNAssistedRANTuning(CNAssistedRANTuning),
    #[asn(key = 18)]
    Id_CoreNetworkAssistanceInformationForInactive(CoreNetworkAssistanceInformationForInactive),
    #[asn(key = 19)]
    Id_CriticalityDiagnostics(CriticalityDiagnostics),
    #[asn(key = 205)]
    Id_Enhanced_CoverageRestriction(Enhanced_CoverageRestriction),
    #[asn(key = 206)]
    Id_Extended_ConnectedTime(Extended_ConnectedTime),
    #[asn(key = 217)]
    Id_LTEUESidelinkAggregateMaximumBitrate(LTEUESidelinkAggregateMaximumBitrate),
    #[asn(key = 215)]
    Id_LTEV2XServicesAuthorized(LTEV2XServicesAuthorized),
    #[asn(key = 218)]
    Id_NRUESidelinkAggregateMaximumBitrate(NRUESidelinkAggregateMaximumBitrate),
    #[asn(key = 216)]
    Id_NRV2XServicesAuthorized(NRV2XServicesAuthorized),
    #[asn(key = 41)]
    Id_NewSecurityContextInd(NewSecurityContextInd),
    #[asn(key = 219)]
    Id_PC5QoSParameters(PC5QoSParameters),
    #[asn(key = 68)]
    Id_PDUSessionResourceReleasedListPSAck(PDUSessionResourceReleasedListPSAck),
    #[asn(key = 77)]
    Id_PDUSessionResourceSwitchedList(PDUSessionResourceSwitchedList),
    #[asn(key = 85)]
    Id_RAN_UE_NGAP_ID(RAN_UE_NGAP_ID),
    #[asn(key = 91)]
    Id_RRCInactiveTransitionReportRequest(RRCInactiveTransitionReportRequest),
    #[asn(key = 146)]
    Id_RedirectionVoiceFallback(RedirectionVoiceFallback),
    #[asn(key = 177)]
    Id_SRVCCOperationPossible(SRVCCOperationPossible),
    #[asn(key = 93)]
    Id_SecurityContext(SecurityContext),
    #[asn(key = 209)]
    Id_UE_DifferentiationInfo(UE_DifferentiationInfo),
    #[asn(key = 234)]
    Id_UE_UP_CIoT_Support(UE_UP_CIoT_Support),
    #[asn(key = 264)]
    Id_UERadioCapabilityID(UERadioCapabilityID),
    #[asn(key = 119)]
    Id_UESecurityCapabilities(UESecurityCapabilities),
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
pub enum PathSwitchRequestAcknowledgeTransferIE_Extensions_EntryExtensionValue {
    #[asn(key = 154)]
    Id_AdditionalNGU_UP_TNLInformation(UPTransportLayerInformationPairList),
    #[asn(key = 185)]
    Id_AdditionalRedundantNGU_UP_TNLInformation(UPTransportLayerInformationPairList),
    #[asn(key = 277)]
    Id_QosFlowParametersList(QosFlowParametersList),
    #[asn(key = 195)]
    Id_RedundantUL_NGU_UP_TNLInformation(UPTransportLayerInformation),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct PathSwitchRequestAcknowledgeTransferIE_Extensions_Entry {
    #[asn(key_field = true)]
    pub id: ProtocolExtensionID,
    pub criticality: Criticality,
    pub extension_value: PathSwitchRequestAcknowledgeTransferIE_Extensions_EntryExtensionValue,
}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct PathSwitchRequestAcknowledgeTransferIE_Extensions(
    Vec<PathSwitchRequestAcknowledgeTransferIE_Extensions_Entry>,
);

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum PathSwitchRequestFailureProtocolIEs_EntryValue {
    #[asn(key = 10)]
    Id_AMF_UE_NGAP_ID(AMF_UE_NGAP_ID),
    #[asn(key = 19)]
    Id_CriticalityDiagnostics(CriticalityDiagnostics),
    #[asn(key = 69)]
    Id_PDUSessionResourceReleasedListPSFail(PDUSessionResourceReleasedListPSFail),
    #[asn(key = 85)]
    Id_RAN_UE_NGAP_ID(RAN_UE_NGAP_ID),
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
#[asn(type = "SEQUENCE", extensible = false)]
pub struct PathSwitchRequestSetupFailedTransferIE_Extensions_Entry {}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct PathSwitchRequestSetupFailedTransferIE_Extensions(
    Vec<PathSwitchRequestSetupFailedTransferIE_Extensions_Entry>,
);

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum PathSwitchRequestTransferIE_Extensions_EntryExtensionValue {
    #[asn(key = 155)]
    Id_AdditionalDLQosFlowPerTNLInformation(QosFlowPerTNLInformationList),
    #[asn(key = 184)]
    Id_AdditionalRedundantDLQosFlowPerTNLInformation(QosFlowPerTNLInformationList),
    #[asn(key = 27)]
    Id_GlobalRANNodeID(GlobalRANNodeID),
    #[asn(key = 191)]
    Id_RedundantDL_NGU_TNLInformationReused(DL_NGU_TNLInformationReused),
    #[asn(key = 192)]
    Id_RedundantDL_NGU_UP_TNLInformation(UPTransportLayerInformation),
    #[asn(key = 198)]
    Id_UsedRSNInformation(RedundantPDUSessionInformation),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct PathSwitchRequestTransferIE_Extensions_Entry {
    #[asn(key_field = true)]
    pub id: ProtocolExtensionID,
    pub criticality: Criticality,
    pub extension_value: PathSwitchRequestTransferIE_Extensions_EntryExtensionValue,
}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct PathSwitchRequestTransferIE_Extensions(
    Vec<PathSwitchRequestTransferIE_Extensions_Entry>,
);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct PathSwitchRequestUnsuccessfulTransferIE_Extensions_Entry {}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct PathSwitchRequestUnsuccessfulTransferIE_Extensions(
    Vec<PathSwitchRequestUnsuccessfulTransferIE_Extensions_Entry>,
);

#[derive(Debug, AperCodec)]
#[asn(type = "INTEGER", lb = "0", ub = "65535")]
pub struct INTEGER_70(u16);

#[derive(Debug, AperCodec)]
#[asn(type = "OBJECT-IDENTIFIER")]
pub struct OBJECT_IDENTIFIER_71;

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
pub struct QoSFlowsUsageReport_ItemIE_Extensions_Entry {}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct QoSFlowsUsageReport_ItemIE_Extensions(Vec<QoSFlowsUsageReport_ItemIE_Extensions_Entry>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct QosCharacteristicschoice_Extensions {}

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum QosFlowAcceptedItemIE_Extensions_EntryExtensionValue {
    #[asn(key = 221)]
    Id_CurrentQoSParaSetIndex(AlternativeQoSParaSetIndex),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct QosFlowAcceptedItemIE_Extensions_Entry {
    #[asn(key_field = true)]
    pub id: ProtocolExtensionID,
    pub criticality: Criticality,
    pub extension_value: QosFlowAcceptedItemIE_Extensions_EntryExtensionValue,
}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct QosFlowAcceptedItemIE_Extensions(Vec<QosFlowAcceptedItemIE_Extensions_Entry>);

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum QosFlowAddOrModifyRequestItemIE_Extensions_EntryExtensionValue {
    #[asn(key = 194)]
    Id_RedundantQosFlowIndicator(RedundantQosFlowIndicator),
    #[asn(key = 196)]
    Id_TSCTrafficCharacteristics(TSCTrafficCharacteristics),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct QosFlowAddOrModifyRequestItemIE_Extensions_Entry {
    #[asn(key_field = true)]
    pub id: ProtocolExtensionID,
    pub criticality: Criticality,
    pub extension_value: QosFlowAddOrModifyRequestItemIE_Extensions_EntryExtensionValue,
}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct QosFlowAddOrModifyRequestItemIE_Extensions(
    Vec<QosFlowAddOrModifyRequestItemIE_Extensions_Entry>,
);

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum QosFlowAddOrModifyResponseItemIE_Extensions_EntryExtensionValue {
    #[asn(key = 221)]
    Id_CurrentQoSParaSetIndex(AlternativeQoSParaSetIndex),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct QosFlowAddOrModifyResponseItemIE_Extensions_Entry {
    #[asn(key_field = true)]
    pub id: ProtocolExtensionID,
    pub criticality: Criticality,
    pub extension_value: QosFlowAddOrModifyResponseItemIE_Extensions_EntryExtensionValue,
}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct QosFlowAddOrModifyResponseItemIE_Extensions(
    Vec<QosFlowAddOrModifyResponseItemIE_Extensions_Entry>,
);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct QosFlowFeedbackItemIE_Extensions_Entry {}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct QosFlowFeedbackItemIE_Extensions(Vec<QosFlowFeedbackItemIE_Extensions_Entry>);

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum QosFlowInformationItemIE_Extensions_EntryExtensionValue {
    #[asn(key = 163)]
    Id_ULForwarding(ULForwarding),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct QosFlowInformationItemIE_Extensions_Entry {
    #[asn(key_field = true)]
    pub id: ProtocolExtensionID,
    pub criticality: Criticality,
    pub extension_value: QosFlowInformationItemIE_Extensions_EntryExtensionValue,
}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct QosFlowInformationItemIE_Extensions(Vec<QosFlowInformationItemIE_Extensions_Entry>);

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum QosFlowItemWithDataForwardingIE_Extensions_EntryExtensionValue {
    #[asn(key = 221)]
    Id_CurrentQoSParaSetIndex(AlternativeQoSParaSetIndex),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct QosFlowItemWithDataForwardingIE_Extensions_Entry {
    #[asn(key_field = true)]
    pub id: ProtocolExtensionID,
    pub criticality: Criticality,
    pub extension_value: QosFlowItemWithDataForwardingIE_Extensions_EntryExtensionValue,
}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct QosFlowItemWithDataForwardingIE_Extensions(
    Vec<QosFlowItemWithDataForwardingIE_Extensions_Entry>,
);

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum QosFlowLevelQosParametersIE_Extensions_EntryExtensionValue {
    #[asn(key = 276)]
    Id_QosMonitoringReportingFrequency(QosMonitoringReportingFrequency),
    #[asn(key = 181)]
    Id_QosMonitoringRequest(QosMonitoringRequest),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct QosFlowLevelQosParametersIE_Extensions_Entry {
    #[asn(key_field = true)]
    pub id: ProtocolExtensionID,
    pub criticality: Criticality,
    pub extension_value: QosFlowLevelQosParametersIE_Extensions_EntryExtensionValue,
}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct QosFlowLevelQosParametersIE_Extensions(
    Vec<QosFlowLevelQosParametersIE_Extensions_Entry>,
);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct QosFlowModifyConfirmItemIE_Extensions_Entry {}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct QosFlowModifyConfirmItemIE_Extensions(Vec<QosFlowModifyConfirmItemIE_Extensions_Entry>);

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum QosFlowNotifyItemIE_Extensions_EntryExtensionValue {
    #[asn(key = 221)]
    Id_CurrentQoSParaSetIndex(AlternativeQoSParaSetNotifyIndex),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct QosFlowNotifyItemIE_Extensions_Entry {
    #[asn(key_field = true)]
    pub id: ProtocolExtensionID,
    pub criticality: Criticality,
    pub extension_value: QosFlowNotifyItemIE_Extensions_EntryExtensionValue,
}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct QosFlowNotifyItemIE_Extensions(Vec<QosFlowNotifyItemIE_Extensions_Entry>);

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum QosFlowParametersItemIE_Extensions_EntryExtensionValue {
    #[asn(key = 279)]
    Id_BurstArrivalTimeDownlink(BurstArrivalTime),
    #[asn(key = 187)]
    Id_CNPacketDelayBudgetDL(ExtendedPacketDelayBudget),
    #[asn(key = 188)]
    Id_CNPacketDelayBudgetUL(ExtendedPacketDelayBudget),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct QosFlowParametersItemIE_Extensions_Entry {
    #[asn(key_field = true)]
    pub id: ProtocolExtensionID,
    pub criticality: Criticality,
    pub extension_value: QosFlowParametersItemIE_Extensions_EntryExtensionValue,
}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct QosFlowParametersItemIE_Extensions(Vec<QosFlowParametersItemIE_Extensions_Entry>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct QosFlowPerTNLInformationIE_Extensions_Entry {}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct QosFlowPerTNLInformationIE_Extensions(Vec<QosFlowPerTNLInformationIE_Extensions_Entry>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct QosFlowPerTNLInformationItemIE_Extensions_Entry {}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct QosFlowPerTNLInformationItemIE_Extensions(
    Vec<QosFlowPerTNLInformationItemIE_Extensions_Entry>,
);

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum QosFlowSetupRequestItemIE_Extensions_EntryExtensionValue {
    #[asn(key = 194)]
    Id_RedundantQosFlowIndicator(RedundantQosFlowIndicator),
    #[asn(key = 196)]
    Id_TSCTrafficCharacteristics(TSCTrafficCharacteristics),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct QosFlowSetupRequestItemIE_Extensions_Entry {
    #[asn(key_field = true)]
    pub id: ProtocolExtensionID,
    pub criticality: Criticality,
    pub extension_value: QosFlowSetupRequestItemIE_Extensions_EntryExtensionValue,
}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct QosFlowSetupRequestItemIE_Extensions(Vec<QosFlowSetupRequestItemIE_Extensions_Entry>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct QosFlowToBeForwardedItemIE_Extensions_Entry {}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct QosFlowToBeForwardedItemIE_Extensions(Vec<QosFlowToBeForwardedItemIE_Extensions_Entry>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct QosFlowWithCauseItemIE_Extensions_Entry {}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct QosFlowWithCauseItemIE_Extensions(Vec<QosFlowWithCauseItemIE_Extensions_Entry>);

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum RANCPRelocationIndicationProtocolIEs_EntryValue {
    #[asn(key = 25)]
    Id_EUTRA_CGI(EUTRA_CGI),
    #[asn(key = 26)]
    Id_FiveG_S_TMSI(FiveG_S_TMSI),
    #[asn(key = 85)]
    Id_RAN_UE_NGAP_ID(RAN_UE_NGAP_ID),
    #[asn(key = 213)]
    Id_TAI(TAI),
    #[asn(key = 211)]
    Id_UL_CP_SecurityInformation(UL_CP_SecurityInformation),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct RANCPRelocationIndicationProtocolIEs_Entry {
    #[asn(key_field = true)]
    pub id: ProtocolIE_ID,
    pub criticality: Criticality,
    pub value: RANCPRelocationIndicationProtocolIEs_EntryValue,
}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "0",
    sz_ub = "65535"
)]
pub struct RANCPRelocationIndicationProtocolIEs(Vec<RANCPRelocationIndicationProtocolIEs_Entry>);

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum RANConfigurationUpdateProtocolIEs_EntryValue {
    #[asn(key = 21)]
    Id_DefaultPagingDRX(PagingDRX),
    #[asn(key = 273)]
    Id_Extended_RANNodeName(Extended_RANNodeName),
    #[asn(key = 27)]
    Id_GlobalRANNodeID(GlobalRANNodeID),
    #[asn(key = 204)]
    Id_NB_IoT_DefaultPagingDRX(NB_IoT_DefaultPagingDRX),
    #[asn(key = 167)]
    Id_NGRAN_TNLAssociationToRemoveList(NGRAN_TNLAssociationToRemoveList),
    #[asn(key = 82)]
    Id_RANNodeName(RANNodeName),
    #[asn(key = 102)]
    Id_SupportedTAList(SupportedTAList),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct RANConfigurationUpdateProtocolIEs_Entry {
    #[asn(key_field = true)]
    pub id: ProtocolIE_ID,
    pub criticality: Criticality,
    pub value: RANConfigurationUpdateProtocolIEs_EntryValue,
}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "0",
    sz_ub = "65535"
)]
pub struct RANConfigurationUpdateProtocolIEs(Vec<RANConfigurationUpdateProtocolIEs_Entry>);

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum RANConfigurationUpdateAcknowledgeProtocolIEs_EntryValue {
    #[asn(key = 19)]
    Id_CriticalityDiagnostics(CriticalityDiagnostics),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct RANConfigurationUpdateAcknowledgeProtocolIEs_Entry {
    #[asn(key_field = true)]
    pub id: ProtocolIE_ID,
    pub criticality: Criticality,
    pub value: RANConfigurationUpdateAcknowledgeProtocolIEs_EntryValue,
}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "0",
    sz_ub = "65535"
)]
pub struct RANConfigurationUpdateAcknowledgeProtocolIEs(
    Vec<RANConfigurationUpdateAcknowledgeProtocolIEs_Entry>,
);

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum RANConfigurationUpdateFailureProtocolIEs_EntryValue {
    #[asn(key = 15)]
    Id_Cause(Cause),
    #[asn(key = 19)]
    Id_CriticalityDiagnostics(CriticalityDiagnostics),
    #[asn(key = 107)]
    Id_TimeToWait(TimeToWait),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct RANConfigurationUpdateFailureProtocolIEs_Entry {
    #[asn(key_field = true)]
    pub id: ProtocolIE_ID,
    pub criticality: Criticality,
    pub value: RANConfigurationUpdateFailureProtocolIEs_EntryValue,
}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "0",
    sz_ub = "65535"
)]
pub struct RANConfigurationUpdateFailureProtocolIEs(
    Vec<RANConfigurationUpdateFailureProtocolIEs_Entry>,
);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct RANStatusTransfer_TransparentContainerIE_Extensions_Entry {}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct RANStatusTransfer_TransparentContainerIE_Extensions(
    Vec<RANStatusTransfer_TransparentContainerIE_Extensions_Entry>,
);

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum RATRestrictions_ItemIE_Extensions_EntryExtensionValue {
    #[asn(key = 180)]
    Id_ExtendedRATRestrictionInformation(ExtendedRATRestrictionInformation),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct RATRestrictions_ItemIE_Extensions_Entry {
    #[asn(key_field = true)]
    pub id: ProtocolExtensionID,
    pub criticality: Criticality,
    pub extension_value: RATRestrictions_ItemIE_Extensions_EntryExtensionValue,
}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct RATRestrictions_ItemIE_Extensions(Vec<RATRestrictions_ItemIE_Extensions_Entry>);

#[derive(Debug, AperCodec)]
#[asn(type = "ENUMERATED", extensible = true, lb = "0", ub = "1")]
pub struct ENUMERATED_73(u8);
impl ENUMERATED_73 {
    const RS_DETECTED: u8 = 0u8;
    const RS_DISAPPEARED: u8 = 1u8;
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct RIMInformationIE_Extensions_Entry {}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct RIMInformationIE_Extensions(Vec<RIMInformationIE_Extensions_Entry>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct RIMInformationTransferIE_Extensions_Entry {}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct RIMInformationTransferIE_Extensions(Vec<RIMInformationTransferIE_Extensions_Entry>);

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum RRCInactiveTransitionReportProtocolIEs_EntryValue {
    #[asn(key = 10)]
    Id_AMF_UE_NGAP_ID(AMF_UE_NGAP_ID),
    #[asn(key = 85)]
    Id_RAN_UE_NGAP_ID(RAN_UE_NGAP_ID),
    #[asn(key = 92)]
    Id_RRCState(RRCState),
    #[asn(key = 121)]
    Id_UserLocationInformation(UserLocationInformation),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct RRCInactiveTransitionReportProtocolIEs_Entry {
    #[asn(key_field = true)]
    pub id: ProtocolIE_ID,
    pub criticality: Criticality,
    pub value: RRCInactiveTransitionReportProtocolIEs_EntryValue,
}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "0",
    sz_ub = "65535"
)]
pub struct RRCInactiveTransitionReportProtocolIEs(
    Vec<RRCInactiveTransitionReportProtocolIEs_Entry>,
);

#[derive(Debug, AperCodec)]
#[asn(type = "INTEGER", lb = "0", ub = "4095")]
pub struct INTEGER_74(u16);

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
pub struct RecommendedRANNodeItemIE_Extensions_Entry {}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct RecommendedRANNodeItemIE_Extensions(Vec<RecommendedRANNodeItemIE_Extensions_Entry>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct RecommendedRANNodesForPagingIE_Extensions_Entry {}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct RecommendedRANNodesForPagingIE_Extensions(
    Vec<RecommendedRANNodesForPagingIE_Extensions_Entry>,
);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct RedundantPDUSessionInformationIE_Extensions_Entry {}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct RedundantPDUSessionInformationIE_Extensions(
    Vec<RedundantPDUSessionInformationIE_Extensions_Entry>,
);

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum RerouteNASRequestProtocolIEs_EntryValue {
    #[asn(key = 10)]
    Id_AMF_UE_NGAP_ID(AMF_UE_NGAP_ID),
    #[asn(key = 3)]
    Id_AMFSetID(AMFSetID),
    #[asn(key = 0)]
    Id_AllowedNSSAI(AllowedNSSAI),
    #[asn(key = 85)]
    Id_RAN_UE_NGAP_ID(RAN_UE_NGAP_ID),
    #[asn(key = 171)]
    Id_SourceToTarget_AMFInformationReroute(SourceToTarget_AMFInformationReroute),
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
pub struct ResetTypechoice_Extensions {}

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum RetrieveUEInformationProtocolIEs_EntryValue {
    #[asn(key = 26)]
    Id_FiveG_S_TMSI(FiveG_S_TMSI),
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
pub struct S_NSSAIIE_Extensions_Entry {}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct S_NSSAIIE_Extensions(Vec<S_NSSAIIE_Extensions_Entry>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct SNPN_MobilityInformationIE_Extensions_Entry {}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct SNPN_MobilityInformationIE_Extensions(Vec<SNPN_MobilityInformationIE_Extensions_Entry>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct SONConfigurationTransferIE_Extensions_Entry {}

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
pub enum SONInformationchoice_ExtensionsValue {
    #[asn(key = 252)]
    Id_SONInformationReport(SONInformationReport),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct SONInformationchoice_Extensions {
    #[asn(key_field = true)]
    pub id: ProtocolIE_ID,
    pub criticality: Criticality,
    pub value: SONInformationchoice_ExtensionsValue,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct SONInformationReplyIE_Extensions_Entry {}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct SONInformationReplyIE_Extensions(Vec<SONInformationReplyIE_Extensions_Entry>);

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
    #[asn(key = 10)]
    Id_AMF_UE_NGAP_ID(AMF_UE_NGAP_ID),
    #[asn(key = 143)]
    Id_HandoverFlag(HandoverFlag),
    #[asn(key = 142)]
    Id_PDUSessionResourceSecondaryRATUsageList(PDUSessionResourceSecondaryRATUsageList),
    #[asn(key = 85)]
    Id_RAN_UE_NGAP_ID(RAN_UE_NGAP_ID),
    #[asn(key = 121)]
    Id_UserLocationInformation(UserLocationInformation),
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
pub struct SecondaryRATDataUsageReportTransferIE_Extensions_Entry {}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct SecondaryRATDataUsageReportTransferIE_Extensions(
    Vec<SecondaryRATDataUsageReportTransferIE_Extensions_Entry>,
);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct SecondaryRATUsageInformationIE_Extension_Entry {}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct SecondaryRATUsageInformationIE_Extension(
    Vec<SecondaryRATUsageInformationIE_Extension_Entry>,
);

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
#[asn(type = "OPEN")]
pub enum SecurityIndicationIE_Extensions_EntryExtensionValue {
    #[asn(key = 151)]
    Id_MaximumIntegrityProtectedDataRate_DL(MaximumIntegrityProtectedDataRate),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct SecurityIndicationIE_Extensions_Entry {
    #[asn(key_field = true)]
    pub id: ProtocolExtensionID,
    pub criticality: Criticality,
    pub extension_value: SecurityIndicationIE_Extensions_EntryExtensionValue,
}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct SecurityIndicationIE_Extensions(Vec<SecurityIndicationIE_Extensions_Entry>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct SecurityResultIE_Extensions_Entry {}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct SecurityResultIE_Extensions(Vec<SecurityResultIE_Extensions_Entry>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct SensorMeasConfigNameItemIE_Extensions_Entry {}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct SensorMeasConfigNameItemIE_Extensions(Vec<SensorMeasConfigNameItemIE_Extensions_Entry>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct SensorMeasurementConfigurationIE_Extensions_Entry {}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct SensorMeasurementConfigurationIE_Extensions(
    Vec<SensorMeasurementConfigurationIE_Extensions_Entry>,
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
pub enum ServedGUAMIItemIE_Extensions_EntryExtensionValue {
    #[asn(key = 176)]
    Id_GUAMIType(GUAMIType),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct ServedGUAMIItemIE_Extensions_Entry {
    #[asn(key_field = true)]
    pub id: ProtocolExtensionID,
    pub criticality: Criticality,
    pub extension_value: ServedGUAMIItemIE_Extensions_EntryExtensionValue,
}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct ServedGUAMIItemIE_Extensions(Vec<ServedGUAMIItemIE_Extensions_Entry>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct ServiceAreaInformation_ItemIE_Extensions_Entry {}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct ServiceAreaInformation_ItemIE_Extensions(
    Vec<ServiceAreaInformation_ItemIE_Extensions_Entry>,
);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct SliceOverloadItemIE_Extensions_Entry {}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct SliceOverloadItemIE_Extensions(Vec<SliceOverloadItemIE_Extensions_Entry>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct SliceSupportItemIE_Extensions_Entry {}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct SliceSupportItemIE_Extensions(Vec<SliceSupportItemIE_Extensions_Entry>);

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum SourceNGRANNode_ToTargetNGRANNode_TransparentContainerIE_Extensions_EntryExtensionValue {
    #[asn(key = 182)]
    Id_SgNB_UE_X2AP_ID(SgNB_UE_X2AP_ID),
    #[asn(key = 253)]
    Id_UEHistoryInformationFromTheUE(UEHistoryInformationFromTheUE),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct SourceNGRANNode_ToTargetNGRANNode_TransparentContainerIE_Extensions_Entry {
    #[asn(key_field = true)]
    pub id: ProtocolExtensionID,
    pub criticality: Criticality,
    pub extension_value:
        SourceNGRANNode_ToTargetNGRANNode_TransparentContainerIE_Extensions_EntryExtensionValue,
}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct SourceNGRANNode_ToTargetNGRANNode_TransparentContainerIE_Extensions(
    Vec<SourceNGRANNode_ToTargetNGRANNode_TransparentContainerIE_Extensions_Entry>,
);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct SourceRANNodeIDIE_Extensions_Entry {}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct SourceRANNodeIDIE_Extensions(Vec<SourceRANNodeIDIE_Extensions_Entry>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct SourceToTarget_AMFInformationRerouteIE_Extensions_Entry {}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct SourceToTarget_AMFInformationRerouteIE_Extensions(
    Vec<SourceToTarget_AMFInformationRerouteIE_Extensions_Entry>,
);

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum SuccessfulOutcomeValue {
    #[asn(key = 0)]
    Id_AMFConfigurationUpdate(AMFConfigurationUpdateAcknowledge),
    #[asn(key = 10)]
    Id_HandoverCancel(HandoverCancelAcknowledge),
    #[asn(key = 12)]
    Id_HandoverPreparation(HandoverCommand),
    #[asn(key = 13)]
    Id_HandoverResourceAllocation(HandoverRequestAcknowledge),
    #[asn(key = 14)]
    Id_InitialContextSetup(InitialContextSetupResponse),
    #[asn(key = 20)]
    Id_NGReset(NGResetAcknowledge),
    #[asn(key = 21)]
    Id_NGSetup(NGSetupResponse),
    #[asn(key = 26)]
    Id_PDUSessionResourceModify(PDUSessionResourceModifyResponse),
    #[asn(key = 27)]
    Id_PDUSessionResourceModifyIndication(PDUSessionResourceModifyConfirm),
    #[asn(key = 28)]
    Id_PDUSessionResourceRelease(PDUSessionResourceReleaseResponse),
    #[asn(key = 29)]
    Id_PDUSessionResourceSetup(PDUSessionResourceSetupResponse),
    #[asn(key = 32)]
    Id_PWSCancel(PWSCancelResponse),
    #[asn(key = 25)]
    Id_PathSwitchRequest(PathSwitchRequestAcknowledge),
    #[asn(key = 35)]
    Id_RANConfigurationUpdate(RANConfigurationUpdateAcknowledge),
    #[asn(key = 40)]
    Id_UEContextModification(UEContextModificationResponse),
    #[asn(key = 41)]
    Id_UEContextRelease(UEContextReleaseComplete),
    #[asn(key = 58)]
    Id_UEContextResume(UEContextResumeResponse),
    #[asn(key = 59)]
    Id_UEContextSuspend(UEContextSuspendResponse),
    #[asn(key = 43)]
    Id_UERadioCapabilityCheck(UERadioCapabilityCheckResponse),
    #[asn(key = 60)]
    Id_UERadioCapabilityIDMapping(UERadioCapabilityIDMappingResponse),
    #[asn(key = 51)]
    Id_WriteReplaceWarning(WriteReplaceWarningResponse),
}

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum SupportedTAItemIE_Extensions_EntryExtensionValue {
    #[asn(key = 272)]
    Id_ConfiguredTACIndication(ConfiguredTACIndication),
    #[asn(key = 179)]
    Id_RAT_Information(RAT_Information),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct SupportedTAItemIE_Extensions_Entry {
    #[asn(key_field = true)]
    pub id: ProtocolExtensionID,
    pub criticality: Criticality,
    pub extension_value: SupportedTAItemIE_Extensions_EntryExtensionValue,
}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct SupportedTAItemIE_Extensions(Vec<SupportedTAItemIE_Extensions_Entry>);

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
pub struct TAIBroadcastEUTRA_ItemIE_Extensions_Entry {}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct TAIBroadcastEUTRA_ItemIE_Extensions(Vec<TAIBroadcastEUTRA_ItemIE_Extensions_Entry>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct TAIBroadcastNR_ItemIE_Extensions_Entry {}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct TAIBroadcastNR_ItemIE_Extensions(Vec<TAIBroadcastNR_ItemIE_Extensions_Entry>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct TAICancelledEUTRA_ItemIE_Extensions_Entry {}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct TAICancelledEUTRA_ItemIE_Extensions(Vec<TAICancelledEUTRA_ItemIE_Extensions_Entry>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct TAICancelledNR_ItemIE_Extensions_Entry {}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct TAICancelledNR_ItemIE_Extensions(Vec<TAICancelledNR_ItemIE_Extensions_Entry>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct TAIListForInactiveItemIE_Extensions_Entry {}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct TAIListForInactiveItemIE_Extensions(Vec<TAIListForInactiveItemIE_Extensions_Entry>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct TAIListForPagingItemIE_Extensions_Entry {}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct TAIListForPagingItemIE_Extensions(Vec<TAIListForPagingItemIE_Extensions_Entry>);

#[derive(Debug, AperCodec)]
#[asn(type = "BITSTRING", sz_extensible = true, sz_lb = "32", sz_ub = "32")]
pub struct BIT_STRING_81(BitVec<Msb0, u8>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct TNGF_IDchoice_Extensions {}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct TNLAssociationItemIE_Extensions_Entry {}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct TNLAssociationItemIE_Extensions(Vec<TNLAssociationItemIE_Extensions_Entry>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct TSCAssistanceInformationIE_Extensions_Entry {}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct TSCAssistanceInformationIE_Extensions(Vec<TSCAssistanceInformationIE_Extensions_Entry>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct TSCTrafficCharacteristicsIE_Extensions_Entry {}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct TSCTrafficCharacteristicsIE_Extensions(
    Vec<TSCTrafficCharacteristicsIE_Extensions_Entry>,
);

#[derive(Debug, AperCodec)]
#[asn(type = "BITSTRING", sz_extensible = true, sz_lb = "32", sz_ub = "32")]
pub struct BIT_STRING_82(BitVec<Msb0, u8>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct TWIF_IDchoice_Extensions {}

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum TargetIDchoice_ExtensionsValue {
    #[asn(key = 178)]
    Id_TargetRNC_ID(TargetRNC_ID),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct TargetIDchoice_Extensions {
    #[asn(key_field = true)]
    pub id: ProtocolIE_ID,
    pub criticality: Criticality,
    pub value: TargetIDchoice_ExtensionsValue,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct TargetNGRANNode_ToSourceNGRANNode_FailureTransparentContainerIE_Extensions_Entry {}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct TargetNGRANNode_ToSourceNGRANNode_FailureTransparentContainerIE_Extensions(
    Vec<TargetNGRANNode_ToSourceNGRANNode_FailureTransparentContainerIE_Extensions_Entry>,
);

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum TargetNGRANNode_ToSourceNGRANNode_TransparentContainerIE_Extensions_EntryExtensionValue {
    #[asn(key = 267)]
    Id_DAPSResponseInfoList(DAPSResponseInfoList),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct TargetNGRANNode_ToSourceNGRANNode_TransparentContainerIE_Extensions_Entry {
    #[asn(key_field = true)]
    pub id: ProtocolExtensionID,
    pub criticality: Criticality,
    pub extension_value:
        TargetNGRANNode_ToSourceNGRANNode_TransparentContainerIE_Extensions_EntryExtensionValue,
}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct TargetNGRANNode_ToSourceNGRANNode_TransparentContainerIE_Extensions(
    Vec<TargetNGRANNode_ToSourceNGRANNode_TransparentContainerIE_Extensions_Entry>,
);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct TargetRANNodeIDIE_Extensions_Entry {}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct TargetRANNodeIDIE_Extensions(Vec<TargetRANNodeIDIE_Extensions_Entry>);

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
#[asn(type = "SEQUENCE", extensible = false)]
pub struct TooearlyIntersystemHOIE_Extensions_Entry {}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct TooearlyIntersystemHOIE_Extensions(Vec<TooearlyIntersystemHOIE_Extensions_Entry>);

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum TraceActivationIE_Extensions_EntryExtensionValue {
    #[asn(key = 255)]
    Id_MDTConfiguration(MDT_Configuration),
    #[asn(key = 257)]
    Id_TraceCollectionEntityURI(URI_address),
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
    #[asn(key = 10)]
    Id_AMF_UE_NGAP_ID(AMF_UE_NGAP_ID),
    #[asn(key = 15)]
    Id_Cause(Cause),
    #[asn(key = 44)]
    Id_NGRANTraceID(NGRANTraceID),
    #[asn(key = 85)]
    Id_RAN_UE_NGAP_ID(RAN_UE_NGAP_ID),
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
    #[asn(key = 10)]
    Id_AMF_UE_NGAP_ID(AMF_UE_NGAP_ID),
    #[asn(key = 85)]
    Id_RAN_UE_NGAP_ID(RAN_UE_NGAP_ID),
    #[asn(key = 108)]
    Id_TraceActivation(TraceActivation),
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
pub struct UE_DifferentiationInfoIE_Extensions_Entry {}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct UE_DifferentiationInfoIE_Extensions(Vec<UE_DifferentiationInfoIE_Extensions_Entry>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct UE_NGAP_ID_pairIE_Extensions_Entry {}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct UE_NGAP_ID_pairIE_Extensions(Vec<UE_NGAP_ID_pairIE_Extensions_Entry>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct UE_NGAP_IDschoice_Extensions {}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct UE_associatedLogicalNG_connectionItemIE_Extensions_Entry {}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct UE_associatedLogicalNG_connectionItemIE_Extensions(
    Vec<UE_associatedLogicalNG_connectionItemIE_Extensions_Entry>,
);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct UEAggregateMaximumBitRateIE_Extensions_Entry {}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct UEAggregateMaximumBitRateIE_Extensions(
    Vec<UEAggregateMaximumBitRateIE_Extensions_Entry>,
);

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum UEContextModificationFailureProtocolIEs_EntryValue {
    #[asn(key = 10)]
    Id_AMF_UE_NGAP_ID(AMF_UE_NGAP_ID),
    #[asn(key = 15)]
    Id_Cause(Cause),
    #[asn(key = 19)]
    Id_CriticalityDiagnostics(CriticalityDiagnostics),
    #[asn(key = 85)]
    Id_RAN_UE_NGAP_ID(RAN_UE_NGAP_ID),
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
pub enum UEContextModificationRequestProtocolIEs_EntryValue {
    #[asn(key = 10)]
    Id_AMF_UE_NGAP_ID(AMF_UE_NGAP_ID),
    #[asn(key = 165)]
    Id_CNAssistedRANTuning(CNAssistedRANTuning),
    #[asn(key = 18)]
    Id_CoreNetworkAssistanceInformationForInactive(CoreNetworkAssistanceInformationForInactive),
    #[asn(key = 24)]
    Id_EmergencyFallbackIndicator(EmergencyFallbackIndicator),
    #[asn(key = 199)]
    Id_IAB_Authorized(IAB_Authorized),
    #[asn(key = 31)]
    Id_IndexToRFSP(IndexToRFSP),
    #[asn(key = 217)]
    Id_LTEUESidelinkAggregateMaximumBitrate(LTEUESidelinkAggregateMaximumBitrate),
    #[asn(key = 215)]
    Id_LTEV2XServicesAuthorized(LTEV2XServicesAuthorized),
    #[asn(key = 218)]
    Id_NRUESidelinkAggregateMaximumBitrate(NRUESidelinkAggregateMaximumBitrate),
    #[asn(key = 216)]
    Id_NRV2XServicesAuthorized(NRV2XServicesAuthorized),
    #[asn(key = 40)]
    Id_NewAMF_UE_NGAP_ID(AMF_UE_NGAP_ID),
    #[asn(key = 162)]
    Id_NewGUAMI(GUAMI),
    #[asn(key = 219)]
    Id_PC5QoSParameters(PC5QoSParameters),
    #[asn(key = 85)]
    Id_RAN_UE_NGAP_ID(RAN_UE_NGAP_ID),
    #[asn(key = 83)]
    Id_RANPagingPriority(RANPagingPriority),
    #[asn(key = 238)]
    Id_RGLevelWirelineAccessCharacteristics(RGLevelWirelineAccessCharacteristics),
    #[asn(key = 91)]
    Id_RRCInactiveTransitionReportRequest(RRCInactiveTransitionReportRequest),
    #[asn(key = 177)]
    Id_SRVCCOperationPossible(SRVCCOperationPossible),
    #[asn(key = 94)]
    Id_SecurityKey(SecurityKey),
    #[asn(key = 110)]
    Id_UEAggregateMaximumBitRate(UEAggregateMaximumBitRate),
    #[asn(key = 264)]
    Id_UERadioCapabilityID(UERadioCapabilityID),
    #[asn(key = 119)]
    Id_UESecurityCapabilities(UESecurityCapabilities),
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
    #[asn(key = 10)]
    Id_AMF_UE_NGAP_ID(AMF_UE_NGAP_ID),
    #[asn(key = 19)]
    Id_CriticalityDiagnostics(CriticalityDiagnostics),
    #[asn(key = 85)]
    Id_RAN_UE_NGAP_ID(RAN_UE_NGAP_ID),
    #[asn(key = 92)]
    Id_RRCState(RRCState),
    #[asn(key = 121)]
    Id_UserLocationInformation(UserLocationInformation),
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
    #[asn(key = 15)]
    Id_Cause(Cause),
    #[asn(key = 114)]
    Id_UE_NGAP_IDs(UE_NGAP_IDs),
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
    #[asn(key = 10)]
    Id_AMF_UE_NGAP_ID(AMF_UE_NGAP_ID),
    #[asn(key = 19)]
    Id_CriticalityDiagnostics(CriticalityDiagnostics),
    #[asn(key = 32)]
    Id_InfoOnRecommendedCellsAndRANNodesForPaging(InfoOnRecommendedCellsAndRANNodesForPaging),
    #[asn(key = 60)]
    Id_PDUSessionResourceListCxtRelCpl(PDUSessionResourceListCxtRelCpl),
    #[asn(key = 207)]
    Id_PagingAssisDataforCEcapabUE(PagingAssisDataforCEcapabUE),
    #[asn(key = 85)]
    Id_RAN_UE_NGAP_ID(RAN_UE_NGAP_ID),
    #[asn(key = 121)]
    Id_UserLocationInformation(UserLocationInformation),
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
    #[asn(key = 10)]
    Id_AMF_UE_NGAP_ID(AMF_UE_NGAP_ID),
    #[asn(key = 15)]
    Id_Cause(Cause),
    #[asn(key = 133)]
    Id_PDUSessionResourceListCxtRelReq(PDUSessionResourceListCxtRelReq),
    #[asn(key = 85)]
    Id_RAN_UE_NGAP_ID(RAN_UE_NGAP_ID),
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
    #[asn(key = 10)]
    Id_AMF_UE_NGAP_ID(AMF_UE_NGAP_ID),
    #[asn(key = 15)]
    Id_Cause(Cause),
    #[asn(key = 19)]
    Id_CriticalityDiagnostics(CriticalityDiagnostics),
    #[asn(key = 85)]
    Id_RAN_UE_NGAP_ID(RAN_UE_NGAP_ID),
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
    #[asn(key = 10)]
    Id_AMF_UE_NGAP_ID(AMF_UE_NGAP_ID),
    #[asn(key = 32)]
    Id_InfoOnRecommendedCellsAndRANNodesForPaging(InfoOnRecommendedCellsAndRANNodesForPaging),
    #[asn(key = 229)]
    Id_PDUSessionResourceFailedToResumeListRESReq(PDUSessionResourceFailedToResumeListRESReq),
    #[asn(key = 232)]
    Id_PDUSessionResourceResumeListRESReq(PDUSessionResourceResumeListRESReq),
    #[asn(key = 207)]
    Id_PagingAssisDataforCEcapabUE(PagingAssisDataforCEcapabUE),
    #[asn(key = 85)]
    Id_RAN_UE_NGAP_ID(RAN_UE_NGAP_ID),
    #[asn(key = 237)]
    Id_RRC_Resume_Cause(RRCEstablishmentCause),
    #[asn(key = 235)]
    Id_Suspend_Request_Indication(Suspend_Request_Indication),
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
#[asn(type = "SEQUENCE", extensible = false)]
pub struct UEContextResumeRequestTransferIE_Extensions_Entry {}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct UEContextResumeRequestTransferIE_Extensions(
    Vec<UEContextResumeRequestTransferIE_Extensions_Entry>,
);

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum UEContextResumeResponseProtocolIEs_EntryValue {
    #[asn(key = 10)]
    Id_AMF_UE_NGAP_ID(AMF_UE_NGAP_ID),
    #[asn(key = 19)]
    Id_CriticalityDiagnostics(CriticalityDiagnostics),
    #[asn(key = 206)]
    Id_Extended_ConnectedTime(Extended_ConnectedTime),
    #[asn(key = 230)]
    Id_PDUSessionResourceFailedToResumeListRESRes(PDUSessionResourceFailedToResumeListRESRes),
    #[asn(key = 233)]
    Id_PDUSessionResourceResumeListRESRes(PDUSessionResourceResumeListRESRes),
    #[asn(key = 85)]
    Id_RAN_UE_NGAP_ID(RAN_UE_NGAP_ID),
    #[asn(key = 93)]
    Id_SecurityContext(SecurityContext),
    #[asn(key = 236)]
    Id_Suspend_Response_Indication(Suspend_Response_Indication),
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
#[asn(type = "SEQUENCE", extensible = false)]
pub struct UEContextResumeResponseTransferIE_Extensions_Entry {}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct UEContextResumeResponseTransferIE_Extensions(
    Vec<UEContextResumeResponseTransferIE_Extensions_Entry>,
);

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum UEContextSuspendFailureProtocolIEs_EntryValue {
    #[asn(key = 10)]
    Id_AMF_UE_NGAP_ID(AMF_UE_NGAP_ID),
    #[asn(key = 15)]
    Id_Cause(Cause),
    #[asn(key = 19)]
    Id_CriticalityDiagnostics(CriticalityDiagnostics),
    #[asn(key = 85)]
    Id_RAN_UE_NGAP_ID(RAN_UE_NGAP_ID),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct UEContextSuspendFailureProtocolIEs_Entry {
    #[asn(key_field = true)]
    pub id: ProtocolIE_ID,
    pub criticality: Criticality,
    pub value: UEContextSuspendFailureProtocolIEs_EntryValue,
}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "0",
    sz_ub = "65535"
)]
pub struct UEContextSuspendFailureProtocolIEs(Vec<UEContextSuspendFailureProtocolIEs_Entry>);

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum UEContextSuspendRequestProtocolIEs_EntryValue {
    #[asn(key = 10)]
    Id_AMF_UE_NGAP_ID(AMF_UE_NGAP_ID),
    #[asn(key = 32)]
    Id_InfoOnRecommendedCellsAndRANNodesForPaging(InfoOnRecommendedCellsAndRANNodesForPaging),
    #[asn(key = 231)]
    Id_PDUSessionResourceSuspendListSUSReq(PDUSessionResourceSuspendListSUSReq),
    #[asn(key = 207)]
    Id_PagingAssisDataforCEcapabUE(PagingAssisDataforCEcapabUE),
    #[asn(key = 85)]
    Id_RAN_UE_NGAP_ID(RAN_UE_NGAP_ID),
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
#[asn(type = "SEQUENCE", extensible = false)]
pub struct UEContextSuspendRequestTransferIE_Extensions_Entry {}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct UEContextSuspendRequestTransferIE_Extensions(
    Vec<UEContextSuspendRequestTransferIE_Extensions_Entry>,
);

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum UEContextSuspendResponseProtocolIEs_EntryValue {
    #[asn(key = 10)]
    Id_AMF_UE_NGAP_ID(AMF_UE_NGAP_ID),
    #[asn(key = 19)]
    Id_CriticalityDiagnostics(CriticalityDiagnostics),
    #[asn(key = 85)]
    Id_RAN_UE_NGAP_ID(RAN_UE_NGAP_ID),
    #[asn(key = 93)]
    Id_SecurityContext(SecurityContext),
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
pub enum UEInformationTransferProtocolIEs_EntryValue {
    #[asn(key = 0)]
    Id_AllowedNSSAI(AllowedNSSAI),
    #[asn(key = 26)]
    Id_FiveG_S_TMSI(FiveG_S_TMSI),
    #[asn(key = 210)]
    Id_NB_IoT_UEPriority(NB_IoT_UEPriority),
    #[asn(key = 148)]
    Id_S_NSSAI(S_NSSAI),
    #[asn(key = 209)]
    Id_UE_DifferentiationInfo(UE_DifferentiationInfo),
    #[asn(key = 117)]
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
#[asn(type = "SEQUENCE", extensible = false)]
pub struct UEPagingIdentitychoice_Extensions {}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct UEPresenceInAreaOfInterestItemIE_Extensions_Entry {}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct UEPresenceInAreaOfInterestItemIE_Extensions(
    Vec<UEPresenceInAreaOfInterestItemIE_Extensions_Entry>,
);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct UERLFReportContainerchoice_Extensions {}

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum UERadioCapabilityCheckRequestProtocolIEs_EntryValue {
    #[asn(key = 10)]
    Id_AMF_UE_NGAP_ID(AMF_UE_NGAP_ID),
    #[asn(key = 85)]
    Id_RAN_UE_NGAP_ID(RAN_UE_NGAP_ID),
    #[asn(key = 117)]
    Id_UERadioCapability(UERadioCapability),
    #[asn(key = 264)]
    Id_UERadioCapabilityID(UERadioCapabilityID),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct UERadioCapabilityCheckRequestProtocolIEs_Entry {
    #[asn(key_field = true)]
    pub id: ProtocolIE_ID,
    pub criticality: Criticality,
    pub value: UERadioCapabilityCheckRequestProtocolIEs_EntryValue,
}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "0",
    sz_ub = "65535"
)]
pub struct UERadioCapabilityCheckRequestProtocolIEs(
    Vec<UERadioCapabilityCheckRequestProtocolIEs_Entry>,
);

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum UERadioCapabilityCheckResponseProtocolIEs_EntryValue {
    #[asn(key = 10)]
    Id_AMF_UE_NGAP_ID(AMF_UE_NGAP_ID),
    #[asn(key = 19)]
    Id_CriticalityDiagnostics(CriticalityDiagnostics),
    #[asn(key = 30)]
    Id_IMSVoiceSupportIndicator(IMSVoiceSupportIndicator),
    #[asn(key = 85)]
    Id_RAN_UE_NGAP_ID(RAN_UE_NGAP_ID),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct UERadioCapabilityCheckResponseProtocolIEs_Entry {
    #[asn(key_field = true)]
    pub id: ProtocolIE_ID,
    pub criticality: Criticality,
    pub value: UERadioCapabilityCheckResponseProtocolIEs_EntryValue,
}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "0",
    sz_ub = "65535"
)]
pub struct UERadioCapabilityCheckResponseProtocolIEs(
    Vec<UERadioCapabilityCheckResponseProtocolIEs_Entry>,
);

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum UERadioCapabilityForPagingIE_Extensions_EntryExtensionValue {
    #[asn(key = 214)]
    Id_UERadioCapabilityForPagingOfNB_IoT(UERadioCapabilityForPagingOfNB_IoT),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct UERadioCapabilityForPagingIE_Extensions_Entry {
    #[asn(key_field = true)]
    pub id: ProtocolExtensionID,
    pub criticality: Criticality,
    pub extension_value: UERadioCapabilityForPagingIE_Extensions_EntryExtensionValue,
}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct UERadioCapabilityForPagingIE_Extensions(
    Vec<UERadioCapabilityForPagingIE_Extensions_Entry>,
);

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum UERadioCapabilityIDMappingRequestProtocolIEs_EntryValue {
    #[asn(key = 264)]
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
    #[asn(key = 19)]
    Id_CriticalityDiagnostics(CriticalityDiagnostics),
    #[asn(key = 117)]
    Id_UERadioCapability(UERadioCapability),
    #[asn(key = 264)]
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
pub enum UERadioCapabilityInfoIndicationProtocolIEs_EntryValue {
    #[asn(key = 10)]
    Id_AMF_UE_NGAP_ID(AMF_UE_NGAP_ID),
    #[asn(key = 85)]
    Id_RAN_UE_NGAP_ID(RAN_UE_NGAP_ID),
    #[asn(key = 117)]
    Id_UERadioCapability(UERadioCapability),
    #[asn(key = 265)]
    Id_UERadioCapability_EUTRA_Format(UERadioCapability),
    #[asn(key = 118)]
    Id_UERadioCapabilityForPaging(UERadioCapabilityForPaging),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct UERadioCapabilityInfoIndicationProtocolIEs_Entry {
    #[asn(key_field = true)]
    pub id: ProtocolIE_ID,
    pub criticality: Criticality,
    pub value: UERadioCapabilityInfoIndicationProtocolIEs_EntryValue,
}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "0",
    sz_ub = "65535"
)]
pub struct UERadioCapabilityInfoIndicationProtocolIEs(
    Vec<UERadioCapabilityInfoIndicationProtocolIEs_Entry>,
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
#[asn(type = "OPEN")]
pub enum UETNLABindingReleaseRequestProtocolIEs_EntryValue {
    #[asn(key = 10)]
    Id_AMF_UE_NGAP_ID(AMF_UE_NGAP_ID),
    #[asn(key = 85)]
    Id_RAN_UE_NGAP_ID(RAN_UE_NGAP_ID),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct UETNLABindingReleaseRequestProtocolIEs_Entry {
    #[asn(key_field = true)]
    pub id: ProtocolIE_ID,
    pub criticality: Criticality,
    pub value: UETNLABindingReleaseRequestProtocolIEs_EntryValue,
}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "0",
    sz_ub = "65535"
)]
pub struct UETNLABindingReleaseRequestProtocolIEs(
    Vec<UETNLABindingReleaseRequestProtocolIEs_Entry>,
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
pub enum UL_NGU_UP_TNLModifyItemIE_Extensions_EntryExtensionValue {
    #[asn(key = 192)]
    Id_RedundantDL_NGU_UP_TNLInformation(UPTransportLayerInformation),
    #[asn(key = 195)]
    Id_RedundantUL_NGU_UP_TNLInformation(UPTransportLayerInformation),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct UL_NGU_UP_TNLModifyItemIE_Extensions_Entry {
    #[asn(key_field = true)]
    pub id: ProtocolExtensionID,
    pub criticality: Criticality,
    pub extension_value: UL_NGU_UP_TNLModifyItemIE_Extensions_EntryExtensionValue,
}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct UL_NGU_UP_TNLModifyItemIE_Extensions(Vec<UL_NGU_UP_TNLModifyItemIE_Extensions_Entry>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct UPTransportLayerInformationchoice_Extensions {}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct UPTransportLayerInformationItemIE_Extensions_Entry {}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct UPTransportLayerInformationItemIE_Extensions(
    Vec<UPTransportLayerInformationItemIE_Extensions_Entry>,
);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct UPTransportLayerInformationPairItemIE_Extensions_Entry {}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct UPTransportLayerInformationPairItemIE_Extensions(
    Vec<UPTransportLayerInformationPairItemIE_Extensions_Entry>,
);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct UnavailableGUAMIItemIE_Extensions_Entry {}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct UnavailableGUAMIItemIE_Extensions(Vec<UnavailableGUAMIItemIE_Extensions_Entry>);

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum UnsuccessfulOutcomeValue {
    #[asn(key = 0)]
    Id_AMFConfigurationUpdate(AMFConfigurationUpdateFailure),
    #[asn(key = 12)]
    Id_HandoverPreparation(HandoverPreparationFailure),
    #[asn(key = 13)]
    Id_HandoverResourceAllocation(HandoverFailure),
    #[asn(key = 14)]
    Id_InitialContextSetup(InitialContextSetupFailure),
    #[asn(key = 21)]
    Id_NGSetup(NGSetupFailure),
    #[asn(key = 25)]
    Id_PathSwitchRequest(PathSwitchRequestFailure),
    #[asn(key = 35)]
    Id_RANConfigurationUpdate(RANConfigurationUpdateFailure),
    #[asn(key = 40)]
    Id_UEContextModification(UEContextModificationFailure),
    #[asn(key = 58)]
    Id_UEContextResume(UEContextResumeFailure),
    #[asn(key = 59)]
    Id_UEContextSuspend(UEContextSuspendFailure),
}

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum UplinkNASTransportProtocolIEs_EntryValue {
    #[asn(key = 10)]
    Id_AMF_UE_NGAP_ID(AMF_UE_NGAP_ID),
    #[asn(key = 38)]
    Id_NAS_PDU(NAS_PDU),
    #[asn(key = 85)]
    Id_RAN_UE_NGAP_ID(RAN_UE_NGAP_ID),
    #[asn(key = 121)]
    Id_UserLocationInformation(UserLocationInformation),
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
pub enum UplinkNonUEAssociatedNRPPaTransportProtocolIEs_EntryValue {
    #[asn(key = 46)]
    Id_NRPPa_PDU(NRPPa_PDU),
    #[asn(key = 89)]
    Id_RoutingID(RoutingID),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct UplinkNonUEAssociatedNRPPaTransportProtocolIEs_Entry {
    #[asn(key_field = true)]
    pub id: ProtocolIE_ID,
    pub criticality: Criticality,
    pub value: UplinkNonUEAssociatedNRPPaTransportProtocolIEs_EntryValue,
}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "0",
    sz_ub = "65535"
)]
pub struct UplinkNonUEAssociatedNRPPaTransportProtocolIEs(
    Vec<UplinkNonUEAssociatedNRPPaTransportProtocolIEs_Entry>,
);

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum UplinkRANConfigurationTransferProtocolIEs_EntryValue {
    #[asn(key = 158)]
    Id_ENDC_SONConfigurationTransferUL(EN_DCSONConfigurationTransfer),
    #[asn(key = 251)]
    Id_IntersystemSONConfigurationTransferUL(IntersystemSONConfigurationTransfer),
    #[asn(key = 99)]
    Id_SONConfigurationTransferUL(SONConfigurationTransfer),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct UplinkRANConfigurationTransferProtocolIEs_Entry {
    #[asn(key_field = true)]
    pub id: ProtocolIE_ID,
    pub criticality: Criticality,
    pub value: UplinkRANConfigurationTransferProtocolIEs_EntryValue,
}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "0",
    sz_ub = "65535"
)]
pub struct UplinkRANConfigurationTransferProtocolIEs(
    Vec<UplinkRANConfigurationTransferProtocolIEs_Entry>,
);

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum UplinkRANEarlyStatusTransferProtocolIEs_EntryValue {
    #[asn(key = 10)]
    Id_AMF_UE_NGAP_ID(AMF_UE_NGAP_ID),
    #[asn(key = 268)]
    Id_EarlyStatusTransfer_TransparentContainer(EarlyStatusTransfer_TransparentContainer),
    #[asn(key = 85)]
    Id_RAN_UE_NGAP_ID(RAN_UE_NGAP_ID),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct UplinkRANEarlyStatusTransferProtocolIEs_Entry {
    #[asn(key_field = true)]
    pub id: ProtocolIE_ID,
    pub criticality: Criticality,
    pub value: UplinkRANEarlyStatusTransferProtocolIEs_EntryValue,
}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "0",
    sz_ub = "65535"
)]
pub struct UplinkRANEarlyStatusTransferProtocolIEs(
    Vec<UplinkRANEarlyStatusTransferProtocolIEs_Entry>,
);

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum UplinkRANStatusTransferProtocolIEs_EntryValue {
    #[asn(key = 10)]
    Id_AMF_UE_NGAP_ID(AMF_UE_NGAP_ID),
    #[asn(key = 85)]
    Id_RAN_UE_NGAP_ID(RAN_UE_NGAP_ID),
    #[asn(key = 84)]
    Id_RANStatusTransfer_TransparentContainer(RANStatusTransfer_TransparentContainer),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct UplinkRANStatusTransferProtocolIEs_Entry {
    #[asn(key_field = true)]
    pub id: ProtocolIE_ID,
    pub criticality: Criticality,
    pub value: UplinkRANStatusTransferProtocolIEs_EntryValue,
}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "0",
    sz_ub = "65535"
)]
pub struct UplinkRANStatusTransferProtocolIEs(Vec<UplinkRANStatusTransferProtocolIEs_Entry>);

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum UplinkRIMInformationTransferProtocolIEs_EntryValue {
    #[asn(key = 175)]
    Id_RIMInformationTransfer(RIMInformationTransfer),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct UplinkRIMInformationTransferProtocolIEs_Entry {
    #[asn(key_field = true)]
    pub id: ProtocolIE_ID,
    pub criticality: Criticality,
    pub value: UplinkRIMInformationTransferProtocolIEs_EntryValue,
}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "0",
    sz_ub = "65535"
)]
pub struct UplinkRIMInformationTransferProtocolIEs(
    Vec<UplinkRIMInformationTransferProtocolIEs_Entry>,
);

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum UplinkUEAssociatedNRPPaTransportProtocolIEs_EntryValue {
    #[asn(key = 10)]
    Id_AMF_UE_NGAP_ID(AMF_UE_NGAP_ID),
    #[asn(key = 46)]
    Id_NRPPa_PDU(NRPPa_PDU),
    #[asn(key = 85)]
    Id_RAN_UE_NGAP_ID(RAN_UE_NGAP_ID),
    #[asn(key = 89)]
    Id_RoutingID(RoutingID),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct UplinkUEAssociatedNRPPaTransportProtocolIEs_Entry {
    #[asn(key_field = true)]
    pub id: ProtocolIE_ID,
    pub criticality: Criticality,
    pub value: UplinkUEAssociatedNRPPaTransportProtocolIEs_EntryValue,
}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "0",
    sz_ub = "65535"
)]
pub struct UplinkUEAssociatedNRPPaTransportProtocolIEs(
    Vec<UplinkUEAssociatedNRPPaTransportProtocolIEs_Entry>,
);

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum UserLocationInformationchoice_ExtensionsValue {
    #[asn(key = 244)]
    Id_UserLocationInformationTNGF(UserLocationInformationTNGF),
    #[asn(key = 248)]
    Id_UserLocationInformationTWIF(UserLocationInformationTWIF),
    #[asn(key = 243)]
    Id_UserLocationInformationW_AGF(UserLocationInformationW_AGF),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct UserLocationInformationchoice_Extensions {
    #[asn(key_field = true)]
    pub id: ProtocolIE_ID,
    pub criticality: Criticality,
    pub value: UserLocationInformationchoice_ExtensionsValue,
}

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum UserLocationInformationEUTRAIE_Extensions_EntryExtensionValue {
    #[asn(key = 149)]
    Id_PSCellInformation(NGRAN_CGI),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct UserLocationInformationEUTRAIE_Extensions_Entry {
    #[asn(key_field = true)]
    pub id: ProtocolExtensionID,
    pub criticality: Criticality,
    pub extension_value: UserLocationInformationEUTRAIE_Extensions_EntryExtensionValue,
}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct UserLocationInformationEUTRAIE_Extensions(
    Vec<UserLocationInformationEUTRAIE_Extensions_Entry>,
);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct UserLocationInformationN3IWFIE_Extensions_Entry {}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct UserLocationInformationN3IWFIE_Extensions(
    Vec<UserLocationInformationN3IWFIE_Extensions_Entry>,
);

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum UserLocationInformationNRIE_Extensions_EntryExtensionValue {
    #[asn(key = 263)]
    Id_NID(NID),
    #[asn(key = 149)]
    Id_PSCellInformation(NGRAN_CGI),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct UserLocationInformationNRIE_Extensions_Entry {
    #[asn(key_field = true)]
    pub id: ProtocolExtensionID,
    pub criticality: Criticality,
    pub extension_value: UserLocationInformationNRIE_Extensions_EntryExtensionValue,
}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct UserLocationInformationNRIE_Extensions(
    Vec<UserLocationInformationNRIE_Extensions_Entry>,
);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct UserLocationInformationTNGFIE_Extensions_Entry {}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct UserLocationInformationTNGFIE_Extensions(
    Vec<UserLocationInformationTNGFIE_Extensions_Entry>,
);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct UserLocationInformationTWIFIE_Extensions_Entry {}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct UserLocationInformationTWIFIE_Extensions(
    Vec<UserLocationInformationTWIFIE_Extensions_Entry>,
);

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum UserLocationInformationW_AGFchoice_ExtensionsValue {
    #[asn(key = 275)]
    Id_GlobalCable_ID(GlobalCable_ID),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct UserLocationInformationW_AGFchoice_Extensions {
    #[asn(key_field = true)]
    pub id: ProtocolIE_ID,
    pub criticality: Criticality,
    pub value: UserLocationInformationW_AGFchoice_ExtensionsValue,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct UserPlaneSecurityInformationIE_Extensions_Entry {}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct UserPlaneSecurityInformationIE_Extensions(
    Vec<UserPlaneSecurityInformationIE_Extensions_Entry>,
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
pub struct VolumeTimedReport_ItemIE_Extensions_Entry {}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct VolumeTimedReport_ItemIE_Extensions(Vec<VolumeTimedReport_ItemIE_Extensions_Entry>);

#[derive(Debug, AperCodec)]
#[asn(type = "BITSTRING", sz_extensible = true, sz_lb = "16", sz_ub = "16")]
pub struct BIT_STRING_93(BitVec<Msb0, u8>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct W_AGF_IDchoice_Extensions {}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct WLANMeasConfigNameItemIE_Extensions_Entry {}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct WLANMeasConfigNameItemIE_Extensions(Vec<WLANMeasConfigNameItemIE_Extensions_Entry>);

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
#[asn(type = "SEQUENCE", extensible = false)]
pub struct WarningAreaListchoice_Extensions {}

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum WriteReplaceWarningRequestProtocolIEs_EntryValue {
    #[asn(key = 17)]
    Id_ConcurrentWarningMessageInd(ConcurrentWarningMessageInd),
    #[asn(key = 20)]
    Id_DataCodingScheme(DataCodingScheme),
    #[asn(key = 35)]
    Id_MessageIdentifier(MessageIdentifier),
    #[asn(key = 47)]
    Id_NumberOfBroadcastsRequested(NumberOfBroadcastsRequested),
    #[asn(key = 87)]
    Id_RepetitionPeriod(RepetitionPeriod),
    #[asn(key = 95)]
    Id_SerialNumber(SerialNumber),
    #[asn(key = 141)]
    Id_WarningAreaCoordinates(WarningAreaCoordinates),
    #[asn(key = 122)]
    Id_WarningAreaList(WarningAreaList),
    #[asn(key = 123)]
    Id_WarningMessageContents(WarningMessageContents),
    #[asn(key = 124)]
    Id_WarningSecurityInfo(WarningSecurityInfo),
    #[asn(key = 125)]
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
    #[asn(key = 13)]
    Id_BroadcastCompletedAreaList(BroadcastCompletedAreaList),
    #[asn(key = 19)]
    Id_CriticalityDiagnostics(CriticalityDiagnostics),
    #[asn(key = 35)]
    Id_MessageIdentifier(MessageIdentifier),
    #[asn(key = 95)]
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
pub enum XnExtTLA_ItemIE_Extensions_EntryExtensionValue {
    #[asn(key = 173)]
    Id_SCTP_TLAs(SCTP_TLAs),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct XnExtTLA_ItemIE_Extensions_Entry {
    #[asn(key_field = true)]
    pub id: ProtocolExtensionID,
    pub criticality: Criticality,
    pub extension_value: XnExtTLA_ItemIE_Extensions_EntryExtensionValue,
}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct XnExtTLA_ItemIE_Extensions(Vec<XnExtTLA_ItemIE_Extensions_Entry>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct XnTNLConfigurationInfoIE_Extensions_Entry {}

#[derive(Debug, AperCodec)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct XnTNLConfigurationInfoIE_Extensions(Vec<XnTNLConfigurationInfoIE_Extensions_Entry>);