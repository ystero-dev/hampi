# ! [allow (dead_code , unreachable_patterns , non_camel_case_types)]use bitvec::vec::BitVec;
use bitvec::order::Msb0;
use asn1_codecs_derive::AperCodec;

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct AMFTNLAssociationSetupItem {
    pub amf_tnl_association_address: CPTransportLayerInformation,
    #[asn(optional_idx = 0)]
    pub ie_extensions: Option<AMFTNLAssociationSetupItemiEExtensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "32")]
pub struct AMFTNLAssociationSetupList(Vec<AMFTNLAssociationSetupItem>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 2)]
pub struct AMFTNLAssociationToAddItem {
    pub amf_tnl_association_address: CPTransportLayerInformation,
    #[asn(optional_idx = 0)]
    pub tnl_association_usage: Option<TNLAssociationUsage>,
    pub tnl_address_weight_factor: TNLAddressWeightFactor,
    #[asn(optional_idx = 1)]
    pub ie_extensions: Option<AMFTNLAssociationToAddItemiEExtensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "32")]
pub struct AMFTNLAssociationToAddList(Vec<AMFTNLAssociationToAddItem>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct AMFTNLAssociationToRemoveItem {
    pub amf_tnl_association_address: CPTransportLayerInformation,
    #[asn(optional_idx = 0)]
    pub ie_extensions: Option<AMFTNLAssociationToRemoveItemiEExtensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "32")]
pub struct AMFTNLAssociationToRemoveList(Vec<AMFTNLAssociationToRemoveItem>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 3)]
pub struct AMFTNLAssociationToUpdateItem {
    pub amf_tnl_association_address: CPTransportLayerInformation,
    #[asn(optional_idx = 0)]
    pub tnl_association_usage: Option<TNLAssociationUsage>,
    #[asn(optional_idx = 1)]
    pub tnl_address_weight_factor: Option<TNLAddressWeightFactor>,
    #[asn(optional_idx = 2)]
    pub ie_extensions: Option<AMFTNLAssociationToUpdateItemiEExtensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "32")]
pub struct AMFTNLAssociationToUpdateList(Vec<AMFTNLAssociationToUpdateItem>);

#[derive(Debug, AperCodec)]
#[asn(type = "INTEGER", lb = "0", ub = "1099511627775")]
pub struct AMFUENGAPID(u64);

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
#[asn(type = "PrintableString", sz_extensible = true, sz_lb = "1", sz_ub = "150")]
pub struct AMFName(String);

#[derive(Debug, AperCodec)]
#[asn(type = "UTF8String", sz_extensible = true, sz_lb = "1", sz_ub = "150")]
pub struct AMFNameUTF8String(String);

#[derive(Debug, AperCodec)]
#[asn(type = "VisibleString", sz_extensible = true, sz_lb = "1", sz_ub = "150")]
pub struct AMFNameVisibleString(String);

#[derive(Debug, AperCodec)]
#[asn(type = "CHOICE", lb = "0", ub = "2", extensible = false)]
pub enum AMFPagingTarget {
    #[asn(key = 0, extended = false)]
    globalRANNodeID(GlobalRANNodeID),
    #[asn(key = 1, extended = false)]
    tAI(TAI),
    #[asn(key = 2, extended = false)]
    choiceExtensions(AMFPagingTargetchoiceExtensions),
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
    pub ie_extensions: Option<AdditionalDLUPTNLInformationForHOItemiEExtensions>,
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
    pub pre_emption_capability: PreemptionCapability,
    pub pre_emption_vulnerability: PreemptionVulnerability,
    #[asn(optional_idx = 0)]
    pub ie_extensions: Option<AllocationAndRetentionPriorityiEExtensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "256")]
pub struct AllowedCAGListperPLMN(Vec<CAGID>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct AllowedPNINPNItem {
    pub plmn_identity: PLMNIdentity,
    pub pni_npn_restricted: ENUMERATED2,
    pub allowed_cag_list_per_plmn: AllowedCAGListperPLMN,
    #[asn(optional_idx = 0)]
    pub ie_extensions: Option<AllowedPNINPNItemiEExtensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "16")]
pub struct AllowedPNINPNList(Vec<AllowedPNINPNItem>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "8")]
pub struct AllowedNSSAI(Vec<AllowedNSSAIItem>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct AllowedNSSAIItem {
    pub s_nssai: SNSSAI,
    #[asn(optional_idx = 0)]
    pub ie_extensions: Option<AllowedNSSAIItemiEExtensions>,
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
    pub ie_extensions: Option<AlternativeQoSParaSetItemiEExtensions>,
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
    pub ie_extensions: Option<AreaOfInterestiEExtensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct AreaOfInterestCellItem {
    pub ngran_cgi: NGRANCGI,
    #[asn(optional_idx = 0)]
    pub ie_extensions: Option<AreaOfInterestCellItemiEExtensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "256")]
pub struct AreaOfInterestCellList(Vec<AreaOfInterestCellItem>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct AreaOfInterestItem {
    pub area_of_interest: AreaOfInterest,
    pub location_reporting_reference_id: LocationReportingReferenceID,
    #[asn(optional_idx = 0)]
    pub ie_extensions: Option<AreaOfInterestItemiEExtensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "64")]
pub struct AreaOfInterestList(Vec<AreaOfInterestItem>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct AreaOfInterestRANNodeItem {
    pub global_ran_node_id: GlobalRANNodeID,
    #[asn(optional_idx = 0)]
    pub ie_extensions: Option<AreaOfInterestRANNodeItemiEExtensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "64")]
pub struct AreaOfInterestRANNodeList(Vec<AreaOfInterestRANNodeItem>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct AreaOfInterestTAIItem {
    pub tai: TAI,
    #[asn(optional_idx = 0)]
    pub ie_extensions: Option<AreaOfInterestTAIItemiEExtensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "16")]
pub struct AreaOfInterestTAIList(Vec<AreaOfInterestTAIItem>);

#[derive(Debug, AperCodec)]
#[asn(type = "CHOICE", lb = "0", ub = "4", extensible = false)]
pub enum AreaScopeOfMDTEUTRA {
    #[asn(key = 0, extended = false)]
    cellBased(CellBasedMDTEUTRA),
    #[asn(key = 1, extended = false)]
    tABased(TABasedMDT),
    #[asn(key = 2, extended = false)]
    pLMNWide(NULL3),
    #[asn(key = 3, extended = false)]
    tAIBased(TAIBasedMDT),
    #[asn(key = 4, extended = false)]
    choiceExtensions(AreaScopeOfMDTEUTRAchoiceExtensions),
}

#[derive(Debug, AperCodec)]
#[asn(type = "CHOICE", lb = "0", ub = "4", extensible = false)]
pub enum AreaScopeOfMDTNR {
    #[asn(key = 0, extended = false)]
    cellBased(CellBasedMDTNR),
    #[asn(key = 1, extended = false)]
    tABased(TABasedMDT),
    #[asn(key = 2, extended = false)]
    pLMNWide(NULL4),
    #[asn(key = 3, extended = false)]
    tAIBased(TAIBasedMDT),
    #[asn(key = 4, extended = false)]
    choiceExtensions(AreaScopeOfMDTNRchoiceExtensions),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 2)]
pub struct AreaScopeOfNeighCellsItem {
    pub nr_frequency_info: NRFrequencyInfo,
    #[asn(optional_idx = 0)]
    pub pci_list_for_mdt: Option<PCIListForMDT>,
    #[asn(optional_idx = 1)]
    pub ie_extensions: Option<AreaScopeOfNeighCellsItemiEExtensions>,
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
    pub ie_extensions: Option<AssistanceDataForPagingiEExtensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct AssistanceDataForRecommendedCells {
    pub recommended_cells_for_paging: RecommendedCellsForPaging,
    #[asn(optional_idx = 0)]
    pub ie_extensions: Option<AssistanceDataForRecommendedCellsiEExtensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 2)]
pub struct AssociatedQosFlowItem {
    pub qos_flow_identifier: QosFlowIdentifier,
    #[asn(optional_idx = 0)]
    pub qos_flow_mapping_indication: Option<ENUMERATED5>,
    #[asn(optional_idx = 1)]
    pub ie_extensions: Option<AssociatedQosFlowItemiEExtensions>,
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
    pub ie_extensions: Option<BluetoothMeasConfigNameItemiEExtensions>,
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
    pub bt_rssi: Option<ENUMERATED6>,
    #[asn(optional_idx = 2)]
    pub ie_extensions: Option<BluetoothMeasurementConfigurationiEExtensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "OCTET-STRING", sz_extensible = false, sz_lb = "1", sz_ub = "248")]
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
    choiceExtensions(BroadcastCancelledAreaListchoiceExtensions),
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
    choiceExtensions(BroadcastCompletedAreaListchoiceExtensions),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct BroadcastPLMNItem {
    pub plmn_identity: PLMNIdentity,
    pub tai_slice_support_list: SliceSupportList,
    #[asn(optional_idx = 0)]
    pub ie_extensions: Option<BroadcastPLMNItemiEExtensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "12")]
pub struct BroadcastPLMNList(Vec<BroadcastPLMNItem>);

#[derive(Debug, AperCodec)]
#[asn(type = "OCTET-STRING")]
pub struct BurstArrivalTime(Vec<u8>);

#[derive(Debug, AperCodec)]
#[asn(type = "BITSTRING", sz_extensible = false, sz_lb = "32", sz_ub = "32")]
pub struct CAGID(BitVec<Msb0, u8>);

#[derive(Debug, AperCodec)]
#[asn(type = "ENUMERATED", extensible = true, lb = "0", ub = "0")]
pub struct CEmodeBSupportIndicator(u8);
impl CEmodeBSupportIndicator {
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
    pub ie_extensions: Option<CNAssistedRANTuningiEExtensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "15")]
pub struct CNTypeRestrictionsForEquivalent(Vec<CNTypeRestrictionsForEquivalentItem>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct CNTypeRestrictionsForEquivalentItem {
    pub plmn_identity: PLMNIdentity,
    pub cn_type: ENUMERATED7,
    #[asn(optional_idx = 0)]
    pub ie_extensions: Option<CNTypeRestrictionsForEquivalentItemiEExtensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "ENUMERATED", extensible = true, lb = "0", ub = "0")]
pub struct CNTypeRestrictionsForServing(u8);
impl CNTypeRestrictionsForServing {
    const EPC_FORBIDDEN: u8 = 0u8;
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct COUNTValueForPDCPSN12 {
    pub pdcp_sn12: INTEGER8,
    pub hfn_pdcp_sn12: INTEGER9,
    #[asn(optional_idx = 0)]
    pub ie_extensions: Option<COUNTValueForPDCPSN12iEExtensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct COUNTValueForPDCPSN18 {
    pub pdcp_sn18: INTEGER10,
    pub hfn_pdcp_sn18: INTEGER11,
    #[asn(optional_idx = 0)]
    pub ie_extensions: Option<COUNTValueForPDCPSN18iEExtensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "CHOICE", lb = "0", ub = "1", extensible = false)]
pub enum CPTransportLayerInformation {
    #[asn(key = 0, extended = false)]
    endpointIPAddress(TransportLayerAddress),
    #[asn(key = 1, extended = false)]
    choiceExtensions(CPTransportLayerInformationchoiceExtensions),
}

#[derive(Debug, AperCodec)]
#[asn(type = "ENUMERATED", extensible = true, lb = "0", ub = "0")]
pub struct CancelAllWarningMessages(u8);
impl CancelAllWarningMessages {
    const TRUE: u8 = 0u8;
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "65535")]
pub struct CancelledCellsInEAIEUTRA(Vec<CancelledCellsInEAIEUTRAItem>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct CancelledCellsInEAIEUTRAItem {
    pub eutra_cgi: EUTRACGI,
    pub number_of_broadcasts: NumberOfBroadcasts,
    #[asn(optional_idx = 0)]
    pub ie_extensions: Option<CancelledCellsInEAIEUTRAItemiEExtensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "65535")]
pub struct CancelledCellsInEAINR(Vec<CancelledCellsInEAINRItem>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct CancelledCellsInEAINRItem {
    pub nr_cgi: NRCGI,
    pub number_of_broadcasts: NumberOfBroadcasts,
    #[asn(optional_idx = 0)]
    pub ie_extensions: Option<CancelledCellsInEAINRItemiEExtensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "65535")]
pub struct CancelledCellsInTAIEUTRA(Vec<CancelledCellsInTAIEUTRAItem>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct CancelledCellsInTAIEUTRAItem {
    pub eutra_cgi: EUTRACGI,
    pub number_of_broadcasts: NumberOfBroadcasts,
    #[asn(optional_idx = 0)]
    pub ie_extensions: Option<CancelledCellsInTAIEUTRAItemiEExtensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "65535")]
pub struct CancelledCellsInTAINR(Vec<CancelledCellsInTAINRItem>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct CancelledCellsInTAINRItem {
    pub nr_cgi: NRCGI,
    pub number_of_broadcasts: NumberOfBroadcasts,
    #[asn(optional_idx = 0)]
    pub ie_extensions: Option<CancelledCellsInTAINRItemiEExtensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "CHOICE", lb = "0", ub = "2", extensible = false)]
pub enum CandidateCell {
    #[asn(key = 0, extended = false)]
    candidateCGI(CandidateCellID),
    #[asn(key = 1, extended = false)]
    candidatePCI(CandidatePCI),
    #[asn(key = 2, extended = false)]
    choiceExtensions(CandidateCellchoiceExtensions),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct CandidateCellID {
    pub candidate_cell_id: NRCGI,
    #[asn(optional_idx = 0)]
    pub ie_extensions: Option<CandidateCellIDiEExtensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct CandidateCellItem {
    pub candidate_cell: CandidateCell,
    #[asn(optional_idx = 0)]
    pub ie_extensions: Option<CandidateCellItemiEExtensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "32")]
pub struct CandidateCellList(Vec<CandidateCellItem>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct CandidatePCI {
    pub candidate_pci: INTEGER12,
    pub candidate_nrarfcn: INTEGER13,
    #[asn(optional_idx = 0)]
    pub ie_extensions: Option<CandidatePCIiEExtensions>,
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
    choiceExtensions(CausechoiceExtensions),
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
pub struct CellCAGInformation {
    pub ngran_cgi: NGRANCGI,
    pub cell_cag_list: CellCAGList,
    #[asn(optional_idx = 0)]
    pub ie_extensions: Option<CellCAGInformationiEExtensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct CellBasedMDTEUTRA {
    pub cell_id_listfor_mdt: CellIdListforMDTEUTRA,
    #[asn(optional_idx = 0)]
    pub ie_extensions: Option<CellBasedMDTEUTRAiEExtensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct CellBasedMDTNR {
    pub cell_id_listfor_mdt: CellIdListforMDTNR,
    #[asn(optional_idx = 0)]
    pub ie_extensions: Option<CellBasedMDTNRiEExtensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "64")]
pub struct CellCAGList(Vec<CAGID>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "65535")]
pub struct CellIDBroadcastEUTRA(Vec<CellIDBroadcastEUTRAItem>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct CellIDBroadcastEUTRAItem {
    pub eutra_cgi: EUTRACGI,
    #[asn(optional_idx = 0)]
    pub ie_extensions: Option<CellIDBroadcastEUTRAItemiEExtensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "65535")]
pub struct CellIDBroadcastNR(Vec<CellIDBroadcastNRItem>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct CellIDBroadcastNRItem {
    pub nr_cgi: NRCGI,
    #[asn(optional_idx = 0)]
    pub ie_extensions: Option<CellIDBroadcastNRItemiEExtensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "65535")]
pub struct CellIDCancelledEUTRA(Vec<CellIDCancelledEUTRAItem>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct CellIDCancelledEUTRAItem {
    pub eutra_cgi: EUTRACGI,
    pub number_of_broadcasts: NumberOfBroadcasts,
    #[asn(optional_idx = 0)]
    pub ie_extensions: Option<CellIDCancelledEUTRAItemiEExtensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "65535")]
pub struct CellIDCancelledNR(Vec<CellIDCancelledNRItem>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct CellIDCancelledNRItem {
    pub nr_cgi: NRCGI,
    pub number_of_broadcasts: NumberOfBroadcasts,
    #[asn(optional_idx = 0)]
    pub ie_extensions: Option<CellIDCancelledNRItemiEExtensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "CHOICE", lb = "0", ub = "2", extensible = false)]
pub enum CellIDListForRestart {
    #[asn(key = 0, extended = false)]
    eUTRACGIListforRestart(EUTRACGIList),
    #[asn(key = 1, extended = false)]
    nRCGIListforRestart(NRCGIList),
    #[asn(key = 2, extended = false)]
    choiceExtensions(CellIDListForRestartchoiceExtensions),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "32")]
pub struct CellIdListforMDTEUTRA(Vec<EUTRACGI>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "32")]
pub struct CellIdListforMDTNR(Vec<NRCGI>);

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
    pub ie_extensions: Option<CellTypeiEExtensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "OCTET-STRING")]
pub struct CommonNetworkInstance(Vec<u8>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "65535")]
pub struct CompletedCellsInEAIEUTRA(Vec<CompletedCellsInEAIEUTRAItem>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct CompletedCellsInEAIEUTRAItem {
    pub eutra_cgi: EUTRACGI,
    #[asn(optional_idx = 0)]
    pub ie_extensions: Option<CompletedCellsInEAIEUTRAItemiEExtensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "65535")]
pub struct CompletedCellsInEAINR(Vec<CompletedCellsInEAINRItem>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct CompletedCellsInEAINRItem {
    pub nr_cgi: NRCGI,
    #[asn(optional_idx = 0)]
    pub ie_extensions: Option<CompletedCellsInEAINRItemiEExtensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "65535")]
pub struct CompletedCellsInTAIEUTRA(Vec<CompletedCellsInTAIEUTRAItem>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct CompletedCellsInTAIEUTRAItem {
    pub eutra_cgi: EUTRACGI,
    #[asn(optional_idx = 0)]
    pub ie_extensions: Option<CompletedCellsInTAIEUTRAItemiEExtensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "65535")]
pub struct CompletedCellsInTAINR(Vec<CompletedCellsInTAINRItem>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct CompletedCellsInTAINRItem {
    pub nr_cgi: NRCGI,
    #[asn(optional_idx = 0)]
    pub ie_extensions: Option<CompletedCellsInTAINRItemiEExtensions>,
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
#[asn(type = "OCTET-STRING", sz_extensible = false, sz_lb = "128", sz_ub = "128")]
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
    pub ie_extensions: Option<CoreNetworkAssistanceInformationForInactiveiEExtensions>,
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
    pub i_es_criticality_diagnostics: Option<CriticalityDiagnosticsIEList>,
    #[asn(optional_idx = 4)]
    pub ie_extensions: Option<CriticalityDiagnosticsiEExtensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct CriticalityDiagnosticsIEItem {
    pub ie_criticality: Criticality,
    pub ie_id: ProtocolIEID,
    pub type_of_error: TypeOfError,
    #[asn(optional_idx = 0)]
    pub ie_extensions: Option<CriticalityDiagnosticsIEItemiEExtensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "256")]
pub struct CriticalityDiagnosticsIEList(Vec<CriticalityDiagnosticsIEItem>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct DAPSRequestInfo {
    pub daps_indicator: ENUMERATED14,
    #[asn(optional_idx = 0)]
    pub ie_extensions: Option<DAPSRequestInfoiEExtensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct DAPSResponseInfo {
    pub dapsresponseindicator: ENUMERATED15,
    #[asn(optional_idx = 0)]
    pub ie_extensions: Option<DAPSResponseInfoiEExtensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct DAPSResponseInfoItem {
    pub drb_id: DRBID,
    pub daps_response_info: DAPSResponseInfo,
    #[asn(optional_idx = 0)]
    pub ie_extension: Option<DAPSResponseInfoItemiEExtension>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "32")]
pub struct DAPSResponseInfoList(Vec<DAPSResponseInfoItem>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct DLCPSecurityInformation {
    pub dl_nas_mac: DLNASMAC,
    #[asn(optional_idx = 0)]
    pub ie_extensions: Option<DLCPSecurityInformationiEExtensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "BITSTRING", sz_extensible = false, sz_lb = "16", sz_ub = "16")]
pub struct DLNASMAC(BitVec<Msb0, u8>);

#[derive(Debug, AperCodec)]
#[asn(type = "ENUMERATED", extensible = true, lb = "0", ub = "0")]
pub struct DLNGUTNLInformationReused(u8);
impl DLNGUTNLInformationReused {
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
pub struct DRBID(u8);

#[derive(Debug, AperCodec)]
#[asn(type = "CHOICE", lb = "0", ub = "2", extensible = false)]
pub enum DRBStatusDL {
    #[asn(key = 0, extended = false)]
    dRBStatusDL12(DRBStatusDL12),
    #[asn(key = 1, extended = false)]
    dRBStatusDL18(DRBStatusDL18),
    #[asn(key = 2, extended = false)]
    choiceExtensions(DRBStatusDLchoiceExtensions),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct DRBStatusDL12 {
    pub dl_count_value: COUNTValueForPDCPSN12,
    #[asn(optional_idx = 0)]
    pub ie_extension: Option<DRBStatusDL12iEExtension>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct DRBStatusDL18 {
    pub dl_count_value: COUNTValueForPDCPSN18,
    #[asn(optional_idx = 0)]
    pub ie_extension: Option<DRBStatusDL18iEExtension>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "CHOICE", lb = "0", ub = "2", extensible = false)]
pub enum DRBStatusUL {
    #[asn(key = 0, extended = false)]
    dRBStatusUL12(DRBStatusUL12),
    #[asn(key = 1, extended = false)]
    dRBStatusUL18(DRBStatusUL18),
    #[asn(key = 2, extended = false)]
    choiceExtensions(DRBStatusULchoiceExtensions),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 2)]
pub struct DRBStatusUL12 {
    pub ul_count_value: COUNTValueForPDCPSN12,
    #[asn(optional_idx = 0)]
    pub receive_status_of_ul_pdcp_sd_us: Option<BITSTRING16>,
    #[asn(optional_idx = 1)]
    pub ie_extension: Option<DRBStatusUL12iEExtension>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 2)]
pub struct DRBStatusUL18 {
    pub ul_count_value: COUNTValueForPDCPSN18,
    #[asn(optional_idx = 0)]
    pub receive_status_of_ul_pdcp_sd_us: Option<BITSTRING17>,
    #[asn(optional_idx = 1)]
    pub ie_extension: Option<DRBStatusUL18iEExtension>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct DRBsSubjectToEarlyStatusTransferItem {
    pub drb_id: DRBID,
    pub first_dlcount: DRBStatusDL,
    #[asn(optional_idx = 0)]
    pub ie_extension: Option<DRBsSubjectToEarlyStatusTransferItemiEExtension>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "32")]
pub struct DRBsSubjectToEarlyStatusTransferList(Vec<DRBsSubjectToEarlyStatusTransferItem>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct DRBsSubjectToStatusTransferItem {
    pub drb_id: DRBID,
    pub drb_status_ul: DRBStatusUL,
    pub drb_status_dl: DRBStatusDL,
    #[asn(optional_idx = 0)]
    pub ie_extension: Option<DRBsSubjectToStatusTransferItemiEExtension>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "32")]
pub struct DRBsSubjectToStatusTransferList(Vec<DRBsSubjectToStatusTransferItem>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct DRBsToQosFlowsMappingItem {
    pub drb_id: DRBID,
    pub associated_qos_flow_list: AssociatedQosFlowList,
    #[asn(optional_idx = 0)]
    pub ie_extensions: Option<DRBsToQosFlowsMappingItemiEExtensions>,
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
    pub drb_id: DRBID,
    #[asn(optional_idx = 0)]
    pub dl_forwarding_up_tnl_information: Option<UPTransportLayerInformation>,
    #[asn(optional_idx = 1)]
    pub ul_forwarding_up_tnl_information: Option<UPTransportLayerInformation>,
    #[asn(optional_idx = 2)]
    pub ie_extensions: Option<DataForwardingResponseDRBItemiEExtensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "32")]
pub struct DataForwardingResponseDRBList(Vec<DataForwardingResponseDRBItem>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "256")]
pub struct DataForwardingResponseERABList(Vec<DataForwardingResponseERABListItem>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct DataForwardingResponseERABListItem {
    pub e_rab_id: ERABID,
    pub dl_forwarding_up_tnl_information: UPTransportLayerInformation,
    #[asn(optional_idx = 0)]
    pub ie_extensions: Option<DataForwardingResponseERABListItemiEExtensions>,
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
    pub ie_extensions: Option<Dynamic5QIDescriptoriEExtensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "INTEGER", lb = "0", ub = "15", extensible = true)]
pub struct ERABID(u8);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 2)]
pub struct ERABInformationItem {
    pub e_rab_id: ERABID,
    #[asn(optional_idx = 0)]
    pub dl_forwarding: Option<DLForwarding>,
    #[asn(optional_idx = 1)]
    pub ie_extensions: Option<ERABInformationItemiEExtensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "256")]
pub struct ERABInformationList(Vec<ERABInformationItem>);

#[derive(Debug, AperCodec)]
#[asn(type = "ENUMERATED", extensible = true, lb = "0", ub = "0")]
pub struct EDTSession(u8);
impl EDTSession {
    const TRUE: u8 = 0u8;
}

#[derive(Debug, AperCodec)]
#[asn(type = "OCTET-STRING")]
pub struct ENDCSONConfigurationTransfer(Vec<u8>);

#[derive(Debug, AperCodec)]
#[asn(type = "CHOICE", lb = "0", ub = "4", extensible = false)]
pub enum ENBID {
    #[asn(key = 0, extended = false)]
    macroENBID(BITSTRING18),
    #[asn(key = 1, extended = false)]
    homeENBID(BITSTRING19),
    #[asn(key = 2, extended = false)]
    shortmacroENBID(BITSTRING20),
    #[asn(key = 3, extended = false)]
    longmacroENBID(BITSTRING21),
    #[asn(key = 4, extended = false)]
    choiceExtensions(ENBIDchoiceExtensions),
}

#[derive(Debug, AperCodec)]
#[asn(type = "OCTET-STRING", sz_extensible = false, sz_lb = "2", sz_ub = "2")]
pub struct EPSTAC(Vec<u8>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct EPSTAI {
    pub plmn_identity: PLMNIdentity,
    pub eps_tac: EPSTAC,
    #[asn(optional_idx = 0)]
    pub ie_extensions: Option<EPSTAIiEExtensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct EUTRACGI {
    pub plmn_identity: PLMNIdentity,
    pub eutra_cell_identity: EUTRACellIdentity,
    #[asn(optional_idx = 0)]
    pub ie_extensions: Option<EUTRACGIiEExtensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "256")]
pub struct EUTRACGIList(Vec<EUTRACGI>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "65535")]
pub struct EUTRACGIListForWarning(Vec<EUTRACGI>);

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
pub struct EarlyStatusTransferTransparentContainer {
    pub procedure_stage: ProcedureStageChoice,
    #[asn(optional_idx = 0)]
    pub ie_extensions: Option<EarlyStatusTransferTransparentContaineriEExtensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "OCTET-STRING", sz_extensible = false, sz_lb = "3", sz_ub = "3")]
pub struct EmergencyAreaID(Vec<u8>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "65535")]
pub struct EmergencyAreaIDBroadcastEUTRA(Vec<EmergencyAreaIDBroadcastEUTRAItem>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct EmergencyAreaIDBroadcastEUTRAItem {
    pub emergency_area_id: EmergencyAreaID,
    pub completed_cells_in_eai_eutra: CompletedCellsInEAIEUTRA,
    #[asn(optional_idx = 0)]
    pub ie_extensions: Option<EmergencyAreaIDBroadcastEUTRAItemiEExtensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "65535")]
pub struct EmergencyAreaIDBroadcastNR(Vec<EmergencyAreaIDBroadcastNRItem>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct EmergencyAreaIDBroadcastNRItem {
    pub emergency_area_id: EmergencyAreaID,
    pub completed_cells_in_eai_nr: CompletedCellsInEAINR,
    #[asn(optional_idx = 0)]
    pub ie_extensions: Option<EmergencyAreaIDBroadcastNRItemiEExtensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "65535")]
pub struct EmergencyAreaIDCancelledEUTRA(Vec<EmergencyAreaIDCancelledEUTRAItem>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct EmergencyAreaIDCancelledEUTRAItem {
    pub emergency_area_id: EmergencyAreaID,
    pub cancelled_cells_in_eai_eutra: CancelledCellsInEAIEUTRA,
    #[asn(optional_idx = 0)]
    pub ie_extensions: Option<EmergencyAreaIDCancelledEUTRAItemiEExtensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "65535")]
pub struct EmergencyAreaIDCancelledNR(Vec<EmergencyAreaIDCancelledNRItem>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct EmergencyAreaIDCancelledNRItem {
    pub emergency_area_id: EmergencyAreaID,
    pub cancelled_cells_in_eai_nr: CancelledCellsInEAINR,
    #[asn(optional_idx = 0)]
    pub ie_extensions: Option<EmergencyAreaIDCancelledNRItemiEExtensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "65535")]
pub struct EmergencyAreaIDList(Vec<EmergencyAreaID>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "256")]
pub struct EmergencyAreaIDListForRestart(Vec<EmergencyAreaID>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 2)]
pub struct EmergencyFallbackIndicator {
    pub emergency_fallback_request_indicator: EmergencyFallbackRequestIndicator,
    #[asn(optional_idx = 0)]
    pub emergency_service_target_cn: Option<EmergencyServiceTargetCN>,
    #[asn(optional_idx = 1)]
    pub ie_extensions: Option<EmergencyFallbackIndicatoriEExtensions>,
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
    pub ie_extensions: Option<EndpointIPAddressAndPortiEExtensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "ENUMERATED", extensible = true, lb = "0", ub = "0")]
pub struct EnhancedCoverageRestriction(u8);
impl EnhancedCoverageRestriction {
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
    pub ie_extensions: Option<EventL1LoggedMDTConfigiEExtensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "CHOICE", lb = "0", ub = "2", extensible = false)]
pub enum EventTrigger {
    #[asn(key = 0, extended = false)]
    outOfCoverage(ENUMERATED22),
    #[asn(key = 1, extended = false)]
    eventL1LoggedMDTConfig(EventL1LoggedMDTConfig),
    #[asn(key = 2, extended = false)]
    choiceExtensions(EventTriggerchoiceExtensions),
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
    pub source_of_ue_activity_behaviour_information:
        Option<SourceOfUEActivityBehaviourInformation>,
    #[asn(optional_idx = 3)]
    pub ie_extensions: Option<ExpectedUEActivityBehaviouriEExtensions>,
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
    pub ie_extensions: Option<ExpectedUEBehaviouriEExtensions>,
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
    pub ngran_cgi: NGRANCGI,
    #[asn(optional_idx = 0)]
    pub time_stayed_in_cell: Option<INTEGER23>,
    #[asn(optional_idx = 1)]
    pub ie_extensions: Option<ExpectedUEMovingTrajectoryItemiEExtensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 3)]
pub struct ExtendedAMFName {
    #[asn(optional_idx = 0)]
    pub amf_name_visible_string: Option<AMFNameVisibleString>,
    #[asn(optional_idx = 1)]
    pub amf_name_utf8_string: Option<AMFNameUTF8String>,
    #[asn(optional_idx = 2)]
    pub ie_extensions: Option<ExtendedAMFNameiEExtensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "INTEGER", lb = "0", ub = "255")]
pub struct ExtendedConnectedTime(u8);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 3)]
pub struct ExtendedRANNodeName {
    #[asn(optional_idx = 0)]
    pub ran_node_name_visible_string: Option<RANNodeNameVisibleString>,
    #[asn(optional_idx = 1)]
    pub ran_node_name_utf8_string: Option<RANNodeNameUTF8String>,
    #[asn(optional_idx = 2)]
    pub ie_extensions: Option<ExtendedRANNodeNameiEExtensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "INTEGER", lb = "1", ub = "65535", extensible = true)]
pub struct ExtendedPacketDelayBudget(u16);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct ExtendedRATRestrictionInformation {
    pub primary_rat_restriction: BITSTRING24,
    pub secondary_rat_restriction: BITSTRING25,
    #[asn(optional_idx = 0)]
    pub ie_extensions: Option<ExtendedRATRestrictionInformationiEExtensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "INTEGER", lb = "4096", ub = "65535")]
pub struct ExtendedRNCID(u16);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "65535")]
pub struct ExtendedSliceSupportList(Vec<SliceSupportItem>);

#[derive(Debug, AperCodec)]
#[asn(type = "BITSTRING", sz_extensible = false, sz_lb = "16", sz_ub = "16")]
pub struct ExtendedUEIdentityIndexValue(BitVec<Msb0, u8>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct FailureIndication {
    pub uerlf_report_container: UERLFReportContainer,
    #[asn(optional_idx = 0)]
    pub ie_extensions: Option<FailureIndicationiEExtensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct FirstDLCount {
    pub dr_bs_subject_to_early_status_transfer: DRBsSubjectToEarlyStatusTransferList,
    #[asn(optional_idx = 0)]
    pub ie_extension: Option<FirstDLCountiEExtension>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct FiveGSTMSI {
    pub amf_set_id: AMFSetID,
    pub amf_pointer: AMFPointer,
    pub five_g_tmsi: FiveGTMSI,
    #[asn(optional_idx = 0)]
    pub ie_extensions: Option<FiveGSTMSIiEExtensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "OCTET-STRING", sz_extensible = false, sz_lb = "4", sz_ub = "4")]
pub struct FiveGTMSI(Vec<u8>);

#[derive(Debug, AperCodec)]
#[asn(type = "INTEGER", lb = "0", ub = "255", extensible = true)]
pub struct FiveQI(u8);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "16")]
pub struct ForbiddenAreaInformation(Vec<ForbiddenAreaInformationItem>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct ForbiddenAreaInformationItem {
    pub plmn_identity: PLMNIdentity,
    pub forbidden_ta_cs: ForbiddenTACs,
    #[asn(optional_idx = 0)]
    pub ie_extensions: Option<ForbiddenAreaInformationItemiEExtensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "4096")]
pub struct ForbiddenTACs(Vec<TAC>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false, optional_fields = 1)]
pub struct FromEUTRANtoNGRAN {
    pub sourcee_nbid: IntersystemSONeNBID,
    pub target_ngra_nnode_id: IntersystemSONNGRANnodeID,
    #[asn(optional_idx = 0)]
    pub ie_extensions: Option<FromEUTRANtoNGRANiEExtensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false, optional_fields = 1)]
pub struct FromNGRANtoEUTRAN {
    pub source_ngra_nnode_id: IntersystemSONNGRANnodeID,
    pub targete_nbid: IntersystemSONeNBID,
    #[asn(optional_idx = 0)]
    pub ie_extensions: Option<FromNGRANtoEUTRANiEExtensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 4)]
pub struct GBRQosInformation {
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
    pub ie_extensions: Option<GBRQosInformationiEExtensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "CHOICE", lb = "0", ub = "1", extensible = false)]
pub enum GNBID {
    #[asn(key = 0, extended = false)]
    gNBID(BITSTRING26),
    #[asn(key = 1, extended = false)]
    choiceExtensions(GNBIDchoiceExtensions),
}

#[derive(Debug, AperCodec)]
#[asn(type = "BITSTRING", sz_extensible = false, sz_lb = "22", sz_ub = "22")]
pub struct GNBSetID(BitVec<Msb0, u8>);

#[derive(Debug, AperCodec)]
#[asn(type = "OCTET-STRING", sz_extensible = false, sz_lb = "4", sz_ub = "4")]
pub struct GTPTEID(Vec<u8>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct GTPTunnel {
    pub transport_layer_address: TransportLayerAddress,
    pub gtp_teid: GTPTEID,
    #[asn(optional_idx = 0)]
    pub ie_extensions: Option<GTPTunneliEExtensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct GUAMI {
    pub plmn_identity: PLMNIdentity,
    pub amf_region_id: AMFRegionID,
    pub amf_set_id: AMFSetID,
    pub amf_pointer: AMFPointer,
    #[asn(optional_idx = 0)]
    pub ie_extensions: Option<GUAMIiEExtensions>,
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
pub struct GlobalCableID(Vec<u8>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct GlobalENBID {
    pub plm_nidentity: PLMNIdentity,
    pub enb_id: ENBID,
    #[asn(optional_idx = 0)]
    pub ie_extensions: Option<GlobalENBIDiEExtensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct GlobalGNBID {
    pub plmn_identity: PLMNIdentity,
    pub gnb_id: GNBID,
    #[asn(optional_idx = 0)]
    pub ie_extensions: Option<GlobalGNBIDiEExtensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 2)]
pub struct GlobalLineID {
    pub global_line_identity: GlobalLineIdentity,
    #[asn(optional_idx = 0)]
    pub line_type: Option<LineType>,
    #[asn(optional_idx = 1)]
    pub ie_extensions: Option<GlobalLineIDiEExtensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "OCTET-STRING")]
pub struct GlobalLineIdentity(Vec<u8>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct GlobalN3IWFID {
    pub plmn_identity: PLMNIdentity,
    pub n3iwf_id: N3IWFID,
    #[asn(optional_idx = 0)]
    pub ie_extensions: Option<GlobalN3IWFIDiEExtensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct GlobalNgENBID {
    pub plmn_identity: PLMNIdentity,
    pub ng_enb_id: NgENBID,
    #[asn(optional_idx = 0)]
    pub ie_extensions: Option<GlobalNgENBIDiEExtensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "CHOICE", lb = "0", ub = "3", extensible = false)]
pub enum GlobalRANNodeID {
    #[asn(key = 0, extended = false)]
    globalGNBID(GlobalGNBID),
    #[asn(key = 1, extended = false)]
    globalNgENBID(GlobalNgENBID),
    #[asn(key = 2, extended = false)]
    globalN3IWFID(GlobalN3IWFID),
    #[asn(key = 3, extended = false)]
    choiceExtensions(GlobalRANNodeIDchoiceExtensions),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct GlobalTNGFID {
    pub plmn_identity: PLMNIdentity,
    pub tngf_id: TNGFID,
    #[asn(optional_idx = 0)]
    pub ie_extensions: Option<GlobalTNGFIDiEExtensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct GlobalTWIFID {
    pub plmn_identity: PLMNIdentity,
    pub twif_id: TWIFID,
    #[asn(optional_idx = 0)]
    pub ie_extensions: Option<GlobalTWIFIDiEExtensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct GlobalWAGFID {
    pub plmn_identity: PLMNIdentity,
    pub w_agf_id: WAGFID,
    #[asn(optional_idx = 0)]
    pub ie_extensions: Option<GlobalWAGFIDiEExtensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "OCTET-STRING")]
pub struct HFCNodeID(Vec<u8>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 6)]
pub struct HOReport {
    pub handover_report_type: ENUMERATED27,
    pub handover_cause: Cause,
    pub sourcecell_cgi: NGRANCGI,
    pub targetcell_cgi: NGRANCGI,
    #[asn(optional_idx = 0)]
    pub reestablishmentcell_cgi: Option<NGRANCGI>,
    #[asn(optional_idx = 1)]
    pub sourcecell_c_rnti: Option<BITSTRING28>,
    #[asn(optional_idx = 2)]
    pub targetcellin_e_utran: Option<EUTRACGI>,
    #[asn(optional_idx = 3)]
    pub mobility_information: Option<MobilityInformation>,
    #[asn(optional_idx = 4)]
    pub uerlf_report_container: Option<UERLFReportContainer>,
    #[asn(optional_idx = 5)]
    pub ie_extensions: Option<HOReportiEExtensions>,
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
    pub dl_forwarding_up_tnl_information: Option<UPTransportLayerInformation>,
    #[asn(optional_idx = 1)]
    pub qos_flow_to_be_forwarded_list: Option<QosFlowToBeForwardedList>,
    #[asn(optional_idx = 2)]
    pub data_forwarding_response_drb_list: Option<DataForwardingResponseDRBList>,
    #[asn(optional_idx = 3)]
    pub ie_extensions: Option<HandoverCommandTransferiEExtensions>,
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
    pub ie_extensions: Option<HandoverPreparationUnsuccessfulTransferiEExtensions>,
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
    pub ie_extensions: Option<HandoverRequestAcknowledgeTransferiEExtensions>,
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
    pub ie_extensions: Option<HandoverRequiredTransferiEExtensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 2)]
pub struct HandoverResourceAllocationUnsuccessfulTransfer {
    pub cause: Cause,
    #[asn(optional_idx = 0)]
    pub criticality_diagnostics: Option<CriticalityDiagnostics>,
    #[asn(optional_idx = 1)]
    pub ie_extensions: Option<HandoverResourceAllocationUnsuccessfulTransferiEExtensions>,
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
pub struct IABAuthorized(u8);
impl IABAuthorized {
    const AUTHORIZED: u8 = 0u8;
    const NOT_AUTHORIZED: u8 = 1u8;
}

#[derive(Debug, AperCodec)]
#[asn(type = "ENUMERATED", extensible = true, lb = "0", ub = "0")]
pub struct IABSupported(u8);
impl IABSupported {
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
    pub mdt_location_info: Option<MDTLocationInfo>,
    #[asn(optional_idx = 8)]
    pub sensor_measurement_configuration: Option<SensorMeasurementConfiguration>,
    #[asn(optional_idx = 9)]
    pub ie_extensions: Option<ImmediateMDTNriEExtensions>,
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
    pub ie_extensions: Option<InfoOnRecommendedCellsAndRANNodesForPagingiEExtensions>,
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
    pub uerlf_report_container: Option<UERLFReportContainer>,
    #[asn(optional_idx = 1)]
    pub ie_extensions: Option<InterSystemFailureIndicationiEExtensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct InterSystemHOReport {
    pub handover_report_type: InterSystemHandoverReportType,
    #[asn(optional_idx = 0)]
    pub ie_extensions: Option<InterSystemHOReportiEExtensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "CHOICE", lb = "0", ub = "2", extensible = false)]
pub enum InterSystemHandoverReportType {
    #[asn(key = 0, extended = false)]
    tooearlyIntersystemHO(TooearlyIntersystemHO),
    #[asn(key = 1, extended = false)]
    intersystemUnnecessaryHO(IntersystemUnnecessaryHO),
    #[asn(key = 2, extended = false)]
    choiceExtensions(InterSystemHandoverReportTypechoiceExtensions),
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
    pub ie_extensions: Option<IntersystemSONConfigurationTransferiEExtensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "CHOICE", lb = "0", ub = "1", extensible = false)]
pub enum IntersystemSONInformation {
    #[asn(key = 0, extended = false)]
    intersystemSONInformationReport(IntersystemSONInformationReport),
    #[asn(key = 1, extended = false)]
    choiceExtensions(IntersystemSONInformationchoiceExtensions),
}

#[derive(Debug, AperCodec)]
#[asn(type = "CHOICE", lb = "0", ub = "2", extensible = false)]
pub enum IntersystemSONInformationReport {
    #[asn(key = 0, extended = false)]
    hOReportInformation(InterSystemHOReport),
    #[asn(key = 1, extended = false)]
    failureIndicationInformation(InterSystemFailureIndication),
    #[asn(key = 2, extended = false)]
    choiceExtensions(IntersystemSONInformationReportchoiceExtensions),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct IntersystemSONNGRANnodeID {
    pub global_ran_node_id: GlobalRANNodeID,
    pub selected_tai: TAI,
    #[asn(optional_idx = 0)]
    pub ie_extensions: Option<IntersystemSONNGRANnodeIDiEExtensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "CHOICE", lb = "0", ub = "2", extensible = false)]
pub enum IntersystemSONTransferType {
    #[asn(key = 0, extended = false)]
    fromEUTRANtoNGRAN(FromEUTRANtoNGRAN),
    #[asn(key = 1, extended = false)]
    fromNGRANtoEUTRAN(FromNGRANtoEUTRAN),
    #[asn(key = 2, extended = false)]
    choiceExtensions(IntersystemSONTransferTypechoiceExtensions),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct IntersystemSONeNBID {
    pub globale_nbid: GlobalENBID,
    pub selected_epstai: EPSTAI,
    #[asn(optional_idx = 0)]
    pub ie_extensions: Option<IntersystemSONeNBIDiEExtensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct IntersystemUnnecessaryHO {
    pub sourcecell_id: NGRANCGI,
    pub targetcell_id: EUTRACGI,
    pub early_iratho: ENUMERATED29,
    pub candidate_cell_list: CandidateCellList,
    #[asn(optional_idx = 0)]
    pub ie_extensions: Option<IntersystemUnnecessaryHOiEExtensions>,
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
    pub ie_extensions: Option<LAIiEExtensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "ENUMERATED", extensible = true, lb = "0", ub = "0")]
pub struct LTEMIndication(u8);
impl LTEMIndication {
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
    pub ie_extensions: Option<LTEUESidelinkAggregateMaximumBitrateiEExtensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 3)]
pub struct LTEV2XServicesAuthorized {
    #[asn(optional_idx = 0)]
    pub vehicle_ue: Option<VehicleUE>,
    #[asn(optional_idx = 1)]
    pub pedestrian_ue: Option<PedestrianUE>,
    #[asn(optional_idx = 2)]
    pub ie_extensions: Option<LTEV2XServicesAuthorizediEExtensions>,
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
    choiceExtensions(LastVisitedCellInformationchoiceExtensions),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct LastVisitedCellItem {
    pub last_visited_cell_information: LastVisitedCellInformation,
    #[asn(optional_idx = 0)]
    pub ie_extensions: Option<LastVisitedCellItemiEExtensions>,
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
    pub global_cell_id: NGRANCGI,
    pub cell_type: CellType,
    pub time_ue_stayed_in_cell: TimeUEStayedInCell,
    #[asn(optional_idx = 0)]
    pub time_ue_stayed_in_cell_enhanced_granularity:
        Option<TimeUEStayedInCellEnhancedGranularity>,
    #[asn(optional_idx = 1)]
    pub ho_cause_value: Option<Cause>,
    #[asn(optional_idx = 2)]
    pub ie_extensions: Option<LastVisitedNGRANCellInformationiEExtensions>,
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
pub struct Linkstolog(u8);
impl Linkstolog {
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
    pub ie_extensions: Option<LocationReportingRequestTypeiEExtensions>,
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
    pub ie_extensions: Option<LoggedMDTNriEExtensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "CHOICE", lb = "0", ub = "2", extensible = false)]
pub enum LoggedMDTTrigger {
    #[asn(key = 0, extended = false)]
    periodical(NULL30),
    #[asn(key = 1, extended = false)]
    eventTrigger(EventTrigger),
    #[asn(key = 2, extended = false)]
    choiceExtensions(LoggedMDTTriggerchoiceExtensions),
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
    pub ie_extensions: Option<M1ConfigurationiEExtensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct M1PeriodicReporting {
    pub report_interval: ReportIntervalMDT,
    pub report_amount: ReportAmountMDT,
    #[asn(optional_idx = 0)]
    pub ie_extensions: Option<M1PeriodicReportingiEExtensions>,
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
    pub ie_extensions: Option<M1ThresholdEventA2iEExtensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "CHOICE", lb = "0", ub = "3", extensible = false)]
pub enum M1ThresholdType {
    #[asn(key = 0, extended = false)]
    thresholdRSRP(ThresholdRSRP),
    #[asn(key = 1, extended = false)]
    thresholdRSRQ(ThresholdRSRQ),
    #[asn(key = 2, extended = false)]
    thresholdSINR(ThresholdSINR),
    #[asn(key = 3, extended = false)]
    choiceExtensions(M1ThresholdTypechoiceExtensions),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct M4Configuration {
    pub m4period: M4period,
    pub m4_links_to_log: Linkstolog,
    #[asn(optional_idx = 0)]
    pub ie_extensions: Option<M4ConfigurationiEExtensions>,
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
    pub m5_links_to_log: Linkstolog,
    #[asn(optional_idx = 0)]
    pub ie_extensions: Option<M5ConfigurationiEExtensions>,
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
    pub m6report_interval: M6reportInterval,
    pub m6_links_to_log: Linkstolog,
    #[asn(optional_idx = 0)]
    pub ie_extensions: Option<M6ConfigurationiEExtensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "ENUMERATED", extensible = true, lb = "0", ub = "13")]
pub struct M6reportInterval(u8);
impl M6reportInterval {
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
    pub m7_links_to_log: Linkstolog,
    #[asn(optional_idx = 0)]
    pub ie_extensions: Option<M7ConfigurationiEExtensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "INTEGER", lb = "1", ub = "60", extensible = true)]
pub struct M7period(u8);

#[derive(Debug, AperCodec)]
#[asn(type = "ENUMERATED", extensible = true, lb = "0", ub = "2")]
pub struct MDTActivation(u8);
impl MDTActivation {
    const IMMEDIATE_MDT_ONLY: u8 = 0u8;
    const LOGGED_MDT_ONLY: u8 = 1u8;
    const IMMEDIATE_MDT_AND_TRACE: u8 = 2u8;
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 3)]
pub struct MDTConfiguration {
    #[asn(optional_idx = 0)]
    pub mdt_config_nr: Option<MDTConfigurationNR>,
    #[asn(optional_idx = 1)]
    pub mdt_config_eutra: Option<MDTConfigurationEUTRA>,
    #[asn(optional_idx = 2)]
    pub ie_extensions: Option<MDTConfigurationiEExtensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 2)]
pub struct MDTConfigurationEUTRA {
    pub mdt_activation: MDTActivation,
    pub area_scope_of_mdt: AreaScopeOfMDTEUTRA,
    pub mdt_mode: MDTModeEutra,
    #[asn(optional_idx = 0)]
    pub signalling_based_mdtplmn_list: Option<MDTPLMNList>,
    #[asn(optional_idx = 1)]
    pub ie_extensions: Option<MDTConfigurationEUTRAiEExtensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 2)]
pub struct MDTConfigurationNR {
    pub mdt_activation: MDTActivation,
    pub area_scope_of_mdt: AreaScopeOfMDTNR,
    pub mdt_mode_nr: MDTModeNr,
    #[asn(optional_idx = 0)]
    pub signalling_based_mdtplmn_list: Option<MDTPLMNList>,
    #[asn(optional_idx = 1)]
    pub ie_extensions: Option<MDTConfigurationNRiEExtensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct MDTLocationInfo {
    pub mdt_location_information: MDTLocationInformation,
    #[asn(optional_idx = 0)]
    pub ie_extensions: Option<MDTLocationInfoiEExtensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "BITSTRING", sz_extensible = false, sz_lb = "8", sz_ub = "8")]
pub struct MDTLocationInformation(BitVec<Msb0, u8>);

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
    choiceExtensions(MDTModeNrchoiceExtensions),
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
    thresholdRSRP(ThresholdRSRP),
    #[asn(key = 1, extended = false)]
    thresholdRSRQ(ThresholdRSRQ),
    #[asn(key = 2, extended = false)]
    choiceExtensions(MeasurementThresholdL1LoggedMDTchoiceExtensions),
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
    pub ie_extensions: Option<MobilityRestrictionListiEExtensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "CHOICE", lb = "0", ub = "1", extensible = false)]
pub enum N3IWFID {
    #[asn(key = 0, extended = false)]
    n3IWFID(BITSTRING31),
    #[asn(key = 1, extended = false)]
    choiceExtensions(N3IWFIDchoiceExtensions),
}

#[derive(Debug, AperCodec)]
#[asn(type = "OCTET-STRING")]
pub struct NASPDU(Vec<u8>);

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
pub struct NBIoTDefaultPagingDRX(u8);
impl NBIoTDefaultPagingDRX {
    const RF128: u8 = 0u8;
    const RF256: u8 = 1u8;
    const RF512: u8 = 2u8;
    const RF1024: u8 = 3u8;
}

#[derive(Debug, AperCodec)]
#[asn(type = "ENUMERATED", extensible = true, lb = "0", ub = "15")]
pub struct NBIoTPagingTimeWindow(u8);
impl NBIoTPagingTimeWindow {
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
pub struct NBIoTPagingeDRXCycle(u8);
impl NBIoTPagingeDRXCycle {
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
pub struct NBIoTPagingeDRXInfo {
    pub nb_io_t_paging_e_drx_cycle: NBIoTPagingeDRXCycle,
    #[asn(optional_idx = 0)]
    pub nb_io_t_paging_time_window: Option<NBIoTPagingTimeWindow>,
    #[asn(optional_idx = 1)]
    pub ie_extensions: Option<NBIoTPagingeDRXInfoiEExtensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "ENUMERATED", extensible = true, lb = "0", ub = "5")]
pub struct NBIoTPagingDRX(u8);
impl NBIoTPagingDRX {
    const RF32: u8 = 0u8;
    const RF64: u8 = 1u8;
    const RF128: u8 = 2u8;
    const RF256: u8 = 3u8;
    const RF512: u8 = 4u8;
    const RF1024: u8 = 5u8;
}

#[derive(Debug, AperCodec)]
#[asn(type = "INTEGER", lb = "0", ub = "255", extensible = true)]
pub struct NBIoTUEPriority(u8);

#[derive(Debug, AperCodec)]
#[asn(type = "CHOICE", lb = "0", ub = "2", extensible = true)]
pub enum NGAPPDU {
    #[asn(key = 0, extended = false)]
    initiatingMessage(InitiatingMessage),
    #[asn(key = 1, extended = false)]
    successfulOutcome(SuccessfulOutcome),
    #[asn(key = 2, extended = false)]
    unsuccessfulOutcome(UnsuccessfulOutcome),
}

#[derive(Debug, AperCodec)]
#[asn(type = "CHOICE", lb = "0", ub = "2", extensible = false)]
pub enum NGRANCGI {
    #[asn(key = 0, extended = false)]
    nRCGI(NRCGI),
    #[asn(key = 1, extended = false)]
    eUTRACGI(EUTRACGI),
    #[asn(key = 2, extended = false)]
    choiceExtensions(NGRANCGIchoiceExtensions),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false, optional_fields = 2)]
pub struct NGRANTNLAssociationToRemoveItem {
    pub tnl_association_transport_layer_address: CPTransportLayerInformation,
    #[asn(optional_idx = 0)]
    pub tnl_association_transport_layer_address_amf: Option<CPTransportLayerInformation>,
    #[asn(optional_idx = 1)]
    pub ie_extensions: Option<NGRANTNLAssociationToRemoveItemiEExtensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "32")]
pub struct NGRANTNLAssociationToRemoveList(Vec<NGRANTNLAssociationToRemoveItem>);

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
pub enum NPNAccessInformation {
    #[asn(key = 0, extended = false)]
    pNINPNAccessInformation(CellCAGList),
    #[asn(key = 1, extended = false)]
    choiceExtensions(NPNAccessInformationchoiceExtensions),
}

#[derive(Debug, AperCodec)]
#[asn(type = "CHOICE", lb = "0", ub = "2", extensible = false)]
pub enum NPNMobilityInformation {
    #[asn(key = 0, extended = false)]
    sNPNMobilityInformation(SNPNMobilityInformation),
    #[asn(key = 1, extended = false)]
    pNINPNMobilityInformation(PNINPNMobilityInformation),
    #[asn(key = 2, extended = false)]
    choiceExtensions(NPNMobilityInformationchoiceExtensions),
}

#[derive(Debug, AperCodec)]
#[asn(type = "CHOICE", lb = "0", ub = "1", extensible = false)]
pub enum NPNPagingAssistanceInformation {
    #[asn(key = 0, extended = false)]
    pNINPNPagingAssistance(AllowedPNINPNList),
    #[asn(key = 1, extended = false)]
    choiceExtensions(NPNPagingAssistanceInformationchoiceExtensions),
}

#[derive(Debug, AperCodec)]
#[asn(type = "CHOICE", lb = "0", ub = "1", extensible = false)]
pub enum NPNSupport {
    #[asn(key = 0, extended = false)]
    sNPN(NID),
    #[asn(key = 1, extended = false)]
    choiceExtensions(NPNSupportchoiceExtensions),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct NRCGI {
    pub plmn_identity: PLMNIdentity,
    pub nr_cell_identity: NRCellIdentity,
    #[asn(optional_idx = 0)]
    pub ie_extensions: Option<NRCGIiEExtensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "16384")]
pub struct NRCGIList(Vec<NRCGI>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "65535")]
pub struct NRCGIListForWarning(Vec<NRCGI>);

#[derive(Debug, AperCodec)]
#[asn(type = "INTEGER", lb = "0", ub = "1007", extensible = true)]
pub struct NRPCI(u16);

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
pub struct NRFrequencyBandList(Vec<NRFrequencyBandItem>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct NRFrequencyBandItem {
    pub nr_frequency_band: NRFrequencyBand,
    #[asn(optional_idx = 0)]
    pub ie_extension: Option<NRFrequencyBandItemiEExtension>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct NRFrequencyInfo {
    pub nr_arfcn: NRARFCN,
    pub frequency_band_list: NRFrequencyBandList,
    #[asn(optional_idx = 0)]
    pub ie_extension: Option<NRFrequencyInfoiEExtension>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "OCTET-STRING")]
pub struct NRMobilityHistoryReport(Vec<u8>);

#[derive(Debug, AperCodec)]
#[asn(type = "OCTET-STRING")]
pub struct NRPPaPDU(Vec<u8>);

#[derive(Debug, AperCodec)]
#[asn(type = "OCTET-STRING")]
pub struct NRUERLFReportContainer(Vec<u8>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct NRUESidelinkAggregateMaximumBitrate {
    pub ue_sidelink_aggregate_maximum_bit_rate: BitRate,
    #[asn(optional_idx = 0)]
    pub ie_extensions: Option<NRUESidelinkAggregateMaximumBitrateiEExtensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 3)]
pub struct NRV2XServicesAuthorized {
    #[asn(optional_idx = 0)]
    pub vehicle_ue: Option<VehicleUE>,
    #[asn(optional_idx = 1)]
    pub pedestrian_ue: Option<PedestrianUE>,
    #[asn(optional_idx = 2)]
    pub ie_extensions: Option<NRV2XServicesAuthorizediEExtensions>,
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
pub enum NgENBID {
    #[asn(key = 0, extended = false)]
    macroNgENBID(BITSTRING32),
    #[asn(key = 1, extended = false)]
    shortMacroNgENBID(BITSTRING33),
    #[asn(key = 2, extended = false)]
    longMacroNgENBID(BITSTRING34),
    #[asn(key = 3, extended = false)]
    choiceExtensions(NgENBIDchoiceExtensions),
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
    pub ie_extensions: Option<NonDynamic5QIDescriptoriEExtensions>,
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
    choiceExtensions(OverloadResponsechoiceExtensions),
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
    pub ie_extensions: Option<OverloadStartNSSAIItemiEExtensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "1024")]
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
    pub ie_extensions: Option<PC5FlowBitRatesiEExtensions>,
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
    pub ie_extensions: Option<PC5QoSFlowItemiEExtensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "2048")]
pub struct PC5QoSFlowList(Vec<PC5QoSFlowItem>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 2)]
pub struct PC5QoSParameters {
    pub pc5_qo_s_flow_list: PC5QoSFlowList,
    #[asn(optional_idx = 0)]
    pub pc5_link_aggregate_bit_rates: Option<BitRate>,
    #[asn(optional_idx = 1)]
    pub ie_extensions: Option<PC5QoSParametersiEExtensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "32")]
pub struct PCIListForMDT(Vec<NRPCI>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct PDUSessionAggregateMaximumBitRate {
    pub pdu_session_aggregate_maximum_bit_rate_dl: BitRate,
    pub pdu_session_aggregate_maximum_bit_rate_ul: BitRate,
    #[asn(optional_idx = 0)]
    pub ie_extensions: Option<PDUSessionAggregateMaximumBitRateiEExtensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "INTEGER", lb = "0", ub = "255")]
pub struct PDUSessionID(u8);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct PDUSessionResourceAdmittedItem {
    pub pdu_session_id: PDUSessionID,
    pub handover_request_acknowledge_transfer: OCTETSTRING35,
    #[asn(optional_idx = 0)]
    pub ie_extensions: Option<PDUSessionResourceAdmittedItemiEExtensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "256")]
pub struct PDUSessionResourceAdmittedList(Vec<PDUSessionResourceAdmittedItem>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct PDUSessionResourceFailedToModifyItemModCfm {
    pub pdu_session_id: PDUSessionID,
    pub pdu_session_resource_modify_indication_unsuccessful_transfer: OCTETSTRING36,
    #[asn(optional_idx = 0)]
    pub ie_extensions: Option<PDUSessionResourceFailedToModifyItemModCfmiEExtensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct PDUSessionResourceFailedToModifyItemModRes {
    pub pdu_session_id: PDUSessionID,
    pub pdu_session_resource_modify_unsuccessful_transfer: OCTETSTRING37,
    #[asn(optional_idx = 0)]
    pub ie_extensions: Option<PDUSessionResourceFailedToModifyItemModResiEExtensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "256")]
pub struct PDUSessionResourceFailedToModifyListModCfm(Vec<PDUSessionResourceFailedToModifyItemModCfm>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "256")]
pub struct PDUSessionResourceFailedToModifyListModRes(Vec<PDUSessionResourceFailedToModifyItemModRes>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct PDUSessionResourceFailedToResumeItemRESReq {
    pub pdu_session_id: PDUSessionID,
    pub cause: Cause,
    #[asn(optional_idx = 0)]
    pub ie_extensions: Option<PDUSessionResourceFailedToResumeItemRESReqiEExtensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct PDUSessionResourceFailedToResumeItemRESRes {
    pub pdu_session_id: PDUSessionID,
    pub cause: Cause,
    #[asn(optional_idx = 0)]
    pub ie_extensions: Option<PDUSessionResourceFailedToResumeItemRESResiEExtensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "256")]
pub struct PDUSessionResourceFailedToResumeListRESReq(Vec<PDUSessionResourceFailedToResumeItemRESReq>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "256")]
pub struct PDUSessionResourceFailedToResumeListRESRes(Vec<PDUSessionResourceFailedToResumeItemRESRes>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct PDUSessionResourceFailedToSetupItemCxtFail {
    pub pdu_session_id: PDUSessionID,
    pub pdu_session_resource_setup_unsuccessful_transfer: OCTETSTRING38,
    #[asn(optional_idx = 0)]
    pub ie_extensions: Option<PDUSessionResourceFailedToSetupItemCxtFailiEExtensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct PDUSessionResourceFailedToSetupItemCxtRes {
    pub pdu_session_id: PDUSessionID,
    pub pdu_session_resource_setup_unsuccessful_transfer: OCTETSTRING39,
    #[asn(optional_idx = 0)]
    pub ie_extensions: Option<PDUSessionResourceFailedToSetupItemCxtResiEExtensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct PDUSessionResourceFailedToSetupItemHOAck {
    pub pdu_session_id: PDUSessionID,
    pub handover_resource_allocation_unsuccessful_transfer: OCTETSTRING40,
    #[asn(optional_idx = 0)]
    pub ie_extensions: Option<PDUSessionResourceFailedToSetupItemHOAckiEExtensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct PDUSessionResourceFailedToSetupItemPSReq {
    pub pdu_session_id: PDUSessionID,
    pub path_switch_request_setup_failed_transfer: OCTETSTRING41,
    #[asn(optional_idx = 0)]
    pub ie_extensions: Option<PDUSessionResourceFailedToSetupItemPSReqiEExtensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct PDUSessionResourceFailedToSetupItemSURes {
    pub pdu_session_id: PDUSessionID,
    pub pdu_session_resource_setup_unsuccessful_transfer: OCTETSTRING42,
    #[asn(optional_idx = 0)]
    pub ie_extensions: Option<PDUSessionResourceFailedToSetupItemSUResiEExtensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "256")]
pub struct PDUSessionResourceFailedToSetupListCxtFail(Vec<PDUSessionResourceFailedToSetupItemCxtFail>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "256")]
pub struct PDUSessionResourceFailedToSetupListCxtRes(Vec<PDUSessionResourceFailedToSetupItemCxtRes>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "256")]
pub struct PDUSessionResourceFailedToSetupListHOAck(Vec<PDUSessionResourceFailedToSetupItemHOAck>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "256")]
pub struct PDUSessionResourceFailedToSetupListPSReq(Vec<PDUSessionResourceFailedToSetupItemPSReq>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "256")]
pub struct PDUSessionResourceFailedToSetupListSURes(Vec<PDUSessionResourceFailedToSetupItemSURes>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct PDUSessionResourceHandoverItem {
    pub pdu_session_id: PDUSessionID,
    pub handover_command_transfer: OCTETSTRING43,
    #[asn(optional_idx = 0)]
    pub ie_extensions: Option<PDUSessionResourceHandoverItemiEExtensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "256")]
pub struct PDUSessionResourceHandoverList(Vec<PDUSessionResourceHandoverItem>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 2)]
pub struct PDUSessionResourceInformationItem {
    pub pdu_session_id: PDUSessionID,
    pub qos_flow_information_list: QosFlowInformationList,
    #[asn(optional_idx = 0)]
    pub dr_bs_to_qos_flows_mapping_list: Option<DRBsToQosFlowsMappingList>,
    #[asn(optional_idx = 1)]
    pub ie_extensions: Option<PDUSessionResourceInformationItemiEExtensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "256")]
pub struct PDUSessionResourceInformationList(Vec<PDUSessionResourceInformationItem>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct PDUSessionResourceItemCxtRelCpl {
    pub pdu_session_id: PDUSessionID,
    #[asn(optional_idx = 0)]
    pub ie_extensions: Option<PDUSessionResourceItemCxtRelCpliEExtensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct PDUSessionResourceItemCxtRelReq {
    pub pdu_session_id: PDUSessionID,
    #[asn(optional_idx = 0)]
    pub ie_extensions: Option<PDUSessionResourceItemCxtRelReqiEExtensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct PDUSessionResourceItemHORqd {
    pub pdu_session_id: PDUSessionID,
    pub handover_required_transfer: OCTETSTRING44,
    #[asn(optional_idx = 0)]
    pub ie_extensions: Option<PDUSessionResourceItemHORqdiEExtensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "256")]
pub struct PDUSessionResourceListCxtRelCpl(Vec<PDUSessionResourceItemCxtRelCpl>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "256")]
pub struct PDUSessionResourceListCxtRelReq(Vec<PDUSessionResourceItemCxtRelReq>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "256")]
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
    pub ulngu_up_tnl_information: UPTransportLayerInformation,
    #[asn(optional_idx = 0)]
    pub additional_ng_uuptnl_information: Option<UPTransportLayerInformationPairList>,
    #[asn(optional_idx = 1)]
    pub qos_flow_failed_to_modify_list: Option<QosFlowListWithCause>,
    #[asn(optional_idx = 2)]
    pub ie_extensions: Option<PDUSessionResourceModifyConfirmTransferiEExtensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true)]
pub struct PDUSessionResourceModifyIndication {
    pub protocol_i_es: PDUSessionResourceModifyIndicationprotocolIEs,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 2)]
pub struct PDUSessionResourceModifyIndicationTransfer {
    pub dl_qos_flow_per_tnl_information: QosFlowPerTNLInformation,
    #[asn(optional_idx = 0)]
    pub additional_dl_qos_flow_per_tnl_information: Option<QosFlowPerTNLInformationList>,
    #[asn(optional_idx = 1)]
    pub ie_extensions: Option<PDUSessionResourceModifyIndicationTransferiEExtensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct PDUSessionResourceModifyIndicationUnsuccessfulTransfer {
    pub cause: Cause,
    #[asn(optional_idx = 0)]
    pub ie_extensions:
        Option<PDUSessionResourceModifyIndicationUnsuccessfulTransferiEExtensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct PDUSessionResourceModifyItemModCfm {
    pub pdu_session_id: PDUSessionID,
    pub pdu_session_resource_modify_confirm_transfer: OCTETSTRING45,
    #[asn(optional_idx = 0)]
    pub ie_extensions: Option<PDUSessionResourceModifyItemModCfmiEExtensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct PDUSessionResourceModifyItemModInd {
    pub pdu_session_id: PDUSessionID,
    pub pdu_session_resource_modify_indication_transfer: OCTETSTRING46,
    #[asn(optional_idx = 0)]
    pub ie_extensions: Option<PDUSessionResourceModifyItemModIndiEExtensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 2)]
pub struct PDUSessionResourceModifyItemModReq {
    pub pdu_session_id: PDUSessionID,
    #[asn(optional_idx = 0)]
    pub nas_pdu: Option<NASPDU>,
    pub pdu_session_resource_modify_request_transfer: OCTETSTRING47,
    #[asn(optional_idx = 1)]
    pub ie_extensions: Option<PDUSessionResourceModifyItemModReqiEExtensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct PDUSessionResourceModifyItemModRes {
    pub pdu_session_id: PDUSessionID,
    pub pdu_session_resource_modify_response_transfer: OCTETSTRING48,
    #[asn(optional_idx = 0)]
    pub ie_extensions: Option<PDUSessionResourceModifyItemModResiEExtensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "256")]
pub struct PDUSessionResourceModifyListModCfm(Vec<PDUSessionResourceModifyItemModCfm>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "256")]
pub struct PDUSessionResourceModifyListModInd(Vec<PDUSessionResourceModifyItemModInd>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "256")]
pub struct PDUSessionResourceModifyListModReq(Vec<PDUSessionResourceModifyItemModReq>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "256")]
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
    pub ie_extensions: Option<PDUSessionResourceModifyResponseTransferiEExtensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 2)]
pub struct PDUSessionResourceModifyUnsuccessfulTransfer {
    pub cause: Cause,
    #[asn(optional_idx = 0)]
    pub criticality_diagnostics: Option<CriticalityDiagnostics>,
    #[asn(optional_idx = 1)]
    pub ie_extensions: Option<PDUSessionResourceModifyUnsuccessfulTransferiEExtensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true)]
pub struct PDUSessionResourceNotify {
    pub protocol_i_es: PDUSessionResourceNotifyprotocolIEs,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct PDUSessionResourceNotifyItem {
    pub pdu_session_id: PDUSessionID,
    pub pdu_session_resource_notify_transfer: OCTETSTRING49,
    #[asn(optional_idx = 0)]
    pub ie_extensions: Option<PDUSessionResourceNotifyItemiEExtensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "256")]
pub struct PDUSessionResourceNotifyList(Vec<PDUSessionResourceNotifyItem>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct PDUSessionResourceNotifyReleasedTransfer {
    pub cause: Cause,
    #[asn(optional_idx = 0)]
    pub ie_extensions: Option<PDUSessionResourceNotifyReleasedTransferiEExtensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 3)]
pub struct PDUSessionResourceNotifyTransfer {
    #[asn(optional_idx = 0)]
    pub qos_flow_notify_list: Option<QosFlowNotifyList>,
    #[asn(optional_idx = 1)]
    pub qos_flow_released_list: Option<QosFlowListWithCause>,
    #[asn(optional_idx = 2)]
    pub ie_extensions: Option<PDUSessionResourceNotifyTransferiEExtensions>,
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
    pub ie_extensions: Option<PDUSessionResourceReleaseCommandTransferiEExtensions>,
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
    pub ie_extensions: Option<PDUSessionResourceReleaseResponseTransferiEExtensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct PDUSessionResourceReleasedItemNot {
    pub pdu_session_id: PDUSessionID,
    pub pdu_session_resource_notify_released_transfer: OCTETSTRING50,
    #[asn(optional_idx = 0)]
    pub ie_extensions: Option<PDUSessionResourceReleasedItemNotiEExtensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct PDUSessionResourceReleasedItemPSAck {
    pub pdu_session_id: PDUSessionID,
    pub path_switch_request_unsuccessful_transfer: OCTETSTRING51,
    #[asn(optional_idx = 0)]
    pub ie_extensions: Option<PDUSessionResourceReleasedItemPSAckiEExtensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct PDUSessionResourceReleasedItemPSFail {
    pub pdu_session_id: PDUSessionID,
    pub path_switch_request_unsuccessful_transfer: OCTETSTRING52,
    #[asn(optional_idx = 0)]
    pub ie_extensions: Option<PDUSessionResourceReleasedItemPSFailiEExtensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct PDUSessionResourceReleasedItemRelRes {
    pub pdu_session_id: PDUSessionID,
    pub pdu_session_resource_release_response_transfer: OCTETSTRING53,
    #[asn(optional_idx = 0)]
    pub ie_extensions: Option<PDUSessionResourceReleasedItemRelResiEExtensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "256")]
pub struct PDUSessionResourceReleasedListNot(Vec<PDUSessionResourceReleasedItemNot>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "256")]
pub struct PDUSessionResourceReleasedListPSAck(Vec<PDUSessionResourceReleasedItemPSAck>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "256")]
pub struct PDUSessionResourceReleasedListPSFail(Vec<PDUSessionResourceReleasedItemPSFail>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "256")]
pub struct PDUSessionResourceReleasedListRelRes(Vec<PDUSessionResourceReleasedItemRelRes>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct PDUSessionResourceResumeItemRESReq {
    pub pdu_session_id: PDUSessionID,
    pub ue_context_resume_request_transfer: OCTETSTRING54,
    #[asn(optional_idx = 0)]
    pub ie_extensions: Option<PDUSessionResourceResumeItemRESReqiEExtensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct PDUSessionResourceResumeItemRESRes {
    pub pdu_session_id: PDUSessionID,
    pub ue_context_resume_response_transfer: OCTETSTRING55,
    #[asn(optional_idx = 0)]
    pub ie_extensions: Option<PDUSessionResourceResumeItemRESResiEExtensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "256")]
pub struct PDUSessionResourceResumeListRESReq(Vec<PDUSessionResourceResumeItemRESReq>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "256")]
pub struct PDUSessionResourceResumeListRESRes(Vec<PDUSessionResourceResumeItemRESRes>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct PDUSessionResourceSecondaryRATUsageItem {
    pub pdu_session_id: PDUSessionID,
    pub secondary_rat_data_usage_report_transfer: OCTETSTRING56,
    #[asn(optional_idx = 0)]
    pub ie_extensions: Option<PDUSessionResourceSecondaryRATUsageItemiEExtensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "256")]
pub struct PDUSessionResourceSecondaryRATUsageList(Vec<PDUSessionResourceSecondaryRATUsageItem>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 2)]
pub struct PDUSessionResourceSetupItemCxtReq {
    pub pdu_session_id: PDUSessionID,
    #[asn(optional_idx = 0)]
    pub nas_pdu: Option<NASPDU>,
    pub s_nssai: SNSSAI,
    pub pdu_session_resource_setup_request_transfer: OCTETSTRING57,
    #[asn(optional_idx = 1)]
    pub ie_extensions: Option<PDUSessionResourceSetupItemCxtReqiEExtensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct PDUSessionResourceSetupItemCxtRes {
    pub pdu_session_id: PDUSessionID,
    pub pdu_session_resource_setup_response_transfer: OCTETSTRING58,
    #[asn(optional_idx = 0)]
    pub ie_extensions: Option<PDUSessionResourceSetupItemCxtResiEExtensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct PDUSessionResourceSetupItemHOReq {
    pub pdu_session_id: PDUSessionID,
    pub s_nssai: SNSSAI,
    pub handover_request_transfer: OCTETSTRING59,
    #[asn(optional_idx = 0)]
    pub ie_extensions: Option<PDUSessionResourceSetupItemHOReqiEExtensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 2)]
pub struct PDUSessionResourceSetupItemSUReq {
    pub pdu_session_id: PDUSessionID,
    #[asn(optional_idx = 0)]
    pub pdu_session_nas_pdu: Option<NASPDU>,
    pub s_nssai: SNSSAI,
    pub pdu_session_resource_setup_request_transfer: OCTETSTRING60,
    #[asn(optional_idx = 1)]
    pub ie_extensions: Option<PDUSessionResourceSetupItemSUReqiEExtensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct PDUSessionResourceSetupItemSURes {
    pub pdu_session_id: PDUSessionID,
    pub pdu_session_resource_setup_response_transfer: OCTETSTRING61,
    #[asn(optional_idx = 0)]
    pub ie_extensions: Option<PDUSessionResourceSetupItemSUResiEExtensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "256")]
pub struct PDUSessionResourceSetupListCxtReq(Vec<PDUSessionResourceSetupItemCxtReq>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "256")]
pub struct PDUSessionResourceSetupListCxtRes(Vec<PDUSessionResourceSetupItemCxtRes>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "256")]
pub struct PDUSessionResourceSetupListHOReq(Vec<PDUSessionResourceSetupItemHOReq>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "256")]
pub struct PDUSessionResourceSetupListSUReq(Vec<PDUSessionResourceSetupItemSUReq>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "256")]
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
    pub dl_qos_flow_per_tnl_information: QosFlowPerTNLInformation,
    #[asn(optional_idx = 0)]
    pub additional_dl_qos_flow_per_tnl_information: Option<QosFlowPerTNLInformationList>,
    #[asn(optional_idx = 1)]
    pub security_result: Option<SecurityResult>,
    #[asn(optional_idx = 2)]
    pub qos_flow_failed_to_setup_list: Option<QosFlowListWithCause>,
    #[asn(optional_idx = 3)]
    pub ie_extensions: Option<PDUSessionResourceSetupResponseTransferiEExtensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 2)]
pub struct PDUSessionResourceSetupUnsuccessfulTransfer {
    pub cause: Cause,
    #[asn(optional_idx = 0)]
    pub criticality_diagnostics: Option<CriticalityDiagnostics>,
    #[asn(optional_idx = 1)]
    pub ie_extensions: Option<PDUSessionResourceSetupUnsuccessfulTransferiEExtensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct PDUSessionResourceSuspendItemSUSReq {
    pub pdu_session_id: PDUSessionID,
    pub ue_context_suspend_request_transfer: OCTETSTRING62,
    #[asn(optional_idx = 0)]
    pub ie_extensions: Option<PDUSessionResourceSuspendItemSUSReqiEExtensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "256")]
pub struct PDUSessionResourceSuspendListSUSReq(Vec<PDUSessionResourceSuspendItemSUSReq>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct PDUSessionResourceSwitchedItem {
    pub pdu_session_id: PDUSessionID,
    pub path_switch_request_acknowledge_transfer: OCTETSTRING63,
    #[asn(optional_idx = 0)]
    pub ie_extensions: Option<PDUSessionResourceSwitchedItemiEExtensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "256")]
pub struct PDUSessionResourceSwitchedList(Vec<PDUSessionResourceSwitchedItem>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct PDUSessionResourceToBeSwitchedDLItem {
    pub pdu_session_id: PDUSessionID,
    pub path_switch_request_transfer: OCTETSTRING64,
    #[asn(optional_idx = 0)]
    pub ie_extensions: Option<PDUSessionResourceToBeSwitchedDLItemiEExtensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "256")]
pub struct PDUSessionResourceToBeSwitchedDLList(Vec<PDUSessionResourceToBeSwitchedDLItem>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct PDUSessionResourceToReleaseItemHOCmd {
    pub pdu_session_id: PDUSessionID,
    pub handover_preparation_unsuccessful_transfer: OCTETSTRING65,
    #[asn(optional_idx = 0)]
    pub ie_extensions: Option<PDUSessionResourceToReleaseItemHOCmdiEExtensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct PDUSessionResourceToReleaseItemRelCmd {
    pub pdu_session_id: PDUSessionID,
    pub pdu_session_resource_release_command_transfer: OCTETSTRING66,
    #[asn(optional_idx = 0)]
    pub ie_extensions: Option<PDUSessionResourceToReleaseItemRelCmdiEExtensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "256")]
pub struct PDUSessionResourceToReleaseListHOCmd(Vec<PDUSessionResourceToReleaseItemHOCmd>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "256")]
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
    pub rat_type: ENUMERATED67,
    pub pdu_session_timed_report_list: VolumeTimedReportList,
    #[asn(optional_idx = 0)]
    pub ie_extensions: Option<PDUSessionUsageReportiEExtensions>,
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
    pub ie_extensions: Option<PLMNSupportItemiEExtensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "12")]
pub struct PLMNSupportList(Vec<PLMNSupportItem>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct PNINPNMobilityInformation {
    pub allowed_pni_npi_list: AllowedPNINPNList,
    #[asn(optional_idx = 0)]
    pub ie_extensions: Option<PNINPNMobilityInformationiEExtensions>,
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
    eUTRACGIPWSFailedList(EUTRACGIList),
    #[asn(key = 1, extended = false)]
    nRCGIPWSFailedList(NRCGIList),
    #[asn(key = 2, extended = false)]
    choiceExtensions(PWSFailedCellIDListchoiceExtensions),
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
    pub per_scalar: INTEGER68,
    pub per_exponent: INTEGER69,
    #[asn(optional_idx = 0)]
    pub ie_extensions: Option<PacketErrorRateiEExtensions>,
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
#[asn(type = "ENUMERATED", extensible = true, lb = "0", ub = "13")]
pub struct PagingeDRXCycle(u8);
impl PagingeDRXCycle {
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
    pub eutra_cgi: EUTRACGI,
    pub coverage_enhancement_level: CoverageEnhancementLevel,
    #[asn(optional_idx = 0)]
    pub ie_extensions: Option<PagingAssisDataforCEcapabUEiEExtensions>,
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
    pub ie_extensions: Option<PagingAttemptInformationiEExtensions>,
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
    pub paging_e_drx_cycle: PagingeDRXCycle,
    #[asn(optional_idx = 0)]
    pub paging_time_window: Option<PagingTimeWindow>,
    #[asn(optional_idx = 1)]
    pub ie_extensions: Option<PagingeDRXInformationiEExtensions>,
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
    pub ul_ngu_up_tnl_information: Option<UPTransportLayerInformation>,
    #[asn(optional_idx = 1)]
    pub security_indication: Option<SecurityIndication>,
    #[asn(optional_idx = 2)]
    pub ie_extensions: Option<PathSwitchRequestAcknowledgeTransferiEExtensions>,
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
    pub ie_extensions: Option<PathSwitchRequestSetupFailedTransferiEExtensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 3)]
pub struct PathSwitchRequestTransfer {
    pub dl_ngu_up_tnl_information: UPTransportLayerInformation,
    #[asn(optional_idx = 0)]
    pub dl_ngu_tnl_information_reused: Option<DLNGUTNLInformationReused>,
    #[asn(optional_idx = 1)]
    pub user_plane_security_information: Option<UserPlaneSecurityInformation>,
    pub qos_flow_accepted_list: QosFlowAcceptedList,
    #[asn(optional_idx = 2)]
    pub ie_extensions: Option<PathSwitchRequestTransferiEExtensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct PathSwitchRequestUnsuccessfulTransfer {
    pub cause: Cause,
    #[asn(optional_idx = 0)]
    pub ie_extensions: Option<PathSwitchRequestUnsuccessfulTransferiEExtensions>,
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
pub struct PreemptionCapability(u8);
impl PreemptionCapability {
    const SHALL_NOT_TRIGGER_PRE_EMPTION: u8 = 0u8;
    const MAY_TRIGGER_PRE_EMPTION: u8 = 1u8;
}

#[derive(Debug, AperCodec)]
#[asn(type = "ENUMERATED", extensible = true, lb = "0", ub = "1")]
pub struct PreemptionVulnerability(u8);
impl PreemptionVulnerability {
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
pub enum PrivateIEID {
    #[asn(key = 0, extended = false)]
    local(INTEGER70),
    #[asn(key = 1, extended = false)]
    global(OBJECTIDENTIFIER71),
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
    firstdlcount(FirstDLCount),
    #[asn(key = 1, extended = false)]
    choiceExtensions(ProcedureStageChoicechoiceExtensions),
}

#[derive(Debug, AperCodec)]
#[asn(type = "INTEGER", lb = "0", ub = "65535")]
pub struct ProtocolExtensionID(u16);

#[derive(Debug, AperCodec)]
#[asn(type = "INTEGER", lb = "0", ub = "65535")]
pub struct ProtocolIEID(u16);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct QoSFlowsUsageReportItem {
    pub qos_flow_identifier: QosFlowIdentifier,
    pub rat_type: ENUMERATED72,
    pub qo_s_flows_timed_report_list: VolumeTimedReportList,
    #[asn(optional_idx = 0)]
    pub ie_extensions: Option<QoSFlowsUsageReportItemiEExtensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "64")]
pub struct QoSFlowsUsageReportList(Vec<QoSFlowsUsageReportItem>);

#[derive(Debug, AperCodec)]
#[asn(type = "CHOICE", lb = "0", ub = "2", extensible = false)]
pub enum QosCharacteristics {
    #[asn(key = 0, extended = false)]
    nonDynamic5QI(NonDynamic5QIDescriptor),
    #[asn(key = 1, extended = false)]
    dynamic5QI(Dynamic5QIDescriptor),
    #[asn(key = 2, extended = false)]
    choiceExtensions(QosCharacteristicschoiceExtensions),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct QosFlowAcceptedItem {
    pub qos_flow_identifier: QosFlowIdentifier,
    #[asn(optional_idx = 0)]
    pub ie_extensions: Option<QosFlowAcceptedItemiEExtensions>,
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
    pub e_rab_id: Option<ERABID>,
    #[asn(optional_idx = 2)]
    pub ie_extensions: Option<QosFlowAddOrModifyRequestItemiEExtensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "64")]
pub struct QosFlowAddOrModifyRequestList(Vec<QosFlowAddOrModifyRequestItem>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct QosFlowAddOrModifyResponseItem {
    pub qos_flow_identifier: QosFlowIdentifier,
    #[asn(optional_idx = 0)]
    pub ie_extensions: Option<QosFlowAddOrModifyResponseItemiEExtensions>,
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
    pub ie_extensions: Option<QosFlowFeedbackItemiEExtensions>,
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
    pub ie_extensions: Option<QosFlowInformationItemiEExtensions>,
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
    pub ie_extensions: Option<QosFlowItemWithDataForwardingiEExtensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 4)]
pub struct QosFlowLevelQosParameters {
    pub qos_characteristics: QosCharacteristics,
    pub allocation_and_retention_priority: AllocationAndRetentionPriority,
    #[asn(optional_idx = 0)]
    pub gbr_qos_information: Option<GBRQosInformation>,
    #[asn(optional_idx = 1)]
    pub reflective_qos_attribute: Option<ReflectiveQosAttribute>,
    #[asn(optional_idx = 2)]
    pub additional_qos_flow_information: Option<AdditionalQosFlowInformation>,
    #[asn(optional_idx = 3)]
    pub ie_extensions: Option<QosFlowLevelQosParametersiEExtensions>,
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
    pub ie_extensions: Option<QosFlowModifyConfirmItemiEExtensions>,
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
    pub ie_extensions: Option<QosFlowNotifyItemiEExtensions>,
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
    pub ie_extensions: Option<QosFlowParametersItemiEExtensions>,
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
    pub ie_extensions: Option<QosFlowPerTNLInformationiEExtensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct QosFlowPerTNLInformationItem {
    pub qos_flow_per_tnl_information: QosFlowPerTNLInformation,
    #[asn(optional_idx = 0)]
    pub ie_extensions: Option<QosFlowPerTNLInformationItemiEExtensions>,
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
    pub e_rab_id: Option<ERABID>,
    #[asn(optional_idx = 1)]
    pub ie_extensions: Option<QosFlowSetupRequestItemiEExtensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "64")]
pub struct QosFlowSetupRequestList(Vec<QosFlowSetupRequestItem>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct QosFlowToBeForwardedItem {
    pub qos_flow_identifier: QosFlowIdentifier,
    #[asn(optional_idx = 0)]
    pub ie_extensions: Option<QosFlowToBeForwardedItemiEExtensions>,
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
    pub ie_extensions: Option<QosFlowWithCauseItemiEExtensions>,
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
pub struct RANUENGAPID(u32);

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
#[asn(type = "PrintableString", sz_extensible = true, sz_lb = "1", sz_ub = "150")]
pub struct RANNodeName(String);

#[derive(Debug, AperCodec)]
#[asn(type = "UTF8String", sz_extensible = true, sz_lb = "1", sz_ub = "150")]
pub struct RANNodeNameUTF8String(String);

#[derive(Debug, AperCodec)]
#[asn(type = "VisibleString", sz_extensible = true, sz_lb = "1", sz_ub = "150")]
pub struct RANNodeNameVisibleString(String);

#[derive(Debug, AperCodec)]
#[asn(type = "INTEGER", lb = "1", ub = "256")]
pub struct RANPagingPriority(u16);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct RANStatusTransferTransparentContainer {
    pub dr_bs_subject_to_status_transfer_list: DRBsSubjectToStatusTransferList,
    #[asn(optional_idx = 0)]
    pub ie_extensions: Option<RANStatusTransferTransparentContaineriEExtensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "ENUMERATED", extensible = true, lb = "0", ub = "1")]
pub struct RATInformation(u8);
impl RATInformation {
    const UNLICENSED: u8 = 0u8;
    const NB_IO_T: u8 = 1u8;
}

#[derive(Debug, AperCodec)]
#[asn(type = "BITSTRING", sz_extensible = true, sz_lb = "8", sz_ub = "8")]
pub struct RATRestrictionInformation(BitVec<Msb0, u8>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "16")]
pub struct RATRestrictions(Vec<RATRestrictionsItem>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct RATRestrictionsItem {
    pub plmn_identity: PLMNIdentity,
    pub rat_restriction_information: RATRestrictionInformation,
    #[asn(optional_idx = 0)]
    pub ie_extensions: Option<RATRestrictionsItemiEExtensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "OCTET-STRING")]
pub struct RGLevelWirelineAccessCharacteristics(Vec<u8>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct RIMInformation {
    pub targetg_nb_set_id: GNBSetID,
    pub rim_rs_detection: ENUMERATED73,
    #[asn(optional_idx = 0)]
    pub ie_extensions: Option<RIMInformationiEExtensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct RIMInformationTransfer {
    pub target_ran_node_id: TargetRANNodeID,
    pub source_ran_node_id: SourceRANNodeID,
    pub rim_information: RIMInformation,
    #[asn(optional_idx = 0)]
    pub ie_extensions: Option<RIMInformationTransferiEExtensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "INTEGER", lb = "0", ub = "4095")]
pub struct RNCID(u16);

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
    pub ngran_cgi: NGRANCGI,
    #[asn(optional_idx = 0)]
    pub time_stayed_in_cell: Option<INTEGER74>,
    #[asn(optional_idx = 1)]
    pub ie_extensions: Option<RecommendedCellItemiEExtensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "16")]
pub struct RecommendedCellList(Vec<RecommendedCellItem>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct RecommendedCellsForPaging {
    pub recommended_cell_list: RecommendedCellList,
    #[asn(optional_idx = 0)]
    pub ie_extensions: Option<RecommendedCellsForPagingiEExtensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct RecommendedRANNodeItem {
    pub amf_paging_target: AMFPagingTarget,
    #[asn(optional_idx = 0)]
    pub ie_extensions: Option<RecommendedRANNodeItemiEExtensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "16")]
pub struct RecommendedRANNodeList(Vec<RecommendedRANNodeItem>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct RecommendedRANNodesForPaging {
    pub recommended_ran_node_list: RecommendedRANNodeList,
    #[asn(optional_idx = 0)]
    pub ie_extensions: Option<RecommendedRANNodesForPagingiEExtensions>,
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
    pub ie_extensions: Option<RedundantPDUSessionInformationiEExtensions>,
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
#[asn(type = "OCTET-STRING", sz_extensible = false, sz_lb = "32", sz_ub = "32")]
pub struct RejectedNSSAIinPLMN(Vec<u8>);

#[derive(Debug, AperCodec)]
#[asn(type = "OCTET-STRING", sz_extensible = false, sz_lb = "32", sz_ub = "32")]
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
    nGInterface(ResetAll),
    #[asn(key = 1, extended = false)]
    partOfNGInterface(UEassociatedLogicalNGconnectionList),
    #[asn(key = 2, extended = false)]
    choiceExtensions(ResetTypechoiceExtensions),
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
pub struct SNSSAI {
    pub sst: SST,
    #[asn(optional_idx = 0)]
    pub sd: Option<SD>,
    #[asn(optional_idx = 1)]
    pub ie_extensions: Option<SNSSAIiEExtensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "2")]
pub struct SCTPTLAs(Vec<TransportLayerAddress>);

#[derive(Debug, AperCodec)]
#[asn(type = "OCTET-STRING", sz_extensible = false, sz_lb = "3", sz_ub = "3")]
pub struct SD(Vec<u8>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct SNPNMobilityInformation {
    pub serving_nid: NID,
    #[asn(optional_idx = 0)]
    pub ie_extensions: Option<SNPNMobilityInformationiEExtensions>,
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
    pub ie_extensions: Option<SONConfigurationTransferiEExtensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "CHOICE", lb = "0", ub = "2", extensible = false)]
pub enum SONInformation {
    #[asn(key = 0, extended = false)]
    sONInformationRequest(SONInformationRequest),
    #[asn(key = 1, extended = false)]
    sONInformationReply(SONInformationReply),
    #[asn(key = 2, extended = false)]
    choiceExtensions(SONInformationchoiceExtensions),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 2)]
pub struct SONInformationReply {
    #[asn(optional_idx = 0)]
    pub xn_tnl_configuration_info: Option<XnTNLConfigurationInfo>,
    #[asn(optional_idx = 1)]
    pub ie_extensions: Option<SONInformationReplyiEExtensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "CHOICE", lb = "0", ub = "2", extensible = false)]
pub enum SONInformationReport {
    #[asn(key = 0, extended = false)]
    failureIndicationInformation(FailureIndication),
    #[asn(key = 1, extended = false)]
    hOReportInformation(HOReport),
    #[asn(key = 2, extended = false)]
    choiceExtensions(SONInformationReportchoiceExtensions),
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
    pub dayof_week: Option<BITSTRING75>,
    #[asn(optional_idx = 1)]
    pub timeof_day_start: Option<INTEGER76>,
    #[asn(optional_idx = 2)]
    pub timeof_day_end: Option<INTEGER77>,
    #[asn(optional_idx = 3)]
    pub ie_extensions: Option<ScheduledCommunicationTimeiEExtensions>,
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
    pub ie_extensions: Option<SecondaryRATDataUsageReportTransferiEExtensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 3)]
pub struct SecondaryRATUsageInformation {
    #[asn(optional_idx = 0)]
    pub pdu_session_usage_report: Option<PDUSessionUsageReport>,
    #[asn(optional_idx = 1)]
    pub qos_flows_usage_report_list: Option<QoSFlowsUsageReportList>,
    #[asn(optional_idx = 2)]
    pub ie_extension: Option<SecondaryRATUsageInformationiEExtension>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct SecurityContext {
    pub next_hop_chaining_count: NextHopChainingCount,
    pub next_hop_nh: SecurityKey,
    #[asn(optional_idx = 0)]
    pub ie_extensions: Option<SecurityContextiEExtensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 2)]
pub struct SecurityIndication {
    pub integrity_protection_indication: IntegrityProtectionIndication,
    pub confidentiality_protection_indication: ConfidentialityProtectionIndication,
    #[asn(optional_idx = 0)]
    pub maximum_integrity_protected_data_rate_ul: Option<MaximumIntegrityProtectedDataRate>,
    #[asn(optional_idx = 1)]
    pub ie_extensions: Option<SecurityIndicationiEExtensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "BITSTRING", sz_extensible = false, sz_lb = "256", sz_ub = "256")]
pub struct SecurityKey(BitVec<Msb0, u8>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct SecurityResult {
    pub integrity_protection_result: IntegrityProtectionResult,
    pub confidentiality_protection_result: ConfidentialityProtectionResult,
    #[asn(optional_idx = 0)]
    pub ie_extensions: Option<SecurityResultiEExtensions>,
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
    pub ie_extensions: Option<SensorMeasConfigNameItemiEExtensions>,
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
    pub ie_extensions: Option<SensorMeasurementConfigurationiEExtensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "CHOICE", lb = "0", ub = "3", extensible = false)]
pub enum SensorNameConfig {
    #[asn(key = 0, extended = false)]
    uncompensatedBarometricConfig(ENUMERATED78),
    #[asn(key = 1, extended = false)]
    ueSpeedConfig(ENUMERATED79),
    #[asn(key = 2, extended = false)]
    ueOrientationConfig(ENUMERATED80),
    #[asn(key = 3, extended = false)]
    choiceExtensions(SensorNameConfigchoiceExtensions),
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
    pub ie_extensions: Option<ServedGUAMIItemiEExtensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "256")]
pub struct ServedGUAMIList(Vec<ServedGUAMIItem>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "16")]
pub struct ServiceAreaInformation(Vec<ServiceAreaInformationItem>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 3)]
pub struct ServiceAreaInformationItem {
    pub plmn_identity: PLMNIdentity,
    #[asn(optional_idx = 0)]
    pub allowed_ta_cs: Option<AllowedTACs>,
    #[asn(optional_idx = 1)]
    pub not_allowed_ta_cs: Option<NotAllowedTACs>,
    #[asn(optional_idx = 2)]
    pub ie_extensions: Option<ServiceAreaInformationItemiEExtensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "INTEGER", lb = "0", ub = "4294967295")]
pub struct SgNBUEX2APID(u32);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct SliceOverloadItem {
    pub s_nssai: SNSSAI,
    #[asn(optional_idx = 0)]
    pub ie_extensions: Option<SliceOverloadItemiEExtensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "1024")]
pub struct SliceOverloadList(Vec<SliceOverloadItem>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct SliceSupportItem {
    pub s_nssai: SNSSAI,
    #[asn(optional_idx = 0)]
    pub ie_extensions: Option<SliceSupportItemiEExtensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "1024")]
pub struct SliceSupportList(Vec<SliceSupportItem>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 4)]
pub struct SourceNGRANNodeToTargetNGRANNodeTransparentContainer {
    pub rrc_container: RRCContainer,
    #[asn(optional_idx = 0)]
    pub pdu_session_resource_information_list: Option<PDUSessionResourceInformationList>,
    #[asn(optional_idx = 1)]
    pub e_rab_information_list: Option<ERABInformationList>,
    pub target_cell_id: NGRANCGI,
    #[asn(optional_idx = 2)]
    pub index_to_rfsp: Option<IndexToRFSP>,
    pub ue_history_information: UEHistoryInformation,
    #[asn(optional_idx = 3)]
    pub ie_extensions: Option<SourceNGRANNodeToTargetNGRANNodeTransparentContaineriEExtensions>,
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
    pub ie_extensions: Option<SourceRANNodeIDiEExtensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 4)]
pub struct SourceToTargetAMFInformationReroute {
    #[asn(optional_idx = 0)]
    pub configured_nssai: Option<ConfiguredNSSAI>,
    #[asn(optional_idx = 1)]
    pub rejected_nssa_iin_plmn: Option<RejectedNSSAIinPLMN>,
    #[asn(optional_idx = 2)]
    pub rejected_nssa_iin_ta: Option<RejectedNSSAIinTA>,
    #[asn(optional_idx = 3)]
    pub ie_extensions: Option<SourceToTargetAMFInformationRerouteiEExtensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "OCTET-STRING")]
pub struct SourceToTargetTransparentContainer(Vec<u8>);

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
    pub tac: TAC,
    pub broadcast_plmn_list: BroadcastPLMNList,
    #[asn(optional_idx = 0)]
    pub ie_extensions: Option<SupportedTAItemiEExtensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "256")]
pub struct SupportedTAList(Vec<SupportedTAItem>);

#[derive(Debug, AperCodec)]
#[asn(type = "ENUMERATED", extensible = true, lb = "0", ub = "0")]
pub struct SuspendRequestIndication(u8);
impl SuspendRequestIndication {
    const SUSPEND_REQUESTED: u8 = 0u8;
}

#[derive(Debug, AperCodec)]
#[asn(type = "ENUMERATED", extensible = true, lb = "0", ub = "0")]
pub struct SuspendResponseIndication(u8);
impl SuspendResponseIndication {
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
    pub ie_extensions: Option<TABasedMDTiEExtensions>,
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
    pub ie_extensions: Option<TAIiEExtensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct TAIBasedMDT {
    pub tai_listfor_mdt: TAIListforMDT,
    #[asn(optional_idx = 0)]
    pub ie_extensions: Option<TAIBasedMDTiEExtensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "65535")]
pub struct TAIBroadcastEUTRA(Vec<TAIBroadcastEUTRAItem>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct TAIBroadcastEUTRAItem {
    pub tai: TAI,
    pub completed_cells_in_tai_eutra: CompletedCellsInTAIEUTRA,
    #[asn(optional_idx = 0)]
    pub ie_extensions: Option<TAIBroadcastEUTRAItemiEExtensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "65535")]
pub struct TAIBroadcastNR(Vec<TAIBroadcastNRItem>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct TAIBroadcastNRItem {
    pub tai: TAI,
    pub completed_cells_in_tai_nr: CompletedCellsInTAINR,
    #[asn(optional_idx = 0)]
    pub ie_extensions: Option<TAIBroadcastNRItemiEExtensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "65535")]
pub struct TAICancelledEUTRA(Vec<TAICancelledEUTRAItem>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct TAICancelledEUTRAItem {
    pub tai: TAI,
    pub cancelled_cells_in_tai_eutra: CancelledCellsInTAIEUTRA,
    #[asn(optional_idx = 0)]
    pub ie_extensions: Option<TAICancelledEUTRAItemiEExtensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "65535")]
pub struct TAICancelledNR(Vec<TAICancelledNRItem>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct TAICancelledNRItem {
    pub tai: TAI,
    pub cancelled_cells_in_tai_nr: CancelledCellsInTAINR,
    #[asn(optional_idx = 0)]
    pub ie_extensions: Option<TAICancelledNRItemiEExtensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "16")]
pub struct TAIListForInactive(Vec<TAIListForInactiveItem>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct TAIListForInactiveItem {
    pub tai: TAI,
    #[asn(optional_idx = 0)]
    pub ie_extensions: Option<TAIListForInactiveItemiEExtensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "16")]
pub struct TAIListForPaging(Vec<TAIListForPagingItem>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct TAIListForPagingItem {
    pub tai: TAI,
    #[asn(optional_idx = 0)]
    pub ie_extensions: Option<TAIListForPagingItemiEExtensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "2048")]
pub struct TAIListForRestart(Vec<TAI>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "65535")]
pub struct TAIListForWarning(Vec<TAI>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "8")]
pub struct TAIListforMDT(Vec<TAI>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "8")]
pub struct TAListforMDT(Vec<TAC>);

#[derive(Debug, AperCodec)]
#[asn(type = "OCTET-STRING")]
pub struct TNAPID(Vec<u8>);

#[derive(Debug, AperCodec)]
#[asn(type = "CHOICE", lb = "0", ub = "1", extensible = false)]
pub enum TNGFID {
    #[asn(key = 0, extended = false)]
    tNGFID(BITSTRING81),
    #[asn(key = 1, extended = false)]
    choiceExtensions(TNGFIDchoiceExtensions),
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
    pub ie_extensions: Option<TNLAssociationItemiEExtensions>,
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
    pub ie_extensions: Option<TSCAssistanceInformationiEExtensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 3)]
pub struct TSCTrafficCharacteristics {
    #[asn(optional_idx = 0)]
    pub tsc_assistance_information_dl: Option<TSCAssistanceInformation>,
    #[asn(optional_idx = 1)]
    pub tsc_assistance_information_ul: Option<TSCAssistanceInformation>,
    #[asn(optional_idx = 2)]
    pub ie_extensions: Option<TSCTrafficCharacteristicsiEExtensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "OCTET-STRING")]
pub struct TWAPID(Vec<u8>);

#[derive(Debug, AperCodec)]
#[asn(type = "CHOICE", lb = "0", ub = "1", extensible = false)]
pub enum TWIFID {
    #[asn(key = 0, extended = false)]
    tWIFID(BITSTRING82),
    #[asn(key = 1, extended = false)]
    choiceExtensions(TWIFIDchoiceExtensions),
}

#[derive(Debug, AperCodec)]
#[asn(type = "CHOICE", lb = "0", ub = "2", extensible = false)]
pub enum TargetID {
    #[asn(key = 0, extended = false)]
    targetRANNodeID(TargetRANNodeID),
    #[asn(key = 1, extended = false)]
    targeteNBID(TargeteNBID),
    #[asn(key = 2, extended = false)]
    choiceExtensions(TargetIDchoiceExtensions),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct TargetNGRANNodeToSourceNGRANNodeFailureTransparentContainer {
    pub cell_cag_information: CellCAGInformation,
    #[asn(optional_idx = 0)]
    pub ie_extensions:
        Option<TargetNGRANNodeToSourceNGRANNodeFailureTransparentContaineriEExtensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct TargetNGRANNodeToSourceNGRANNodeTransparentContainer {
    pub rrc_container: RRCContainer,
    #[asn(optional_idx = 0)]
    pub ie_extensions: Option<TargetNGRANNodeToSourceNGRANNodeTransparentContaineriEExtensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct TargetRANNodeID {
    pub global_ran_node_id: GlobalRANNodeID,
    pub selected_tai: TAI,
    #[asn(optional_idx = 0)]
    pub ie_extensions: Option<TargetRANNodeIDiEExtensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 2)]
pub struct TargetRNCID {
    pub lai: LAI,
    pub rnc_id: RNCID,
    #[asn(optional_idx = 0)]
    pub extended_rnc_id: Option<ExtendedRNCID>,
    #[asn(optional_idx = 1)]
    pub ie_extensions: Option<TargetRNCIDiEExtensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "OCTET-STRING")]
pub struct TargetToSourceTransparentContainer(Vec<u8>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct TargeteNBID {
    pub global_enb_id: GlobalNgENBID,
    pub selected_eps_tai: EPSTAI,
    #[asn(optional_idx = 0)]
    pub ie_extensions: Option<TargeteNBIDiEExtensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "OCTET-STRING")]
pub struct TargettoSourceFailureTransparentContainer(Vec<u8>);

#[derive(Debug, AperCodec)]
#[asn(type = "INTEGER", lb = "0", ub = "127")]
pub struct ThresholdRSRP(u8);

#[derive(Debug, AperCodec)]
#[asn(type = "INTEGER", lb = "0", ub = "127")]
pub struct ThresholdRSRQ(u8);

#[derive(Debug, AperCodec)]
#[asn(type = "INTEGER", lb = "0", ub = "127")]
pub struct ThresholdSINR(u8);

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
    pub sourcecell_id: EUTRACGI,
    pub failurecell_id: NGRANCGI,
    #[asn(optional_idx = 0)]
    pub uerlf_report_container: Option<UERLFReportContainer>,
    #[asn(optional_idx = 1)]
    pub ie_extensions: Option<TooearlyIntersystemHOiEExtensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct TraceActivation {
    pub ngran_trace_id: NGRANTraceID,
    pub interfaces_to_trace: InterfacesToTrace,
    pub trace_depth: TraceDepth,
    pub trace_collection_entity_ip_address: TransportLayerAddress,
    #[asn(optional_idx = 0)]
    pub ie_extensions: Option<TraceActivationiEExtensions>,
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
pub struct UEDifferentiationInfo {
    #[asn(optional_idx = 0)]
    pub periodic_communication_indicator: Option<ENUMERATED83>,
    #[asn(optional_idx = 1)]
    pub periodic_time: Option<INTEGER84>,
    #[asn(optional_idx = 2)]
    pub scheduled_communication_time: Option<ScheduledCommunicationTime>,
    #[asn(optional_idx = 3)]
    pub stationary_indication: Option<ENUMERATED85>,
    #[asn(optional_idx = 4)]
    pub traffic_profile: Option<ENUMERATED86>,
    #[asn(optional_idx = 5)]
    pub battery_indication: Option<ENUMERATED87>,
    #[asn(optional_idx = 6)]
    pub ie_extensions: Option<UEDifferentiationInfoiEExtensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct UENGAPIDpair {
    pub amf_ue_ngap_id: AMFUENGAPID,
    pub ran_ue_ngap_id: RANUENGAPID,
    #[asn(optional_idx = 0)]
    pub ie_extensions: Option<UENGAPIDpairiEExtensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "CHOICE", lb = "0", ub = "2", extensible = false)]
pub enum UENGAPIDs {
    #[asn(key = 0, extended = false)]
    uENGAPIDpair(UENGAPIDpair),
    #[asn(key = 1, extended = false)]
    aMFUENGAPID(AMFUENGAPID),
    #[asn(key = 2, extended = false)]
    choiceExtensions(UENGAPIDschoiceExtensions),
}

#[derive(Debug, AperCodec)]
#[asn(type = "ENUMERATED", extensible = true, lb = "0", ub = "0")]
pub struct UEUPCIoTSupport(u8);
impl UEUPCIoTSupport {
    const SUPPORTED: u8 = 0u8;
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 3)]
pub struct UEassociatedLogicalNGconnectionItem {
    #[asn(optional_idx = 0)]
    pub amf_ue_ngap_id: Option<AMFUENGAPID>,
    #[asn(optional_idx = 1)]
    pub ran_ue_ngap_id: Option<RANUENGAPID>,
    #[asn(optional_idx = 2)]
    pub ie_extensions: Option<UEassociatedLogicalNGconnectionItemiEExtensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "65536")]
pub struct UEassociatedLogicalNGconnectionList(Vec<UEassociatedLogicalNGconnectionItem>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct UEAggregateMaximumBitRate {
    pub ue_aggregate_maximum_bit_rate_dl: BitRate,
    pub ue_aggregate_maximum_bit_rate_ul: BitRate,
    #[asn(optional_idx = 0)]
    pub ie_extensions: Option<UEAggregateMaximumBitRateiEExtensions>,
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
    pub ie_extensions: Option<UEContextResumeRequestTransferiEExtensions>,
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
    pub ie_extensions: Option<UEContextResumeResponseTransferiEExtensions>,
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
    pub ie_extensions: Option<UEContextSuspendRequestTransferiEExtensions>,
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
    choiceExtensions(UEHistoryInformationFromTheUEchoiceExtensions),
}

#[derive(Debug, AperCodec)]
#[asn(type = "CHOICE", lb = "0", ub = "1", extensible = false)]
pub enum UEIdentityIndexValue {
    #[asn(key = 0, extended = false)]
    indexLength10(BITSTRING88),
    #[asn(key = 1, extended = false)]
    choiceExtensions(UEIdentityIndexValuechoiceExtensions),
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
    fiveGSTMSI(FiveGSTMSI),
    #[asn(key = 1, extended = false)]
    choiceExtensions(UEPagingIdentitychoiceExtensions),
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
    pub ie_extensions: Option<UEPresenceInAreaOfInterestItemiEExtensions>,
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
    choiceExtensions(UERLFReportContainerchoiceExtensions),
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
    pub ue_radio_capability_for_paging_of_nr: Option<UERadioCapabilityForPagingOfNR>,
    #[asn(optional_idx = 1)]
    pub ue_radio_capability_for_paging_of_eutra: Option<UERadioCapabilityForPagingOfEUTRA>,
    #[asn(optional_idx = 2)]
    pub ie_extensions: Option<UERadioCapabilityForPagingiEExtensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "OCTET-STRING")]
pub struct UERadioCapabilityForPagingOfEUTRA(Vec<u8>);

#[derive(Debug, AperCodec)]
#[asn(type = "OCTET-STRING")]
pub struct UERadioCapabilityForPagingOfNBIoT(Vec<u8>);

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
    pub eutr_aencryption_algorithms: EUTRAencryptionAlgorithms,
    pub eutr_aintegrity_protection_algorithms: EUTRAintegrityProtectionAlgorithms,
    #[asn(optional_idx = 0)]
    pub ie_extensions: Option<UESecurityCapabilitiesiEExtensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true)]
pub struct UETNLABindingReleaseRequest {
    pub protocol_i_es: UETNLABindingReleaseRequestprotocolIEs,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct ULCPSecurityInformation {
    pub ul_nas_mac: ULNASMAC,
    pub ul_nas_count: ULNASCount,
    #[asn(optional_idx = 0)]
    pub ie_extensions: Option<ULCPSecurityInformationiEExtensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "BITSTRING", sz_extensible = false, sz_lb = "5", sz_ub = "5")]
pub struct ULNASCount(BitVec<Msb0, u8>);

#[derive(Debug, AperCodec)]
#[asn(type = "BITSTRING", sz_extensible = false, sz_lb = "16", sz_ub = "16")]
pub struct ULNASMAC(BitVec<Msb0, u8>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct ULNGUUPTNLModifyItem {
    pub ul_ngu_up_tnl_information: UPTransportLayerInformation,
    pub dl_ngu_up_tnl_information: UPTransportLayerInformation,
    #[asn(optional_idx = 0)]
    pub ie_extensions: Option<ULNGUUPTNLModifyItemiEExtensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "4")]
pub struct ULNGUUPTNLModifyList(Vec<ULNGUUPTNLModifyItem>);

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
    choiceExtensions(UPTransportLayerInformationchoiceExtensions),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct UPTransportLayerInformationItem {
    pub ngu_up_tnl_information: UPTransportLayerInformation,
    #[asn(optional_idx = 0)]
    pub ie_extensions: Option<UPTransportLayerInformationItemiEExtensions>,
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
    pub ie_extensions: Option<UPTransportLayerInformationPairItemiEExtensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "3")]
pub struct UPTransportLayerInformationPairList(Vec<UPTransportLayerInformationPairItem>);

#[derive(Debug, AperCodec)]
#[asn(type = "VisibleString")]
pub struct URIaddress(String);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 3)]
pub struct UnavailableGUAMIItem {
    pub guami: GUAMI,
    #[asn(optional_idx = 0)]
    pub timer_approach_for_guami_removal: Option<TimerApproachForGUAMIRemoval>,
    #[asn(optional_idx = 1)]
    pub backup_amf_name: Option<AMFName>,
    #[asn(optional_idx = 2)]
    pub ie_extensions: Option<UnavailableGUAMIItemiEExtensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "256")]
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
    choiceExtensions(UserLocationInformationchoiceExtensions),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 2)]
pub struct UserLocationInformationEUTRA {
    pub eutra_cgi: EUTRACGI,
    pub tai: TAI,
    #[asn(optional_idx = 0)]
    pub time_stamp: Option<TimeStamp>,
    #[asn(optional_idx = 1)]
    pub ie_extensions: Option<UserLocationInformationEUTRAiEExtensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct UserLocationInformationN3IWF {
    pub ip_address: TransportLayerAddress,
    pub port_number: PortNumber,
    #[asn(optional_idx = 0)]
    pub ie_extensions: Option<UserLocationInformationN3IWFiEExtensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 2)]
pub struct UserLocationInformationNR {
    pub nr_cgi: NRCGI,
    pub tai: TAI,
    #[asn(optional_idx = 0)]
    pub time_stamp: Option<TimeStamp>,
    #[asn(optional_idx = 1)]
    pub ie_extensions: Option<UserLocationInformationNRiEExtensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 2)]
pub struct UserLocationInformationTNGF {
    pub tnap_id: TNAPID,
    pub ip_address: TransportLayerAddress,
    #[asn(optional_idx = 0)]
    pub port_number: Option<PortNumber>,
    #[asn(optional_idx = 1)]
    pub ie_extensions: Option<UserLocationInformationTNGFiEExtensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 2)]
pub struct UserLocationInformationTWIF {
    pub twap_id: TWAPID,
    pub ip_address: TransportLayerAddress,
    #[asn(optional_idx = 0)]
    pub port_number: Option<PortNumber>,
    #[asn(optional_idx = 1)]
    pub ie_extensions: Option<UserLocationInformationTWIFiEExtensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "CHOICE", lb = "0", ub = "2", extensible = false)]
pub enum UserLocationInformationWAGF {
    #[asn(key = 0, extended = false)]
    globalLineID(GlobalLineID),
    #[asn(key = 1, extended = false)]
    hFCNodeID(HFCNodeID),
    #[asn(key = 2, extended = false)]
    choiceExtensions(UserLocationInformationWAGFchoiceExtensions),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct UserPlaneSecurityInformation {
    pub security_result: SecurityResult,
    pub security_indication: SecurityIndication,
    #[asn(optional_idx = 0)]
    pub ie_extensions: Option<UserPlaneSecurityInformationiEExtensions>,
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
pub struct VolumeTimedReportItem {
    pub start_time_stamp: OCTETSTRING89,
    pub end_time_stamp: OCTETSTRING90,
    pub usage_count_ul: INTEGER91,
    pub usage_count_dl: INTEGER92,
    #[asn(optional_idx = 0)]
    pub ie_extensions: Option<VolumeTimedReportItemiEExtensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "2")]
pub struct VolumeTimedReportList(Vec<VolumeTimedReportItem>);

#[derive(Debug, AperCodec)]
#[asn(type = "CHOICE", lb = "0", ub = "1", extensible = false)]
pub enum WAGFID {
    #[asn(key = 0, extended = false)]
    wAGFID(BITSTRING93),
    #[asn(key = 1, extended = false)]
    choiceExtensions(WAGFIDchoiceExtensions),
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
    pub ie_extensions: Option<WLANMeasConfigNameItemiEExtensions>,
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
    pub wlan_rssi: Option<ENUMERATED94>,
    #[asn(optional_idx = 2)]
    pub wlan_rtt: Option<ENUMERATED95>,
    #[asn(optional_idx = 3)]
    pub ie_extensions: Option<WLANMeasurementConfigurationiEExtensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "OCTET-STRING", sz_extensible = false, sz_lb = "1", sz_ub = "32")]
pub struct WLANName(Vec<u8>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct WUSAssistanceInformation {
    pub paging_probability_information: PagingProbabilityInformation,
    #[asn(optional_idx = 0)]
    pub ie_extensions: Option<WUSAssistanceInformationiEExtensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "OCTET-STRING", sz_extensible = false, sz_lb = "1", sz_ub = "1024")]
pub struct WarningAreaCoordinates(Vec<u8>);

#[derive(Debug, AperCodec)]
#[asn(type = "CHOICE", lb = "0", ub = "4", extensible = false)]
pub enum WarningAreaList {
    #[asn(key = 0, extended = false)]
    eUTRACGIListForWarning(EUTRACGIListForWarning),
    #[asn(key = 1, extended = false)]
    nRCGIListForWarning(NRCGIListForWarning),
    #[asn(key = 2, extended = false)]
    tAIListForWarning(TAIListForWarning),
    #[asn(key = 3, extended = false)]
    emergencyAreaIDList(EmergencyAreaIDList),
    #[asn(key = 4, extended = false)]
    choiceExtensions(WarningAreaListchoiceExtensions),
}

#[derive(Debug, AperCodec)]
#[asn(type = "OCTET-STRING", sz_extensible = false, sz_lb = "1", sz_ub = "9600")]
pub struct WarningMessageContents(Vec<u8>);

#[derive(Debug, AperCodec)]
#[asn(type = "OCTET-STRING", sz_extensible = false, sz_lb = "50", sz_ub = "50")]
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
pub struct XnExtTLAItem {
    #[asn(optional_idx = 0)]
    pub i_psec_tla: Option<TransportLayerAddress>,
    #[asn(optional_idx = 1)]
    pub gtp_tl_as: Option<XnGTPTLAs>,
    #[asn(optional_idx = 2)]
    pub ie_extensions: Option<XnExtTLAItemiEExtensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "16")]
pub struct XnExtTLAs(Vec<XnExtTLAItem>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "16")]
pub struct XnGTPTLAs(Vec<TransportLayerAddress>);

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
    pub ie_extensions: Option<XnTNLConfigurationInfoiEExtensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct AMFTNLAssociationSetupItemiEExtensions_Item {}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "65535")]
pub struct AMFTNLAssociationSetupItemiEExtensions(Vec<AMFTNLAssociationSetupItemiEExtensions_Item>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct AMFTNLAssociationToAddItemiEExtensions_Item {}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "65535")]
pub struct AMFTNLAssociationToAddItemiEExtensions(Vec<AMFTNLAssociationToAddItemiEExtensions_Item>);

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum AMFTNLAssociationToRemoveItemiEExtensions_ItemextensionValue {
    #[asn(key = 168)]
    idTNLAssociationTransportLayerAddressNGRAN(CPTransportLayerInformation),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct AMFTNLAssociationToRemoveItemiEExtensions_Item {
    #[asn(key_field = true)]
    pub id: ProtocolExtensionID,
    pub criticality: Criticality,
    pub extension_value: AMFTNLAssociationToRemoveItemiEExtensions_ItemextensionValue,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "65535")]
pub struct AMFTNLAssociationToRemoveItemiEExtensions(Vec<AMFTNLAssociationToRemoveItemiEExtensions_Item>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct AMFTNLAssociationToUpdateItemiEExtensions_Item {}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "65535")]
pub struct AMFTNLAssociationToUpdateItemiEExtensions(Vec<AMFTNLAssociationToUpdateItemiEExtensions_Item>);

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum AMFCPRelocationIndicationprotocolIEs_Itemvalue {
    #[asn(key = 10)]
    idAMFUENGAPID(AMFUENGAPID),
    #[asn(key = 0)]
    idAllowedNSSAI(AllowedNSSAI),
    #[asn(key = 85)]
    idRANUENGAPID(RANUENGAPID),
    #[asn(key = 148)]
    idSNSSAI(SNSSAI),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct AMFCPRelocationIndicationprotocolIEs_Item {
    #[asn(key_field = true)]
    pub id: ProtocolIEID,
    pub criticality: Criticality,
    pub value: AMFCPRelocationIndicationprotocolIEs_Itemvalue,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "0", sz_ub = "65535")]
pub struct AMFCPRelocationIndicationprotocolIEs(Vec<AMFCPRelocationIndicationprotocolIEs_Item>);

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum AMFConfigurationUpdateprotocolIEs_Itemvalue {
    #[asn(key = 6)]
    idAMFTNLAssociationToAddList(AMFTNLAssociationToAddList),
    #[asn(key = 7)]
    idAMFTNLAssociationToRemoveList(AMFTNLAssociationToRemoveList),
    #[asn(key = 8)]
    idAMFTNLAssociationToUpdateList(AMFTNLAssociationToUpdateList),
    #[asn(key = 1)]
    idAMFName(AMFName),
    #[asn(key = 274)]
    idExtendedAMFName(ExtendedAMFName),
    #[asn(key = 80)]
    idPLMNSupportList(PLMNSupportList),
    #[asn(key = 86)]
    idRelativeAMFCapacity(RelativeAMFCapacity),
    #[asn(key = 96)]
    idServedGUAMIList(ServedGUAMIList),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct AMFConfigurationUpdateprotocolIEs_Item {
    #[asn(key_field = true)]
    pub id: ProtocolIEID,
    pub criticality: Criticality,
    pub value: AMFConfigurationUpdateprotocolIEs_Itemvalue,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "0", sz_ub = "65535")]
pub struct AMFConfigurationUpdateprotocolIEs(Vec<AMFConfigurationUpdateprotocolIEs_Item>);

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum AMFConfigurationUpdateAcknowledgeprotocolIEs_Itemvalue {
    #[asn(key = 4)]
    idAMFTNLAssociationFailedToSetupList(TNLAssociationList),
    #[asn(key = 5)]
    idAMFTNLAssociationSetupList(AMFTNLAssociationSetupList),
    #[asn(key = 19)]
    idCriticalityDiagnostics(CriticalityDiagnostics),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct AMFConfigurationUpdateAcknowledgeprotocolIEs_Item {
    #[asn(key_field = true)]
    pub id: ProtocolIEID,
    pub criticality: Criticality,
    pub value: AMFConfigurationUpdateAcknowledgeprotocolIEs_Itemvalue,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "0", sz_ub = "65535")]
pub struct AMFConfigurationUpdateAcknowledgeprotocolIEs(Vec<AMFConfigurationUpdateAcknowledgeprotocolIEs_Item>);

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum AMFConfigurationUpdateFailureprotocolIEs_Itemvalue {
    #[asn(key = 15)]
    idCause(Cause),
    #[asn(key = 19)]
    idCriticalityDiagnostics(CriticalityDiagnostics),
    #[asn(key = 107)]
    idTimeToWait(TimeToWait),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct AMFConfigurationUpdateFailureprotocolIEs_Item {
    #[asn(key_field = true)]
    pub id: ProtocolIEID,
    pub criticality: Criticality,
    pub value: AMFConfigurationUpdateFailureprotocolIEs_Itemvalue,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "0", sz_ub = "65535")]
pub struct AMFConfigurationUpdateFailureprotocolIEs(Vec<AMFConfigurationUpdateFailureprotocolIEs_Item>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct AMFPagingTargetchoiceExtensions {}

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum AMFStatusIndicationprotocolIEs_Itemvalue {
    #[asn(key = 120)]
    idUnavailableGUAMIList(UnavailableGUAMIList),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct AMFStatusIndicationprotocolIEs_Item {
    #[asn(key_field = true)]
    pub id: ProtocolIEID,
    pub criticality: Criticality,
    pub value: AMFStatusIndicationprotocolIEs_Itemvalue,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "0", sz_ub = "65535")]
pub struct AMFStatusIndicationprotocolIEs(Vec<AMFStatusIndicationprotocolIEs_Item>);

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum AdditionalDLUPTNLInformationForHOItemiEExtensions_ItemextensionValue {
    #[asn(key = 183)]
    idAdditionalRedundantDLNGUUPTNLInformation(UPTransportLayerInformation),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct AdditionalDLUPTNLInformationForHOItemiEExtensions_Item {
    #[asn(key_field = true)]
    pub id: ProtocolExtensionID,
    pub criticality: Criticality,
    pub extension_value: AdditionalDLUPTNLInformationForHOItemiEExtensions_ItemextensionValue,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "65535")]
pub struct AdditionalDLUPTNLInformationForHOItemiEExtensions(Vec<AdditionalDLUPTNLInformationForHOItemiEExtensions_Item>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct AllocationAndRetentionPriorityiEExtensions_Item {}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "65535")]
pub struct AllocationAndRetentionPriorityiEExtensions(Vec<AllocationAndRetentionPriorityiEExtensions_Item>);

#[derive(Debug, AperCodec)]
#[asn(type = "ENUMERATED", extensible = true, lb = "0", ub = "1")]
pub struct ENUMERATED2(u8);
impl ENUMERATED2 {
    const RESTRICTED: u8 = 0u8;
    const NOT_RESTRICTED: u8 = 1u8;
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct AllowedPNINPNItemiEExtensions_Item {}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "65535")]
pub struct AllowedPNINPNItemiEExtensions(Vec<AllowedPNINPNItemiEExtensions_Item>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct AllowedNSSAIItemiEExtensions_Item {}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "65535")]
pub struct AllowedNSSAIItemiEExtensions(Vec<AllowedNSSAIItemiEExtensions_Item>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct AlternativeQoSParaSetItemiEExtensions_Item {}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "65535")]
pub struct AlternativeQoSParaSetItemiEExtensions(Vec<AlternativeQoSParaSetItemiEExtensions_Item>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct AreaOfInterestiEExtensions_Item {}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "65535")]
pub struct AreaOfInterestiEExtensions(Vec<AreaOfInterestiEExtensions_Item>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct AreaOfInterestCellItemiEExtensions_Item {}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "65535")]
pub struct AreaOfInterestCellItemiEExtensions(Vec<AreaOfInterestCellItemiEExtensions_Item>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct AreaOfInterestItemiEExtensions_Item {}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "65535")]
pub struct AreaOfInterestItemiEExtensions(Vec<AreaOfInterestItemiEExtensions_Item>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct AreaOfInterestRANNodeItemiEExtensions_Item {}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "65535")]
pub struct AreaOfInterestRANNodeItemiEExtensions(Vec<AreaOfInterestRANNodeItemiEExtensions_Item>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct AreaOfInterestTAIItemiEExtensions_Item {}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "65535")]
pub struct AreaOfInterestTAIItemiEExtensions(Vec<AreaOfInterestTAIItemiEExtensions_Item>);

#[derive(Debug, AperCodec)]
#[asn(type = "NULL")]
pub struct NULL3;

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct AreaScopeOfMDTEUTRAchoiceExtensions {}

#[derive(Debug, AperCodec)]
#[asn(type = "NULL")]
pub struct NULL4;

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct AreaScopeOfMDTNRchoiceExtensions {}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct AreaScopeOfNeighCellsItemiEExtensions_Item {}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "65535")]
pub struct AreaScopeOfNeighCellsItemiEExtensions(Vec<AreaScopeOfNeighCellsItemiEExtensions_Item>);

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum AssistanceDataForPagingiEExtensions_ItemextensionValue {
    #[asn(key = 260)]
    idNPNPagingAssistanceInformation(NPNPagingAssistanceInformation),
    #[asn(key = 207)]
    idPagingAssisDataforCEcapabUE(PagingAssisDataforCEcapabUE),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct AssistanceDataForPagingiEExtensions_Item {
    #[asn(key_field = true)]
    pub id: ProtocolExtensionID,
    pub criticality: Criticality,
    pub extension_value: AssistanceDataForPagingiEExtensions_ItemextensionValue,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "65535")]
pub struct AssistanceDataForPagingiEExtensions(Vec<AssistanceDataForPagingiEExtensions_Item>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct AssistanceDataForRecommendedCellsiEExtensions_Item {}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "65535")]
pub struct AssistanceDataForRecommendedCellsiEExtensions(Vec<AssistanceDataForRecommendedCellsiEExtensions_Item>);

#[derive(Debug, AperCodec)]
#[asn(type = "ENUMERATED", extensible = true, lb = "0", ub = "1")]
pub struct ENUMERATED5(u8);
impl ENUMERATED5 {
    const UL: u8 = 0u8;
    const DL: u8 = 1u8;
}

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum AssociatedQosFlowItemiEExtensions_ItemextensionValue {
    #[asn(key = 221)]
    idCurrentQoSParaSetIndex(AlternativeQoSParaSetIndex),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct AssociatedQosFlowItemiEExtensions_Item {
    #[asn(key_field = true)]
    pub id: ProtocolExtensionID,
    pub criticality: Criticality,
    pub extension_value: AssociatedQosFlowItemiEExtensions_ItemextensionValue,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "65535")]
pub struct AssociatedQosFlowItemiEExtensions(Vec<AssociatedQosFlowItemiEExtensions_Item>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct BluetoothMeasConfigNameItemiEExtensions_Item {}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "65535")]
pub struct BluetoothMeasConfigNameItemiEExtensions(Vec<BluetoothMeasConfigNameItemiEExtensions_Item>);

#[derive(Debug, AperCodec)]
#[asn(type = "ENUMERATED", extensible = true, lb = "0", ub = "0")]
pub struct ENUMERATED6(u8);
impl ENUMERATED6 {
    const TRUE: u8 = 0u8;
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct BluetoothMeasurementConfigurationiEExtensions_Item {}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "65535")]
pub struct BluetoothMeasurementConfigurationiEExtensions(Vec<BluetoothMeasurementConfigurationiEExtensions_Item>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct BroadcastCancelledAreaListchoiceExtensions {}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct BroadcastCompletedAreaListchoiceExtensions {}

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum BroadcastPLMNItemiEExtensions_ItemextensionValue {
    #[asn(key = 271)]
    idExtendedTAISliceSupportList(ExtendedSliceSupportList),
    #[asn(key = 258)]
    idNPNSupport(NPNSupport),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct BroadcastPLMNItemiEExtensions_Item {
    #[asn(key_field = true)]
    pub id: ProtocolExtensionID,
    pub criticality: Criticality,
    pub extension_value: BroadcastPLMNItemiEExtensions_ItemextensionValue,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "65535")]
pub struct BroadcastPLMNItemiEExtensions(Vec<BroadcastPLMNItemiEExtensions_Item>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct CNAssistedRANTuningiEExtensions_Item {}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "65535")]
pub struct CNAssistedRANTuningiEExtensions(Vec<CNAssistedRANTuningiEExtensions_Item>);

#[derive(Debug, AperCodec)]
#[asn(type = "ENUMERATED", extensible = true, lb = "0", ub = "1")]
pub struct ENUMERATED7(u8);
impl ENUMERATED7 {
    const EPC_FORBIDDEN: u8 = 0u8;
    const FIVE_GC_FORBIDDEN: u8 = 1u8;
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct CNTypeRestrictionsForEquivalentItemiEExtensions_Item {}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "65535")]
pub struct CNTypeRestrictionsForEquivalentItemiEExtensions(Vec<CNTypeRestrictionsForEquivalentItemiEExtensions_Item>);

#[derive(Debug, AperCodec)]
#[asn(type = "INTEGER", lb = "0", ub = "4095")]
pub struct INTEGER8(u16);

#[derive(Debug, AperCodec)]
#[asn(type = "INTEGER", lb = "0", ub = "1048575")]
pub struct INTEGER9(u32);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct COUNTValueForPDCPSN12iEExtensions_Item {}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "65535")]
pub struct COUNTValueForPDCPSN12iEExtensions(Vec<COUNTValueForPDCPSN12iEExtensions_Item>);

#[derive(Debug, AperCodec)]
#[asn(type = "INTEGER", lb = "0", ub = "262143")]
pub struct INTEGER10(u32);

#[derive(Debug, AperCodec)]
#[asn(type = "INTEGER", lb = "0", ub = "16383")]
pub struct INTEGER11(u16);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct COUNTValueForPDCPSN18iEExtensions_Item {}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "65535")]
pub struct COUNTValueForPDCPSN18iEExtensions(Vec<COUNTValueForPDCPSN18iEExtensions_Item>);

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum CPTransportLayerInformationchoiceExtensionsvalue {
    #[asn(key = 169)]
    idEndpointIPAddressAndPort(EndpointIPAddressAndPort),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct CPTransportLayerInformationchoiceExtensions {
    #[asn(key_field = true)]
    pub id: ProtocolIEID,
    pub criticality: Criticality,
    pub value: CPTransportLayerInformationchoiceExtensionsvalue,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct CancelledCellsInEAIEUTRAItemiEExtensions_Item {}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "65535")]
pub struct CancelledCellsInEAIEUTRAItemiEExtensions(Vec<CancelledCellsInEAIEUTRAItemiEExtensions_Item>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct CancelledCellsInEAINRItemiEExtensions_Item {}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "65535")]
pub struct CancelledCellsInEAINRItemiEExtensions(Vec<CancelledCellsInEAINRItemiEExtensions_Item>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct CancelledCellsInTAIEUTRAItemiEExtensions_Item {}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "65535")]
pub struct CancelledCellsInTAIEUTRAItemiEExtensions(Vec<CancelledCellsInTAIEUTRAItemiEExtensions_Item>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct CancelledCellsInTAINRItemiEExtensions_Item {}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "65535")]
pub struct CancelledCellsInTAINRItemiEExtensions(Vec<CancelledCellsInTAINRItemiEExtensions_Item>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct CandidateCellchoiceExtensions {}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct CandidateCellIDiEExtensions_Item {}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "65535")]
pub struct CandidateCellIDiEExtensions(Vec<CandidateCellIDiEExtensions_Item>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct CandidateCellItemiEExtensions_Item {}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "65535")]
pub struct CandidateCellItemiEExtensions(Vec<CandidateCellItemiEExtensions_Item>);

#[derive(Debug, AperCodec)]
#[asn(type = "INTEGER", lb = "0", ub = "1007", extensible = true)]
pub struct INTEGER12(u16);

#[derive(Debug, AperCodec)]
#[asn(type = "INTEGER", lb = "0", ub = "3279165")]
pub struct INTEGER13(u32);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct CandidatePCIiEExtensions_Item {}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "65535")]
pub struct CandidatePCIiEExtensions(Vec<CandidatePCIiEExtensions_Item>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct CausechoiceExtensions {}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct CellCAGInformationiEExtensions_Item {}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "65535")]
pub struct CellCAGInformationiEExtensions(Vec<CellCAGInformationiEExtensions_Item>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct CellBasedMDTEUTRAiEExtensions_Item {}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "65535")]
pub struct CellBasedMDTEUTRAiEExtensions(Vec<CellBasedMDTEUTRAiEExtensions_Item>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct CellBasedMDTNRiEExtensions_Item {}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "65535")]
pub struct CellBasedMDTNRiEExtensions(Vec<CellBasedMDTNRiEExtensions_Item>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct CellIDBroadcastEUTRAItemiEExtensions_Item {}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "65535")]
pub struct CellIDBroadcastEUTRAItemiEExtensions(Vec<CellIDBroadcastEUTRAItemiEExtensions_Item>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct CellIDBroadcastNRItemiEExtensions_Item {}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "65535")]
pub struct CellIDBroadcastNRItemiEExtensions(Vec<CellIDBroadcastNRItemiEExtensions_Item>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct CellIDCancelledEUTRAItemiEExtensions_Item {}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "65535")]
pub struct CellIDCancelledEUTRAItemiEExtensions(Vec<CellIDCancelledEUTRAItemiEExtensions_Item>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct CellIDCancelledNRItemiEExtensions_Item {}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "65535")]
pub struct CellIDCancelledNRItemiEExtensions(Vec<CellIDCancelledNRItemiEExtensions_Item>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct CellIDListForRestartchoiceExtensions {}

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum CellTrafficTraceprotocolIEs_Itemvalue {
    #[asn(key = 10)]
    idAMFUENGAPID(AMFUENGAPID),
    #[asn(key = 43)]
    idNGRANCGI(NGRANCGI),
    #[asn(key = 44)]
    idNGRANTraceID(NGRANTraceID),
    #[asn(key = 256)]
    idPrivacyIndicator(PrivacyIndicator),
    #[asn(key = 85)]
    idRANUENGAPID(RANUENGAPID),
    #[asn(key = 109)]
    idTraceCollectionEntityIPAddress(TransportLayerAddress),
    #[asn(key = 257)]
    idTraceCollectionEntityURI(URIaddress),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct CellTrafficTraceprotocolIEs_Item {
    #[asn(key_field = true)]
    pub id: ProtocolIEID,
    pub criticality: Criticality,
    pub value: CellTrafficTraceprotocolIEs_Itemvalue,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "0", sz_ub = "65535")]
pub struct CellTrafficTraceprotocolIEs(Vec<CellTrafficTraceprotocolIEs_Item>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct CellTypeiEExtensions_Item {}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "65535")]
pub struct CellTypeiEExtensions(Vec<CellTypeiEExtensions_Item>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct CompletedCellsInEAIEUTRAItemiEExtensions_Item {}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "65535")]
pub struct CompletedCellsInEAIEUTRAItemiEExtensions(Vec<CompletedCellsInEAIEUTRAItemiEExtensions_Item>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct CompletedCellsInEAINRItemiEExtensions_Item {}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "65535")]
pub struct CompletedCellsInEAINRItemiEExtensions(Vec<CompletedCellsInEAINRItemiEExtensions_Item>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct CompletedCellsInTAIEUTRAItemiEExtensions_Item {}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "65535")]
pub struct CompletedCellsInTAIEUTRAItemiEExtensions(Vec<CompletedCellsInTAIEUTRAItemiEExtensions_Item>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct CompletedCellsInTAINRItemiEExtensions_Item {}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "65535")]
pub struct CompletedCellsInTAINRItemiEExtensions(Vec<CompletedCellsInTAINRItemiEExtensions_Item>);

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum ConnectionEstablishmentIndicationprotocolIEs_Itemvalue {
    #[asn(key = 10)]
    idAMFUENGAPID(AMFUENGAPID),
    #[asn(key = 0)]
    idAllowedNSSAI(AllowedNSSAI),
    #[asn(key = 222)]
    idCEmodeBrestricted(CEmodeBrestricted),
    #[asn(key = 212)]
    idDLCPSecurityInformation(DLCPSecurityInformation),
    #[asn(key = 226)]
    idEndIndication(EndIndication),
    #[asn(key = 205)]
    idEnhancedCoverageRestriction(EnhancedCoverageRestriction),
    #[asn(key = 210)]
    idNBIoTUEPriority(NBIoTUEPriority),
    #[asn(key = 85)]
    idRANUENGAPID(RANUENGAPID),
    #[asn(key = 148)]
    idSNSSAI(SNSSAI),
    #[asn(key = 209)]
    idUEDifferentiationInfo(UEDifferentiationInfo),
    #[asn(key = 117)]
    idUERadioCapability(UERadioCapability),
    #[asn(key = 264)]
    idUERadioCapabilityID(UERadioCapabilityID),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct ConnectionEstablishmentIndicationprotocolIEs_Item {
    #[asn(key_field = true)]
    pub id: ProtocolIEID,
    pub criticality: Criticality,
    pub value: ConnectionEstablishmentIndicationprotocolIEs_Itemvalue,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "0", sz_ub = "65535")]
pub struct ConnectionEstablishmentIndicationprotocolIEs(Vec<ConnectionEstablishmentIndicationprotocolIEs_Item>);

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum CoreNetworkAssistanceInformationForInactiveiEExtensions_ItemextensionValue {
    #[asn(key = 280)]
    idExtendedUEIdentityIndexValue(ExtendedUEIdentityIndexValue),
    #[asn(key = 223)]
    idPagingeDRXInformation(PagingeDRXInformation),
    #[asn(key = 118)]
    idUERadioCapabilityForPaging(UERadioCapabilityForPaging),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct CoreNetworkAssistanceInformationForInactiveiEExtensions_Item {
    #[asn(key_field = true)]
    pub id: ProtocolExtensionID,
    pub criticality: Criticality,
    pub extension_value: CoreNetworkAssistanceInformationForInactiveiEExtensions_ItemextensionValue,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "65535")]
pub struct CoreNetworkAssistanceInformationForInactiveiEExtensions(Vec<CoreNetworkAssistanceInformationForInactiveiEExtensions_Item>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct CriticalityDiagnosticsiEExtensions_Item {}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "65535")]
pub struct CriticalityDiagnosticsiEExtensions(Vec<CriticalityDiagnosticsiEExtensions_Item>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct CriticalityDiagnosticsIEItemiEExtensions_Item {}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "65535")]
pub struct CriticalityDiagnosticsIEItemiEExtensions(Vec<CriticalityDiagnosticsIEItemiEExtensions_Item>);

#[derive(Debug, AperCodec)]
#[asn(type = "ENUMERATED", extensible = true, lb = "0", ub = "0")]
pub struct ENUMERATED14(u8);
impl ENUMERATED14 {
    const DAPS_HO_REQUIRED: u8 = 0u8;
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct DAPSRequestInfoiEExtensions_Item {}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "65535")]
pub struct DAPSRequestInfoiEExtensions(Vec<DAPSRequestInfoiEExtensions_Item>);

#[derive(Debug, AperCodec)]
#[asn(type = "ENUMERATED", extensible = true, lb = "0", ub = "1")]
pub struct ENUMERATED15(u8);
impl ENUMERATED15 {
    const DAPS_HO_ACCEPTED: u8 = 0u8;
    const DAPS_HO_NOT_ACCEPTED: u8 = 1u8;
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct DAPSResponseInfoiEExtensions_Item {}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "65535")]
pub struct DAPSResponseInfoiEExtensions(Vec<DAPSResponseInfoiEExtensions_Item>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct DAPSResponseInfoItemiEExtension_Item {}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "65535")]
pub struct DAPSResponseInfoItemiEExtension(Vec<DAPSResponseInfoItemiEExtension_Item>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct DLCPSecurityInformationiEExtensions_Item {}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "65535")]
pub struct DLCPSecurityInformationiEExtensions(Vec<DLCPSecurityInformationiEExtensions_Item>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct DRBStatusDLchoiceExtensions {}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct DRBStatusDL12iEExtension_Item {}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "65535")]
pub struct DRBStatusDL12iEExtension(Vec<DRBStatusDL12iEExtension_Item>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct DRBStatusDL18iEExtension_Item {}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "65535")]
pub struct DRBStatusDL18iEExtension(Vec<DRBStatusDL18iEExtension_Item>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct DRBStatusULchoiceExtensions {}

#[derive(Debug, AperCodec)]
#[asn(type = "BITSTRING", sz_extensible = false, sz_lb = "1", sz_ub = "2048")]
pub struct BITSTRING16(BitVec<Msb0, u8>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct DRBStatusUL12iEExtension_Item {}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "65535")]
pub struct DRBStatusUL12iEExtension(Vec<DRBStatusUL12iEExtension_Item>);

#[derive(Debug, AperCodec)]
#[asn(type = "BITSTRING", sz_extensible = false, sz_lb = "1", sz_ub = "131072")]
pub struct BITSTRING17(BitVec<Msb0, u8>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct DRBStatusUL18iEExtension_Item {}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "65535")]
pub struct DRBStatusUL18iEExtension(Vec<DRBStatusUL18iEExtension_Item>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct DRBsSubjectToEarlyStatusTransferItemiEExtension_Item {}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "65535")]
pub struct DRBsSubjectToEarlyStatusTransferItemiEExtension(Vec<DRBsSubjectToEarlyStatusTransferItemiEExtension_Item>);

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum DRBsSubjectToStatusTransferItemiEExtension_ItemextensionValue {
    #[asn(key = 159)]
    idOldAssociatedQosFlowListULendmarkerexpected(AssociatedQosFlowList),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct DRBsSubjectToStatusTransferItemiEExtension_Item {
    #[asn(key_field = true)]
    pub id: ProtocolExtensionID,
    pub criticality: Criticality,
    pub extension_value: DRBsSubjectToStatusTransferItemiEExtension_ItemextensionValue,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "65535")]
pub struct DRBsSubjectToStatusTransferItemiEExtension(Vec<DRBsSubjectToStatusTransferItemiEExtension_Item>);

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum DRBsToQosFlowsMappingItemiEExtensions_ItemextensionValue {
    #[asn(key = 266)]
    idDAPSRequestInfo(DAPSRequestInfo),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct DRBsToQosFlowsMappingItemiEExtensions_Item {
    #[asn(key_field = true)]
    pub id: ProtocolExtensionID,
    pub criticality: Criticality,
    pub extension_value: DRBsToQosFlowsMappingItemiEExtensions_ItemextensionValue,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "65535")]
pub struct DRBsToQosFlowsMappingItemiEExtensions(Vec<DRBsToQosFlowsMappingItemiEExtensions_Item>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct DataForwardingResponseDRBItemiEExtensions_Item {}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "65535")]
pub struct DataForwardingResponseDRBItemiEExtensions(Vec<DataForwardingResponseDRBItemiEExtensions_Item>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct DataForwardingResponseERABListItemiEExtensions_Item {}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "65535")]
pub struct DataForwardingResponseERABListItemiEExtensions(Vec<DataForwardingResponseERABListItemiEExtensions_Item>);

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum DeactivateTraceprotocolIEs_Itemvalue {
    #[asn(key = 10)]
    idAMFUENGAPID(AMFUENGAPID),
    #[asn(key = 44)]
    idNGRANTraceID(NGRANTraceID),
    #[asn(key = 85)]
    idRANUENGAPID(RANUENGAPID),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct DeactivateTraceprotocolIEs_Item {
    #[asn(key_field = true)]
    pub id: ProtocolIEID,
    pub criticality: Criticality,
    pub value: DeactivateTraceprotocolIEs_Itemvalue,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "0", sz_ub = "65535")]
pub struct DeactivateTraceprotocolIEs(Vec<DeactivateTraceprotocolIEs_Item>);

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum DownlinkNASTransportprotocolIEs_Itemvalue {
    #[asn(key = 10)]
    idAMFUENGAPID(AMFUENGAPID),
    #[asn(key = 0)]
    idAllowedNSSAI(AllowedNSSAI),
    #[asn(key = 222)]
    idCEmodeBrestricted(CEmodeBrestricted),
    #[asn(key = 226)]
    idEndIndication(EndIndication),
    #[asn(key = 205)]
    idEnhancedCoverageRestriction(EnhancedCoverageRestriction),
    #[asn(key = 206)]
    idExtendedConnectedTime(ExtendedConnectedTime),
    #[asn(key = 31)]
    idIndexToRFSP(IndexToRFSP),
    #[asn(key = 36)]
    idMobilityRestrictionList(MobilityRestrictionList),
    #[asn(key = 38)]
    idNASPDU(NASPDU),
    #[asn(key = 48)]
    idOldAMF(AMFName),
    #[asn(key = 85)]
    idRANUENGAPID(RANUENGAPID),
    #[asn(key = 83)]
    idRANPagingPriority(RANPagingPriority),
    #[asn(key = 177)]
    idSRVCCOperationPossible(SRVCCOperationPossible),
    #[asn(key = 209)]
    idUEDifferentiationInfo(UEDifferentiationInfo),
    #[asn(key = 110)]
    idUEAggregateMaximumBitRate(UEAggregateMaximumBitRate),
    #[asn(key = 228)]
    idUECapabilityInfoRequest(UECapabilityInfoRequest),
    #[asn(key = 117)]
    idUERadioCapability(UERadioCapability),
    #[asn(key = 264)]
    idUERadioCapabilityID(UERadioCapabilityID),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct DownlinkNASTransportprotocolIEs_Item {
    #[asn(key_field = true)]
    pub id: ProtocolIEID,
    pub criticality: Criticality,
    pub value: DownlinkNASTransportprotocolIEs_Itemvalue,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "0", sz_ub = "65535")]
pub struct DownlinkNASTransportprotocolIEs(Vec<DownlinkNASTransportprotocolIEs_Item>);

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum DownlinkNonUEAssociatedNRPPaTransportprotocolIEs_Itemvalue {
    #[asn(key = 46)]
    idNRPPaPDU(NRPPaPDU),
    #[asn(key = 89)]
    idRoutingID(RoutingID),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct DownlinkNonUEAssociatedNRPPaTransportprotocolIEs_Item {
    #[asn(key_field = true)]
    pub id: ProtocolIEID,
    pub criticality: Criticality,
    pub value: DownlinkNonUEAssociatedNRPPaTransportprotocolIEs_Itemvalue,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "0", sz_ub = "65535")]
pub struct DownlinkNonUEAssociatedNRPPaTransportprotocolIEs(Vec<DownlinkNonUEAssociatedNRPPaTransportprotocolIEs_Item>);

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum DownlinkRANConfigurationTransferprotocolIEs_Itemvalue {
    #[asn(key = 157)]
    idENDCSONConfigurationTransferDL(ENDCSONConfigurationTransfer),
    #[asn(key = 250)]
    idIntersystemSONConfigurationTransferDL(IntersystemSONConfigurationTransfer),
    #[asn(key = 98)]
    idSONConfigurationTransferDL(SONConfigurationTransfer),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct DownlinkRANConfigurationTransferprotocolIEs_Item {
    #[asn(key_field = true)]
    pub id: ProtocolIEID,
    pub criticality: Criticality,
    pub value: DownlinkRANConfigurationTransferprotocolIEs_Itemvalue,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "0", sz_ub = "65535")]
pub struct DownlinkRANConfigurationTransferprotocolIEs(Vec<DownlinkRANConfigurationTransferprotocolIEs_Item>);

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum DownlinkRANEarlyStatusTransferprotocolIEs_Itemvalue {
    #[asn(key = 10)]
    idAMFUENGAPID(AMFUENGAPID),
    #[asn(key = 268)]
    idEarlyStatusTransferTransparentContainer(EarlyStatusTransferTransparentContainer),
    #[asn(key = 85)]
    idRANUENGAPID(RANUENGAPID),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct DownlinkRANEarlyStatusTransferprotocolIEs_Item {
    #[asn(key_field = true)]
    pub id: ProtocolIEID,
    pub criticality: Criticality,
    pub value: DownlinkRANEarlyStatusTransferprotocolIEs_Itemvalue,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "0", sz_ub = "65535")]
pub struct DownlinkRANEarlyStatusTransferprotocolIEs(Vec<DownlinkRANEarlyStatusTransferprotocolIEs_Item>);

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum DownlinkRANStatusTransferprotocolIEs_Itemvalue {
    #[asn(key = 10)]
    idAMFUENGAPID(AMFUENGAPID),
    #[asn(key = 85)]
    idRANUENGAPID(RANUENGAPID),
    #[asn(key = 84)]
    idRANStatusTransferTransparentContainer(RANStatusTransferTransparentContainer),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct DownlinkRANStatusTransferprotocolIEs_Item {
    #[asn(key_field = true)]
    pub id: ProtocolIEID,
    pub criticality: Criticality,
    pub value: DownlinkRANStatusTransferprotocolIEs_Itemvalue,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "0", sz_ub = "65535")]
pub struct DownlinkRANStatusTransferprotocolIEs(Vec<DownlinkRANStatusTransferprotocolIEs_Item>);

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum DownlinkRIMInformationTransferprotocolIEs_Itemvalue {
    #[asn(key = 175)]
    idRIMInformationTransfer(RIMInformationTransfer),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct DownlinkRIMInformationTransferprotocolIEs_Item {
    #[asn(key_field = true)]
    pub id: ProtocolIEID,
    pub criticality: Criticality,
    pub value: DownlinkRIMInformationTransferprotocolIEs_Itemvalue,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "0", sz_ub = "65535")]
pub struct DownlinkRIMInformationTransferprotocolIEs(Vec<DownlinkRIMInformationTransferprotocolIEs_Item>);

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum DownlinkUEAssociatedNRPPaTransportprotocolIEs_Itemvalue {
    #[asn(key = 10)]
    idAMFUENGAPID(AMFUENGAPID),
    #[asn(key = 46)]
    idNRPPaPDU(NRPPaPDU),
    #[asn(key = 85)]
    idRANUENGAPID(RANUENGAPID),
    #[asn(key = 89)]
    idRoutingID(RoutingID),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct DownlinkUEAssociatedNRPPaTransportprotocolIEs_Item {
    #[asn(key_field = true)]
    pub id: ProtocolIEID,
    pub criticality: Criticality,
    pub value: DownlinkUEAssociatedNRPPaTransportprotocolIEs_Itemvalue,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "0", sz_ub = "65535")]
pub struct DownlinkUEAssociatedNRPPaTransportprotocolIEs(Vec<DownlinkUEAssociatedNRPPaTransportprotocolIEs_Item>);

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum Dynamic5QIDescriptoriEExtensions_ItemextensionValue {
    #[asn(key = 187)]
    idCNPacketDelayBudgetDL(ExtendedPacketDelayBudget),
    #[asn(key = 188)]
    idCNPacketDelayBudgetUL(ExtendedPacketDelayBudget),
    #[asn(key = 189)]
    idExtendedPacketDelayBudget(ExtendedPacketDelayBudget),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct Dynamic5QIDescriptoriEExtensions_Item {
    #[asn(key_field = true)]
    pub id: ProtocolExtensionID,
    pub criticality: Criticality,
    pub extension_value: Dynamic5QIDescriptoriEExtensions_ItemextensionValue,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "65535")]
pub struct Dynamic5QIDescriptoriEExtensions(Vec<Dynamic5QIDescriptoriEExtensions_Item>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct ERABInformationItemiEExtensions_Item {}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "65535")]
pub struct ERABInformationItemiEExtensions(Vec<ERABInformationItemiEExtensions_Item>);

#[derive(Debug, AperCodec)]
#[asn(type = "BITSTRING", sz_extensible = false, sz_lb = "20", sz_ub = "20")]
pub struct BITSTRING18(BitVec<Msb0, u8>);

#[derive(Debug, AperCodec)]
#[asn(type = "BITSTRING", sz_extensible = false, sz_lb = "28", sz_ub = "28")]
pub struct BITSTRING19(BitVec<Msb0, u8>);

#[derive(Debug, AperCodec)]
#[asn(type = "BITSTRING", sz_extensible = false, sz_lb = "18", sz_ub = "18")]
pub struct BITSTRING20(BitVec<Msb0, u8>);

#[derive(Debug, AperCodec)]
#[asn(type = "BITSTRING", sz_extensible = false, sz_lb = "21", sz_ub = "21")]
pub struct BITSTRING21(BitVec<Msb0, u8>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct ENBIDchoiceExtensions {}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct EPSTAIiEExtensions_Item {}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "65535")]
pub struct EPSTAIiEExtensions(Vec<EPSTAIiEExtensions_Item>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct EUTRACGIiEExtensions_Item {}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "65535")]
pub struct EUTRACGIiEExtensions(Vec<EUTRACGIiEExtensions_Item>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct EarlyStatusTransferTransparentContaineriEExtensions_Item {}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "65535")]
pub struct EarlyStatusTransferTransparentContaineriEExtensions(Vec<EarlyStatusTransferTransparentContaineriEExtensions_Item>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct EmergencyAreaIDBroadcastEUTRAItemiEExtensions_Item {}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "65535")]
pub struct EmergencyAreaIDBroadcastEUTRAItemiEExtensions(Vec<EmergencyAreaIDBroadcastEUTRAItemiEExtensions_Item>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct EmergencyAreaIDBroadcastNRItemiEExtensions_Item {}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "65535")]
pub struct EmergencyAreaIDBroadcastNRItemiEExtensions(Vec<EmergencyAreaIDBroadcastNRItemiEExtensions_Item>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct EmergencyAreaIDCancelledEUTRAItemiEExtensions_Item {}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "65535")]
pub struct EmergencyAreaIDCancelledEUTRAItemiEExtensions(Vec<EmergencyAreaIDCancelledEUTRAItemiEExtensions_Item>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct EmergencyAreaIDCancelledNRItemiEExtensions_Item {}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "65535")]
pub struct EmergencyAreaIDCancelledNRItemiEExtensions(Vec<EmergencyAreaIDCancelledNRItemiEExtensions_Item>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct EmergencyFallbackIndicatoriEExtensions_Item {}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "65535")]
pub struct EmergencyFallbackIndicatoriEExtensions(Vec<EmergencyFallbackIndicatoriEExtensions_Item>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct EndpointIPAddressAndPortiEExtensions_Item {}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "65535")]
pub struct EndpointIPAddressAndPortiEExtensions(Vec<EndpointIPAddressAndPortiEExtensions_Item>);

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum ErrorIndicationprotocolIEs_Itemvalue {
    #[asn(key = 10)]
    idAMFUENGAPID(AMFUENGAPID),
    #[asn(key = 15)]
    idCause(Cause),
    #[asn(key = 19)]
    idCriticalityDiagnostics(CriticalityDiagnostics),
    #[asn(key = 26)]
    idFiveGSTMSI(FiveGSTMSI),
    #[asn(key = 85)]
    idRANUENGAPID(RANUENGAPID),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct ErrorIndicationprotocolIEs_Item {
    #[asn(key_field = true)]
    pub id: ProtocolIEID,
    pub criticality: Criticality,
    pub value: ErrorIndicationprotocolIEs_Itemvalue,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "0", sz_ub = "65535")]
pub struct ErrorIndicationprotocolIEs(Vec<ErrorIndicationprotocolIEs_Item>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct EventL1LoggedMDTConfigiEExtensions_Item {}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "65535")]
pub struct EventL1LoggedMDTConfigiEExtensions(Vec<EventL1LoggedMDTConfigiEExtensions_Item>);

#[derive(Debug, AperCodec)]
#[asn(type = "ENUMERATED", extensible = true, lb = "0", ub = "0")]
pub struct ENUMERATED22(u8);
impl ENUMERATED22 {
    const TRUE: u8 = 0u8;
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct EventTriggerchoiceExtensions {}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct ExpectedUEActivityBehaviouriEExtensions_Item {}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "65535")]
pub struct ExpectedUEActivityBehaviouriEExtensions(Vec<ExpectedUEActivityBehaviouriEExtensions_Item>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct ExpectedUEBehaviouriEExtensions_Item {}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "65535")]
pub struct ExpectedUEBehaviouriEExtensions(Vec<ExpectedUEBehaviouriEExtensions_Item>);

#[derive(Debug, AperCodec)]
#[asn(type = "INTEGER", lb = "0", ub = "4095")]
pub struct INTEGER23(u16);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct ExpectedUEMovingTrajectoryItemiEExtensions_Item {}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "65535")]
pub struct ExpectedUEMovingTrajectoryItemiEExtensions(Vec<ExpectedUEMovingTrajectoryItemiEExtensions_Item>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct ExtendedAMFNameiEExtensions_Item {}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "65535")]
pub struct ExtendedAMFNameiEExtensions(Vec<ExtendedAMFNameiEExtensions_Item>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct ExtendedRANNodeNameiEExtensions_Item {}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "65535")]
pub struct ExtendedRANNodeNameiEExtensions(Vec<ExtendedRANNodeNameiEExtensions_Item>);

#[derive(Debug, AperCodec)]
#[asn(type = "BITSTRING", sz_extensible = true, sz_lb = "8", sz_ub = "8")]
pub struct BITSTRING24(BitVec<Msb0, u8>);

#[derive(Debug, AperCodec)]
#[asn(type = "BITSTRING", sz_extensible = true, sz_lb = "8", sz_ub = "8")]
pub struct BITSTRING25(BitVec<Msb0, u8>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct ExtendedRATRestrictionInformationiEExtensions_Item {}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "65535")]
pub struct ExtendedRATRestrictionInformationiEExtensions(Vec<ExtendedRATRestrictionInformationiEExtensions_Item>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct FailureIndicationiEExtensions_Item {}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "65535")]
pub struct FailureIndicationiEExtensions(Vec<FailureIndicationiEExtensions_Item>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct FirstDLCountiEExtension_Item {}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "65535")]
pub struct FirstDLCountiEExtension(Vec<FirstDLCountiEExtension_Item>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct FiveGSTMSIiEExtensions_Item {}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "65535")]
pub struct FiveGSTMSIiEExtensions(Vec<FiveGSTMSIiEExtensions_Item>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct ForbiddenAreaInformationItemiEExtensions_Item {}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "65535")]
pub struct ForbiddenAreaInformationItemiEExtensions(Vec<ForbiddenAreaInformationItemiEExtensions_Item>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct FromEUTRANtoNGRANiEExtensions_Item {}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "65535")]
pub struct FromEUTRANtoNGRANiEExtensions(Vec<FromEUTRANtoNGRANiEExtensions_Item>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct FromNGRANtoEUTRANiEExtensions_Item {}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "65535")]
pub struct FromNGRANtoEUTRANiEExtensions(Vec<FromNGRANtoEUTRANiEExtensions_Item>);

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum GBRQosInformationiEExtensions_ItemextensionValue {
    #[asn(key = 220)]
    idAlternativeQoSParaSetList(AlternativeQoSParaSetList),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct GBRQosInformationiEExtensions_Item {
    #[asn(key_field = true)]
    pub id: ProtocolExtensionID,
    pub criticality: Criticality,
    pub extension_value: GBRQosInformationiEExtensions_ItemextensionValue,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "65535")]
pub struct GBRQosInformationiEExtensions(Vec<GBRQosInformationiEExtensions_Item>);

#[derive(Debug, AperCodec)]
#[asn(type = "BITSTRING", sz_extensible = false, sz_lb = "22", sz_ub = "32")]
pub struct BITSTRING26(BitVec<Msb0, u8>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct GNBIDchoiceExtensions {}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct GTPTunneliEExtensions_Item {}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "65535")]
pub struct GTPTunneliEExtensions(Vec<GTPTunneliEExtensions_Item>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct GUAMIiEExtensions_Item {}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "65535")]
pub struct GUAMIiEExtensions(Vec<GUAMIiEExtensions_Item>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct GlobalENBIDiEExtensions_Item {}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "65535")]
pub struct GlobalENBIDiEExtensions(Vec<GlobalENBIDiEExtensions_Item>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct GlobalGNBIDiEExtensions_Item {}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "65535")]
pub struct GlobalGNBIDiEExtensions(Vec<GlobalGNBIDiEExtensions_Item>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct GlobalLineIDiEExtensions_Item {}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "65535")]
pub struct GlobalLineIDiEExtensions(Vec<GlobalLineIDiEExtensions_Item>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct GlobalN3IWFIDiEExtensions_Item {}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "65535")]
pub struct GlobalN3IWFIDiEExtensions(Vec<GlobalN3IWFIDiEExtensions_Item>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct GlobalNgENBIDiEExtensions_Item {}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "65535")]
pub struct GlobalNgENBIDiEExtensions(Vec<GlobalNgENBIDiEExtensions_Item>);

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum GlobalRANNodeIDchoiceExtensionsvalue {
    #[asn(key = 240)]
    idGlobalTNGFID(GlobalTNGFID),
    #[asn(key = 241)]
    idGlobalTWIFID(GlobalTWIFID),
    #[asn(key = 242)]
    idGlobalWAGFID(GlobalWAGFID),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct GlobalRANNodeIDchoiceExtensions {
    #[asn(key_field = true)]
    pub id: ProtocolIEID,
    pub criticality: Criticality,
    pub value: GlobalRANNodeIDchoiceExtensionsvalue,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct GlobalTNGFIDiEExtensions_Item {}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "65535")]
pub struct GlobalTNGFIDiEExtensions(Vec<GlobalTNGFIDiEExtensions_Item>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct GlobalTWIFIDiEExtensions_Item {}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "65535")]
pub struct GlobalTWIFIDiEExtensions(Vec<GlobalTWIFIDiEExtensions_Item>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct GlobalWAGFIDiEExtensions_Item {}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "65535")]
pub struct GlobalWAGFIDiEExtensions(Vec<GlobalWAGFIDiEExtensions_Item>);

#[derive(Debug, AperCodec)]
#[asn(type = "ENUMERATED", extensible = true, lb = "0", ub = "2")]
pub struct ENUMERATED27(u8);
impl ENUMERATED27 {
    const HO_TOO_EARLY: u8 = 0u8;
    const HO_TO_WRONG_CELL: u8 = 1u8;
    const INTERSYSTEM_PING_PONG: u8 = 2u8;
}

#[derive(Debug, AperCodec)]
#[asn(type = "BITSTRING", sz_extensible = false, sz_lb = "16", sz_ub = "16")]
pub struct BITSTRING28(BitVec<Msb0, u8>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct HOReportiEExtensions_Item {}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "65535")]
pub struct HOReportiEExtensions(Vec<HOReportiEExtensions_Item>);

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum HandoverCancelprotocolIEs_Itemvalue {
    #[asn(key = 10)]
    idAMFUENGAPID(AMFUENGAPID),
    #[asn(key = 15)]
    idCause(Cause),
    #[asn(key = 85)]
    idRANUENGAPID(RANUENGAPID),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct HandoverCancelprotocolIEs_Item {
    #[asn(key_field = true)]
    pub id: ProtocolIEID,
    pub criticality: Criticality,
    pub value: HandoverCancelprotocolIEs_Itemvalue,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "0", sz_ub = "65535")]
pub struct HandoverCancelprotocolIEs(Vec<HandoverCancelprotocolIEs_Item>);

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum HandoverCancelAcknowledgeprotocolIEs_Itemvalue {
    #[asn(key = 10)]
    idAMFUENGAPID(AMFUENGAPID),
    #[asn(key = 19)]
    idCriticalityDiagnostics(CriticalityDiagnostics),
    #[asn(key = 85)]
    idRANUENGAPID(RANUENGAPID),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct HandoverCancelAcknowledgeprotocolIEs_Item {
    #[asn(key_field = true)]
    pub id: ProtocolIEID,
    pub criticality: Criticality,
    pub value: HandoverCancelAcknowledgeprotocolIEs_Itemvalue,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "0", sz_ub = "65535")]
pub struct HandoverCancelAcknowledgeprotocolIEs(Vec<HandoverCancelAcknowledgeprotocolIEs_Item>);

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum HandoverCommandprotocolIEs_Itemvalue {
    #[asn(key = 10)]
    idAMFUENGAPID(AMFUENGAPID),
    #[asn(key = 19)]
    idCriticalityDiagnostics(CriticalityDiagnostics),
    #[asn(key = 29)]
    idHandoverType(HandoverType),
    #[asn(key = 39)]
    idNASSecurityParametersFromNGRAN(NASSecurityParametersFromNGRAN),
    #[asn(key = 59)]
    idPDUSessionResourceHandoverList(PDUSessionResourceHandoverList),
    #[asn(key = 78)]
    idPDUSessionResourceToReleaseListHOCmd(PDUSessionResourceToReleaseListHOCmd),
    #[asn(key = 85)]
    idRANUENGAPID(RANUENGAPID),
    #[asn(key = 106)]
    idTargetToSourceTransparentContainer(TargetToSourceTransparentContainer),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct HandoverCommandprotocolIEs_Item {
    #[asn(key_field = true)]
    pub id: ProtocolIEID,
    pub criticality: Criticality,
    pub value: HandoverCommandprotocolIEs_Itemvalue,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "0", sz_ub = "65535")]
pub struct HandoverCommandprotocolIEs(Vec<HandoverCommandprotocolIEs_Item>);

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum HandoverCommandTransferiEExtensions_ItemextensionValue {
    #[asn(key = 152)]
    idAdditionalDLForwardingUPTNLInformation(QosFlowPerTNLInformationList),
    #[asn(key = 172)]
    idAdditionalULForwardingUPTNLInformation(UPTransportLayerInformationList),
    #[asn(key = 249)]
    idDataForwardingResponseERABList(DataForwardingResponseERABList),
    #[asn(key = 164)]
    idULForwardingUPTNLInformation(UPTransportLayerInformation),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct HandoverCommandTransferiEExtensions_Item {
    #[asn(key_field = true)]
    pub id: ProtocolExtensionID,
    pub criticality: Criticality,
    pub extension_value: HandoverCommandTransferiEExtensions_ItemextensionValue,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "65535")]
pub struct HandoverCommandTransferiEExtensions(Vec<HandoverCommandTransferiEExtensions_Item>);

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum HandoverFailureprotocolIEs_Itemvalue {
    #[asn(key = 10)]
    idAMFUENGAPID(AMFUENGAPID),
    #[asn(key = 15)]
    idCause(Cause),
    #[asn(key = 19)]
    idCriticalityDiagnostics(CriticalityDiagnostics),
    #[asn(key = 262)]
    idTargettoSourceFailureTransparentContainer(TargettoSourceFailureTransparentContainer),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct HandoverFailureprotocolIEs_Item {
    #[asn(key_field = true)]
    pub id: ProtocolIEID,
    pub criticality: Criticality,
    pub value: HandoverFailureprotocolIEs_Itemvalue,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "0", sz_ub = "65535")]
pub struct HandoverFailureprotocolIEs(Vec<HandoverFailureprotocolIEs_Item>);

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum HandoverNotifyprotocolIEs_Itemvalue {
    #[asn(key = 10)]
    idAMFUENGAPID(AMFUENGAPID),
    #[asn(key = 269)]
    idNotifySourceNGRANNode(NotifySourceNGRANNode),
    #[asn(key = 85)]
    idRANUENGAPID(RANUENGAPID),
    #[asn(key = 121)]
    idUserLocationInformation(UserLocationInformation),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct HandoverNotifyprotocolIEs_Item {
    #[asn(key_field = true)]
    pub id: ProtocolIEID,
    pub criticality: Criticality,
    pub value: HandoverNotifyprotocolIEs_Itemvalue,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "0", sz_ub = "65535")]
pub struct HandoverNotifyprotocolIEs(Vec<HandoverNotifyprotocolIEs_Item>);

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum HandoverPreparationFailureprotocolIEs_Itemvalue {
    #[asn(key = 10)]
    idAMFUENGAPID(AMFUENGAPID),
    #[asn(key = 15)]
    idCause(Cause),
    #[asn(key = 19)]
    idCriticalityDiagnostics(CriticalityDiagnostics),
    #[asn(key = 85)]
    idRANUENGAPID(RANUENGAPID),
    #[asn(key = 262)]
    idTargettoSourceFailureTransparentContainer(TargettoSourceFailureTransparentContainer),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct HandoverPreparationFailureprotocolIEs_Item {
    #[asn(key_field = true)]
    pub id: ProtocolIEID,
    pub criticality: Criticality,
    pub value: HandoverPreparationFailureprotocolIEs_Itemvalue,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "0", sz_ub = "65535")]
pub struct HandoverPreparationFailureprotocolIEs(Vec<HandoverPreparationFailureprotocolIEs_Item>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct HandoverPreparationUnsuccessfulTransferiEExtensions_Item {}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "65535")]
pub struct HandoverPreparationUnsuccessfulTransferiEExtensions(Vec<HandoverPreparationUnsuccessfulTransferiEExtensions_Item>);

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum HandoverRequestprotocolIEs_Itemvalue {
    #[asn(key = 10)]
    idAMFUENGAPID(AMFUENGAPID),
    #[asn(key = 0)]
    idAllowedNSSAI(AllowedNSSAI),
    #[asn(key = 222)]
    idCEmodeBrestricted(CEmodeBrestricted),
    #[asn(key = 165)]
    idCNAssistedRANTuning(CNAssistedRANTuning),
    #[asn(key = 15)]
    idCause(Cause),
    #[asn(key = 18)]
    idCoreNetworkAssistanceInformationForInactive(CoreNetworkAssistanceInformationForInactive),
    #[asn(key = 205)]
    idEnhancedCoverageRestriction(EnhancedCoverageRestriction),
    #[asn(key = 206)]
    idExtendedConnectedTime(ExtendedConnectedTime),
    #[asn(key = 28)]
    idGUAMI(GUAMI),
    #[asn(key = 29)]
    idHandoverType(HandoverType),
    #[asn(key = 199)]
    idIABAuthorized(IABAuthorized),
    #[asn(key = 217)]
    idLTEUESidelinkAggregateMaximumBitrate(LTEUESidelinkAggregateMaximumBitrate),
    #[asn(key = 215)]
    idLTEV2XServicesAuthorized(LTEV2XServicesAuthorized),
    #[asn(key = 33)]
    idLocationReportingRequestType(LocationReportingRequestType),
    #[asn(key = 254)]
    idManagementBasedMDTPLMNList(MDTPLMNList),
    #[asn(key = 34)]
    idMaskedIMEISV(MaskedIMEISV),
    #[asn(key = 36)]
    idMobilityRestrictionList(MobilityRestrictionList),
    #[asn(key = 37)]
    idNASC(NASPDU),
    #[asn(key = 218)]
    idNRUESidelinkAggregateMaximumBitrate(NRUESidelinkAggregateMaximumBitrate),
    #[asn(key = 216)]
    idNRV2XServicesAuthorized(NRV2XServicesAuthorized),
    #[asn(key = 41)]
    idNewSecurityContextInd(NewSecurityContextInd),
    #[asn(key = 219)]
    idPC5QoSParameters(PC5QoSParameters),
    #[asn(key = 73)]
    idPDUSessionResourceSetupListHOReq(PDUSessionResourceSetupListHOReq),
    #[asn(key = 91)]
    idRRCInactiveTransitionReportRequest(RRCInactiveTransitionReportRequest),
    #[asn(key = 146)]
    idRedirectionVoiceFallback(RedirectionVoiceFallback),
    #[asn(key = 177)]
    idSRVCCOperationPossible(SRVCCOperationPossible),
    #[asn(key = 93)]
    idSecurityContext(SecurityContext),
    #[asn(key = 101)]
    idSourceToTargetTransparentContainer(SourceToTargetTransparentContainer),
    #[asn(key = 108)]
    idTraceActivation(TraceActivation),
    #[asn(key = 209)]
    idUEDifferentiationInfo(UEDifferentiationInfo),
    #[asn(key = 234)]
    idUEUPCIoTSupport(UEUPCIoTSupport),
    #[asn(key = 110)]
    idUEAggregateMaximumBitRate(UEAggregateMaximumBitRate),
    #[asn(key = 264)]
    idUERadioCapabilityID(UERadioCapabilityID),
    #[asn(key = 119)]
    idUESecurityCapabilities(UESecurityCapabilities),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct HandoverRequestprotocolIEs_Item {
    #[asn(key_field = true)]
    pub id: ProtocolIEID,
    pub criticality: Criticality,
    pub value: HandoverRequestprotocolIEs_Itemvalue,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "0", sz_ub = "65535")]
pub struct HandoverRequestprotocolIEs(Vec<HandoverRequestprotocolIEs_Item>);

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum HandoverRequestAcknowledgeprotocolIEs_Itemvalue {
    #[asn(key = 10)]
    idAMFUENGAPID(AMFUENGAPID),
    #[asn(key = 19)]
    idCriticalityDiagnostics(CriticalityDiagnostics),
    #[asn(key = 53)]
    idPDUSessionResourceAdmittedList(PDUSessionResourceAdmittedList),
    #[asn(key = 56)]
    idPDUSessionResourceFailedToSetupListHOAck(PDUSessionResourceFailedToSetupListHOAck),
    #[asn(key = 85)]
    idRANUENGAPID(RANUENGAPID),
    #[asn(key = 106)]
    idTargetToSourceTransparentContainer(TargetToSourceTransparentContainer),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct HandoverRequestAcknowledgeprotocolIEs_Item {
    #[asn(key_field = true)]
    pub id: ProtocolIEID,
    pub criticality: Criticality,
    pub value: HandoverRequestAcknowledgeprotocolIEs_Itemvalue,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "0", sz_ub = "65535")]
pub struct HandoverRequestAcknowledgeprotocolIEs(Vec<HandoverRequestAcknowledgeprotocolIEs_Item>);

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum HandoverRequestAcknowledgeTransferiEExtensions_ItemextensionValue {
    #[asn(key = 153)]
    idAdditionalDLUPTNLInformationForHOList(AdditionalDLUPTNLInformationForHOList),
    #[asn(key = 172)]
    idAdditionalULForwardingUPTNLInformation(UPTransportLayerInformationList),
    #[asn(key = 249)]
    idDataForwardingResponseERABList(DataForwardingResponseERABList),
    #[asn(key = 27)]
    idGlobalRANNodeID(GlobalRANNodeID),
    #[asn(key = 192)]
    idRedundantDLNGUUPTNLInformation(UPTransportLayerInformation),
    #[asn(key = 164)]
    idULForwardingUPTNLInformation(UPTransportLayerInformation),
    #[asn(key = 198)]
    idUsedRSNInformation(RedundantPDUSessionInformation),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct HandoverRequestAcknowledgeTransferiEExtensions_Item {
    #[asn(key_field = true)]
    pub id: ProtocolExtensionID,
    pub criticality: Criticality,
    pub extension_value: HandoverRequestAcknowledgeTransferiEExtensions_ItemextensionValue,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "65535")]
pub struct HandoverRequestAcknowledgeTransferiEExtensions(Vec<HandoverRequestAcknowledgeTransferiEExtensions_Item>);

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum HandoverRequiredprotocolIEs_Itemvalue {
    #[asn(key = 10)]
    idAMFUENGAPID(AMFUENGAPID),
    #[asn(key = 15)]
    idCause(Cause),
    #[asn(key = 22)]
    idDirectForwardingPathAvailability(DirectForwardingPathAvailability),
    #[asn(key = 29)]
    idHandoverType(HandoverType),
    #[asn(key = 61)]
    idPDUSessionResourceListHORqd(PDUSessionResourceListHORqd),
    #[asn(key = 85)]
    idRANUENGAPID(RANUENGAPID),
    #[asn(key = 101)]
    idSourceToTargetTransparentContainer(SourceToTargetTransparentContainer),
    #[asn(key = 105)]
    idTargetID(TargetID),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct HandoverRequiredprotocolIEs_Item {
    #[asn(key_field = true)]
    pub id: ProtocolIEID,
    pub criticality: Criticality,
    pub value: HandoverRequiredprotocolIEs_Itemvalue,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "0", sz_ub = "65535")]
pub struct HandoverRequiredprotocolIEs(Vec<HandoverRequiredprotocolIEs_Item>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct HandoverRequiredTransferiEExtensions_Item {}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "65535")]
pub struct HandoverRequiredTransferiEExtensions(Vec<HandoverRequiredTransferiEExtensions_Item>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct HandoverResourceAllocationUnsuccessfulTransferiEExtensions_Item {}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "65535")]
pub struct HandoverResourceAllocationUnsuccessfulTransferiEExtensions(Vec<HandoverResourceAllocationUnsuccessfulTransferiEExtensions_Item>);

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum HandoverSuccessprotocolIEs_Itemvalue {
    #[asn(key = 10)]
    idAMFUENGAPID(AMFUENGAPID),
    #[asn(key = 85)]
    idRANUENGAPID(RANUENGAPID),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct HandoverSuccessprotocolIEs_Item {
    #[asn(key_field = true)]
    pub id: ProtocolIEID,
    pub criticality: Criticality,
    pub value: HandoverSuccessprotocolIEs_Itemvalue,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "0", sz_ub = "65535")]
pub struct HandoverSuccessprotocolIEs(Vec<HandoverSuccessprotocolIEs_Item>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct ImmediateMDTNriEExtensions_Item {}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "65535")]
pub struct ImmediateMDTNriEExtensions(Vec<ImmediateMDTNriEExtensions_Item>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct InfoOnRecommendedCellsAndRANNodesForPagingiEExtensions_Item {}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "65535")]
pub struct InfoOnRecommendedCellsAndRANNodesForPagingiEExtensions(Vec<InfoOnRecommendedCellsAndRANNodesForPagingiEExtensions_Item>);

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum InitialContextSetupFailureprotocolIEs_Itemvalue {
    #[asn(key = 10)]
    idAMFUENGAPID(AMFUENGAPID),
    #[asn(key = 15)]
    idCause(Cause),
    #[asn(key = 19)]
    idCriticalityDiagnostics(CriticalityDiagnostics),
    #[asn(key = 132)]
    idPDUSessionResourceFailedToSetupListCxtFail(PDUSessionResourceFailedToSetupListCxtFail),
    #[asn(key = 85)]
    idRANUENGAPID(RANUENGAPID),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct InitialContextSetupFailureprotocolIEs_Item {
    #[asn(key_field = true)]
    pub id: ProtocolIEID,
    pub criticality: Criticality,
    pub value: InitialContextSetupFailureprotocolIEs_Itemvalue,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "0", sz_ub = "65535")]
pub struct InitialContextSetupFailureprotocolIEs(Vec<InitialContextSetupFailureprotocolIEs_Item>);

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum InitialContextSetupRequestprotocolIEs_Itemvalue {
    #[asn(key = 10)]
    idAMFUENGAPID(AMFUENGAPID),
    #[asn(key = 0)]
    idAllowedNSSAI(AllowedNSSAI),
    #[asn(key = 222)]
    idCEmodeBrestricted(CEmodeBrestricted),
    #[asn(key = 165)]
    idCNAssistedRANTuning(CNAssistedRANTuning),
    #[asn(key = 18)]
    idCoreNetworkAssistanceInformationForInactive(CoreNetworkAssistanceInformationForInactive),
    #[asn(key = 24)]
    idEmergencyFallbackIndicator(EmergencyFallbackIndicator),
    #[asn(key = 205)]
    idEnhancedCoverageRestriction(EnhancedCoverageRestriction),
    #[asn(key = 206)]
    idExtendedConnectedTime(ExtendedConnectedTime),
    #[asn(key = 28)]
    idGUAMI(GUAMI),
    #[asn(key = 199)]
    idIABAuthorized(IABAuthorized),
    #[asn(key = 31)]
    idIndexToRFSP(IndexToRFSP),
    #[asn(key = 217)]
    idLTEUESidelinkAggregateMaximumBitrate(LTEUESidelinkAggregateMaximumBitrate),
    #[asn(key = 215)]
    idLTEV2XServicesAuthorized(LTEV2XServicesAuthorized),
    #[asn(key = 33)]
    idLocationReportingRequestType(LocationReportingRequestType),
    #[asn(key = 254)]
    idManagementBasedMDTPLMNList(MDTPLMNList),
    #[asn(key = 34)]
    idMaskedIMEISV(MaskedIMEISV),
    #[asn(key = 36)]
    idMobilityRestrictionList(MobilityRestrictionList),
    #[asn(key = 38)]
    idNASPDU(NASPDU),
    #[asn(key = 218)]
    idNRUESidelinkAggregateMaximumBitrate(NRUESidelinkAggregateMaximumBitrate),
    #[asn(key = 216)]
    idNRV2XServicesAuthorized(NRV2XServicesAuthorized),
    #[asn(key = 48)]
    idOldAMF(AMFName),
    #[asn(key = 219)]
    idPC5QoSParameters(PC5QoSParameters),
    #[asn(key = 71)]
    idPDUSessionResourceSetupListCxtReq(PDUSessionResourceSetupListCxtReq),
    #[asn(key = 85)]
    idRANUENGAPID(RANUENGAPID),
    #[asn(key = 238)]
    idRGLevelWirelineAccessCharacteristics(RGLevelWirelineAccessCharacteristics),
    #[asn(key = 91)]
    idRRCInactiveTransitionReportRequest(RRCInactiveTransitionReportRequest),
    #[asn(key = 146)]
    idRedirectionVoiceFallback(RedirectionVoiceFallback),
    #[asn(key = 177)]
    idSRVCCOperationPossible(SRVCCOperationPossible),
    #[asn(key = 94)]
    idSecurityKey(SecurityKey),
    #[asn(key = 108)]
    idTraceActivation(TraceActivation),
    #[asn(key = 209)]
    idUEDifferentiationInfo(UEDifferentiationInfo),
    #[asn(key = 234)]
    idUEUPCIoTSupport(UEUPCIoTSupport),
    #[asn(key = 110)]
    idUEAggregateMaximumBitRate(UEAggregateMaximumBitRate),
    #[asn(key = 117)]
    idUERadioCapability(UERadioCapability),
    #[asn(key = 118)]
    idUERadioCapabilityForPaging(UERadioCapabilityForPaging),
    #[asn(key = 264)]
    idUERadioCapabilityID(UERadioCapabilityID),
    #[asn(key = 119)]
    idUESecurityCapabilities(UESecurityCapabilities),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct InitialContextSetupRequestprotocolIEs_Item {
    #[asn(key_field = true)]
    pub id: ProtocolIEID,
    pub criticality: Criticality,
    pub value: InitialContextSetupRequestprotocolIEs_Itemvalue,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "0", sz_ub = "65535")]
pub struct InitialContextSetupRequestprotocolIEs(Vec<InitialContextSetupRequestprotocolIEs_Item>);

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum InitialContextSetupResponseprotocolIEs_Itemvalue {
    #[asn(key = 10)]
    idAMFUENGAPID(AMFUENGAPID),
    #[asn(key = 19)]
    idCriticalityDiagnostics(CriticalityDiagnostics),
    #[asn(key = 55)]
    idPDUSessionResourceFailedToSetupListCxtRes(PDUSessionResourceFailedToSetupListCxtRes),
    #[asn(key = 72)]
    idPDUSessionResourceSetupListCxtRes(PDUSessionResourceSetupListCxtRes),
    #[asn(key = 85)]
    idRANUENGAPID(RANUENGAPID),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct InitialContextSetupResponseprotocolIEs_Item {
    #[asn(key_field = true)]
    pub id: ProtocolIEID,
    pub criticality: Criticality,
    pub value: InitialContextSetupResponseprotocolIEs_Itemvalue,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "0", sz_ub = "65535")]
pub struct InitialContextSetupResponseprotocolIEs(Vec<InitialContextSetupResponseprotocolIEs_Item>);

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum InitialUEMessageprotocolIEs_Itemvalue {
    #[asn(key = 3)]
    idAMFSetID(AMFSetID),
    #[asn(key = 0)]
    idAllowedNSSAI(AllowedNSSAI),
    #[asn(key = 245)]
    idAuthenticatedIndication(AuthenticatedIndication),
    #[asn(key = 224)]
    idCEmodeBSupportIndicator(CEmodeBSupportIndicator),
    #[asn(key = 227)]
    idEDTSession(EDTSession),
    #[asn(key = 26)]
    idFiveGSTMSI(FiveGSTMSI),
    #[asn(key = 201)]
    idIABNodeIndication(IABNodeIndication),
    #[asn(key = 225)]
    idLTEMIndication(LTEMIndication),
    #[asn(key = 38)]
    idNASPDU(NASPDU),
    #[asn(key = 259)]
    idNPNAccessInformation(NPNAccessInformation),
    #[asn(key = 85)]
    idRANUENGAPID(RANUENGAPID),
    #[asn(key = 90)]
    idRRCEstablishmentCause(RRCEstablishmentCause),
    #[asn(key = 174)]
    idSelectedPLMNIdentity(PLMNIdentity),
    #[asn(key = 171)]
    idSourceToTargetAMFInformationReroute(SourceToTargetAMFInformationReroute),
    #[asn(key = 112)]
    idUEContextRequest(UEContextRequest),
    #[asn(key = 121)]
    idUserLocationInformation(UserLocationInformation),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct InitialUEMessageprotocolIEs_Item {
    #[asn(key_field = true)]
    pub id: ProtocolIEID,
    pub criticality: Criticality,
    pub value: InitialUEMessageprotocolIEs_Itemvalue,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "0", sz_ub = "65535")]
pub struct InitialUEMessageprotocolIEs(Vec<InitialUEMessageprotocolIEs_Item>);

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum InitiatingMessagevalue {
    #[asn(key = 64)]
    idAMFCPRelocationIndication(AMFCPRelocationIndication),
    #[asn(key = 0)]
    idAMFConfigurationUpdate(AMFConfigurationUpdate),
    #[asn(key = 1)]
    idAMFStatusIndication(AMFStatusIndication),
    #[asn(key = 2)]
    idCellTrafficTrace(CellTrafficTrace),
    #[asn(key = 65)]
    idConnectionEstablishmentIndication(ConnectionEstablishmentIndication),
    #[asn(key = 3)]
    idDeactivateTrace(DeactivateTrace),
    #[asn(key = 4)]
    idDownlinkNASTransport(DownlinkNASTransport),
    #[asn(key = 5)]
    idDownlinkNonUEAssociatedNRPPaTransport(DownlinkNonUEAssociatedNRPPaTransport),
    #[asn(key = 6)]
    idDownlinkRANConfigurationTransfer(DownlinkRANConfigurationTransfer),
    #[asn(key = 63)]
    idDownlinkRANEarlyStatusTransfer(DownlinkRANEarlyStatusTransfer),
    #[asn(key = 7)]
    idDownlinkRANStatusTransfer(DownlinkRANStatusTransfer),
    #[asn(key = 54)]
    idDownlinkRIMInformationTransfer(DownlinkRIMInformationTransfer),
    #[asn(key = 8)]
    idDownlinkUEAssociatedNRPPaTransport(DownlinkUEAssociatedNRPPaTransport),
    #[asn(key = 9)]
    idErrorIndication(ErrorIndication),
    #[asn(key = 10)]
    idHandoverCancel(HandoverCancel),
    #[asn(key = 11)]
    idHandoverNotification(HandoverNotify),
    #[asn(key = 12)]
    idHandoverPreparation(HandoverRequired),
    #[asn(key = 13)]
    idHandoverResourceAllocation(HandoverRequest),
    #[asn(key = 61)]
    idHandoverSuccess(HandoverSuccess),
    #[asn(key = 14)]
    idInitialContextSetup(InitialContextSetupRequest),
    #[asn(key = 15)]
    idInitialUEMessage(InitialUEMessage),
    #[asn(key = 18)]
    idLocationReport(LocationReport),
    #[asn(key = 16)]
    idLocationReportingControl(LocationReportingControl),
    #[asn(key = 17)]
    idLocationReportingFailureIndication(LocationReportingFailureIndication),
    #[asn(key = 19)]
    idNASNonDeliveryIndication(NASNonDeliveryIndication),
    #[asn(key = 20)]
    idNGReset(NGReset),
    #[asn(key = 21)]
    idNGSetup(NGSetupRequest),
    #[asn(key = 22)]
    idOverloadStart(OverloadStart),
    #[asn(key = 23)]
    idOverloadStop(OverloadStop),
    #[asn(key = 26)]
    idPDUSessionResourceModify(PDUSessionResourceModifyRequest),
    #[asn(key = 27)]
    idPDUSessionResourceModifyIndication(PDUSessionResourceModifyIndication),
    #[asn(key = 30)]
    idPDUSessionResourceNotify(PDUSessionResourceNotify),
    #[asn(key = 28)]
    idPDUSessionResourceRelease(PDUSessionResourceReleaseCommand),
    #[asn(key = 29)]
    idPDUSessionResourceSetup(PDUSessionResourceSetupRequest),
    #[asn(key = 32)]
    idPWSCancel(PWSCancelRequest),
    #[asn(key = 33)]
    idPWSFailureIndication(PWSFailureIndication),
    #[asn(key = 34)]
    idPWSRestartIndication(PWSRestartIndication),
    #[asn(key = 24)]
    idPaging(Paging),
    #[asn(key = 25)]
    idPathSwitchRequest(PathSwitchRequest),
    #[asn(key = 31)]
    idPrivateMessage(PrivateMessage),
    #[asn(key = 57)]
    idRANCPRelocationIndication(RANCPRelocationIndication),
    #[asn(key = 35)]
    idRANConfigurationUpdate(RANConfigurationUpdate),
    #[asn(key = 37)]
    idRRCInactiveTransitionReport(RRCInactiveTransitionReport),
    #[asn(key = 36)]
    idRerouteNASRequest(RerouteNASRequest),
    #[asn(key = 55)]
    idRetrieveUEInformation(RetrieveUEInformation),
    #[asn(key = 52)]
    idSecondaryRATDataUsageReport(SecondaryRATDataUsageReport),
    #[asn(key = 38)]
    idTraceFailureIndication(TraceFailureIndication),
    #[asn(key = 39)]
    idTraceStart(TraceStart),
    #[asn(key = 40)]
    idUEContextModification(UEContextModificationRequest),
    #[asn(key = 41)]
    idUEContextRelease(UEContextReleaseCommand),
    #[asn(key = 42)]
    idUEContextReleaseRequest(UEContextReleaseRequest),
    #[asn(key = 58)]
    idUEContextResume(UEContextResumeRequest),
    #[asn(key = 59)]
    idUEContextSuspend(UEContextSuspendRequest),
    #[asn(key = 56)]
    idUEInformationTransfer(UEInformationTransfer),
    #[asn(key = 43)]
    idUERadioCapabilityCheck(UERadioCapabilityCheckRequest),
    #[asn(key = 60)]
    idUERadioCapabilityIDMapping(UERadioCapabilityIDMappingRequest),
    #[asn(key = 44)]
    idUERadioCapabilityInfoIndication(UERadioCapabilityInfoIndication),
    #[asn(key = 45)]
    idUETNLABindingRelease(UETNLABindingReleaseRequest),
    #[asn(key = 46)]
    idUplinkNASTransport(UplinkNASTransport),
    #[asn(key = 47)]
    idUplinkNonUEAssociatedNRPPaTransport(UplinkNonUEAssociatedNRPPaTransport),
    #[asn(key = 48)]
    idUplinkRANConfigurationTransfer(UplinkRANConfigurationTransfer),
    #[asn(key = 62)]
    idUplinkRANEarlyStatusTransfer(UplinkRANEarlyStatusTransfer),
    #[asn(key = 49)]
    idUplinkRANStatusTransfer(UplinkRANStatusTransfer),
    #[asn(key = 53)]
    idUplinkRIMInformationTransfer(UplinkRIMInformationTransfer),
    #[asn(key = 50)]
    idUplinkUEAssociatedNRPPaTransport(UplinkUEAssociatedNRPPaTransport),
    #[asn(key = 51)]
    idWriteReplaceWarning(WriteReplaceWarningRequest),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct InterSystemFailureIndicationiEExtensions_Item {}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "65535")]
pub struct InterSystemFailureIndicationiEExtensions(Vec<InterSystemFailureIndicationiEExtensions_Item>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct InterSystemHOReportiEExtensions_Item {}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "65535")]
pub struct InterSystemHOReportiEExtensions(Vec<InterSystemHOReportiEExtensions_Item>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct InterSystemHandoverReportTypechoiceExtensions {}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct IntersystemSONConfigurationTransferiEExtensions_Item {}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "65535")]
pub struct IntersystemSONConfigurationTransferiEExtensions(Vec<IntersystemSONConfigurationTransferiEExtensions_Item>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct IntersystemSONInformationchoiceExtensions {}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct IntersystemSONInformationReportchoiceExtensions {}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct IntersystemSONNGRANnodeIDiEExtensions_Item {}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "65535")]
pub struct IntersystemSONNGRANnodeIDiEExtensions(Vec<IntersystemSONNGRANnodeIDiEExtensions_Item>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct IntersystemSONTransferTypechoiceExtensions {}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct IntersystemSONeNBIDiEExtensions_Item {}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "65535")]
pub struct IntersystemSONeNBIDiEExtensions(Vec<IntersystemSONeNBIDiEExtensions_Item>);

#[derive(Debug, AperCodec)]
#[asn(type = "ENUMERATED", extensible = true, lb = "0", ub = "1")]
pub struct ENUMERATED29(u8);
impl ENUMERATED29 {
    const TRUE: u8 = 0u8;
    const FALSE: u8 = 1u8;
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct IntersystemUnnecessaryHOiEExtensions_Item {}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "65535")]
pub struct IntersystemUnnecessaryHOiEExtensions(Vec<IntersystemUnnecessaryHOiEExtensions_Item>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct LAIiEExtensions_Item {}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "65535")]
pub struct LAIiEExtensions(Vec<LAIiEExtensions_Item>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct LTEUESidelinkAggregateMaximumBitrateiEExtensions_Item {}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "65535")]
pub struct LTEUESidelinkAggregateMaximumBitrateiEExtensions(Vec<LTEUESidelinkAggregateMaximumBitrateiEExtensions_Item>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct LTEV2XServicesAuthorizediEExtensions_Item {}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "65535")]
pub struct LTEV2XServicesAuthorizediEExtensions(Vec<LTEV2XServicesAuthorizediEExtensions_Item>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct LastVisitedCellInformationchoiceExtensions {}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct LastVisitedCellItemiEExtensions_Item {}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "65535")]
pub struct LastVisitedCellItemiEExtensions(Vec<LastVisitedCellItemiEExtensions_Item>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct LastVisitedNGRANCellInformationiEExtensions_Item {}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "65535")]
pub struct LastVisitedNGRANCellInformationiEExtensions(Vec<LastVisitedNGRANCellInformationiEExtensions_Item>);

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum LocationReportprotocolIEs_Itemvalue {
    #[asn(key = 10)]
    idAMFUENGAPID(AMFUENGAPID),
    #[asn(key = 33)]
    idLocationReportingRequestType(LocationReportingRequestType),
    #[asn(key = 85)]
    idRANUENGAPID(RANUENGAPID),
    #[asn(key = 116)]
    idUEPresenceInAreaOfInterestList(UEPresenceInAreaOfInterestList),
    #[asn(key = 121)]
    idUserLocationInformation(UserLocationInformation),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct LocationReportprotocolIEs_Item {
    #[asn(key_field = true)]
    pub id: ProtocolIEID,
    pub criticality: Criticality,
    pub value: LocationReportprotocolIEs_Itemvalue,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "0", sz_ub = "65535")]
pub struct LocationReportprotocolIEs(Vec<LocationReportprotocolIEs_Item>);

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum LocationReportingControlprotocolIEs_Itemvalue {
    #[asn(key = 10)]
    idAMFUENGAPID(AMFUENGAPID),
    #[asn(key = 33)]
    idLocationReportingRequestType(LocationReportingRequestType),
    #[asn(key = 85)]
    idRANUENGAPID(RANUENGAPID),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct LocationReportingControlprotocolIEs_Item {
    #[asn(key_field = true)]
    pub id: ProtocolIEID,
    pub criticality: Criticality,
    pub value: LocationReportingControlprotocolIEs_Itemvalue,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "0", sz_ub = "65535")]
pub struct LocationReportingControlprotocolIEs(Vec<LocationReportingControlprotocolIEs_Item>);

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum LocationReportingFailureIndicationprotocolIEs_Itemvalue {
    #[asn(key = 10)]
    idAMFUENGAPID(AMFUENGAPID),
    #[asn(key = 15)]
    idCause(Cause),
    #[asn(key = 85)]
    idRANUENGAPID(RANUENGAPID),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct LocationReportingFailureIndicationprotocolIEs_Item {
    #[asn(key_field = true)]
    pub id: ProtocolIEID,
    pub criticality: Criticality,
    pub value: LocationReportingFailureIndicationprotocolIEs_Itemvalue,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "0", sz_ub = "65535")]
pub struct LocationReportingFailureIndicationprotocolIEs(Vec<LocationReportingFailureIndicationprotocolIEs_Item>);

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum LocationReportingRequestTypeiEExtensions_ItemextensionValue {
    #[asn(key = 170)]
    idLocationReportingAdditionalInfo(LocationReportingAdditionalInfo),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct LocationReportingRequestTypeiEExtensions_Item {
    #[asn(key_field = true)]
    pub id: ProtocolExtensionID,
    pub criticality: Criticality,
    pub extension_value: LocationReportingRequestTypeiEExtensions_ItemextensionValue,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "65535")]
pub struct LocationReportingRequestTypeiEExtensions(Vec<LocationReportingRequestTypeiEExtensions_Item>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct LoggedMDTNriEExtensions_Item {}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "65535")]
pub struct LoggedMDTNriEExtensions(Vec<LoggedMDTNriEExtensions_Item>);

#[derive(Debug, AperCodec)]
#[asn(type = "NULL")]
pub struct NULL30;

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct LoggedMDTTriggerchoiceExtensions {}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct M1ConfigurationiEExtensions_Item {}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "65535")]
pub struct M1ConfigurationiEExtensions(Vec<M1ConfigurationiEExtensions_Item>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct M1PeriodicReportingiEExtensions_Item {}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "65535")]
pub struct M1PeriodicReportingiEExtensions(Vec<M1PeriodicReportingiEExtensions_Item>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct M1ThresholdEventA2iEExtensions_Item {}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "65535")]
pub struct M1ThresholdEventA2iEExtensions(Vec<M1ThresholdEventA2iEExtensions_Item>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct M1ThresholdTypechoiceExtensions {}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct M4ConfigurationiEExtensions_Item {}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "65535")]
pub struct M4ConfigurationiEExtensions(Vec<M4ConfigurationiEExtensions_Item>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct M5ConfigurationiEExtensions_Item {}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "65535")]
pub struct M5ConfigurationiEExtensions(Vec<M5ConfigurationiEExtensions_Item>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct M6ConfigurationiEExtensions_Item {}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "65535")]
pub struct M6ConfigurationiEExtensions(Vec<M6ConfigurationiEExtensions_Item>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct M7ConfigurationiEExtensions_Item {}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "65535")]
pub struct M7ConfigurationiEExtensions(Vec<M7ConfigurationiEExtensions_Item>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct MDTConfigurationiEExtensions_Item {}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "65535")]
pub struct MDTConfigurationiEExtensions(Vec<MDTConfigurationiEExtensions_Item>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct MDTConfigurationEUTRAiEExtensions_Item {}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "65535")]
pub struct MDTConfigurationEUTRAiEExtensions(Vec<MDTConfigurationEUTRAiEExtensions_Item>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct MDTConfigurationNRiEExtensions_Item {}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "65535")]
pub struct MDTConfigurationNRiEExtensions(Vec<MDTConfigurationNRiEExtensions_Item>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct MDTLocationInfoiEExtensions_Item {}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "65535")]
pub struct MDTLocationInfoiEExtensions(Vec<MDTLocationInfoiEExtensions_Item>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct MDTModeNrchoiceExtensions {}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct MeasurementThresholdL1LoggedMDTchoiceExtensions {}

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum MobilityRestrictionListiEExtensions_ItemextensionValue {
    #[asn(key = 160)]
    idCNTypeRestrictionsForEquivalent(CNTypeRestrictionsForEquivalent),
    #[asn(key = 161)]
    idCNTypeRestrictionsForServing(CNTypeRestrictionsForServing),
    #[asn(key = 150)]
    idLastEUTRANPLMNIdentity(PLMNIdentity),
    #[asn(key = 261)]
    idNPNMobilityInformation(NPNMobilityInformation),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct MobilityRestrictionListiEExtensions_Item {
    #[asn(key_field = true)]
    pub id: ProtocolExtensionID,
    pub criticality: Criticality,
    pub extension_value: MobilityRestrictionListiEExtensions_ItemextensionValue,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "65535")]
pub struct MobilityRestrictionListiEExtensions(Vec<MobilityRestrictionListiEExtensions_Item>);

#[derive(Debug, AperCodec)]
#[asn(type = "BITSTRING", sz_extensible = false, sz_lb = "16", sz_ub = "16")]
pub struct BITSTRING31(BitVec<Msb0, u8>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct N3IWFIDchoiceExtensions {}

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum NASNonDeliveryIndicationprotocolIEs_Itemvalue {
    #[asn(key = 10)]
    idAMFUENGAPID(AMFUENGAPID),
    #[asn(key = 15)]
    idCause(Cause),
    #[asn(key = 38)]
    idNASPDU(NASPDU),
    #[asn(key = 85)]
    idRANUENGAPID(RANUENGAPID),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct NASNonDeliveryIndicationprotocolIEs_Item {
    #[asn(key_field = true)]
    pub id: ProtocolIEID,
    pub criticality: Criticality,
    pub value: NASNonDeliveryIndicationprotocolIEs_Itemvalue,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "0", sz_ub = "65535")]
pub struct NASNonDeliveryIndicationprotocolIEs(Vec<NASNonDeliveryIndicationprotocolIEs_Item>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct NBIoTPagingeDRXInfoiEExtensions_Item {}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "65535")]
pub struct NBIoTPagingeDRXInfoiEExtensions(Vec<NBIoTPagingeDRXInfoiEExtensions_Item>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct NGRANCGIchoiceExtensions {}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct NGRANTNLAssociationToRemoveItemiEExtensions_Item {}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "65535")]
pub struct NGRANTNLAssociationToRemoveItemiEExtensions(Vec<NGRANTNLAssociationToRemoveItemiEExtensions_Item>);

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum NGResetprotocolIEs_Itemvalue {
    #[asn(key = 15)]
    idCause(Cause),
    #[asn(key = 88)]
    idResetType(ResetType),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct NGResetprotocolIEs_Item {
    #[asn(key_field = true)]
    pub id: ProtocolIEID,
    pub criticality: Criticality,
    pub value: NGResetprotocolIEs_Itemvalue,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "0", sz_ub = "65535")]
pub struct NGResetprotocolIEs(Vec<NGResetprotocolIEs_Item>);

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum NGResetAcknowledgeprotocolIEs_Itemvalue {
    #[asn(key = 19)]
    idCriticalityDiagnostics(CriticalityDiagnostics),
    #[asn(key = 111)]
    idUEassociatedLogicalNGconnectionList(UEassociatedLogicalNGconnectionList),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct NGResetAcknowledgeprotocolIEs_Item {
    #[asn(key_field = true)]
    pub id: ProtocolIEID,
    pub criticality: Criticality,
    pub value: NGResetAcknowledgeprotocolIEs_Itemvalue,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "0", sz_ub = "65535")]
pub struct NGResetAcknowledgeprotocolIEs(Vec<NGResetAcknowledgeprotocolIEs_Item>);

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum NGSetupFailureprotocolIEs_Itemvalue {
    #[asn(key = 15)]
    idCause(Cause),
    #[asn(key = 19)]
    idCriticalityDiagnostics(CriticalityDiagnostics),
    #[asn(key = 107)]
    idTimeToWait(TimeToWait),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct NGSetupFailureprotocolIEs_Item {
    #[asn(key_field = true)]
    pub id: ProtocolIEID,
    pub criticality: Criticality,
    pub value: NGSetupFailureprotocolIEs_Itemvalue,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "0", sz_ub = "65535")]
pub struct NGSetupFailureprotocolIEs(Vec<NGSetupFailureprotocolIEs_Item>);

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum NGSetupRequestprotocolIEs_Itemvalue {
    #[asn(key = 21)]
    idDefaultPagingDRX(PagingDRX),
    #[asn(key = 273)]
    idExtendedRANNodeName(ExtendedRANNodeName),
    #[asn(key = 27)]
    idGlobalRANNodeID(GlobalRANNodeID),
    #[asn(key = 204)]
    idNBIoTDefaultPagingDRX(NBIoTDefaultPagingDRX),
    #[asn(key = 82)]
    idRANNodeName(RANNodeName),
    #[asn(key = 102)]
    idSupportedTAList(SupportedTAList),
    #[asn(key = 147)]
    idUERetentionInformation(UERetentionInformation),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct NGSetupRequestprotocolIEs_Item {
    #[asn(key_field = true)]
    pub id: ProtocolIEID,
    pub criticality: Criticality,
    pub value: NGSetupRequestprotocolIEs_Itemvalue,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "0", sz_ub = "65535")]
pub struct NGSetupRequestprotocolIEs(Vec<NGSetupRequestprotocolIEs_Item>);

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum NGSetupResponseprotocolIEs_Itemvalue {
    #[asn(key = 1)]
    idAMFName(AMFName),
    #[asn(key = 19)]
    idCriticalityDiagnostics(CriticalityDiagnostics),
    #[asn(key = 274)]
    idExtendedAMFName(ExtendedAMFName),
    #[asn(key = 200)]
    idIABSupported(IABSupported),
    #[asn(key = 80)]
    idPLMNSupportList(PLMNSupportList),
    #[asn(key = 86)]
    idRelativeAMFCapacity(RelativeAMFCapacity),
    #[asn(key = 96)]
    idServedGUAMIList(ServedGUAMIList),
    #[asn(key = 147)]
    idUERetentionInformation(UERetentionInformation),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct NGSetupResponseprotocolIEs_Item {
    #[asn(key_field = true)]
    pub id: ProtocolIEID,
    pub criticality: Criticality,
    pub value: NGSetupResponseprotocolIEs_Itemvalue,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "0", sz_ub = "65535")]
pub struct NGSetupResponseprotocolIEs(Vec<NGSetupResponseprotocolIEs_Item>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct NPNAccessInformationchoiceExtensions {}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct NPNMobilityInformationchoiceExtensions {}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct NPNPagingAssistanceInformationchoiceExtensions {}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct NPNSupportchoiceExtensions {}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct NRCGIiEExtensions_Item {}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "65535")]
pub struct NRCGIiEExtensions(Vec<NRCGIiEExtensions_Item>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct NRFrequencyBandItemiEExtension_Item {}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "65535")]
pub struct NRFrequencyBandItemiEExtension(Vec<NRFrequencyBandItemiEExtension_Item>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct NRFrequencyInfoiEExtension_Item {}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "65535")]
pub struct NRFrequencyInfoiEExtension(Vec<NRFrequencyInfoiEExtension_Item>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct NRUESidelinkAggregateMaximumBitrateiEExtensions_Item {}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "65535")]
pub struct NRUESidelinkAggregateMaximumBitrateiEExtensions(Vec<NRUESidelinkAggregateMaximumBitrateiEExtensions_Item>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct NRV2XServicesAuthorizediEExtensions_Item {}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "65535")]
pub struct NRV2XServicesAuthorizediEExtensions(Vec<NRV2XServicesAuthorizediEExtensions_Item>);

#[derive(Debug, AperCodec)]
#[asn(type = "BITSTRING", sz_extensible = false, sz_lb = "20", sz_ub = "20")]
pub struct BITSTRING32(BitVec<Msb0, u8>);

#[derive(Debug, AperCodec)]
#[asn(type = "BITSTRING", sz_extensible = false, sz_lb = "18", sz_ub = "18")]
pub struct BITSTRING33(BitVec<Msb0, u8>);

#[derive(Debug, AperCodec)]
#[asn(type = "BITSTRING", sz_extensible = false, sz_lb = "21", sz_ub = "21")]
pub struct BITSTRING34(BitVec<Msb0, u8>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct NgENBIDchoiceExtensions {}

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum NonDynamic5QIDescriptoriEExtensions_ItemextensionValue {
    #[asn(key = 187)]
    idCNPacketDelayBudgetDL(ExtendedPacketDelayBudget),
    #[asn(key = 188)]
    idCNPacketDelayBudgetUL(ExtendedPacketDelayBudget),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct NonDynamic5QIDescriptoriEExtensions_Item {
    #[asn(key_field = true)]
    pub id: ProtocolExtensionID,
    pub criticality: Criticality,
    pub extension_value: NonDynamic5QIDescriptoriEExtensions_ItemextensionValue,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "65535")]
pub struct NonDynamic5QIDescriptoriEExtensions(Vec<NonDynamic5QIDescriptoriEExtensions_Item>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct OverloadResponsechoiceExtensions {}

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum OverloadStartprotocolIEs_Itemvalue {
    #[asn(key = 2)]
    idAMFOverloadResponse(OverloadResponse),
    #[asn(key = 9)]
    idAMFTrafficLoadReductionIndication(TrafficLoadReductionIndication),
    #[asn(key = 49)]
    idOverloadStartNSSAIList(OverloadStartNSSAIList),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct OverloadStartprotocolIEs_Item {
    #[asn(key_field = true)]
    pub id: ProtocolIEID,
    pub criticality: Criticality,
    pub value: OverloadStartprotocolIEs_Itemvalue,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "0", sz_ub = "65535")]
pub struct OverloadStartprotocolIEs(Vec<OverloadStartprotocolIEs_Item>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct OverloadStartNSSAIItemiEExtensions_Item {}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "65535")]
pub struct OverloadStartNSSAIItemiEExtensions(Vec<OverloadStartNSSAIItemiEExtensions_Item>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct OverloadStopprotocolIEs_Item {}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "0", sz_ub = "65535")]
pub struct OverloadStopprotocolIEs(Vec<OverloadStopprotocolIEs_Item>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct PC5FlowBitRatesiEExtensions_Item {}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "65535")]
pub struct PC5FlowBitRatesiEExtensions(Vec<PC5FlowBitRatesiEExtensions_Item>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct PC5QoSFlowItemiEExtensions_Item {}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "65535")]
pub struct PC5QoSFlowItemiEExtensions(Vec<PC5QoSFlowItemiEExtensions_Item>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct PC5QoSParametersiEExtensions_Item {}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "65535")]
pub struct PC5QoSParametersiEExtensions(Vec<PC5QoSParametersiEExtensions_Item>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct PDUSessionAggregateMaximumBitRateiEExtensions_Item {}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "65535")]
pub struct PDUSessionAggregateMaximumBitRateiEExtensions(Vec<PDUSessionAggregateMaximumBitRateiEExtensions_Item>);

#[derive(Debug, AperCodec)]
#[asn(type = "OCTET-STRING")]
pub struct OCTETSTRING35(Vec<u8>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct PDUSessionResourceAdmittedItemiEExtensions_Item {}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "65535")]
pub struct PDUSessionResourceAdmittedItemiEExtensions(Vec<PDUSessionResourceAdmittedItemiEExtensions_Item>);

#[derive(Debug, AperCodec)]
#[asn(type = "OCTET-STRING")]
pub struct OCTETSTRING36(Vec<u8>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct PDUSessionResourceFailedToModifyItemModCfmiEExtensions_Item {}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "65535")]
pub struct PDUSessionResourceFailedToModifyItemModCfmiEExtensions(Vec<PDUSessionResourceFailedToModifyItemModCfmiEExtensions_Item>);

#[derive(Debug, AperCodec)]
#[asn(type = "OCTET-STRING")]
pub struct OCTETSTRING37(Vec<u8>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct PDUSessionResourceFailedToModifyItemModResiEExtensions_Item {}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "65535")]
pub struct PDUSessionResourceFailedToModifyItemModResiEExtensions(Vec<PDUSessionResourceFailedToModifyItemModResiEExtensions_Item>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct PDUSessionResourceFailedToResumeItemRESReqiEExtensions_Item {}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "65535")]
pub struct PDUSessionResourceFailedToResumeItemRESReqiEExtensions(Vec<PDUSessionResourceFailedToResumeItemRESReqiEExtensions_Item>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct PDUSessionResourceFailedToResumeItemRESResiEExtensions_Item {}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "65535")]
pub struct PDUSessionResourceFailedToResumeItemRESResiEExtensions(Vec<PDUSessionResourceFailedToResumeItemRESResiEExtensions_Item>);

#[derive(Debug, AperCodec)]
#[asn(type = "OCTET-STRING")]
pub struct OCTETSTRING38(Vec<u8>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct PDUSessionResourceFailedToSetupItemCxtFailiEExtensions_Item {}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "65535")]
pub struct PDUSessionResourceFailedToSetupItemCxtFailiEExtensions(Vec<PDUSessionResourceFailedToSetupItemCxtFailiEExtensions_Item>);

#[derive(Debug, AperCodec)]
#[asn(type = "OCTET-STRING")]
pub struct OCTETSTRING39(Vec<u8>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct PDUSessionResourceFailedToSetupItemCxtResiEExtensions_Item {}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "65535")]
pub struct PDUSessionResourceFailedToSetupItemCxtResiEExtensions(Vec<PDUSessionResourceFailedToSetupItemCxtResiEExtensions_Item>);

#[derive(Debug, AperCodec)]
#[asn(type = "OCTET-STRING")]
pub struct OCTETSTRING40(Vec<u8>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct PDUSessionResourceFailedToSetupItemHOAckiEExtensions_Item {}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "65535")]
pub struct PDUSessionResourceFailedToSetupItemHOAckiEExtensions(Vec<PDUSessionResourceFailedToSetupItemHOAckiEExtensions_Item>);

#[derive(Debug, AperCodec)]
#[asn(type = "OCTET-STRING")]
pub struct OCTETSTRING41(Vec<u8>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct PDUSessionResourceFailedToSetupItemPSReqiEExtensions_Item {}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "65535")]
pub struct PDUSessionResourceFailedToSetupItemPSReqiEExtensions(Vec<PDUSessionResourceFailedToSetupItemPSReqiEExtensions_Item>);

#[derive(Debug, AperCodec)]
#[asn(type = "OCTET-STRING")]
pub struct OCTETSTRING42(Vec<u8>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct PDUSessionResourceFailedToSetupItemSUResiEExtensions_Item {}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "65535")]
pub struct PDUSessionResourceFailedToSetupItemSUResiEExtensions(Vec<PDUSessionResourceFailedToSetupItemSUResiEExtensions_Item>);

#[derive(Debug, AperCodec)]
#[asn(type = "OCTET-STRING")]
pub struct OCTETSTRING43(Vec<u8>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct PDUSessionResourceHandoverItemiEExtensions_Item {}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "65535")]
pub struct PDUSessionResourceHandoverItemiEExtensions(Vec<PDUSessionResourceHandoverItemiEExtensions_Item>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct PDUSessionResourceInformationItemiEExtensions_Item {}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "65535")]
pub struct PDUSessionResourceInformationItemiEExtensions(Vec<PDUSessionResourceInformationItemiEExtensions_Item>);

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum PDUSessionResourceItemCxtRelCpliEExtensions_ItemextensionValue { }

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct PDUSessionResourceItemCxtRelCpliEExtensions_Item {
    #[asn(key_field = true)]
    pub id: ProtocolExtensionID,
    pub criticality: Criticality,
    pub extension_value: PDUSessionResourceItemCxtRelCpliEExtensions_ItemextensionValue,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "65535")]
pub struct PDUSessionResourceItemCxtRelCpliEExtensions(Vec<PDUSessionResourceItemCxtRelCpliEExtensions_Item>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct PDUSessionResourceItemCxtRelReqiEExtensions_Item {}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "65535")]
pub struct PDUSessionResourceItemCxtRelReqiEExtensions(Vec<PDUSessionResourceItemCxtRelReqiEExtensions_Item>);

#[derive(Debug, AperCodec)]
#[asn(type = "OCTET-STRING")]
pub struct OCTETSTRING44(Vec<u8>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct PDUSessionResourceItemHORqdiEExtensions_Item {}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "65535")]
pub struct PDUSessionResourceItemHORqdiEExtensions(Vec<PDUSessionResourceItemHORqdiEExtensions_Item>);

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum PDUSessionResourceModifyConfirmprotocolIEs_Itemvalue {
    #[asn(key = 10)]
    idAMFUENGAPID(AMFUENGAPID),
    #[asn(key = 19)]
    idCriticalityDiagnostics(CriticalityDiagnostics),
    #[asn(key = 131)]
    idPDUSessionResourceFailedToModifyListModCfm(PDUSessionResourceFailedToModifyListModCfm),
    #[asn(key = 62)]
    idPDUSessionResourceModifyListModCfm(PDUSessionResourceModifyListModCfm),
    #[asn(key = 85)]
    idRANUENGAPID(RANUENGAPID),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct PDUSessionResourceModifyConfirmprotocolIEs_Item {
    #[asn(key_field = true)]
    pub id: ProtocolIEID,
    pub criticality: Criticality,
    pub value: PDUSessionResourceModifyConfirmprotocolIEs_Itemvalue,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "0", sz_ub = "65535")]
pub struct PDUSessionResourceModifyConfirmprotocolIEs(Vec<PDUSessionResourceModifyConfirmprotocolIEs_Item>);

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum PDUSessionResourceModifyConfirmTransferiEExtensions_ItemextensionValue {
    #[asn(key = 185)]
    idAdditionalRedundantNGUUPTNLInformation(UPTransportLayerInformationPairList),
    #[asn(key = 195)]
    idRedundantULNGUUPTNLInformation(UPTransportLayerInformation),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct PDUSessionResourceModifyConfirmTransferiEExtensions_Item {
    #[asn(key_field = true)]
    pub id: ProtocolExtensionID,
    pub criticality: Criticality,
    pub extension_value: PDUSessionResourceModifyConfirmTransferiEExtensions_ItemextensionValue,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "65535")]
pub struct PDUSessionResourceModifyConfirmTransferiEExtensions(Vec<PDUSessionResourceModifyConfirmTransferiEExtensions_Item>);

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum PDUSessionResourceModifyIndicationprotocolIEs_Itemvalue {
    #[asn(key = 10)]
    idAMFUENGAPID(AMFUENGAPID),
    #[asn(key = 63)]
    idPDUSessionResourceModifyListModInd(PDUSessionResourceModifyListModInd),
    #[asn(key = 85)]
    idRANUENGAPID(RANUENGAPID),
    #[asn(key = 121)]
    idUserLocationInformation(UserLocationInformation),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct PDUSessionResourceModifyIndicationprotocolIEs_Item {
    #[asn(key_field = true)]
    pub id: ProtocolIEID,
    pub criticality: Criticality,
    pub value: PDUSessionResourceModifyIndicationprotocolIEs_Itemvalue,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "0", sz_ub = "65535")]
pub struct PDUSessionResourceModifyIndicationprotocolIEs(Vec<PDUSessionResourceModifyIndicationprotocolIEs_Item>);

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum PDUSessionResourceModifyIndicationTransferiEExtensions_ItemextensionValue {
    #[asn(key = 184)]
    idAdditionalRedundantDLQosFlowPerTNLInformation(QosFlowPerTNLInformationList),
    #[asn(key = 27)]
    idGlobalRANNodeID(GlobalRANNodeID),
    #[asn(key = 193)]
    idRedundantDLQosFlowPerTNLInformation(QosFlowPerTNLInformation),
    #[asn(key = 144)]
    idSecondaryRATUsageInformation(SecondaryRATUsageInformation),
    #[asn(key = 156)]
    idSecurityResult(SecurityResult),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct PDUSessionResourceModifyIndicationTransferiEExtensions_Item {
    #[asn(key_field = true)]
    pub id: ProtocolExtensionID,
    pub criticality: Criticality,
    pub extension_value: PDUSessionResourceModifyIndicationTransferiEExtensions_ItemextensionValue,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "65535")]
pub struct PDUSessionResourceModifyIndicationTransferiEExtensions(Vec<PDUSessionResourceModifyIndicationTransferiEExtensions_Item>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct PDUSessionResourceModifyIndicationUnsuccessfulTransferiEExtensions_Item {}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "65535")]
pub struct PDUSessionResourceModifyIndicationUnsuccessfulTransferiEExtensions(Vec<PDUSessionResourceModifyIndicationUnsuccessfulTransferiEExtensions_Item>);

#[derive(Debug, AperCodec)]
#[asn(type = "OCTET-STRING")]
pub struct OCTETSTRING45(Vec<u8>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct PDUSessionResourceModifyItemModCfmiEExtensions_Item {}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "65535")]
pub struct PDUSessionResourceModifyItemModCfmiEExtensions(Vec<PDUSessionResourceModifyItemModCfmiEExtensions_Item>);

#[derive(Debug, AperCodec)]
#[asn(type = "OCTET-STRING")]
pub struct OCTETSTRING46(Vec<u8>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct PDUSessionResourceModifyItemModIndiEExtensions_Item {}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "65535")]
pub struct PDUSessionResourceModifyItemModIndiEExtensions(Vec<PDUSessionResourceModifyItemModIndiEExtensions_Item>);

#[derive(Debug, AperCodec)]
#[asn(type = "OCTET-STRING")]
pub struct OCTETSTRING47(Vec<u8>);

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum PDUSessionResourceModifyItemModReqiEExtensions_ItemextensionValue {
    #[asn(key = 148)]
    idSNSSAI(SNSSAI),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct PDUSessionResourceModifyItemModReqiEExtensions_Item {
    #[asn(key_field = true)]
    pub id: ProtocolExtensionID,
    pub criticality: Criticality,
    pub extension_value: PDUSessionResourceModifyItemModReqiEExtensions_ItemextensionValue,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "65535")]
pub struct PDUSessionResourceModifyItemModReqiEExtensions(Vec<PDUSessionResourceModifyItemModReqiEExtensions_Item>);

#[derive(Debug, AperCodec)]
#[asn(type = "OCTET-STRING")]
pub struct OCTETSTRING48(Vec<u8>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct PDUSessionResourceModifyItemModResiEExtensions_Item {}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "65535")]
pub struct PDUSessionResourceModifyItemModResiEExtensions(Vec<PDUSessionResourceModifyItemModResiEExtensions_Item>);

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum PDUSessionResourceModifyRequestprotocolIEs_Itemvalue {
    #[asn(key = 10)]
    idAMFUENGAPID(AMFUENGAPID),
    #[asn(key = 64)]
    idPDUSessionResourceModifyListModReq(PDUSessionResourceModifyListModReq),
    #[asn(key = 85)]
    idRANUENGAPID(RANUENGAPID),
    #[asn(key = 83)]
    idRANPagingPriority(RANPagingPriority),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct PDUSessionResourceModifyRequestprotocolIEs_Item {
    #[asn(key_field = true)]
    pub id: ProtocolIEID,
    pub criticality: Criticality,
    pub value: PDUSessionResourceModifyRequestprotocolIEs_Itemvalue,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "0", sz_ub = "65535")]
pub struct PDUSessionResourceModifyRequestprotocolIEs(Vec<PDUSessionResourceModifyRequestprotocolIEs_Item>);

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum PDUSessionResourceModifyRequestTransferprotocolIEs_Itemvalue {
    #[asn(key = 186)]
    idAdditionalRedundantULNGUUPTNLInformation(UPTransportLayerInformationList),
    #[asn(key = 126)]
    idAdditionalULNGUUPTNLInformation(UPTransportLayerInformationList),
    #[asn(key = 166)]
    idCommonNetworkInstance(CommonNetworkInstance),
    #[asn(key = 129)]
    idNetworkInstance(NetworkInstance),
    #[asn(key = 130)]
    idPDUSessionAggregateMaximumBitRate(PDUSessionAggregateMaximumBitRate),
    #[asn(key = 135)]
    idQosFlowAddOrModifyRequestList(QosFlowAddOrModifyRequestList),
    #[asn(key = 137)]
    idQosFlowToReleaseList(QosFlowListWithCause),
    #[asn(key = 190)]
    idRedundantCommonNetworkInstance(CommonNetworkInstance),
    #[asn(key = 195)]
    idRedundantULNGUUPTNLInformation(UPTransportLayerInformation),
    #[asn(key = 138)]
    idSecurityIndication(SecurityIndication),
    #[asn(key = 140)]
    idULNGUUPTNLModifyList(ULNGUUPTNLModifyList),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct PDUSessionResourceModifyRequestTransferprotocolIEs_Item {
    #[asn(key_field = true)]
    pub id: ProtocolIEID,
    pub criticality: Criticality,
    pub value: PDUSessionResourceModifyRequestTransferprotocolIEs_Itemvalue,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "0", sz_ub = "65535")]
pub struct PDUSessionResourceModifyRequestTransferprotocolIEs(Vec<PDUSessionResourceModifyRequestTransferprotocolIEs_Item>);

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum PDUSessionResourceModifyResponseprotocolIEs_Itemvalue {
    #[asn(key = 10)]
    idAMFUENGAPID(AMFUENGAPID),
    #[asn(key = 19)]
    idCriticalityDiagnostics(CriticalityDiagnostics),
    #[asn(key = 54)]
    idPDUSessionResourceFailedToModifyListModRes(PDUSessionResourceFailedToModifyListModRes),
    #[asn(key = 65)]
    idPDUSessionResourceModifyListModRes(PDUSessionResourceModifyListModRes),
    #[asn(key = 85)]
    idRANUENGAPID(RANUENGAPID),
    #[asn(key = 121)]
    idUserLocationInformation(UserLocationInformation),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct PDUSessionResourceModifyResponseprotocolIEs_Item {
    #[asn(key_field = true)]
    pub id: ProtocolIEID,
    pub criticality: Criticality,
    pub value: PDUSessionResourceModifyResponseprotocolIEs_Itemvalue,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "0", sz_ub = "65535")]
pub struct PDUSessionResourceModifyResponseprotocolIEs(Vec<PDUSessionResourceModifyResponseprotocolIEs_Item>);

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum PDUSessionResourceModifyResponseTransferiEExtensions_ItemextensionValue {
    #[asn(key = 154)]
    idAdditionalNGUUPTNLInformation(UPTransportLayerInformationPairList),
    #[asn(key = 184)]
    idAdditionalRedundantDLQosFlowPerTNLInformation(QosFlowPerTNLInformationList),
    #[asn(key = 185)]
    idAdditionalRedundantNGUUPTNLInformation(UPTransportLayerInformationPairList),
    #[asn(key = 192)]
    idRedundantDLNGUUPTNLInformation(UPTransportLayerInformation),
    #[asn(key = 195)]
    idRedundantULNGUUPTNLInformation(UPTransportLayerInformation),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct PDUSessionResourceModifyResponseTransferiEExtensions_Item {
    #[asn(key_field = true)]
    pub id: ProtocolExtensionID,
    pub criticality: Criticality,
    pub extension_value: PDUSessionResourceModifyResponseTransferiEExtensions_ItemextensionValue,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "65535")]
pub struct PDUSessionResourceModifyResponseTransferiEExtensions(Vec<PDUSessionResourceModifyResponseTransferiEExtensions_Item>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct PDUSessionResourceModifyUnsuccessfulTransferiEExtensions_Item {}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "65535")]
pub struct PDUSessionResourceModifyUnsuccessfulTransferiEExtensions(Vec<PDUSessionResourceModifyUnsuccessfulTransferiEExtensions_Item>);

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum PDUSessionResourceNotifyprotocolIEs_Itemvalue {
    #[asn(key = 10)]
    idAMFUENGAPID(AMFUENGAPID),
    #[asn(key = 66)]
    idPDUSessionResourceNotifyList(PDUSessionResourceNotifyList),
    #[asn(key = 67)]
    idPDUSessionResourceReleasedListNot(PDUSessionResourceReleasedListNot),
    #[asn(key = 85)]
    idRANUENGAPID(RANUENGAPID),
    #[asn(key = 121)]
    idUserLocationInformation(UserLocationInformation),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct PDUSessionResourceNotifyprotocolIEs_Item {
    #[asn(key_field = true)]
    pub id: ProtocolIEID,
    pub criticality: Criticality,
    pub value: PDUSessionResourceNotifyprotocolIEs_Itemvalue,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "0", sz_ub = "65535")]
pub struct PDUSessionResourceNotifyprotocolIEs(Vec<PDUSessionResourceNotifyprotocolIEs_Item>);

#[derive(Debug, AperCodec)]
#[asn(type = "OCTET-STRING")]
pub struct OCTETSTRING49(Vec<u8>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct PDUSessionResourceNotifyItemiEExtensions_Item {}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "65535")]
pub struct PDUSessionResourceNotifyItemiEExtensions(Vec<PDUSessionResourceNotifyItemiEExtensions_Item>);

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum PDUSessionResourceNotifyReleasedTransferiEExtensions_ItemextensionValue {
    #[asn(key = 144)]
    idSecondaryRATUsageInformation(SecondaryRATUsageInformation),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct PDUSessionResourceNotifyReleasedTransferiEExtensions_Item {
    #[asn(key_field = true)]
    pub id: ProtocolExtensionID,
    pub criticality: Criticality,
    pub extension_value: PDUSessionResourceNotifyReleasedTransferiEExtensions_ItemextensionValue,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "65535")]
pub struct PDUSessionResourceNotifyReleasedTransferiEExtensions(Vec<PDUSessionResourceNotifyReleasedTransferiEExtensions_Item>);

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum PDUSessionResourceNotifyTransferiEExtensions_ItemextensionValue {
    #[asn(key = 278)]
    idQosFlowFeedbackList(QosFlowFeedbackList),
    #[asn(key = 144)]
    idSecondaryRATUsageInformation(SecondaryRATUsageInformation),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct PDUSessionResourceNotifyTransferiEExtensions_Item {
    #[asn(key_field = true)]
    pub id: ProtocolExtensionID,
    pub criticality: Criticality,
    pub extension_value: PDUSessionResourceNotifyTransferiEExtensions_ItemextensionValue,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "65535")]
pub struct PDUSessionResourceNotifyTransferiEExtensions(Vec<PDUSessionResourceNotifyTransferiEExtensions_Item>);

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum PDUSessionResourceReleaseCommandprotocolIEs_Itemvalue {
    #[asn(key = 10)]
    idAMFUENGAPID(AMFUENGAPID),
    #[asn(key = 38)]
    idNASPDU(NASPDU),
    #[asn(key = 79)]
    idPDUSessionResourceToReleaseListRelCmd(PDUSessionResourceToReleaseListRelCmd),
    #[asn(key = 85)]
    idRANUENGAPID(RANUENGAPID),
    #[asn(key = 83)]
    idRANPagingPriority(RANPagingPriority),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct PDUSessionResourceReleaseCommandprotocolIEs_Item {
    #[asn(key_field = true)]
    pub id: ProtocolIEID,
    pub criticality: Criticality,
    pub value: PDUSessionResourceReleaseCommandprotocolIEs_Itemvalue,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "0", sz_ub = "65535")]
pub struct PDUSessionResourceReleaseCommandprotocolIEs(Vec<PDUSessionResourceReleaseCommandprotocolIEs_Item>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct PDUSessionResourceReleaseCommandTransferiEExtensions_Item {}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "65535")]
pub struct PDUSessionResourceReleaseCommandTransferiEExtensions(Vec<PDUSessionResourceReleaseCommandTransferiEExtensions_Item>);

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum PDUSessionResourceReleaseResponseprotocolIEs_Itemvalue {
    #[asn(key = 10)]
    idAMFUENGAPID(AMFUENGAPID),
    #[asn(key = 19)]
    idCriticalityDiagnostics(CriticalityDiagnostics),
    #[asn(key = 70)]
    idPDUSessionResourceReleasedListRelRes(PDUSessionResourceReleasedListRelRes),
    #[asn(key = 85)]
    idRANUENGAPID(RANUENGAPID),
    #[asn(key = 121)]
    idUserLocationInformation(UserLocationInformation),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct PDUSessionResourceReleaseResponseprotocolIEs_Item {
    #[asn(key_field = true)]
    pub id: ProtocolIEID,
    pub criticality: Criticality,
    pub value: PDUSessionResourceReleaseResponseprotocolIEs_Itemvalue,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "0", sz_ub = "65535")]
pub struct PDUSessionResourceReleaseResponseprotocolIEs(Vec<PDUSessionResourceReleaseResponseprotocolIEs_Item>);

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum PDUSessionResourceReleaseResponseTransferiEExtensions_ItemextensionValue {
    #[asn(key = 144)]
    idSecondaryRATUsageInformation(SecondaryRATUsageInformation),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct PDUSessionResourceReleaseResponseTransferiEExtensions_Item {
    #[asn(key_field = true)]
    pub id: ProtocolExtensionID,
    pub criticality: Criticality,
    pub extension_value: PDUSessionResourceReleaseResponseTransferiEExtensions_ItemextensionValue,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "65535")]
pub struct PDUSessionResourceReleaseResponseTransferiEExtensions(Vec<PDUSessionResourceReleaseResponseTransferiEExtensions_Item>);

#[derive(Debug, AperCodec)]
#[asn(type = "OCTET-STRING")]
pub struct OCTETSTRING50(Vec<u8>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct PDUSessionResourceReleasedItemNotiEExtensions_Item {}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "65535")]
pub struct PDUSessionResourceReleasedItemNotiEExtensions(Vec<PDUSessionResourceReleasedItemNotiEExtensions_Item>);

#[derive(Debug, AperCodec)]
#[asn(type = "OCTET-STRING")]
pub struct OCTETSTRING51(Vec<u8>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct PDUSessionResourceReleasedItemPSAckiEExtensions_Item {}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "65535")]
pub struct PDUSessionResourceReleasedItemPSAckiEExtensions(Vec<PDUSessionResourceReleasedItemPSAckiEExtensions_Item>);

#[derive(Debug, AperCodec)]
#[asn(type = "OCTET-STRING")]
pub struct OCTETSTRING52(Vec<u8>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct PDUSessionResourceReleasedItemPSFailiEExtensions_Item {}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "65535")]
pub struct PDUSessionResourceReleasedItemPSFailiEExtensions(Vec<PDUSessionResourceReleasedItemPSFailiEExtensions_Item>);

#[derive(Debug, AperCodec)]
#[asn(type = "OCTET-STRING")]
pub struct OCTETSTRING53(Vec<u8>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct PDUSessionResourceReleasedItemRelResiEExtensions_Item {}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "65535")]
pub struct PDUSessionResourceReleasedItemRelResiEExtensions(Vec<PDUSessionResourceReleasedItemRelResiEExtensions_Item>);

#[derive(Debug, AperCodec)]
#[asn(type = "OCTET-STRING")]
pub struct OCTETSTRING54(Vec<u8>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct PDUSessionResourceResumeItemRESReqiEExtensions_Item {}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "65535")]
pub struct PDUSessionResourceResumeItemRESReqiEExtensions(Vec<PDUSessionResourceResumeItemRESReqiEExtensions_Item>);

#[derive(Debug, AperCodec)]
#[asn(type = "OCTET-STRING")]
pub struct OCTETSTRING55(Vec<u8>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct PDUSessionResourceResumeItemRESResiEExtensions_Item {}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "65535")]
pub struct PDUSessionResourceResumeItemRESResiEExtensions(Vec<PDUSessionResourceResumeItemRESResiEExtensions_Item>);

#[derive(Debug, AperCodec)]
#[asn(type = "OCTET-STRING")]
pub struct OCTETSTRING56(Vec<u8>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct PDUSessionResourceSecondaryRATUsageItemiEExtensions_Item {}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "65535")]
pub struct PDUSessionResourceSecondaryRATUsageItemiEExtensions(Vec<PDUSessionResourceSecondaryRATUsageItemiEExtensions_Item>);

#[derive(Debug, AperCodec)]
#[asn(type = "OCTET-STRING")]
pub struct OCTETSTRING57(Vec<u8>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct PDUSessionResourceSetupItemCxtReqiEExtensions_Item {}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "65535")]
pub struct PDUSessionResourceSetupItemCxtReqiEExtensions(Vec<PDUSessionResourceSetupItemCxtReqiEExtensions_Item>);

#[derive(Debug, AperCodec)]
#[asn(type = "OCTET-STRING")]
pub struct OCTETSTRING58(Vec<u8>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct PDUSessionResourceSetupItemCxtResiEExtensions_Item {}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "65535")]
pub struct PDUSessionResourceSetupItemCxtResiEExtensions(Vec<PDUSessionResourceSetupItemCxtResiEExtensions_Item>);

#[derive(Debug, AperCodec)]
#[asn(type = "OCTET-STRING")]
pub struct OCTETSTRING59(Vec<u8>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct PDUSessionResourceSetupItemHOReqiEExtensions_Item {}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "65535")]
pub struct PDUSessionResourceSetupItemHOReqiEExtensions(Vec<PDUSessionResourceSetupItemHOReqiEExtensions_Item>);

#[derive(Debug, AperCodec)]
#[asn(type = "OCTET-STRING")]
pub struct OCTETSTRING60(Vec<u8>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct PDUSessionResourceSetupItemSUReqiEExtensions_Item {}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "65535")]
pub struct PDUSessionResourceSetupItemSUReqiEExtensions(Vec<PDUSessionResourceSetupItemSUReqiEExtensions_Item>);

#[derive(Debug, AperCodec)]
#[asn(type = "OCTET-STRING")]
pub struct OCTETSTRING61(Vec<u8>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct PDUSessionResourceSetupItemSUResiEExtensions_Item {}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "65535")]
pub struct PDUSessionResourceSetupItemSUResiEExtensions(Vec<PDUSessionResourceSetupItemSUResiEExtensions_Item>);

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum PDUSessionResourceSetupRequestprotocolIEs_Itemvalue {
    #[asn(key = 10)]
    idAMFUENGAPID(AMFUENGAPID),
    #[asn(key = 38)]
    idNASPDU(NASPDU),
    #[asn(key = 74)]
    idPDUSessionResourceSetupListSUReq(PDUSessionResourceSetupListSUReq),
    #[asn(key = 85)]
    idRANUENGAPID(RANUENGAPID),
    #[asn(key = 83)]
    idRANPagingPriority(RANPagingPriority),
    #[asn(key = 110)]
    idUEAggregateMaximumBitRate(UEAggregateMaximumBitRate),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct PDUSessionResourceSetupRequestprotocolIEs_Item {
    #[asn(key_field = true)]
    pub id: ProtocolIEID,
    pub criticality: Criticality,
    pub value: PDUSessionResourceSetupRequestprotocolIEs_Itemvalue,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "0", sz_ub = "65535")]
pub struct PDUSessionResourceSetupRequestprotocolIEs(Vec<PDUSessionResourceSetupRequestprotocolIEs_Item>);

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum PDUSessionResourceSetupRequestTransferprotocolIEs_Itemvalue {
    #[asn(key = 186)]
    idAdditionalRedundantULNGUUPTNLInformation(UPTransportLayerInformationList),
    #[asn(key = 126)]
    idAdditionalULNGUUPTNLInformation(UPTransportLayerInformationList),
    #[asn(key = 166)]
    idCommonNetworkInstance(CommonNetworkInstance),
    #[asn(key = 127)]
    idDataForwardingNotPossible(DataForwardingNotPossible),
    #[asn(key = 22)]
    idDirectForwardingPathAvailability(DirectForwardingPathAvailability),
    #[asn(key = 129)]
    idNetworkInstance(NetworkInstance),
    #[asn(key = 130)]
    idPDUSessionAggregateMaximumBitRate(PDUSessionAggregateMaximumBitRate),
    #[asn(key = 134)]
    idPDUSessionType(PDUSessionType),
    #[asn(key = 136)]
    idQosFlowSetupRequestList(QosFlowSetupRequestList),
    #[asn(key = 190)]
    idRedundantCommonNetworkInstance(CommonNetworkInstance),
    #[asn(key = 197)]
    idRedundantPDUSessionInformation(RedundantPDUSessionInformation),
    #[asn(key = 195)]
    idRedundantULNGUUPTNLInformation(UPTransportLayerInformation),
    #[asn(key = 138)]
    idSecurityIndication(SecurityIndication),
    #[asn(key = 139)]
    idULNGUUPTNLInformation(UPTransportLayerInformation),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct PDUSessionResourceSetupRequestTransferprotocolIEs_Item {
    #[asn(key_field = true)]
    pub id: ProtocolIEID,
    pub criticality: Criticality,
    pub value: PDUSessionResourceSetupRequestTransferprotocolIEs_Itemvalue,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "0", sz_ub = "65535")]
pub struct PDUSessionResourceSetupRequestTransferprotocolIEs(Vec<PDUSessionResourceSetupRequestTransferprotocolIEs_Item>);

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum PDUSessionResourceSetupResponseprotocolIEs_Itemvalue {
    #[asn(key = 10)]
    idAMFUENGAPID(AMFUENGAPID),
    #[asn(key = 19)]
    idCriticalityDiagnostics(CriticalityDiagnostics),
    #[asn(key = 58)]
    idPDUSessionResourceFailedToSetupListSURes(PDUSessionResourceFailedToSetupListSURes),
    #[asn(key = 75)]
    idPDUSessionResourceSetupListSURes(PDUSessionResourceSetupListSURes),
    #[asn(key = 85)]
    idRANUENGAPID(RANUENGAPID),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct PDUSessionResourceSetupResponseprotocolIEs_Item {
    #[asn(key_field = true)]
    pub id: ProtocolIEID,
    pub criticality: Criticality,
    pub value: PDUSessionResourceSetupResponseprotocolIEs_Itemvalue,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "0", sz_ub = "65535")]
pub struct PDUSessionResourceSetupResponseprotocolIEs(Vec<PDUSessionResourceSetupResponseprotocolIEs_Item>);

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum PDUSessionResourceSetupResponseTransferiEExtensions_ItemextensionValue {
    #[asn(key = 184)]
    idAdditionalRedundantDLQosFlowPerTNLInformation(QosFlowPerTNLInformationList),
    #[asn(key = 27)]
    idGlobalRANNodeID(GlobalRANNodeID),
    #[asn(key = 193)]
    idRedundantDLQosFlowPerTNLInformation(QosFlowPerTNLInformation),
    #[asn(key = 198)]
    idUsedRSNInformation(RedundantPDUSessionInformation),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct PDUSessionResourceSetupResponseTransferiEExtensions_Item {
    #[asn(key_field = true)]
    pub id: ProtocolExtensionID,
    pub criticality: Criticality,
    pub extension_value: PDUSessionResourceSetupResponseTransferiEExtensions_ItemextensionValue,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "65535")]
pub struct PDUSessionResourceSetupResponseTransferiEExtensions(Vec<PDUSessionResourceSetupResponseTransferiEExtensions_Item>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct PDUSessionResourceSetupUnsuccessfulTransferiEExtensions_Item {}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "65535")]
pub struct PDUSessionResourceSetupUnsuccessfulTransferiEExtensions(Vec<PDUSessionResourceSetupUnsuccessfulTransferiEExtensions_Item>);

#[derive(Debug, AperCodec)]
#[asn(type = "OCTET-STRING")]
pub struct OCTETSTRING62(Vec<u8>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct PDUSessionResourceSuspendItemSUSReqiEExtensions_Item {}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "65535")]
pub struct PDUSessionResourceSuspendItemSUSReqiEExtensions(Vec<PDUSessionResourceSuspendItemSUSReqiEExtensions_Item>);

#[derive(Debug, AperCodec)]
#[asn(type = "OCTET-STRING")]
pub struct OCTETSTRING63(Vec<u8>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct PDUSessionResourceSwitchedItemiEExtensions_Item {}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "65535")]
pub struct PDUSessionResourceSwitchedItemiEExtensions(Vec<PDUSessionResourceSwitchedItemiEExtensions_Item>);

#[derive(Debug, AperCodec)]
#[asn(type = "OCTET-STRING")]
pub struct OCTETSTRING64(Vec<u8>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct PDUSessionResourceToBeSwitchedDLItemiEExtensions_Item {}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "65535")]
pub struct PDUSessionResourceToBeSwitchedDLItemiEExtensions(Vec<PDUSessionResourceToBeSwitchedDLItemiEExtensions_Item>);

#[derive(Debug, AperCodec)]
#[asn(type = "OCTET-STRING")]
pub struct OCTETSTRING65(Vec<u8>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct PDUSessionResourceToReleaseItemHOCmdiEExtensions_Item {}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "65535")]
pub struct PDUSessionResourceToReleaseItemHOCmdiEExtensions(Vec<PDUSessionResourceToReleaseItemHOCmdiEExtensions_Item>);

#[derive(Debug, AperCodec)]
#[asn(type = "OCTET-STRING")]
pub struct OCTETSTRING66(Vec<u8>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct PDUSessionResourceToReleaseItemRelCmdiEExtensions_Item {}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "65535")]
pub struct PDUSessionResourceToReleaseItemRelCmdiEExtensions(Vec<PDUSessionResourceToReleaseItemRelCmdiEExtensions_Item>);

#[derive(Debug, AperCodec)]
#[asn(type = "ENUMERATED", extensible = true, lb = "0", ub = "1")]
pub struct ENUMERATED67(u8);
impl ENUMERATED67 {
    const NR: u8 = 0u8;
    const EUTRA: u8 = 1u8;
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct PDUSessionUsageReportiEExtensions_Item {}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "65535")]
pub struct PDUSessionUsageReportiEExtensions(Vec<PDUSessionUsageReportiEExtensions_Item>);

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum PLMNSupportItemiEExtensions_ItemextensionValue {
    #[asn(key = 270)]
    idExtendedSliceSupportList(ExtendedSliceSupportList),
    #[asn(key = 258)]
    idNPNSupport(NPNSupport),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct PLMNSupportItemiEExtensions_Item {
    #[asn(key_field = true)]
    pub id: ProtocolExtensionID,
    pub criticality: Criticality,
    pub extension_value: PLMNSupportItemiEExtensions_ItemextensionValue,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "65535")]
pub struct PLMNSupportItemiEExtensions(Vec<PLMNSupportItemiEExtensions_Item>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct PNINPNMobilityInformationiEExtensions_Item {}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "65535")]
pub struct PNINPNMobilityInformationiEExtensions(Vec<PNINPNMobilityInformationiEExtensions_Item>);

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum PWSCancelRequestprotocolIEs_Itemvalue {
    #[asn(key = 14)]
    idCancelAllWarningMessages(CancelAllWarningMessages),
    #[asn(key = 35)]
    idMessageIdentifier(MessageIdentifier),
    #[asn(key = 95)]
    idSerialNumber(SerialNumber),
    #[asn(key = 122)]
    idWarningAreaList(WarningAreaList),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct PWSCancelRequestprotocolIEs_Item {
    #[asn(key_field = true)]
    pub id: ProtocolIEID,
    pub criticality: Criticality,
    pub value: PWSCancelRequestprotocolIEs_Itemvalue,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "0", sz_ub = "65535")]
pub struct PWSCancelRequestprotocolIEs(Vec<PWSCancelRequestprotocolIEs_Item>);

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum PWSCancelResponseprotocolIEs_Itemvalue {
    #[asn(key = 12)]
    idBroadcastCancelledAreaList(BroadcastCancelledAreaList),
    #[asn(key = 19)]
    idCriticalityDiagnostics(CriticalityDiagnostics),
    #[asn(key = 35)]
    idMessageIdentifier(MessageIdentifier),
    #[asn(key = 95)]
    idSerialNumber(SerialNumber),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct PWSCancelResponseprotocolIEs_Item {
    #[asn(key_field = true)]
    pub id: ProtocolIEID,
    pub criticality: Criticality,
    pub value: PWSCancelResponseprotocolIEs_Itemvalue,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "0", sz_ub = "65535")]
pub struct PWSCancelResponseprotocolIEs(Vec<PWSCancelResponseprotocolIEs_Item>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct PWSFailedCellIDListchoiceExtensions {}

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum PWSFailureIndicationprotocolIEs_Itemvalue {
    #[asn(key = 27)]
    idGlobalRANNodeID(GlobalRANNodeID),
    #[asn(key = 81)]
    idPWSFailedCellIDList(PWSFailedCellIDList),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct PWSFailureIndicationprotocolIEs_Item {
    #[asn(key_field = true)]
    pub id: ProtocolIEID,
    pub criticality: Criticality,
    pub value: PWSFailureIndicationprotocolIEs_Itemvalue,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "0", sz_ub = "65535")]
pub struct PWSFailureIndicationprotocolIEs(Vec<PWSFailureIndicationprotocolIEs_Item>);

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum PWSRestartIndicationprotocolIEs_Itemvalue {
    #[asn(key = 16)]
    idCellIDListForRestart(CellIDListForRestart),
    #[asn(key = 23)]
    idEmergencyAreaIDListForRestart(EmergencyAreaIDListForRestart),
    #[asn(key = 27)]
    idGlobalRANNodeID(GlobalRANNodeID),
    #[asn(key = 104)]
    idTAIListForRestart(TAIListForRestart),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct PWSRestartIndicationprotocolIEs_Item {
    #[asn(key_field = true)]
    pub id: ProtocolIEID,
    pub criticality: Criticality,
    pub value: PWSRestartIndicationprotocolIEs_Itemvalue,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "0", sz_ub = "65535")]
pub struct PWSRestartIndicationprotocolIEs(Vec<PWSRestartIndicationprotocolIEs_Item>);

#[derive(Debug, AperCodec)]
#[asn(type = "INTEGER", lb = "0", ub = "9", extensible = true)]
pub struct INTEGER68(u8);

#[derive(Debug, AperCodec)]
#[asn(type = "INTEGER", lb = "0", ub = "9", extensible = true)]
pub struct INTEGER69(u8);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct PacketErrorRateiEExtensions_Item {}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "65535")]
pub struct PacketErrorRateiEExtensions(Vec<PacketErrorRateiEExtensions_Item>);

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum PagingprotocolIEs_Itemvalue {
    #[asn(key = 11)]
    idAssistanceDataForPaging(AssistanceDataForPaging),
    #[asn(key = 222)]
    idCEmodeBrestricted(CEmodeBrestricted),
    #[asn(key = 205)]
    idEnhancedCoverageRestriction(EnhancedCoverageRestriction),
    #[asn(key = 203)]
    idNBIoTPagingeDRXInfo(NBIoTPagingeDRXInfo),
    #[asn(key = 202)]
    idNBIoTPagingDRX(NBIoTPagingDRX),
    #[asn(key = 50)]
    idPagingDRX(PagingDRX),
    #[asn(key = 51)]
    idPagingOrigin(PagingOrigin),
    #[asn(key = 52)]
    idPagingPriority(PagingPriority),
    #[asn(key = 223)]
    idPagingeDRXInformation(PagingeDRXInformation),
    #[asn(key = 103)]
    idTAIListForPaging(TAIListForPaging),
    #[asn(key = 115)]
    idUEPagingIdentity(UEPagingIdentity),
    #[asn(key = 118)]
    idUERadioCapabilityForPaging(UERadioCapabilityForPaging),
    #[asn(key = 208)]
    idWUSAssistanceInformation(WUSAssistanceInformation),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct PagingprotocolIEs_Item {
    #[asn(key_field = true)]
    pub id: ProtocolIEID,
    pub criticality: Criticality,
    pub value: PagingprotocolIEs_Itemvalue,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "0", sz_ub = "65535")]
pub struct PagingprotocolIEs(Vec<PagingprotocolIEs_Item>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct PagingAssisDataforCEcapabUEiEExtensions_Item {}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "65535")]
pub struct PagingAssisDataforCEcapabUEiEExtensions(Vec<PagingAssisDataforCEcapabUEiEExtensions_Item>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct PagingAttemptInformationiEExtensions_Item {}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "65535")]
pub struct PagingAttemptInformationiEExtensions(Vec<PagingAttemptInformationiEExtensions_Item>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct PagingeDRXInformationiEExtensions_Item {}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "65535")]
pub struct PagingeDRXInformationiEExtensions(Vec<PagingeDRXInformationiEExtensions_Item>);

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum PathSwitchRequestprotocolIEs_Itemvalue {
    #[asn(key = 57)]
    idPDUSessionResourceFailedToSetupListPSReq(PDUSessionResourceFailedToSetupListPSReq),
    #[asn(key = 76)]
    idPDUSessionResourceToBeSwitchedDLList(PDUSessionResourceToBeSwitchedDLList),
    #[asn(key = 85)]
    idRANUENGAPID(RANUENGAPID),
    #[asn(key = 237)]
    idRRCResumeCause(RRCEstablishmentCause),
    #[asn(key = 100)]
    idSourceAMFUENGAPID(AMFUENGAPID),
    #[asn(key = 119)]
    idUESecurityCapabilities(UESecurityCapabilities),
    #[asn(key = 121)]
    idUserLocationInformation(UserLocationInformation),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct PathSwitchRequestprotocolIEs_Item {
    #[asn(key_field = true)]
    pub id: ProtocolIEID,
    pub criticality: Criticality,
    pub value: PathSwitchRequestprotocolIEs_Itemvalue,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "0", sz_ub = "65535")]
pub struct PathSwitchRequestprotocolIEs(Vec<PathSwitchRequestprotocolIEs_Item>);

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum PathSwitchRequestAcknowledgeprotocolIEs_Itemvalue {
    #[asn(key = 10)]
    idAMFUENGAPID(AMFUENGAPID),
    #[asn(key = 0)]
    idAllowedNSSAI(AllowedNSSAI),
    #[asn(key = 222)]
    idCEmodeBrestricted(CEmodeBrestricted),
    #[asn(key = 165)]
    idCNAssistedRANTuning(CNAssistedRANTuning),
    #[asn(key = 18)]
    idCoreNetworkAssistanceInformationForInactive(CoreNetworkAssistanceInformationForInactive),
    #[asn(key = 19)]
    idCriticalityDiagnostics(CriticalityDiagnostics),
    #[asn(key = 205)]
    idEnhancedCoverageRestriction(EnhancedCoverageRestriction),
    #[asn(key = 206)]
    idExtendedConnectedTime(ExtendedConnectedTime),
    #[asn(key = 217)]
    idLTEUESidelinkAggregateMaximumBitrate(LTEUESidelinkAggregateMaximumBitrate),
    #[asn(key = 215)]
    idLTEV2XServicesAuthorized(LTEV2XServicesAuthorized),
    #[asn(key = 218)]
    idNRUESidelinkAggregateMaximumBitrate(NRUESidelinkAggregateMaximumBitrate),
    #[asn(key = 216)]
    idNRV2XServicesAuthorized(NRV2XServicesAuthorized),
    #[asn(key = 41)]
    idNewSecurityContextInd(NewSecurityContextInd),
    #[asn(key = 219)]
    idPC5QoSParameters(PC5QoSParameters),
    #[asn(key = 68)]
    idPDUSessionResourceReleasedListPSAck(PDUSessionResourceReleasedListPSAck),
    #[asn(key = 77)]
    idPDUSessionResourceSwitchedList(PDUSessionResourceSwitchedList),
    #[asn(key = 85)]
    idRANUENGAPID(RANUENGAPID),
    #[asn(key = 91)]
    idRRCInactiveTransitionReportRequest(RRCInactiveTransitionReportRequest),
    #[asn(key = 146)]
    idRedirectionVoiceFallback(RedirectionVoiceFallback),
    #[asn(key = 177)]
    idSRVCCOperationPossible(SRVCCOperationPossible),
    #[asn(key = 93)]
    idSecurityContext(SecurityContext),
    #[asn(key = 209)]
    idUEDifferentiationInfo(UEDifferentiationInfo),
    #[asn(key = 234)]
    idUEUPCIoTSupport(UEUPCIoTSupport),
    #[asn(key = 264)]
    idUERadioCapabilityID(UERadioCapabilityID),
    #[asn(key = 119)]
    idUESecurityCapabilities(UESecurityCapabilities),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct PathSwitchRequestAcknowledgeprotocolIEs_Item {
    #[asn(key_field = true)]
    pub id: ProtocolIEID,
    pub criticality: Criticality,
    pub value: PathSwitchRequestAcknowledgeprotocolIEs_Itemvalue,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "0", sz_ub = "65535")]
pub struct PathSwitchRequestAcknowledgeprotocolIEs(Vec<PathSwitchRequestAcknowledgeprotocolIEs_Item>);

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum PathSwitchRequestAcknowledgeTransferiEExtensions_ItemextensionValue {
    #[asn(key = 154)]
    idAdditionalNGUUPTNLInformation(UPTransportLayerInformationPairList),
    #[asn(key = 185)]
    idAdditionalRedundantNGUUPTNLInformation(UPTransportLayerInformationPairList),
    #[asn(key = 277)]
    idQosFlowParametersList(QosFlowParametersList),
    #[asn(key = 195)]
    idRedundantULNGUUPTNLInformation(UPTransportLayerInformation),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct PathSwitchRequestAcknowledgeTransferiEExtensions_Item {
    #[asn(key_field = true)]
    pub id: ProtocolExtensionID,
    pub criticality: Criticality,
    pub extension_value: PathSwitchRequestAcknowledgeTransferiEExtensions_ItemextensionValue,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "65535")]
pub struct PathSwitchRequestAcknowledgeTransferiEExtensions(Vec<PathSwitchRequestAcknowledgeTransferiEExtensions_Item>);

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum PathSwitchRequestFailureprotocolIEs_Itemvalue {
    #[asn(key = 10)]
    idAMFUENGAPID(AMFUENGAPID),
    #[asn(key = 19)]
    idCriticalityDiagnostics(CriticalityDiagnostics),
    #[asn(key = 69)]
    idPDUSessionResourceReleasedListPSFail(PDUSessionResourceReleasedListPSFail),
    #[asn(key = 85)]
    idRANUENGAPID(RANUENGAPID),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct PathSwitchRequestFailureprotocolIEs_Item {
    #[asn(key_field = true)]
    pub id: ProtocolIEID,
    pub criticality: Criticality,
    pub value: PathSwitchRequestFailureprotocolIEs_Itemvalue,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "0", sz_ub = "65535")]
pub struct PathSwitchRequestFailureprotocolIEs(Vec<PathSwitchRequestFailureprotocolIEs_Item>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct PathSwitchRequestSetupFailedTransferiEExtensions_Item {}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "65535")]
pub struct PathSwitchRequestSetupFailedTransferiEExtensions(Vec<PathSwitchRequestSetupFailedTransferiEExtensions_Item>);

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum PathSwitchRequestTransferiEExtensions_ItemextensionValue {
    #[asn(key = 155)]
    idAdditionalDLQosFlowPerTNLInformation(QosFlowPerTNLInformationList),
    #[asn(key = 184)]
    idAdditionalRedundantDLQosFlowPerTNLInformation(QosFlowPerTNLInformationList),
    #[asn(key = 27)]
    idGlobalRANNodeID(GlobalRANNodeID),
    #[asn(key = 191)]
    idRedundantDLNGUTNLInformationReused(DLNGUTNLInformationReused),
    #[asn(key = 192)]
    idRedundantDLNGUUPTNLInformation(UPTransportLayerInformation),
    #[asn(key = 198)]
    idUsedRSNInformation(RedundantPDUSessionInformation),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct PathSwitchRequestTransferiEExtensions_Item {
    #[asn(key_field = true)]
    pub id: ProtocolExtensionID,
    pub criticality: Criticality,
    pub extension_value: PathSwitchRequestTransferiEExtensions_ItemextensionValue,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "65535")]
pub struct PathSwitchRequestTransferiEExtensions(Vec<PathSwitchRequestTransferiEExtensions_Item>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct PathSwitchRequestUnsuccessfulTransferiEExtensions_Item {}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "65535")]
pub struct PathSwitchRequestUnsuccessfulTransferiEExtensions(Vec<PathSwitchRequestUnsuccessfulTransferiEExtensions_Item>);

#[derive(Debug, AperCodec)]
#[asn(type = "INTEGER", lb = "0", ub = "65535")]
pub struct INTEGER70(u16);

#[derive(Debug, AperCodec)]
#[asn(type = "OBJECT-IDENTIFIER")]
pub struct OBJECTIDENTIFIER71;

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct PrivateMessageprivateIEs_Item {}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "65535")]
pub struct PrivateMessageprivateIEs(Vec<PrivateMessageprivateIEs_Item>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct ProcedureStageChoicechoiceExtensions {}

#[derive(Debug, AperCodec)]
#[asn(type = "ENUMERATED", extensible = true, lb = "0", ub = "1")]
pub struct ENUMERATED72(u8);
impl ENUMERATED72 {
    const NR: u8 = 0u8;
    const EUTRA: u8 = 1u8;
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct QoSFlowsUsageReportItemiEExtensions_Item {}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "65535")]
pub struct QoSFlowsUsageReportItemiEExtensions(Vec<QoSFlowsUsageReportItemiEExtensions_Item>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct QosCharacteristicschoiceExtensions {}

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum QosFlowAcceptedItemiEExtensions_ItemextensionValue {
    #[asn(key = 221)]
    idCurrentQoSParaSetIndex(AlternativeQoSParaSetIndex),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct QosFlowAcceptedItemiEExtensions_Item {
    #[asn(key_field = true)]
    pub id: ProtocolExtensionID,
    pub criticality: Criticality,
    pub extension_value: QosFlowAcceptedItemiEExtensions_ItemextensionValue,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "65535")]
pub struct QosFlowAcceptedItemiEExtensions(Vec<QosFlowAcceptedItemiEExtensions_Item>);

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum QosFlowAddOrModifyRequestItemiEExtensions_ItemextensionValue {
    #[asn(key = 194)]
    idRedundantQosFlowIndicator(RedundantQosFlowIndicator),
    #[asn(key = 196)]
    idTSCTrafficCharacteristics(TSCTrafficCharacteristics),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct QosFlowAddOrModifyRequestItemiEExtensions_Item {
    #[asn(key_field = true)]
    pub id: ProtocolExtensionID,
    pub criticality: Criticality,
    pub extension_value: QosFlowAddOrModifyRequestItemiEExtensions_ItemextensionValue,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "65535")]
pub struct QosFlowAddOrModifyRequestItemiEExtensions(Vec<QosFlowAddOrModifyRequestItemiEExtensions_Item>);

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum QosFlowAddOrModifyResponseItemiEExtensions_ItemextensionValue {
    #[asn(key = 221)]
    idCurrentQoSParaSetIndex(AlternativeQoSParaSetIndex),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct QosFlowAddOrModifyResponseItemiEExtensions_Item {
    #[asn(key_field = true)]
    pub id: ProtocolExtensionID,
    pub criticality: Criticality,
    pub extension_value: QosFlowAddOrModifyResponseItemiEExtensions_ItemextensionValue,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "65535")]
pub struct QosFlowAddOrModifyResponseItemiEExtensions(Vec<QosFlowAddOrModifyResponseItemiEExtensions_Item>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct QosFlowFeedbackItemiEExtensions_Item {}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "65535")]
pub struct QosFlowFeedbackItemiEExtensions(Vec<QosFlowFeedbackItemiEExtensions_Item>);

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum QosFlowInformationItemiEExtensions_ItemextensionValue {
    #[asn(key = 163)]
    idULForwarding(ULForwarding),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct QosFlowInformationItemiEExtensions_Item {
    #[asn(key_field = true)]
    pub id: ProtocolExtensionID,
    pub criticality: Criticality,
    pub extension_value: QosFlowInformationItemiEExtensions_ItemextensionValue,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "65535")]
pub struct QosFlowInformationItemiEExtensions(Vec<QosFlowInformationItemiEExtensions_Item>);

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum QosFlowItemWithDataForwardingiEExtensions_ItemextensionValue {
    #[asn(key = 221)]
    idCurrentQoSParaSetIndex(AlternativeQoSParaSetIndex),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct QosFlowItemWithDataForwardingiEExtensions_Item {
    #[asn(key_field = true)]
    pub id: ProtocolExtensionID,
    pub criticality: Criticality,
    pub extension_value: QosFlowItemWithDataForwardingiEExtensions_ItemextensionValue,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "65535")]
pub struct QosFlowItemWithDataForwardingiEExtensions(Vec<QosFlowItemWithDataForwardingiEExtensions_Item>);

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum QosFlowLevelQosParametersiEExtensions_ItemextensionValue {
    #[asn(key = 276)]
    idQosMonitoringReportingFrequency(QosMonitoringReportingFrequency),
    #[asn(key = 181)]
    idQosMonitoringRequest(QosMonitoringRequest),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct QosFlowLevelQosParametersiEExtensions_Item {
    #[asn(key_field = true)]
    pub id: ProtocolExtensionID,
    pub criticality: Criticality,
    pub extension_value: QosFlowLevelQosParametersiEExtensions_ItemextensionValue,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "65535")]
pub struct QosFlowLevelQosParametersiEExtensions(Vec<QosFlowLevelQosParametersiEExtensions_Item>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct QosFlowModifyConfirmItemiEExtensions_Item {}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "65535")]
pub struct QosFlowModifyConfirmItemiEExtensions(Vec<QosFlowModifyConfirmItemiEExtensions_Item>);

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum QosFlowNotifyItemiEExtensions_ItemextensionValue {
    #[asn(key = 221)]
    idCurrentQoSParaSetIndex(AlternativeQoSParaSetNotifyIndex),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct QosFlowNotifyItemiEExtensions_Item {
    #[asn(key_field = true)]
    pub id: ProtocolExtensionID,
    pub criticality: Criticality,
    pub extension_value: QosFlowNotifyItemiEExtensions_ItemextensionValue,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "65535")]
pub struct QosFlowNotifyItemiEExtensions(Vec<QosFlowNotifyItemiEExtensions_Item>);

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum QosFlowParametersItemiEExtensions_ItemextensionValue {
    #[asn(key = 279)]
    idBurstArrivalTimeDownlink(BurstArrivalTime),
    #[asn(key = 187)]
    idCNPacketDelayBudgetDL(ExtendedPacketDelayBudget),
    #[asn(key = 188)]
    idCNPacketDelayBudgetUL(ExtendedPacketDelayBudget),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct QosFlowParametersItemiEExtensions_Item {
    #[asn(key_field = true)]
    pub id: ProtocolExtensionID,
    pub criticality: Criticality,
    pub extension_value: QosFlowParametersItemiEExtensions_ItemextensionValue,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "65535")]
pub struct QosFlowParametersItemiEExtensions(Vec<QosFlowParametersItemiEExtensions_Item>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct QosFlowPerTNLInformationiEExtensions_Item {}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "65535")]
pub struct QosFlowPerTNLInformationiEExtensions(Vec<QosFlowPerTNLInformationiEExtensions_Item>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct QosFlowPerTNLInformationItemiEExtensions_Item {}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "65535")]
pub struct QosFlowPerTNLInformationItemiEExtensions(Vec<QosFlowPerTNLInformationItemiEExtensions_Item>);

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum QosFlowSetupRequestItemiEExtensions_ItemextensionValue {
    #[asn(key = 194)]
    idRedundantQosFlowIndicator(RedundantQosFlowIndicator),
    #[asn(key = 196)]
    idTSCTrafficCharacteristics(TSCTrafficCharacteristics),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct QosFlowSetupRequestItemiEExtensions_Item {
    #[asn(key_field = true)]
    pub id: ProtocolExtensionID,
    pub criticality: Criticality,
    pub extension_value: QosFlowSetupRequestItemiEExtensions_ItemextensionValue,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "65535")]
pub struct QosFlowSetupRequestItemiEExtensions(Vec<QosFlowSetupRequestItemiEExtensions_Item>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct QosFlowToBeForwardedItemiEExtensions_Item {}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "65535")]
pub struct QosFlowToBeForwardedItemiEExtensions(Vec<QosFlowToBeForwardedItemiEExtensions_Item>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct QosFlowWithCauseItemiEExtensions_Item {}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "65535")]
pub struct QosFlowWithCauseItemiEExtensions(Vec<QosFlowWithCauseItemiEExtensions_Item>);

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum RANCPRelocationIndicationprotocolIEs_Itemvalue {
    #[asn(key = 25)]
    idEUTRACGI(EUTRACGI),
    #[asn(key = 26)]
    idFiveGSTMSI(FiveGSTMSI),
    #[asn(key = 85)]
    idRANUENGAPID(RANUENGAPID),
    #[asn(key = 213)]
    idTAI(TAI),
    #[asn(key = 211)]
    idULCPSecurityInformation(ULCPSecurityInformation),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct RANCPRelocationIndicationprotocolIEs_Item {
    #[asn(key_field = true)]
    pub id: ProtocolIEID,
    pub criticality: Criticality,
    pub value: RANCPRelocationIndicationprotocolIEs_Itemvalue,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "0", sz_ub = "65535")]
pub struct RANCPRelocationIndicationprotocolIEs(Vec<RANCPRelocationIndicationprotocolIEs_Item>);

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum RANConfigurationUpdateprotocolIEs_Itemvalue {
    #[asn(key = 21)]
    idDefaultPagingDRX(PagingDRX),
    #[asn(key = 273)]
    idExtendedRANNodeName(ExtendedRANNodeName),
    #[asn(key = 27)]
    idGlobalRANNodeID(GlobalRANNodeID),
    #[asn(key = 204)]
    idNBIoTDefaultPagingDRX(NBIoTDefaultPagingDRX),
    #[asn(key = 167)]
    idNGRANTNLAssociationToRemoveList(NGRANTNLAssociationToRemoveList),
    #[asn(key = 82)]
    idRANNodeName(RANNodeName),
    #[asn(key = 102)]
    idSupportedTAList(SupportedTAList),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct RANConfigurationUpdateprotocolIEs_Item {
    #[asn(key_field = true)]
    pub id: ProtocolIEID,
    pub criticality: Criticality,
    pub value: RANConfigurationUpdateprotocolIEs_Itemvalue,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "0", sz_ub = "65535")]
pub struct RANConfigurationUpdateprotocolIEs(Vec<RANConfigurationUpdateprotocolIEs_Item>);

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum RANConfigurationUpdateAcknowledgeprotocolIEs_Itemvalue {
    #[asn(key = 19)]
    idCriticalityDiagnostics(CriticalityDiagnostics),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct RANConfigurationUpdateAcknowledgeprotocolIEs_Item {
    #[asn(key_field = true)]
    pub id: ProtocolIEID,
    pub criticality: Criticality,
    pub value: RANConfigurationUpdateAcknowledgeprotocolIEs_Itemvalue,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "0", sz_ub = "65535")]
pub struct RANConfigurationUpdateAcknowledgeprotocolIEs(Vec<RANConfigurationUpdateAcknowledgeprotocolIEs_Item>);

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum RANConfigurationUpdateFailureprotocolIEs_Itemvalue {
    #[asn(key = 15)]
    idCause(Cause),
    #[asn(key = 19)]
    idCriticalityDiagnostics(CriticalityDiagnostics),
    #[asn(key = 107)]
    idTimeToWait(TimeToWait),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct RANConfigurationUpdateFailureprotocolIEs_Item {
    #[asn(key_field = true)]
    pub id: ProtocolIEID,
    pub criticality: Criticality,
    pub value: RANConfigurationUpdateFailureprotocolIEs_Itemvalue,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "0", sz_ub = "65535")]
pub struct RANConfigurationUpdateFailureprotocolIEs(Vec<RANConfigurationUpdateFailureprotocolIEs_Item>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct RANStatusTransferTransparentContaineriEExtensions_Item {}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "65535")]
pub struct RANStatusTransferTransparentContaineriEExtensions(Vec<RANStatusTransferTransparentContaineriEExtensions_Item>);

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum RATRestrictionsItemiEExtensions_ItemextensionValue {
    #[asn(key = 180)]
    idExtendedRATRestrictionInformation(ExtendedRATRestrictionInformation),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct RATRestrictionsItemiEExtensions_Item {
    #[asn(key_field = true)]
    pub id: ProtocolExtensionID,
    pub criticality: Criticality,
    pub extension_value: RATRestrictionsItemiEExtensions_ItemextensionValue,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "65535")]
pub struct RATRestrictionsItemiEExtensions(Vec<RATRestrictionsItemiEExtensions_Item>);

#[derive(Debug, AperCodec)]
#[asn(type = "ENUMERATED", extensible = true, lb = "0", ub = "1")]
pub struct ENUMERATED73(u8);
impl ENUMERATED73 {
    const RS_DETECTED: u8 = 0u8;
    const RS_DISAPPEARED: u8 = 1u8;
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct RIMInformationiEExtensions_Item {}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "65535")]
pub struct RIMInformationiEExtensions(Vec<RIMInformationiEExtensions_Item>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct RIMInformationTransferiEExtensions_Item {}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "65535")]
pub struct RIMInformationTransferiEExtensions(Vec<RIMInformationTransferiEExtensions_Item>);

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum RRCInactiveTransitionReportprotocolIEs_Itemvalue {
    #[asn(key = 10)]
    idAMFUENGAPID(AMFUENGAPID),
    #[asn(key = 85)]
    idRANUENGAPID(RANUENGAPID),
    #[asn(key = 92)]
    idRRCState(RRCState),
    #[asn(key = 121)]
    idUserLocationInformation(UserLocationInformation),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct RRCInactiveTransitionReportprotocolIEs_Item {
    #[asn(key_field = true)]
    pub id: ProtocolIEID,
    pub criticality: Criticality,
    pub value: RRCInactiveTransitionReportprotocolIEs_Itemvalue,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "0", sz_ub = "65535")]
pub struct RRCInactiveTransitionReportprotocolIEs(Vec<RRCInactiveTransitionReportprotocolIEs_Item>);

#[derive(Debug, AperCodec)]
#[asn(type = "INTEGER", lb = "0", ub = "4095")]
pub struct INTEGER74(u16);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct RecommendedCellItemiEExtensions_Item {}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "65535")]
pub struct RecommendedCellItemiEExtensions(Vec<RecommendedCellItemiEExtensions_Item>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct RecommendedCellsForPagingiEExtensions_Item {}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "65535")]
pub struct RecommendedCellsForPagingiEExtensions(Vec<RecommendedCellsForPagingiEExtensions_Item>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct RecommendedRANNodeItemiEExtensions_Item {}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "65535")]
pub struct RecommendedRANNodeItemiEExtensions(Vec<RecommendedRANNodeItemiEExtensions_Item>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct RecommendedRANNodesForPagingiEExtensions_Item {}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "65535")]
pub struct RecommendedRANNodesForPagingiEExtensions(Vec<RecommendedRANNodesForPagingiEExtensions_Item>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct RedundantPDUSessionInformationiEExtensions_Item {}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "65535")]
pub struct RedundantPDUSessionInformationiEExtensions(Vec<RedundantPDUSessionInformationiEExtensions_Item>);

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum RerouteNASRequestprotocolIEs_Itemvalue {
    #[asn(key = 10)]
    idAMFUENGAPID(AMFUENGAPID),
    #[asn(key = 3)]
    idAMFSetID(AMFSetID),
    #[asn(key = 0)]
    idAllowedNSSAI(AllowedNSSAI),
    #[asn(key = 85)]
    idRANUENGAPID(RANUENGAPID),
    #[asn(key = 171)]
    idSourceToTargetAMFInformationReroute(SourceToTargetAMFInformationReroute),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct RerouteNASRequestprotocolIEs_Item {
    #[asn(key_field = true)]
    pub id: ProtocolIEID,
    pub criticality: Criticality,
    pub value: RerouteNASRequestprotocolIEs_Itemvalue,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "0", sz_ub = "65535")]
pub struct RerouteNASRequestprotocolIEs(Vec<RerouteNASRequestprotocolIEs_Item>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct ResetTypechoiceExtensions {}

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum RetrieveUEInformationprotocolIEs_Itemvalue {
    #[asn(key = 26)]
    idFiveGSTMSI(FiveGSTMSI),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct RetrieveUEInformationprotocolIEs_Item {
    #[asn(key_field = true)]
    pub id: ProtocolIEID,
    pub criticality: Criticality,
    pub value: RetrieveUEInformationprotocolIEs_Itemvalue,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "0", sz_ub = "65535")]
pub struct RetrieveUEInformationprotocolIEs(Vec<RetrieveUEInformationprotocolIEs_Item>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct SNSSAIiEExtensions_Item {}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "65535")]
pub struct SNSSAIiEExtensions(Vec<SNSSAIiEExtensions_Item>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct SNPNMobilityInformationiEExtensions_Item {}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "65535")]
pub struct SNPNMobilityInformationiEExtensions(Vec<SNPNMobilityInformationiEExtensions_Item>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct SONConfigurationTransferiEExtensions_Item {}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "65535")]
pub struct SONConfigurationTransferiEExtensions(Vec<SONConfigurationTransferiEExtensions_Item>);

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum SONInformationchoiceExtensionsvalue {
    #[asn(key = 252)]
    idSONInformationReport(SONInformationReport),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct SONInformationchoiceExtensions {
    #[asn(key_field = true)]
    pub id: ProtocolIEID,
    pub criticality: Criticality,
    pub value: SONInformationchoiceExtensionsvalue,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct SONInformationReplyiEExtensions_Item {}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "65535")]
pub struct SONInformationReplyiEExtensions(Vec<SONInformationReplyiEExtensions_Item>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct SONInformationReportchoiceExtensions {}

#[derive(Debug, AperCodec)]
#[asn(type = "BITSTRING", sz_extensible = false, sz_lb = "7", sz_ub = "7")]
pub struct BITSTRING75(BitVec<Msb0, u8>);

#[derive(Debug, AperCodec)]
#[asn(type = "INTEGER", lb = "0", ub = "86399", extensible = true)]
pub struct INTEGER76(u32);

#[derive(Debug, AperCodec)]
#[asn(type = "INTEGER", lb = "0", ub = "86399", extensible = true)]
pub struct INTEGER77(u32);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct ScheduledCommunicationTimeiEExtensions_Item {}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "65535")]
pub struct ScheduledCommunicationTimeiEExtensions(Vec<ScheduledCommunicationTimeiEExtensions_Item>);

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum SecondaryRATDataUsageReportprotocolIEs_Itemvalue {
    #[asn(key = 10)]
    idAMFUENGAPID(AMFUENGAPID),
    #[asn(key = 143)]
    idHandoverFlag(HandoverFlag),
    #[asn(key = 142)]
    idPDUSessionResourceSecondaryRATUsageList(PDUSessionResourceSecondaryRATUsageList),
    #[asn(key = 85)]
    idRANUENGAPID(RANUENGAPID),
    #[asn(key = 121)]
    idUserLocationInformation(UserLocationInformation),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct SecondaryRATDataUsageReportprotocolIEs_Item {
    #[asn(key_field = true)]
    pub id: ProtocolIEID,
    pub criticality: Criticality,
    pub value: SecondaryRATDataUsageReportprotocolIEs_Itemvalue,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "0", sz_ub = "65535")]
pub struct SecondaryRATDataUsageReportprotocolIEs(Vec<SecondaryRATDataUsageReportprotocolIEs_Item>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct SecondaryRATDataUsageReportTransferiEExtensions_Item {}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "65535")]
pub struct SecondaryRATDataUsageReportTransferiEExtensions(Vec<SecondaryRATDataUsageReportTransferiEExtensions_Item>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct SecondaryRATUsageInformationiEExtension_Item {}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "65535")]
pub struct SecondaryRATUsageInformationiEExtension(Vec<SecondaryRATUsageInformationiEExtension_Item>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct SecurityContextiEExtensions_Item {}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "65535")]
pub struct SecurityContextiEExtensions(Vec<SecurityContextiEExtensions_Item>);

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum SecurityIndicationiEExtensions_ItemextensionValue {
    #[asn(key = 151)]
    idMaximumIntegrityProtectedDataRateDL(MaximumIntegrityProtectedDataRate),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct SecurityIndicationiEExtensions_Item {
    #[asn(key_field = true)]
    pub id: ProtocolExtensionID,
    pub criticality: Criticality,
    pub extension_value: SecurityIndicationiEExtensions_ItemextensionValue,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "65535")]
pub struct SecurityIndicationiEExtensions(Vec<SecurityIndicationiEExtensions_Item>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct SecurityResultiEExtensions_Item {}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "65535")]
pub struct SecurityResultiEExtensions(Vec<SecurityResultiEExtensions_Item>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct SensorMeasConfigNameItemiEExtensions_Item {}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "65535")]
pub struct SensorMeasConfigNameItemiEExtensions(Vec<SensorMeasConfigNameItemiEExtensions_Item>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct SensorMeasurementConfigurationiEExtensions_Item {}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "65535")]
pub struct SensorMeasurementConfigurationiEExtensions(Vec<SensorMeasurementConfigurationiEExtensions_Item>);

#[derive(Debug, AperCodec)]
#[asn(type = "ENUMERATED", extensible = true, lb = "0", ub = "0")]
pub struct ENUMERATED78(u8);
impl ENUMERATED78 {
    const TRUE: u8 = 0u8;
}

#[derive(Debug, AperCodec)]
#[asn(type = "ENUMERATED", extensible = true, lb = "0", ub = "0")]
pub struct ENUMERATED79(u8);
impl ENUMERATED79 {
    const TRUE: u8 = 0u8;
}

#[derive(Debug, AperCodec)]
#[asn(type = "ENUMERATED", extensible = true, lb = "0", ub = "0")]
pub struct ENUMERATED80(u8);
impl ENUMERATED80 {
    const TRUE: u8 = 0u8;
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct SensorNameConfigchoiceExtensions {}

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum ServedGUAMIItemiEExtensions_ItemextensionValue {
    #[asn(key = 176)]
    idGUAMIType(GUAMIType),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct ServedGUAMIItemiEExtensions_Item {
    #[asn(key_field = true)]
    pub id: ProtocolExtensionID,
    pub criticality: Criticality,
    pub extension_value: ServedGUAMIItemiEExtensions_ItemextensionValue,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "65535")]
pub struct ServedGUAMIItemiEExtensions(Vec<ServedGUAMIItemiEExtensions_Item>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct ServiceAreaInformationItemiEExtensions_Item {}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "65535")]
pub struct ServiceAreaInformationItemiEExtensions(Vec<ServiceAreaInformationItemiEExtensions_Item>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct SliceOverloadItemiEExtensions_Item {}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "65535")]
pub struct SliceOverloadItemiEExtensions(Vec<SliceOverloadItemiEExtensions_Item>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct SliceSupportItemiEExtensions_Item {}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "65535")]
pub struct SliceSupportItemiEExtensions(Vec<SliceSupportItemiEExtensions_Item>);

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum SourceNGRANNodeToTargetNGRANNodeTransparentContaineriEExtensions_ItemextensionValue {
    #[asn(key = 182)]
    idSgNBUEX2APID(SgNBUEX2APID),
    #[asn(key = 253)]
    idUEHistoryInformationFromTheUE(UEHistoryInformationFromTheUE),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct SourceNGRANNodeToTargetNGRANNodeTransparentContaineriEExtensions_Item {
    #[asn(key_field = true)]
    pub id: ProtocolExtensionID,
    pub criticality: Criticality,
    pub extension_value:
        SourceNGRANNodeToTargetNGRANNodeTransparentContaineriEExtensions_ItemextensionValue,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "65535")]
pub struct SourceNGRANNodeToTargetNGRANNodeTransparentContaineriEExtensions(Vec<SourceNGRANNodeToTargetNGRANNodeTransparentContaineriEExtensions_Item>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct SourceRANNodeIDiEExtensions_Item {}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "65535")]
pub struct SourceRANNodeIDiEExtensions(Vec<SourceRANNodeIDiEExtensions_Item>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct SourceToTargetAMFInformationRerouteiEExtensions_Item {}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "65535")]
pub struct SourceToTargetAMFInformationRerouteiEExtensions(Vec<SourceToTargetAMFInformationRerouteiEExtensions_Item>);

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum SuccessfulOutcomevalue {
    #[asn(key = 0)]
    idAMFConfigurationUpdate(AMFConfigurationUpdateAcknowledge),
    #[asn(key = 10)]
    idHandoverCancel(HandoverCancelAcknowledge),
    #[asn(key = 12)]
    idHandoverPreparation(HandoverCommand),
    #[asn(key = 13)]
    idHandoverResourceAllocation(HandoverRequestAcknowledge),
    #[asn(key = 14)]
    idInitialContextSetup(InitialContextSetupResponse),
    #[asn(key = 20)]
    idNGReset(NGResetAcknowledge),
    #[asn(key = 21)]
    idNGSetup(NGSetupResponse),
    #[asn(key = 26)]
    idPDUSessionResourceModify(PDUSessionResourceModifyResponse),
    #[asn(key = 27)]
    idPDUSessionResourceModifyIndication(PDUSessionResourceModifyConfirm),
    #[asn(key = 28)]
    idPDUSessionResourceRelease(PDUSessionResourceReleaseResponse),
    #[asn(key = 29)]
    idPDUSessionResourceSetup(PDUSessionResourceSetupResponse),
    #[asn(key = 32)]
    idPWSCancel(PWSCancelResponse),
    #[asn(key = 25)]
    idPathSwitchRequest(PathSwitchRequestAcknowledge),
    #[asn(key = 35)]
    idRANConfigurationUpdate(RANConfigurationUpdateAcknowledge),
    #[asn(key = 40)]
    idUEContextModification(UEContextModificationResponse),
    #[asn(key = 41)]
    idUEContextRelease(UEContextReleaseComplete),
    #[asn(key = 58)]
    idUEContextResume(UEContextResumeResponse),
    #[asn(key = 59)]
    idUEContextSuspend(UEContextSuspendResponse),
    #[asn(key = 43)]
    idUERadioCapabilityCheck(UERadioCapabilityCheckResponse),
    #[asn(key = 60)]
    idUERadioCapabilityIDMapping(UERadioCapabilityIDMappingResponse),
    #[asn(key = 51)]
    idWriteReplaceWarning(WriteReplaceWarningResponse),
}

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum SupportedTAItemiEExtensions_ItemextensionValue {
    #[asn(key = 272)]
    idConfiguredTACIndication(ConfiguredTACIndication),
    #[asn(key = 179)]
    idRATInformation(RATInformation),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct SupportedTAItemiEExtensions_Item {
    #[asn(key_field = true)]
    pub id: ProtocolExtensionID,
    pub criticality: Criticality,
    pub extension_value: SupportedTAItemiEExtensions_ItemextensionValue,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "65535")]
pub struct SupportedTAItemiEExtensions(Vec<SupportedTAItemiEExtensions_Item>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct TABasedMDTiEExtensions_Item {}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "65535")]
pub struct TABasedMDTiEExtensions(Vec<TABasedMDTiEExtensions_Item>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct TAIiEExtensions_Item {}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "65535")]
pub struct TAIiEExtensions(Vec<TAIiEExtensions_Item>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct TAIBasedMDTiEExtensions_Item {}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "65535")]
pub struct TAIBasedMDTiEExtensions(Vec<TAIBasedMDTiEExtensions_Item>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct TAIBroadcastEUTRAItemiEExtensions_Item {}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "65535")]
pub struct TAIBroadcastEUTRAItemiEExtensions(Vec<TAIBroadcastEUTRAItemiEExtensions_Item>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct TAIBroadcastNRItemiEExtensions_Item {}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "65535")]
pub struct TAIBroadcastNRItemiEExtensions(Vec<TAIBroadcastNRItemiEExtensions_Item>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct TAICancelledEUTRAItemiEExtensions_Item {}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "65535")]
pub struct TAICancelledEUTRAItemiEExtensions(Vec<TAICancelledEUTRAItemiEExtensions_Item>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct TAICancelledNRItemiEExtensions_Item {}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "65535")]
pub struct TAICancelledNRItemiEExtensions(Vec<TAICancelledNRItemiEExtensions_Item>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct TAIListForInactiveItemiEExtensions_Item {}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "65535")]
pub struct TAIListForInactiveItemiEExtensions(Vec<TAIListForInactiveItemiEExtensions_Item>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct TAIListForPagingItemiEExtensions_Item {}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "65535")]
pub struct TAIListForPagingItemiEExtensions(Vec<TAIListForPagingItemiEExtensions_Item>);

#[derive(Debug, AperCodec)]
#[asn(type = "BITSTRING", sz_extensible = true, sz_lb = "32", sz_ub = "32")]
pub struct BITSTRING81(BitVec<Msb0, u8>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct TNGFIDchoiceExtensions {}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct TNLAssociationItemiEExtensions_Item {}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "65535")]
pub struct TNLAssociationItemiEExtensions(Vec<TNLAssociationItemiEExtensions_Item>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct TSCAssistanceInformationiEExtensions_Item {}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "65535")]
pub struct TSCAssistanceInformationiEExtensions(Vec<TSCAssistanceInformationiEExtensions_Item>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct TSCTrafficCharacteristicsiEExtensions_Item {}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "65535")]
pub struct TSCTrafficCharacteristicsiEExtensions(Vec<TSCTrafficCharacteristicsiEExtensions_Item>);

#[derive(Debug, AperCodec)]
#[asn(type = "BITSTRING", sz_extensible = true, sz_lb = "32", sz_ub = "32")]
pub struct BITSTRING82(BitVec<Msb0, u8>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct TWIFIDchoiceExtensions {}

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum TargetIDchoiceExtensionsvalue {
    #[asn(key = 178)]
    idTargetRNCID(TargetRNCID),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct TargetIDchoiceExtensions {
    #[asn(key_field = true)]
    pub id: ProtocolIEID,
    pub criticality: Criticality,
    pub value: TargetIDchoiceExtensionsvalue,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct TargetNGRANNodeToSourceNGRANNodeFailureTransparentContaineriEExtensions_Item {}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "65535")]
pub struct TargetNGRANNodeToSourceNGRANNodeFailureTransparentContaineriEExtensions(Vec<TargetNGRANNodeToSourceNGRANNodeFailureTransparentContaineriEExtensions_Item>);

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum TargetNGRANNodeToSourceNGRANNodeTransparentContaineriEExtensions_ItemextensionValue {
    #[asn(key = 267)]
    idDAPSResponseInfoList(DAPSResponseInfoList),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct TargetNGRANNodeToSourceNGRANNodeTransparentContaineriEExtensions_Item {
    #[asn(key_field = true)]
    pub id: ProtocolExtensionID,
    pub criticality: Criticality,
    pub extension_value:
        TargetNGRANNodeToSourceNGRANNodeTransparentContaineriEExtensions_ItemextensionValue,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "65535")]
pub struct TargetNGRANNodeToSourceNGRANNodeTransparentContaineriEExtensions(Vec<TargetNGRANNodeToSourceNGRANNodeTransparentContaineriEExtensions_Item>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct TargetRANNodeIDiEExtensions_Item {}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "65535")]
pub struct TargetRANNodeIDiEExtensions(Vec<TargetRANNodeIDiEExtensions_Item>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct TargetRNCIDiEExtensions_Item {}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "65535")]
pub struct TargetRNCIDiEExtensions(Vec<TargetRNCIDiEExtensions_Item>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct TargeteNBIDiEExtensions_Item {}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "65535")]
pub struct TargeteNBIDiEExtensions(Vec<TargeteNBIDiEExtensions_Item>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct TooearlyIntersystemHOiEExtensions_Item {}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "65535")]
pub struct TooearlyIntersystemHOiEExtensions(Vec<TooearlyIntersystemHOiEExtensions_Item>);

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum TraceActivationiEExtensions_ItemextensionValue {
    #[asn(key = 255)]
    idMDTConfiguration(MDTConfiguration),
    #[asn(key = 257)]
    idTraceCollectionEntityURI(URIaddress),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct TraceActivationiEExtensions_Item {
    #[asn(key_field = true)]
    pub id: ProtocolExtensionID,
    pub criticality: Criticality,
    pub extension_value: TraceActivationiEExtensions_ItemextensionValue,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "65535")]
pub struct TraceActivationiEExtensions(Vec<TraceActivationiEExtensions_Item>);

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum TraceFailureIndicationprotocolIEs_Itemvalue {
    #[asn(key = 10)]
    idAMFUENGAPID(AMFUENGAPID),
    #[asn(key = 15)]
    idCause(Cause),
    #[asn(key = 44)]
    idNGRANTraceID(NGRANTraceID),
    #[asn(key = 85)]
    idRANUENGAPID(RANUENGAPID),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct TraceFailureIndicationprotocolIEs_Item {
    #[asn(key_field = true)]
    pub id: ProtocolIEID,
    pub criticality: Criticality,
    pub value: TraceFailureIndicationprotocolIEs_Itemvalue,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "0", sz_ub = "65535")]
pub struct TraceFailureIndicationprotocolIEs(Vec<TraceFailureIndicationprotocolIEs_Item>);

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum TraceStartprotocolIEs_Itemvalue {
    #[asn(key = 10)]
    idAMFUENGAPID(AMFUENGAPID),
    #[asn(key = 85)]
    idRANUENGAPID(RANUENGAPID),
    #[asn(key = 108)]
    idTraceActivation(TraceActivation),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct TraceStartprotocolIEs_Item {
    #[asn(key_field = true)]
    pub id: ProtocolIEID,
    pub criticality: Criticality,
    pub value: TraceStartprotocolIEs_Itemvalue,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "0", sz_ub = "65535")]
pub struct TraceStartprotocolIEs(Vec<TraceStartprotocolIEs_Item>);

#[derive(Debug, AperCodec)]
#[asn(type = "ENUMERATED", extensible = true, lb = "0", ub = "1")]
pub struct ENUMERATED83(u8);
impl ENUMERATED83 {
    const PERIODICALLY: u8 = 0u8;
    const ONDEMAND: u8 = 1u8;
}

#[derive(Debug, AperCodec)]
#[asn(type = "INTEGER", lb = "1", ub = "3600", extensible = true)]
pub struct INTEGER84(u16);

#[derive(Debug, AperCodec)]
#[asn(type = "ENUMERATED", extensible = true, lb = "0", ub = "1")]
pub struct ENUMERATED85(u8);
impl ENUMERATED85 {
    const STATIONARY: u8 = 0u8;
    const MOBILE: u8 = 1u8;
}

#[derive(Debug, AperCodec)]
#[asn(type = "ENUMERATED", extensible = true, lb = "0", ub = "2")]
pub struct ENUMERATED86(u8);
impl ENUMERATED86 {
    const SINGLE_PACKET: u8 = 0u8;
    const DUAL_PACKETS: u8 = 1u8;
    const MULTIPLE_PACKETS: u8 = 2u8;
}

#[derive(Debug, AperCodec)]
#[asn(type = "ENUMERATED", extensible = true, lb = "0", ub = "2")]
pub struct ENUMERATED87(u8);
impl ENUMERATED87 {
    const BATTERY_POWERED: u8 = 0u8;
    const BATTERY_POWERED_NOT_RECHARGEABLE_OR_REPLACEABLE: u8 = 1u8;
    const NOT_BATTERY_POWERED: u8 = 2u8;
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct UEDifferentiationInfoiEExtensions_Item {}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "65535")]
pub struct UEDifferentiationInfoiEExtensions(Vec<UEDifferentiationInfoiEExtensions_Item>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct UENGAPIDpairiEExtensions_Item {}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "65535")]
pub struct UENGAPIDpairiEExtensions(Vec<UENGAPIDpairiEExtensions_Item>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct UENGAPIDschoiceExtensions {}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct UEassociatedLogicalNGconnectionItemiEExtensions_Item {}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "65535")]
pub struct UEassociatedLogicalNGconnectionItemiEExtensions(Vec<UEassociatedLogicalNGconnectionItemiEExtensions_Item>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct UEAggregateMaximumBitRateiEExtensions_Item {}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "65535")]
pub struct UEAggregateMaximumBitRateiEExtensions(Vec<UEAggregateMaximumBitRateiEExtensions_Item>);

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum UEContextModificationFailureprotocolIEs_Itemvalue {
    #[asn(key = 10)]
    idAMFUENGAPID(AMFUENGAPID),
    #[asn(key = 15)]
    idCause(Cause),
    #[asn(key = 19)]
    idCriticalityDiagnostics(CriticalityDiagnostics),
    #[asn(key = 85)]
    idRANUENGAPID(RANUENGAPID),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct UEContextModificationFailureprotocolIEs_Item {
    #[asn(key_field = true)]
    pub id: ProtocolIEID,
    pub criticality: Criticality,
    pub value: UEContextModificationFailureprotocolIEs_Itemvalue,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "0", sz_ub = "65535")]
pub struct UEContextModificationFailureprotocolIEs(Vec<UEContextModificationFailureprotocolIEs_Item>);

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum UEContextModificationRequestprotocolIEs_Itemvalue {
    #[asn(key = 10)]
    idAMFUENGAPID(AMFUENGAPID),
    #[asn(key = 165)]
    idCNAssistedRANTuning(CNAssistedRANTuning),
    #[asn(key = 18)]
    idCoreNetworkAssistanceInformationForInactive(CoreNetworkAssistanceInformationForInactive),
    #[asn(key = 24)]
    idEmergencyFallbackIndicator(EmergencyFallbackIndicator),
    #[asn(key = 199)]
    idIABAuthorized(IABAuthorized),
    #[asn(key = 31)]
    idIndexToRFSP(IndexToRFSP),
    #[asn(key = 217)]
    idLTEUESidelinkAggregateMaximumBitrate(LTEUESidelinkAggregateMaximumBitrate),
    #[asn(key = 215)]
    idLTEV2XServicesAuthorized(LTEV2XServicesAuthorized),
    #[asn(key = 218)]
    idNRUESidelinkAggregateMaximumBitrate(NRUESidelinkAggregateMaximumBitrate),
    #[asn(key = 216)]
    idNRV2XServicesAuthorized(NRV2XServicesAuthorized),
    #[asn(key = 40)]
    idNewAMFUENGAPID(AMFUENGAPID),
    #[asn(key = 162)]
    idNewGUAMI(GUAMI),
    #[asn(key = 219)]
    idPC5QoSParameters(PC5QoSParameters),
    #[asn(key = 85)]
    idRANUENGAPID(RANUENGAPID),
    #[asn(key = 83)]
    idRANPagingPriority(RANPagingPriority),
    #[asn(key = 238)]
    idRGLevelWirelineAccessCharacteristics(RGLevelWirelineAccessCharacteristics),
    #[asn(key = 91)]
    idRRCInactiveTransitionReportRequest(RRCInactiveTransitionReportRequest),
    #[asn(key = 177)]
    idSRVCCOperationPossible(SRVCCOperationPossible),
    #[asn(key = 94)]
    idSecurityKey(SecurityKey),
    #[asn(key = 110)]
    idUEAggregateMaximumBitRate(UEAggregateMaximumBitRate),
    #[asn(key = 264)]
    idUERadioCapabilityID(UERadioCapabilityID),
    #[asn(key = 119)]
    idUESecurityCapabilities(UESecurityCapabilities),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct UEContextModificationRequestprotocolIEs_Item {
    #[asn(key_field = true)]
    pub id: ProtocolIEID,
    pub criticality: Criticality,
    pub value: UEContextModificationRequestprotocolIEs_Itemvalue,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "0", sz_ub = "65535")]
pub struct UEContextModificationRequestprotocolIEs(Vec<UEContextModificationRequestprotocolIEs_Item>);

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum UEContextModificationResponseprotocolIEs_Itemvalue {
    #[asn(key = 10)]
    idAMFUENGAPID(AMFUENGAPID),
    #[asn(key = 19)]
    idCriticalityDiagnostics(CriticalityDiagnostics),
    #[asn(key = 85)]
    idRANUENGAPID(RANUENGAPID),
    #[asn(key = 92)]
    idRRCState(RRCState),
    #[asn(key = 121)]
    idUserLocationInformation(UserLocationInformation),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct UEContextModificationResponseprotocolIEs_Item {
    #[asn(key_field = true)]
    pub id: ProtocolIEID,
    pub criticality: Criticality,
    pub value: UEContextModificationResponseprotocolIEs_Itemvalue,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "0", sz_ub = "65535")]
pub struct UEContextModificationResponseprotocolIEs(Vec<UEContextModificationResponseprotocolIEs_Item>);

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum UEContextReleaseCommandprotocolIEs_Itemvalue {
    #[asn(key = 15)]
    idCause(Cause),
    #[asn(key = 114)]
    idUENGAPIDs(UENGAPIDs),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct UEContextReleaseCommandprotocolIEs_Item {
    #[asn(key_field = true)]
    pub id: ProtocolIEID,
    pub criticality: Criticality,
    pub value: UEContextReleaseCommandprotocolIEs_Itemvalue,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "0", sz_ub = "65535")]
pub struct UEContextReleaseCommandprotocolIEs(Vec<UEContextReleaseCommandprotocolIEs_Item>);

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum UEContextReleaseCompleteprotocolIEs_Itemvalue {
    #[asn(key = 10)]
    idAMFUENGAPID(AMFUENGAPID),
    #[asn(key = 19)]
    idCriticalityDiagnostics(CriticalityDiagnostics),
    #[asn(key = 32)]
    idInfoOnRecommendedCellsAndRANNodesForPaging(InfoOnRecommendedCellsAndRANNodesForPaging),
    #[asn(key = 60)]
    idPDUSessionResourceListCxtRelCpl(PDUSessionResourceListCxtRelCpl),
    #[asn(key = 207)]
    idPagingAssisDataforCEcapabUE(PagingAssisDataforCEcapabUE),
    #[asn(key = 85)]
    idRANUENGAPID(RANUENGAPID),
    #[asn(key = 121)]
    idUserLocationInformation(UserLocationInformation),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct UEContextReleaseCompleteprotocolIEs_Item {
    #[asn(key_field = true)]
    pub id: ProtocolIEID,
    pub criticality: Criticality,
    pub value: UEContextReleaseCompleteprotocolIEs_Itemvalue,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "0", sz_ub = "65535")]
pub struct UEContextReleaseCompleteprotocolIEs(Vec<UEContextReleaseCompleteprotocolIEs_Item>);

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum UEContextReleaseRequestprotocolIEs_Itemvalue {
    #[asn(key = 10)]
    idAMFUENGAPID(AMFUENGAPID),
    #[asn(key = 15)]
    idCause(Cause),
    #[asn(key = 133)]
    idPDUSessionResourceListCxtRelReq(PDUSessionResourceListCxtRelReq),
    #[asn(key = 85)]
    idRANUENGAPID(RANUENGAPID),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct UEContextReleaseRequestprotocolIEs_Item {
    #[asn(key_field = true)]
    pub id: ProtocolIEID,
    pub criticality: Criticality,
    pub value: UEContextReleaseRequestprotocolIEs_Itemvalue,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "0", sz_ub = "65535")]
pub struct UEContextReleaseRequestprotocolIEs(Vec<UEContextReleaseRequestprotocolIEs_Item>);

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum UEContextResumeFailureprotocolIEs_Itemvalue {
    #[asn(key = 10)]
    idAMFUENGAPID(AMFUENGAPID),
    #[asn(key = 15)]
    idCause(Cause),
    #[asn(key = 19)]
    idCriticalityDiagnostics(CriticalityDiagnostics),
    #[asn(key = 85)]
    idRANUENGAPID(RANUENGAPID),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct UEContextResumeFailureprotocolIEs_Item {
    #[asn(key_field = true)]
    pub id: ProtocolIEID,
    pub criticality: Criticality,
    pub value: UEContextResumeFailureprotocolIEs_Itemvalue,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "0", sz_ub = "65535")]
pub struct UEContextResumeFailureprotocolIEs(Vec<UEContextResumeFailureprotocolIEs_Item>);

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum UEContextResumeRequestprotocolIEs_Itemvalue {
    #[asn(key = 10)]
    idAMFUENGAPID(AMFUENGAPID),
    #[asn(key = 32)]
    idInfoOnRecommendedCellsAndRANNodesForPaging(InfoOnRecommendedCellsAndRANNodesForPaging),
    #[asn(key = 229)]
    idPDUSessionResourceFailedToResumeListRESReq(PDUSessionResourceFailedToResumeListRESReq),
    #[asn(key = 232)]
    idPDUSessionResourceResumeListRESReq(PDUSessionResourceResumeListRESReq),
    #[asn(key = 207)]
    idPagingAssisDataforCEcapabUE(PagingAssisDataforCEcapabUE),
    #[asn(key = 85)]
    idRANUENGAPID(RANUENGAPID),
    #[asn(key = 237)]
    idRRCResumeCause(RRCEstablishmentCause),
    #[asn(key = 235)]
    idSuspendRequestIndication(SuspendRequestIndication),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct UEContextResumeRequestprotocolIEs_Item {
    #[asn(key_field = true)]
    pub id: ProtocolIEID,
    pub criticality: Criticality,
    pub value: UEContextResumeRequestprotocolIEs_Itemvalue,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "0", sz_ub = "65535")]
pub struct UEContextResumeRequestprotocolIEs(Vec<UEContextResumeRequestprotocolIEs_Item>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct UEContextResumeRequestTransferiEExtensions_Item {}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "65535")]
pub struct UEContextResumeRequestTransferiEExtensions(Vec<UEContextResumeRequestTransferiEExtensions_Item>);

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum UEContextResumeResponseprotocolIEs_Itemvalue {
    #[asn(key = 10)]
    idAMFUENGAPID(AMFUENGAPID),
    #[asn(key = 19)]
    idCriticalityDiagnostics(CriticalityDiagnostics),
    #[asn(key = 206)]
    idExtendedConnectedTime(ExtendedConnectedTime),
    #[asn(key = 230)]
    idPDUSessionResourceFailedToResumeListRESRes(PDUSessionResourceFailedToResumeListRESRes),
    #[asn(key = 233)]
    idPDUSessionResourceResumeListRESRes(PDUSessionResourceResumeListRESRes),
    #[asn(key = 85)]
    idRANUENGAPID(RANUENGAPID),
    #[asn(key = 93)]
    idSecurityContext(SecurityContext),
    #[asn(key = 236)]
    idSuspendResponseIndication(SuspendResponseIndication),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct UEContextResumeResponseprotocolIEs_Item {
    #[asn(key_field = true)]
    pub id: ProtocolIEID,
    pub criticality: Criticality,
    pub value: UEContextResumeResponseprotocolIEs_Itemvalue,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "0", sz_ub = "65535")]
pub struct UEContextResumeResponseprotocolIEs(Vec<UEContextResumeResponseprotocolIEs_Item>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct UEContextResumeResponseTransferiEExtensions_Item {}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "65535")]
pub struct UEContextResumeResponseTransferiEExtensions(Vec<UEContextResumeResponseTransferiEExtensions_Item>);

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum UEContextSuspendFailureprotocolIEs_Itemvalue {
    #[asn(key = 10)]
    idAMFUENGAPID(AMFUENGAPID),
    #[asn(key = 15)]
    idCause(Cause),
    #[asn(key = 19)]
    idCriticalityDiagnostics(CriticalityDiagnostics),
    #[asn(key = 85)]
    idRANUENGAPID(RANUENGAPID),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct UEContextSuspendFailureprotocolIEs_Item {
    #[asn(key_field = true)]
    pub id: ProtocolIEID,
    pub criticality: Criticality,
    pub value: UEContextSuspendFailureprotocolIEs_Itemvalue,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "0", sz_ub = "65535")]
pub struct UEContextSuspendFailureprotocolIEs(Vec<UEContextSuspendFailureprotocolIEs_Item>);

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum UEContextSuspendRequestprotocolIEs_Itemvalue {
    #[asn(key = 10)]
    idAMFUENGAPID(AMFUENGAPID),
    #[asn(key = 32)]
    idInfoOnRecommendedCellsAndRANNodesForPaging(InfoOnRecommendedCellsAndRANNodesForPaging),
    #[asn(key = 231)]
    idPDUSessionResourceSuspendListSUSReq(PDUSessionResourceSuspendListSUSReq),
    #[asn(key = 207)]
    idPagingAssisDataforCEcapabUE(PagingAssisDataforCEcapabUE),
    #[asn(key = 85)]
    idRANUENGAPID(RANUENGAPID),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct UEContextSuspendRequestprotocolIEs_Item {
    #[asn(key_field = true)]
    pub id: ProtocolIEID,
    pub criticality: Criticality,
    pub value: UEContextSuspendRequestprotocolIEs_Itemvalue,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "0", sz_ub = "65535")]
pub struct UEContextSuspendRequestprotocolIEs(Vec<UEContextSuspendRequestprotocolIEs_Item>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct UEContextSuspendRequestTransferiEExtensions_Item {}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "65535")]
pub struct UEContextSuspendRequestTransferiEExtensions(Vec<UEContextSuspendRequestTransferiEExtensions_Item>);

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum UEContextSuspendResponseprotocolIEs_Itemvalue {
    #[asn(key = 10)]
    idAMFUENGAPID(AMFUENGAPID),
    #[asn(key = 19)]
    idCriticalityDiagnostics(CriticalityDiagnostics),
    #[asn(key = 85)]
    idRANUENGAPID(RANUENGAPID),
    #[asn(key = 93)]
    idSecurityContext(SecurityContext),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct UEContextSuspendResponseprotocolIEs_Item {
    #[asn(key_field = true)]
    pub id: ProtocolIEID,
    pub criticality: Criticality,
    pub value: UEContextSuspendResponseprotocolIEs_Itemvalue,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "0", sz_ub = "65535")]
pub struct UEContextSuspendResponseprotocolIEs(Vec<UEContextSuspendResponseprotocolIEs_Item>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct UEHistoryInformationFromTheUEchoiceExtensions {}

#[derive(Debug, AperCodec)]
#[asn(type = "BITSTRING", sz_extensible = false, sz_lb = "10", sz_ub = "10")]
pub struct BITSTRING88(BitVec<Msb0, u8>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct UEIdentityIndexValuechoiceExtensions {}

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum UEInformationTransferprotocolIEs_Itemvalue {
    #[asn(key = 0)]
    idAllowedNSSAI(AllowedNSSAI),
    #[asn(key = 26)]
    idFiveGSTMSI(FiveGSTMSI),
    #[asn(key = 210)]
    idNBIoTUEPriority(NBIoTUEPriority),
    #[asn(key = 148)]
    idSNSSAI(SNSSAI),
    #[asn(key = 209)]
    idUEDifferentiationInfo(UEDifferentiationInfo),
    #[asn(key = 117)]
    idUERadioCapability(UERadioCapability),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct UEInformationTransferprotocolIEs_Item {
    #[asn(key_field = true)]
    pub id: ProtocolIEID,
    pub criticality: Criticality,
    pub value: UEInformationTransferprotocolIEs_Itemvalue,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "0", sz_ub = "65535")]
pub struct UEInformationTransferprotocolIEs(Vec<UEInformationTransferprotocolIEs_Item>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct UEPagingIdentitychoiceExtensions {}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct UEPresenceInAreaOfInterestItemiEExtensions_Item {}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "65535")]
pub struct UEPresenceInAreaOfInterestItemiEExtensions(Vec<UEPresenceInAreaOfInterestItemiEExtensions_Item>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct UERLFReportContainerchoiceExtensions {}

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum UERadioCapabilityCheckRequestprotocolIEs_Itemvalue {
    #[asn(key = 10)]
    idAMFUENGAPID(AMFUENGAPID),
    #[asn(key = 85)]
    idRANUENGAPID(RANUENGAPID),
    #[asn(key = 117)]
    idUERadioCapability(UERadioCapability),
    #[asn(key = 264)]
    idUERadioCapabilityID(UERadioCapabilityID),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct UERadioCapabilityCheckRequestprotocolIEs_Item {
    #[asn(key_field = true)]
    pub id: ProtocolIEID,
    pub criticality: Criticality,
    pub value: UERadioCapabilityCheckRequestprotocolIEs_Itemvalue,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "0", sz_ub = "65535")]
pub struct UERadioCapabilityCheckRequestprotocolIEs(Vec<UERadioCapabilityCheckRequestprotocolIEs_Item>);

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum UERadioCapabilityCheckResponseprotocolIEs_Itemvalue {
    #[asn(key = 10)]
    idAMFUENGAPID(AMFUENGAPID),
    #[asn(key = 19)]
    idCriticalityDiagnostics(CriticalityDiagnostics),
    #[asn(key = 30)]
    idIMSVoiceSupportIndicator(IMSVoiceSupportIndicator),
    #[asn(key = 85)]
    idRANUENGAPID(RANUENGAPID),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct UERadioCapabilityCheckResponseprotocolIEs_Item {
    #[asn(key_field = true)]
    pub id: ProtocolIEID,
    pub criticality: Criticality,
    pub value: UERadioCapabilityCheckResponseprotocolIEs_Itemvalue,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "0", sz_ub = "65535")]
pub struct UERadioCapabilityCheckResponseprotocolIEs(Vec<UERadioCapabilityCheckResponseprotocolIEs_Item>);

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum UERadioCapabilityForPagingiEExtensions_ItemextensionValue {
    #[asn(key = 214)]
    idUERadioCapabilityForPagingOfNBIoT(UERadioCapabilityForPagingOfNBIoT),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct UERadioCapabilityForPagingiEExtensions_Item {
    #[asn(key_field = true)]
    pub id: ProtocolExtensionID,
    pub criticality: Criticality,
    pub extension_value: UERadioCapabilityForPagingiEExtensions_ItemextensionValue,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "65535")]
pub struct UERadioCapabilityForPagingiEExtensions(Vec<UERadioCapabilityForPagingiEExtensions_Item>);

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum UERadioCapabilityIDMappingRequestprotocolIEs_Itemvalue {
    #[asn(key = 264)]
    idUERadioCapabilityID(UERadioCapabilityID),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct UERadioCapabilityIDMappingRequestprotocolIEs_Item {
    #[asn(key_field = true)]
    pub id: ProtocolIEID,
    pub criticality: Criticality,
    pub value: UERadioCapabilityIDMappingRequestprotocolIEs_Itemvalue,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "0", sz_ub = "65535")]
pub struct UERadioCapabilityIDMappingRequestprotocolIEs(Vec<UERadioCapabilityIDMappingRequestprotocolIEs_Item>);

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum UERadioCapabilityIDMappingResponseprotocolIEs_Itemvalue {
    #[asn(key = 19)]
    idCriticalityDiagnostics(CriticalityDiagnostics),
    #[asn(key = 117)]
    idUERadioCapability(UERadioCapability),
    #[asn(key = 264)]
    idUERadioCapabilityID(UERadioCapabilityID),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct UERadioCapabilityIDMappingResponseprotocolIEs_Item {
    #[asn(key_field = true)]
    pub id: ProtocolIEID,
    pub criticality: Criticality,
    pub value: UERadioCapabilityIDMappingResponseprotocolIEs_Itemvalue,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "0", sz_ub = "65535")]
pub struct UERadioCapabilityIDMappingResponseprotocolIEs(Vec<UERadioCapabilityIDMappingResponseprotocolIEs_Item>);

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum UERadioCapabilityInfoIndicationprotocolIEs_Itemvalue {
    #[asn(key = 10)]
    idAMFUENGAPID(AMFUENGAPID),
    #[asn(key = 85)]
    idRANUENGAPID(RANUENGAPID),
    #[asn(key = 117)]
    idUERadioCapability(UERadioCapability),
    #[asn(key = 265)]
    idUERadioCapabilityEUTRAFormat(UERadioCapability),
    #[asn(key = 118)]
    idUERadioCapabilityForPaging(UERadioCapabilityForPaging),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct UERadioCapabilityInfoIndicationprotocolIEs_Item {
    #[asn(key_field = true)]
    pub id: ProtocolIEID,
    pub criticality: Criticality,
    pub value: UERadioCapabilityInfoIndicationprotocolIEs_Itemvalue,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "0", sz_ub = "65535")]
pub struct UERadioCapabilityInfoIndicationprotocolIEs(Vec<UERadioCapabilityInfoIndicationprotocolIEs_Item>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct UESecurityCapabilitiesiEExtensions_Item {}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "65535")]
pub struct UESecurityCapabilitiesiEExtensions(Vec<UESecurityCapabilitiesiEExtensions_Item>);

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum UETNLABindingReleaseRequestprotocolIEs_Itemvalue {
    #[asn(key = 10)]
    idAMFUENGAPID(AMFUENGAPID),
    #[asn(key = 85)]
    idRANUENGAPID(RANUENGAPID),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct UETNLABindingReleaseRequestprotocolIEs_Item {
    #[asn(key_field = true)]
    pub id: ProtocolIEID,
    pub criticality: Criticality,
    pub value: UETNLABindingReleaseRequestprotocolIEs_Itemvalue,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "0", sz_ub = "65535")]
pub struct UETNLABindingReleaseRequestprotocolIEs(Vec<UETNLABindingReleaseRequestprotocolIEs_Item>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct ULCPSecurityInformationiEExtensions_Item {}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "65535")]
pub struct ULCPSecurityInformationiEExtensions(Vec<ULCPSecurityInformationiEExtensions_Item>);

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum ULNGUUPTNLModifyItemiEExtensions_ItemextensionValue {
    #[asn(key = 192)]
    idRedundantDLNGUUPTNLInformation(UPTransportLayerInformation),
    #[asn(key = 195)]
    idRedundantULNGUUPTNLInformation(UPTransportLayerInformation),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct ULNGUUPTNLModifyItemiEExtensions_Item {
    #[asn(key_field = true)]
    pub id: ProtocolExtensionID,
    pub criticality: Criticality,
    pub extension_value: ULNGUUPTNLModifyItemiEExtensions_ItemextensionValue,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "65535")]
pub struct ULNGUUPTNLModifyItemiEExtensions(Vec<ULNGUUPTNLModifyItemiEExtensions_Item>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct UPTransportLayerInformationchoiceExtensions {}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct UPTransportLayerInformationItemiEExtensions_Item {}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "65535")]
pub struct UPTransportLayerInformationItemiEExtensions(Vec<UPTransportLayerInformationItemiEExtensions_Item>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct UPTransportLayerInformationPairItemiEExtensions_Item {}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "65535")]
pub struct UPTransportLayerInformationPairItemiEExtensions(Vec<UPTransportLayerInformationPairItemiEExtensions_Item>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct UnavailableGUAMIItemiEExtensions_Item {}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "65535")]
pub struct UnavailableGUAMIItemiEExtensions(Vec<UnavailableGUAMIItemiEExtensions_Item>);

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum UnsuccessfulOutcomevalue {
    #[asn(key = 0)]
    idAMFConfigurationUpdate(AMFConfigurationUpdateFailure),
    #[asn(key = 12)]
    idHandoverPreparation(HandoverPreparationFailure),
    #[asn(key = 13)]
    idHandoverResourceAllocation(HandoverFailure),
    #[asn(key = 14)]
    idInitialContextSetup(InitialContextSetupFailure),
    #[asn(key = 21)]
    idNGSetup(NGSetupFailure),
    #[asn(key = 25)]
    idPathSwitchRequest(PathSwitchRequestFailure),
    #[asn(key = 35)]
    idRANConfigurationUpdate(RANConfigurationUpdateFailure),
    #[asn(key = 40)]
    idUEContextModification(UEContextModificationFailure),
    #[asn(key = 58)]
    idUEContextResume(UEContextResumeFailure),
    #[asn(key = 59)]
    idUEContextSuspend(UEContextSuspendFailure),
}

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum UplinkNASTransportprotocolIEs_Itemvalue {
    #[asn(key = 10)]
    idAMFUENGAPID(AMFUENGAPID),
    #[asn(key = 38)]
    idNASPDU(NASPDU),
    #[asn(key = 85)]
    idRANUENGAPID(RANUENGAPID),
    #[asn(key = 121)]
    idUserLocationInformation(UserLocationInformation),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct UplinkNASTransportprotocolIEs_Item {
    #[asn(key_field = true)]
    pub id: ProtocolIEID,
    pub criticality: Criticality,
    pub value: UplinkNASTransportprotocolIEs_Itemvalue,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "0", sz_ub = "65535")]
pub struct UplinkNASTransportprotocolIEs(Vec<UplinkNASTransportprotocolIEs_Item>);

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum UplinkNonUEAssociatedNRPPaTransportprotocolIEs_Itemvalue {
    #[asn(key = 46)]
    idNRPPaPDU(NRPPaPDU),
    #[asn(key = 89)]
    idRoutingID(RoutingID),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct UplinkNonUEAssociatedNRPPaTransportprotocolIEs_Item {
    #[asn(key_field = true)]
    pub id: ProtocolIEID,
    pub criticality: Criticality,
    pub value: UplinkNonUEAssociatedNRPPaTransportprotocolIEs_Itemvalue,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "0", sz_ub = "65535")]
pub struct UplinkNonUEAssociatedNRPPaTransportprotocolIEs(Vec<UplinkNonUEAssociatedNRPPaTransportprotocolIEs_Item>);

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum UplinkRANConfigurationTransferprotocolIEs_Itemvalue {
    #[asn(key = 158)]
    idENDCSONConfigurationTransferUL(ENDCSONConfigurationTransfer),
    #[asn(key = 251)]
    idIntersystemSONConfigurationTransferUL(IntersystemSONConfigurationTransfer),
    #[asn(key = 99)]
    idSONConfigurationTransferUL(SONConfigurationTransfer),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct UplinkRANConfigurationTransferprotocolIEs_Item {
    #[asn(key_field = true)]
    pub id: ProtocolIEID,
    pub criticality: Criticality,
    pub value: UplinkRANConfigurationTransferprotocolIEs_Itemvalue,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "0", sz_ub = "65535")]
pub struct UplinkRANConfigurationTransferprotocolIEs(Vec<UplinkRANConfigurationTransferprotocolIEs_Item>);

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum UplinkRANEarlyStatusTransferprotocolIEs_Itemvalue {
    #[asn(key = 10)]
    idAMFUENGAPID(AMFUENGAPID),
    #[asn(key = 268)]
    idEarlyStatusTransferTransparentContainer(EarlyStatusTransferTransparentContainer),
    #[asn(key = 85)]
    idRANUENGAPID(RANUENGAPID),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct UplinkRANEarlyStatusTransferprotocolIEs_Item {
    #[asn(key_field = true)]
    pub id: ProtocolIEID,
    pub criticality: Criticality,
    pub value: UplinkRANEarlyStatusTransferprotocolIEs_Itemvalue,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "0", sz_ub = "65535")]
pub struct UplinkRANEarlyStatusTransferprotocolIEs(Vec<UplinkRANEarlyStatusTransferprotocolIEs_Item>);

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum UplinkRANStatusTransferprotocolIEs_Itemvalue {
    #[asn(key = 10)]
    idAMFUENGAPID(AMFUENGAPID),
    #[asn(key = 85)]
    idRANUENGAPID(RANUENGAPID),
    #[asn(key = 84)]
    idRANStatusTransferTransparentContainer(RANStatusTransferTransparentContainer),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct UplinkRANStatusTransferprotocolIEs_Item {
    #[asn(key_field = true)]
    pub id: ProtocolIEID,
    pub criticality: Criticality,
    pub value: UplinkRANStatusTransferprotocolIEs_Itemvalue,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "0", sz_ub = "65535")]
pub struct UplinkRANStatusTransferprotocolIEs(Vec<UplinkRANStatusTransferprotocolIEs_Item>);

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum UplinkRIMInformationTransferprotocolIEs_Itemvalue {
    #[asn(key = 175)]
    idRIMInformationTransfer(RIMInformationTransfer),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct UplinkRIMInformationTransferprotocolIEs_Item {
    #[asn(key_field = true)]
    pub id: ProtocolIEID,
    pub criticality: Criticality,
    pub value: UplinkRIMInformationTransferprotocolIEs_Itemvalue,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "0", sz_ub = "65535")]
pub struct UplinkRIMInformationTransferprotocolIEs(Vec<UplinkRIMInformationTransferprotocolIEs_Item>);

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum UplinkUEAssociatedNRPPaTransportprotocolIEs_Itemvalue {
    #[asn(key = 10)]
    idAMFUENGAPID(AMFUENGAPID),
    #[asn(key = 46)]
    idNRPPaPDU(NRPPaPDU),
    #[asn(key = 85)]
    idRANUENGAPID(RANUENGAPID),
    #[asn(key = 89)]
    idRoutingID(RoutingID),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct UplinkUEAssociatedNRPPaTransportprotocolIEs_Item {
    #[asn(key_field = true)]
    pub id: ProtocolIEID,
    pub criticality: Criticality,
    pub value: UplinkUEAssociatedNRPPaTransportprotocolIEs_Itemvalue,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "0", sz_ub = "65535")]
pub struct UplinkUEAssociatedNRPPaTransportprotocolIEs(Vec<UplinkUEAssociatedNRPPaTransportprotocolIEs_Item>);

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum UserLocationInformationchoiceExtensionsvalue {
    #[asn(key = 244)]
    idUserLocationInformationTNGF(UserLocationInformationTNGF),
    #[asn(key = 248)]
    idUserLocationInformationTWIF(UserLocationInformationTWIF),
    #[asn(key = 243)]
    idUserLocationInformationWAGF(UserLocationInformationWAGF),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct UserLocationInformationchoiceExtensions {
    #[asn(key_field = true)]
    pub id: ProtocolIEID,
    pub criticality: Criticality,
    pub value: UserLocationInformationchoiceExtensionsvalue,
}

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum UserLocationInformationEUTRAiEExtensions_ItemextensionValue {
    #[asn(key = 149)]
    idPSCellInformation(NGRANCGI),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct UserLocationInformationEUTRAiEExtensions_Item {
    #[asn(key_field = true)]
    pub id: ProtocolExtensionID,
    pub criticality: Criticality,
    pub extension_value: UserLocationInformationEUTRAiEExtensions_ItemextensionValue,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "65535")]
pub struct UserLocationInformationEUTRAiEExtensions(Vec<UserLocationInformationEUTRAiEExtensions_Item>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct UserLocationInformationN3IWFiEExtensions_Item {}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "65535")]
pub struct UserLocationInformationN3IWFiEExtensions(Vec<UserLocationInformationN3IWFiEExtensions_Item>);

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum UserLocationInformationNRiEExtensions_ItemextensionValue {
    #[asn(key = 263)]
    idNID(NID),
    #[asn(key = 149)]
    idPSCellInformation(NGRANCGI),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct UserLocationInformationNRiEExtensions_Item {
    #[asn(key_field = true)]
    pub id: ProtocolExtensionID,
    pub criticality: Criticality,
    pub extension_value: UserLocationInformationNRiEExtensions_ItemextensionValue,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "65535")]
pub struct UserLocationInformationNRiEExtensions(Vec<UserLocationInformationNRiEExtensions_Item>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct UserLocationInformationTNGFiEExtensions_Item {}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "65535")]
pub struct UserLocationInformationTNGFiEExtensions(Vec<UserLocationInformationTNGFiEExtensions_Item>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct UserLocationInformationTWIFiEExtensions_Item {}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "65535")]
pub struct UserLocationInformationTWIFiEExtensions(Vec<UserLocationInformationTWIFiEExtensions_Item>);

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum UserLocationInformationWAGFchoiceExtensionsvalue {
    #[asn(key = 275)]
    idGlobalCableID(GlobalCableID),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct UserLocationInformationWAGFchoiceExtensions {
    #[asn(key_field = true)]
    pub id: ProtocolIEID,
    pub criticality: Criticality,
    pub value: UserLocationInformationWAGFchoiceExtensionsvalue,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct UserPlaneSecurityInformationiEExtensions_Item {}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "65535")]
pub struct UserPlaneSecurityInformationiEExtensions(Vec<UserPlaneSecurityInformationiEExtensions_Item>);

#[derive(Debug, AperCodec)]
#[asn(type = "OCTET-STRING", sz_extensible = false, sz_lb = "4", sz_ub = "4")]
pub struct OCTETSTRING89(Vec<u8>);

#[derive(Debug, AperCodec)]
#[asn(type = "OCTET-STRING", sz_extensible = false, sz_lb = "4", sz_ub = "4")]
pub struct OCTETSTRING90(Vec<u8>);

#[derive(Debug, AperCodec)]
#[asn(type = "INTEGER", lb = "0", ub = "18446744073709551615")]
pub struct INTEGER91(u64);

#[derive(Debug, AperCodec)]
#[asn(type = "INTEGER", lb = "0", ub = "18446744073709551615")]
pub struct INTEGER92(u64);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct VolumeTimedReportItemiEExtensions_Item {}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "65535")]
pub struct VolumeTimedReportItemiEExtensions(Vec<VolumeTimedReportItemiEExtensions_Item>);

#[derive(Debug, AperCodec)]
#[asn(type = "BITSTRING", sz_extensible = true, sz_lb = "16", sz_ub = "16")]
pub struct BITSTRING93(BitVec<Msb0, u8>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct WAGFIDchoiceExtensions {}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct WLANMeasConfigNameItemiEExtensions_Item {}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "65535")]
pub struct WLANMeasConfigNameItemiEExtensions(Vec<WLANMeasConfigNameItemiEExtensions_Item>);

#[derive(Debug, AperCodec)]
#[asn(type = "ENUMERATED", extensible = true, lb = "0", ub = "0")]
pub struct ENUMERATED94(u8);
impl ENUMERATED94 {
    const TRUE: u8 = 0u8;
}

#[derive(Debug, AperCodec)]
#[asn(type = "ENUMERATED", extensible = true, lb = "0", ub = "0")]
pub struct ENUMERATED95(u8);
impl ENUMERATED95 {
    const TRUE: u8 = 0u8;
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct WLANMeasurementConfigurationiEExtensions_Item {}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "65535")]
pub struct WLANMeasurementConfigurationiEExtensions(Vec<WLANMeasurementConfigurationiEExtensions_Item>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct WUSAssistanceInformationiEExtensions_Item {}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "65535")]
pub struct WUSAssistanceInformationiEExtensions(Vec<WUSAssistanceInformationiEExtensions_Item>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct WarningAreaListchoiceExtensions {}

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum WriteReplaceWarningRequestprotocolIEs_Itemvalue {
    #[asn(key = 17)]
    idConcurrentWarningMessageInd(ConcurrentWarningMessageInd),
    #[asn(key = 20)]
    idDataCodingScheme(DataCodingScheme),
    #[asn(key = 35)]
    idMessageIdentifier(MessageIdentifier),
    #[asn(key = 47)]
    idNumberOfBroadcastsRequested(NumberOfBroadcastsRequested),
    #[asn(key = 87)]
    idRepetitionPeriod(RepetitionPeriod),
    #[asn(key = 95)]
    idSerialNumber(SerialNumber),
    #[asn(key = 141)]
    idWarningAreaCoordinates(WarningAreaCoordinates),
    #[asn(key = 122)]
    idWarningAreaList(WarningAreaList),
    #[asn(key = 123)]
    idWarningMessageContents(WarningMessageContents),
    #[asn(key = 124)]
    idWarningSecurityInfo(WarningSecurityInfo),
    #[asn(key = 125)]
    idWarningType(WarningType),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct WriteReplaceWarningRequestprotocolIEs_Item {
    #[asn(key_field = true)]
    pub id: ProtocolIEID,
    pub criticality: Criticality,
    pub value: WriteReplaceWarningRequestprotocolIEs_Itemvalue,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "0", sz_ub = "65535")]
pub struct WriteReplaceWarningRequestprotocolIEs(Vec<WriteReplaceWarningRequestprotocolIEs_Item>);

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum WriteReplaceWarningResponseprotocolIEs_Itemvalue {
    #[asn(key = 13)]
    idBroadcastCompletedAreaList(BroadcastCompletedAreaList),
    #[asn(key = 19)]
    idCriticalityDiagnostics(CriticalityDiagnostics),
    #[asn(key = 35)]
    idMessageIdentifier(MessageIdentifier),
    #[asn(key = 95)]
    idSerialNumber(SerialNumber),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct WriteReplaceWarningResponseprotocolIEs_Item {
    #[asn(key_field = true)]
    pub id: ProtocolIEID,
    pub criticality: Criticality,
    pub value: WriteReplaceWarningResponseprotocolIEs_Itemvalue,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "0", sz_ub = "65535")]
pub struct WriteReplaceWarningResponseprotocolIEs(Vec<WriteReplaceWarningResponseprotocolIEs_Item>);

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum XnExtTLAItemiEExtensions_ItemextensionValue {
    #[asn(key = 173)]
    idSCTPTLAs(SCTPTLAs),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct XnExtTLAItemiEExtensions_Item {
    #[asn(key_field = true)]
    pub id: ProtocolExtensionID,
    pub criticality: Criticality,
    pub extension_value: XnExtTLAItemiEExtensions_ItemextensionValue,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "65535")]
pub struct XnExtTLAItemiEExtensions(Vec<XnExtTLAItemiEExtensions_Item>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct XnTNLConfigurationInfoiEExtensions_Item {}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "65535")]
pub struct XnTNLConfigurationInfoiEExtensions(Vec<XnTNLConfigurationInfoiEExtensions_Item>);

