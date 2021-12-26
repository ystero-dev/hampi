# ! [allow (dead_code , unreachable_patterns , non_camel_case_types)]use bitvec::vec::BitVec;
use bitvec::order::Msb0;
use asn1_codecs_derive::AperCodec;

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct AmfTnlAssociationSetupItem {
    pub amf_tnl_association_address: CpTransportLayerInformation,
    #[asn(optional_idx = 0)]
    pub ie_extensions: Option<AmfTnlAssociationSetupItemiEExtensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "32")]
pub struct AmfTnlAssociationSetupList(Vec<AmfTnlAssociationSetupItem>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 2)]
pub struct AmfTnlAssociationToAddItem {
    pub amf_tnl_association_address: CpTransportLayerInformation,
    #[asn(optional_idx = 0)]
    pub tnl_association_usage: Option<TnlAssociationUsage>,
    pub tnl_address_weight_factor: TnlAddressWeightFactor,
    #[asn(optional_idx = 1)]
    pub ie_extensions: Option<AmfTnlAssociationToAddItemiEExtensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "32")]
pub struct AmfTnlAssociationToAddList(Vec<AmfTnlAssociationToAddItem>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct AmfTnlAssociationToRemoveItem {
    pub amf_tnl_association_address: CpTransportLayerInformation,
    #[asn(optional_idx = 0)]
    pub ie_extensions: Option<AmfTnlAssociationToRemoveItemiEExtensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "32")]
pub struct AmfTnlAssociationToRemoveList(Vec<AmfTnlAssociationToRemoveItem>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 3)]
pub struct AmfTnlAssociationToUpdateItem {
    pub amf_tnl_association_address: CpTransportLayerInformation,
    #[asn(optional_idx = 0)]
    pub tnl_association_usage: Option<TnlAssociationUsage>,
    #[asn(optional_idx = 1)]
    pub tnl_address_weight_factor: Option<TnlAddressWeightFactor>,
    #[asn(optional_idx = 2)]
    pub ie_extensions: Option<AmfTnlAssociationToUpdateItemiEExtensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "32")]
pub struct AmfTnlAssociationToUpdateList(Vec<AmfTnlAssociationToUpdateItem>);

#[derive(Debug, AperCodec)]
#[asn(type = "INTEGER", lb = "0", ub = "1099511627775")]
pub struct AmfUeNgapId(u64);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true)]
pub struct AmfcpRelocationIndication {
    pub protocol_i_es: AmfcpRelocationIndicationprotocolIEs,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true)]
pub struct AmfConfigurationUpdate {
    pub protocol_i_es: AmfConfigurationUpdateprotocolIEs,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true)]
pub struct AmfConfigurationUpdateAcknowledge {
    pub protocol_i_es: AmfConfigurationUpdateAcknowledgeprotocolIEs,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true)]
pub struct AmfConfigurationUpdateFailure {
    pub protocol_i_es: AmfConfigurationUpdateFailureprotocolIEs,
}

#[derive(Debug, AperCodec)]
#[asn(type = "PrintableString", sz_extensible = true, sz_lb = "1", sz_ub = "150")]
pub struct AmfName(String);

#[derive(Debug, AperCodec)]
#[asn(type = "UTF8String", sz_extensible = true, sz_lb = "1", sz_ub = "150")]
pub struct AmfNameUtf8String(String);

#[derive(Debug, AperCodec)]
#[asn(type = "VisibleString", sz_extensible = true, sz_lb = "1", sz_ub = "150")]
pub struct AmfNameVisibleString(String);

#[derive(Debug, AperCodec)]
#[asn(type = "CHOICE", lb = "0", ub = "2", extensible = false)]
pub enum AmfPagingTarget {
    #[asn(key = 0, extended = false)]
    GlobalRanNodeId(GlobalRanNodeId),
    #[asn(key = 1, extended = false)]
    TAi(Tai),
    #[asn(key = 2, extended = false)]
    ChoiceExtensions(AmfPagingTargetchoiceExtensions),
}

#[derive(Debug, AperCodec)]
#[asn(type = "BITSTRING", sz_extensible = false, sz_lb = "6", sz_ub = "6")]
pub struct AmfPointer(BitVec<Msb0, u8>);

#[derive(Debug, AperCodec)]
#[asn(type = "BITSTRING", sz_extensible = false, sz_lb = "8", sz_ub = "8")]
pub struct AmfRegionId(BitVec<Msb0, u8>);

#[derive(Debug, AperCodec)]
#[asn(type = "BITSTRING", sz_extensible = false, sz_lb = "10", sz_ub = "10")]
pub struct AmfSetId(BitVec<Msb0, u8>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true)]
pub struct AmfStatusIndication {
    pub protocol_i_es: AmfStatusIndicationprotocolIEs,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 2)]
pub struct AdditionalDluptnlInformationForHoItem {
    pub additional_dl_ngu_up_tnl_information: UpTransportLayerInformation,
    pub additional_qos_flow_setup_response_list: QosFlowListWithDataForwarding,
    #[asn(optional_idx = 0)]
    pub additional_dl_forwarding_uptnl_information: Option<UpTransportLayerInformation>,
    #[asn(optional_idx = 1)]
    pub ie_extensions: Option<AdditionalDluptnlInformationForHoItemiEExtensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "3")]
pub struct AdditionalDluptnlInformationForHoList(Vec<AdditionalDluptnlInformationForHoItem>);

#[derive(Debug, AperCodec)]
#[asn(type = "ENUMERATED", extensible = true, lb = "0", ub = "0")]
pub struct AdditionalQosFlowInformation(u8);
impl AdditionalQosFlowInformation {
    const MORE_LIKELY: u8 = 0u8;
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct AllocationAndRetentionPriority {
    pub priority_level_arp: PriorityLevelArp,
    pub pre_emption_capability: PreEmptionCapability,
    pub pre_emption_vulnerability: PreEmptionVulnerability,
    #[asn(optional_idx = 0)]
    pub ie_extensions: Option<AllocationAndRetentionPriorityiEExtensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "256")]
pub struct AllowedCagListPerPlmn(Vec<CagId>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct AllowedPniNpnItem {
    pub plmn_identity: PlmnIdentity,
    pub pni_npn_restricted: Enumerated2,
    pub allowed_cag_list_per_plmn: AllowedCagListPerPlmn,
    #[asn(optional_idx = 0)]
    pub ie_extensions: Option<AllowedPniNpnItemiEExtensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "16")]
pub struct AllowedPniNpnList(Vec<AllowedPniNpnItem>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "8")]
pub struct AllowedNssai(Vec<AllowedNssaiItem>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct AllowedNssaiItem {
    pub s_nssai: SNssai,
    #[asn(optional_idx = 0)]
    pub ie_extensions: Option<AllowedNssaiItemiEExtensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "16")]
pub struct AllowedTaCs(Vec<Tac>);

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
    pub area_of_interest_tai_list: Option<AreaOfInterestTaiList>,
    #[asn(optional_idx = 1)]
    pub area_of_interest_cell_list: Option<AreaOfInterestCellList>,
    #[asn(optional_idx = 2)]
    pub area_of_interest_ran_node_list: Option<AreaOfInterestRanNodeList>,
    #[asn(optional_idx = 3)]
    pub ie_extensions: Option<AreaOfInterestiEExtensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct AreaOfInterestCellItem {
    pub ngran_cgi: NgranCgi,
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
    pub location_reporting_reference_id: LocationReportingReferenceId,
    #[asn(optional_idx = 0)]
    pub ie_extensions: Option<AreaOfInterestItemiEExtensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "64")]
pub struct AreaOfInterestList(Vec<AreaOfInterestItem>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct AreaOfInterestRanNodeItem {
    pub global_ran_node_id: GlobalRanNodeId,
    #[asn(optional_idx = 0)]
    pub ie_extensions: Option<AreaOfInterestRanNodeItemiEExtensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "64")]
pub struct AreaOfInterestRanNodeList(Vec<AreaOfInterestRanNodeItem>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct AreaOfInterestTaiItem {
    pub tai: Tai,
    #[asn(optional_idx = 0)]
    pub ie_extensions: Option<AreaOfInterestTaiItemiEExtensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "16")]
pub struct AreaOfInterestTaiList(Vec<AreaOfInterestTaiItem>);

#[derive(Debug, AperCodec)]
#[asn(type = "CHOICE", lb = "0", ub = "4", extensible = false)]
pub enum AreaScopeOfMdtEutra {
    #[asn(key = 0, extended = false)]
    CellBased(CellBasedMdtEutra),
    #[asn(key = 1, extended = false)]
    TABased(TaBasedMdt),
    #[asn(key = 2, extended = false)]
    PLmnWide(Null3),
    #[asn(key = 3, extended = false)]
    TAiBased(TaiBasedMdt),
    #[asn(key = 4, extended = false)]
    ChoiceExtensions(AreaScopeOfMdtEutrAchoiceExtensions),
}

#[derive(Debug, AperCodec)]
#[asn(type = "CHOICE", lb = "0", ub = "4", extensible = false)]
pub enum AreaScopeOfMdtNr {
    #[asn(key = 0, extended = false)]
    CellBased(CellBasedMdtNr),
    #[asn(key = 1, extended = false)]
    TABased(TaBasedMdt),
    #[asn(key = 2, extended = false)]
    PLmnWide(Null4),
    #[asn(key = 3, extended = false)]
    TAiBased(TaiBasedMdt),
    #[asn(key = 4, extended = false)]
    ChoiceExtensions(AreaScopeOfMdtNRchoiceExtensions),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 2)]
pub struct AreaScopeOfNeighCellsItem {
    pub nr_frequency_info: NrFrequencyInfo,
    #[asn(optional_idx = 0)]
    pub pci_list_for_mdt: Option<PciListForMdt>,
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
    pub qos_flow_mapping_indication: Option<Enumerated5>,
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
    pub bt_rssi: Option<Enumerated6>,
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
    CellIdCancelledEutra(CellIdCancelledEutra),
    #[asn(key = 1, extended = false)]
    TAiCancelledEutra(TaiCancelledEutra),
    #[asn(key = 2, extended = false)]
    EmergencyAreaIdCancelledEutra(EmergencyAreaIdCancelledEutra),
    #[asn(key = 3, extended = false)]
    CellIdCancelledNr(CellIdCancelledNr),
    #[asn(key = 4, extended = false)]
    TAiCancelledNr(TaiCancelledNr),
    #[asn(key = 5, extended = false)]
    EmergencyAreaIdCancelledNr(EmergencyAreaIdCancelledNr),
    #[asn(key = 6, extended = false)]
    ChoiceExtensions(BroadcastCancelledAreaListchoiceExtensions),
}

#[derive(Debug, AperCodec)]
#[asn(type = "CHOICE", lb = "0", ub = "6", extensible = false)]
pub enum BroadcastCompletedAreaList {
    #[asn(key = 0, extended = false)]
    CellIdBroadcastEutra(CellIdBroadcastEutra),
    #[asn(key = 1, extended = false)]
    TAiBroadcastEutra(TaiBroadcastEutra),
    #[asn(key = 2, extended = false)]
    EmergencyAreaIdBroadcastEutra(EmergencyAreaIdBroadcastEutra),
    #[asn(key = 3, extended = false)]
    CellIdBroadcastNr(CellIdBroadcastNr),
    #[asn(key = 4, extended = false)]
    TAiBroadcastNr(TaiBroadcastNr),
    #[asn(key = 5, extended = false)]
    EmergencyAreaIdBroadcastNr(EmergencyAreaIdBroadcastNr),
    #[asn(key = 6, extended = false)]
    ChoiceExtensions(BroadcastCompletedAreaListchoiceExtensions),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct BroadcastPlmnItem {
    pub plmn_identity: PlmnIdentity,
    pub tai_slice_support_list: SliceSupportList,
    #[asn(optional_idx = 0)]
    pub ie_extensions: Option<BroadcastPlmnItemiEExtensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "12")]
pub struct BroadcastPlmnList(Vec<BroadcastPlmnItem>);

#[derive(Debug, AperCodec)]
#[asn(type = "OCTET-STRING")]
pub struct BurstArrivalTime(Vec<u8>);

#[derive(Debug, AperCodec)]
#[asn(type = "BITSTRING", sz_extensible = false, sz_lb = "32", sz_ub = "32")]
pub struct CagId(BitVec<Msb0, u8>);

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
pub struct CnAssistedRanTuning {
    #[asn(optional_idx = 0)]
    pub expected_ue_behaviour: Option<ExpectedUeBehaviour>,
    #[asn(optional_idx = 1)]
    pub ie_extensions: Option<CnAssistedRanTuningiEExtensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "15")]
pub struct CnTypeRestrictionsForEquivalent(Vec<CnTypeRestrictionsForEquivalentItem>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct CnTypeRestrictionsForEquivalentItem {
    pub plmn_identity: PlmnIdentity,
    pub cn_type: Enumerated7,
    #[asn(optional_idx = 0)]
    pub ie_extensions: Option<CnTypeRestrictionsForEquivalentItemiEExtensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "ENUMERATED", extensible = true, lb = "0", ub = "0")]
pub struct CnTypeRestrictionsForServing(u8);
impl CnTypeRestrictionsForServing {
    const EPC_FORBIDDEN: u8 = 0u8;
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct CountValueForPdcpSn12 {
    pub pdcp_sn12: Integer8,
    pub hfn_pdcp_sn12: Integer9,
    #[asn(optional_idx = 0)]
    pub ie_extensions: Option<CountValueForPdcpSn12iEExtensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct CountValueForPdcpSn18 {
    pub pdcp_sn18: Integer10,
    pub hfn_pdcp_sn18: Integer11,
    #[asn(optional_idx = 0)]
    pub ie_extensions: Option<CountValueForPdcpSn18iEExtensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "CHOICE", lb = "0", ub = "1", extensible = false)]
pub enum CpTransportLayerInformation {
    #[asn(key = 0, extended = false)]
    EndpointIpAddress(TransportLayerAddress),
    #[asn(key = 1, extended = false)]
    ChoiceExtensions(CpTransportLayerInformationchoiceExtensions),
}

#[derive(Debug, AperCodec)]
#[asn(type = "ENUMERATED", extensible = true, lb = "0", ub = "0")]
pub struct CancelAllWarningMessages(u8);
impl CancelAllWarningMessages {
    const TRUE: u8 = 0u8;
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "65535")]
pub struct CancelledCellsInEaiEutra(Vec<CancelledCellsInEaiEutraItem>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct CancelledCellsInEaiEutraItem {
    pub eutra_cgi: EutraCgi,
    pub number_of_broadcasts: NumberOfBroadcasts,
    #[asn(optional_idx = 0)]
    pub ie_extensions: Option<CancelledCellsInEaiEutraItemiEExtensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "65535")]
pub struct CancelledCellsInEaiNr(Vec<CancelledCellsInEaiNrItem>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct CancelledCellsInEaiNrItem {
    pub nr_cgi: NrCgi,
    pub number_of_broadcasts: NumberOfBroadcasts,
    #[asn(optional_idx = 0)]
    pub ie_extensions: Option<CancelledCellsInEaiNrItemiEExtensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "65535")]
pub struct CancelledCellsInTaiEutra(Vec<CancelledCellsInTaiEutraItem>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct CancelledCellsInTaiEutraItem {
    pub eutra_cgi: EutraCgi,
    pub number_of_broadcasts: NumberOfBroadcasts,
    #[asn(optional_idx = 0)]
    pub ie_extensions: Option<CancelledCellsInTaiEutraItemiEExtensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "65535")]
pub struct CancelledCellsInTaiNr(Vec<CancelledCellsInTaiNrItem>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct CancelledCellsInTaiNrItem {
    pub nr_cgi: NrCgi,
    pub number_of_broadcasts: NumberOfBroadcasts,
    #[asn(optional_idx = 0)]
    pub ie_extensions: Option<CancelledCellsInTaiNrItemiEExtensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "CHOICE", lb = "0", ub = "2", extensible = false)]
pub enum CandidateCell {
    #[asn(key = 0, extended = false)]
    CandidateCgi(CandidateCellId),
    #[asn(key = 1, extended = false)]
    CandidatePci(CandidatePci),
    #[asn(key = 2, extended = false)]
    ChoiceExtensions(CandidateCellchoiceExtensions),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct CandidateCellId {
    pub candidate_cell_id: NrCgi,
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
pub struct CandidatePci {
    pub candidate_pci: Integer12,
    pub candidate_nrarfcn: Integer13,
    #[asn(optional_idx = 0)]
    pub ie_extensions: Option<CandidatePcIiEExtensions>,
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
    ChoiceExtensions(CausechoiceExtensions),
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
pub struct CellCagInformation {
    pub ngran_cgi: NgranCgi,
    pub cell_cag_list: CellCagList,
    #[asn(optional_idx = 0)]
    pub ie_extensions: Option<CellCagInformationiEExtensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct CellBasedMdtEutra {
    pub cell_id_listfor_mdt: CellIdListforMdtEutra,
    #[asn(optional_idx = 0)]
    pub ie_extensions: Option<CellBasedMdtEutrAiEExtensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct CellBasedMdtNr {
    pub cell_id_listfor_mdt: CellIdListforMdtNr,
    #[asn(optional_idx = 0)]
    pub ie_extensions: Option<CellBasedMdtNRiEExtensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "64")]
pub struct CellCagList(Vec<CagId>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "65535")]
pub struct CellIdBroadcastEutra(Vec<CellIdBroadcastEutraItem>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct CellIdBroadcastEutraItem {
    pub eutra_cgi: EutraCgi,
    #[asn(optional_idx = 0)]
    pub ie_extensions: Option<CellIdBroadcastEutraItemiEExtensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "65535")]
pub struct CellIdBroadcastNr(Vec<CellIdBroadcastNrItem>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct CellIdBroadcastNrItem {
    pub nr_cgi: NrCgi,
    #[asn(optional_idx = 0)]
    pub ie_extensions: Option<CellIdBroadcastNrItemiEExtensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "65535")]
pub struct CellIdCancelledEutra(Vec<CellIdCancelledEutraItem>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct CellIdCancelledEutraItem {
    pub eutra_cgi: EutraCgi,
    pub number_of_broadcasts: NumberOfBroadcasts,
    #[asn(optional_idx = 0)]
    pub ie_extensions: Option<CellIdCancelledEutraItemiEExtensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "65535")]
pub struct CellIdCancelledNr(Vec<CellIdCancelledNrItem>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct CellIdCancelledNrItem {
    pub nr_cgi: NrCgi,
    pub number_of_broadcasts: NumberOfBroadcasts,
    #[asn(optional_idx = 0)]
    pub ie_extensions: Option<CellIdCancelledNrItemiEExtensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "CHOICE", lb = "0", ub = "2", extensible = false)]
pub enum CellIdListForRestart {
    #[asn(key = 0, extended = false)]
    EUtraCgiListforRestart(EutraCgiList),
    #[asn(key = 1, extended = false)]
    NRCgiListforRestart(NrCgiList),
    #[asn(key = 2, extended = false)]
    ChoiceExtensions(CellIdListForRestartchoiceExtensions),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "32")]
pub struct CellIdListforMdtEutra(Vec<EutraCgi>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "32")]
pub struct CellIdListforMdtNr(Vec<NrCgi>);

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
pub struct CompletedCellsInEaiEutra(Vec<CompletedCellsInEaiEutraItem>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct CompletedCellsInEaiEutraItem {
    pub eutra_cgi: EutraCgi,
    #[asn(optional_idx = 0)]
    pub ie_extensions: Option<CompletedCellsInEaiEutraItemiEExtensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "65535")]
pub struct CompletedCellsInEaiNr(Vec<CompletedCellsInEaiNrItem>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct CompletedCellsInEaiNrItem {
    pub nr_cgi: NrCgi,
    #[asn(optional_idx = 0)]
    pub ie_extensions: Option<CompletedCellsInEaiNrItemiEExtensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "65535")]
pub struct CompletedCellsInTaiEutra(Vec<CompletedCellsInTaiEutraItem>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct CompletedCellsInTaiEutraItem {
    pub eutra_cgi: EutraCgi,
    #[asn(optional_idx = 0)]
    pub ie_extensions: Option<CompletedCellsInTaiEutraItemiEExtensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "65535")]
pub struct CompletedCellsInTaiNr(Vec<CompletedCellsInTaiNrItem>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct CompletedCellsInTaiNrItem {
    pub nr_cgi: NrCgi,
    #[asn(optional_idx = 0)]
    pub ie_extensions: Option<CompletedCellsInTaiNrItemiEExtensions>,
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
pub struct ConfiguredNssai(Vec<u8>);

#[derive(Debug, AperCodec)]
#[asn(type = "ENUMERATED", extensible = true, lb = "0", ub = "0")]
pub struct ConfiguredTacIndication(u8);
impl ConfiguredTacIndication {
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
    pub ue_identity_index_value: UeIdentityIndexValue,
    #[asn(optional_idx = 0)]
    pub ue_specific_drx: Option<PagingDrx>,
    pub periodic_registration_update_timer: PeriodicRegistrationUpdateTimer,
    #[asn(optional_idx = 1)]
    pub mico_mode_indication: Option<MicoModeIndication>,
    pub tai_list_for_inactive: TaiListForInactive,
    #[asn(optional_idx = 2)]
    pub expected_ue_behaviour: Option<ExpectedUeBehaviour>,
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
    pub i_es_criticality_diagnostics: Option<CriticalityDiagnosticsIeList>,
    #[asn(optional_idx = 4)]
    pub ie_extensions: Option<CriticalityDiagnosticsiEExtensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct CriticalityDiagnosticsIeItem {
    pub ie_criticality: Criticality,
    pub ie_id: ProtocolIeId,
    pub type_of_error: TypeOfError,
    #[asn(optional_idx = 0)]
    pub ie_extensions: Option<CriticalityDiagnosticsIeItemiEExtensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "256")]
pub struct CriticalityDiagnosticsIeList(Vec<CriticalityDiagnosticsIeItem>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct DapsRequestInfo {
    pub daps_indicator: Enumerated14,
    #[asn(optional_idx = 0)]
    pub ie_extensions: Option<DapsRequestInfoiEExtensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct DapsResponseInfo {
    pub dapsresponseindicator: Enumerated15,
    #[asn(optional_idx = 0)]
    pub ie_extensions: Option<DapsResponseInfoiEExtensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct DapsResponseInfoItem {
    pub drb_id: DrbId,
    pub daps_response_info: DapsResponseInfo,
    #[asn(optional_idx = 0)]
    pub ie_extension: Option<DapsResponseInfoItemiEExtension>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "32")]
pub struct DapsResponseInfoList(Vec<DapsResponseInfoItem>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct DlCpSecurityInformation {
    pub dl_nas_mac: DlNasMac,
    #[asn(optional_idx = 0)]
    pub ie_extensions: Option<DlCpSecurityInformationiEExtensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "BITSTRING", sz_extensible = false, sz_lb = "16", sz_ub = "16")]
pub struct DlNasMac(BitVec<Msb0, u8>);

#[derive(Debug, AperCodec)]
#[asn(type = "ENUMERATED", extensible = true, lb = "0", ub = "0")]
pub struct DlNguTnlInformationReused(u8);
impl DlNguTnlInformationReused {
    const TRUE: u8 = 0u8;
}

#[derive(Debug, AperCodec)]
#[asn(type = "ENUMERATED", extensible = true, lb = "0", ub = "0")]
pub struct DlForwarding(u8);
impl DlForwarding {
    const DL_FORWARDING_PROPOSED: u8 = 0u8;
}

#[derive(Debug, AperCodec)]
#[asn(type = "INTEGER", lb = "1", ub = "32", extensible = true)]
pub struct DrbId(u8);

#[derive(Debug, AperCodec)]
#[asn(type = "CHOICE", lb = "0", ub = "2", extensible = false)]
pub enum DrbStatusDl {
    #[asn(key = 0, extended = false)]
    DRbStatusDl12(DrbStatusDl12),
    #[asn(key = 1, extended = false)]
    DRbStatusDl18(DrbStatusDl18),
    #[asn(key = 2, extended = false)]
    ChoiceExtensions(DrbStatusDLchoiceExtensions),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct DrbStatusDl12 {
    pub dl_count_value: CountValueForPdcpSn12,
    #[asn(optional_idx = 0)]
    pub ie_extension: Option<DrbStatusDl12iEExtension>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct DrbStatusDl18 {
    pub dl_count_value: CountValueForPdcpSn18,
    #[asn(optional_idx = 0)]
    pub ie_extension: Option<DrbStatusDl18iEExtension>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "CHOICE", lb = "0", ub = "2", extensible = false)]
pub enum DrbStatusUl {
    #[asn(key = 0, extended = false)]
    DRbStatusUl12(DrbStatusUl12),
    #[asn(key = 1, extended = false)]
    DRbStatusUl18(DrbStatusUl18),
    #[asn(key = 2, extended = false)]
    ChoiceExtensions(DrbStatusULchoiceExtensions),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 2)]
pub struct DrbStatusUl12 {
    pub ul_count_value: CountValueForPdcpSn12,
    #[asn(optional_idx = 0)]
    pub receive_status_of_ul_pdcp_sd_us: Option<BitString16>,
    #[asn(optional_idx = 1)]
    pub ie_extension: Option<DrbStatusUl12iEExtension>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 2)]
pub struct DrbStatusUl18 {
    pub ul_count_value: CountValueForPdcpSn18,
    #[asn(optional_idx = 0)]
    pub receive_status_of_ul_pdcp_sd_us: Option<BitString17>,
    #[asn(optional_idx = 1)]
    pub ie_extension: Option<DrbStatusUl18iEExtension>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct DrBsSubjectToEarlyStatusTransferItem {
    pub drb_id: DrbId,
    pub first_dlcount: DrbStatusDl,
    #[asn(optional_idx = 0)]
    pub ie_extension: Option<DrBsSubjectToEarlyStatusTransferItemiEExtension>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "32")]
pub struct DrBsSubjectToEarlyStatusTransferList(Vec<DrBsSubjectToEarlyStatusTransferItem>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct DrBsSubjectToStatusTransferItem {
    pub drb_id: DrbId,
    pub drb_status_ul: DrbStatusUl,
    pub drb_status_dl: DrbStatusDl,
    #[asn(optional_idx = 0)]
    pub ie_extension: Option<DrBsSubjectToStatusTransferItemiEExtension>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "32")]
pub struct DrBsSubjectToStatusTransferList(Vec<DrBsSubjectToStatusTransferItem>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct DrBsToQosFlowsMappingItem {
    pub drb_id: DrbId,
    pub associated_qos_flow_list: AssociatedQosFlowList,
    #[asn(optional_idx = 0)]
    pub ie_extensions: Option<DrBsToQosFlowsMappingItemiEExtensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "32")]
pub struct DrBsToQosFlowsMappingList(Vec<DrBsToQosFlowsMappingItem>);

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
pub struct DataForwardingResponseDrbItem {
    pub drb_id: DrbId,
    #[asn(optional_idx = 0)]
    pub dl_forwarding_up_tnl_information: Option<UpTransportLayerInformation>,
    #[asn(optional_idx = 1)]
    pub ul_forwarding_up_tnl_information: Option<UpTransportLayerInformation>,
    #[asn(optional_idx = 2)]
    pub ie_extensions: Option<DataForwardingResponseDrbItemiEExtensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "32")]
pub struct DataForwardingResponseDrbList(Vec<DataForwardingResponseDrbItem>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "256")]
pub struct DataForwardingResponseErabList(Vec<DataForwardingResponseErabListItem>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct DataForwardingResponseErabListItem {
    pub e_rab_id: ERabId,
    pub dl_forwarding_up_tnl_information: UpTransportLayerInformation,
    #[asn(optional_idx = 0)]
    pub ie_extensions: Option<DataForwardingResponseErabListItemiEExtensions>,
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
pub struct DownlinkNasTransport {
    pub protocol_i_es: DownlinkNasTransportprotocolIEs,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true)]
pub struct DownlinkNonUeAssociatedNrpPaTransport {
    pub protocol_i_es: DownlinkNonUeAssociatedNrpPaTransportprotocolIEs,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true)]
pub struct DownlinkRanConfigurationTransfer {
    pub protocol_i_es: DownlinkRanConfigurationTransferprotocolIEs,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true)]
pub struct DownlinkRanEarlyStatusTransfer {
    pub protocol_i_es: DownlinkRanEarlyStatusTransferprotocolIEs,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true)]
pub struct DownlinkRanStatusTransfer {
    pub protocol_i_es: DownlinkRanStatusTransferprotocolIEs,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true)]
pub struct DownlinkRimInformationTransfer {
    pub protocol_i_es: DownlinkRimInformationTransferprotocolIEs,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true)]
pub struct DownlinkUeAssociatedNrpPaTransport {
    pub protocol_i_es: DownlinkUeAssociatedNrpPaTransportprotocolIEs,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 5)]
pub struct Dynamic5QiDescriptor {
    pub priority_level_qos: PriorityLevelQos,
    pub packet_delay_budget: PacketDelayBudget,
    pub packet_error_rate: PacketErrorRate,
    #[asn(optional_idx = 0)]
    pub five_qi: Option<FiveQi>,
    #[asn(optional_idx = 1)]
    pub delay_critical: Option<DelayCritical>,
    #[asn(optional_idx = 2)]
    pub averaging_window: Option<AveragingWindow>,
    #[asn(optional_idx = 3)]
    pub maximum_data_burst_volume: Option<MaximumDataBurstVolume>,
    #[asn(optional_idx = 4)]
    pub ie_extensions: Option<Dynamic5QiDescriptoriEExtensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "INTEGER", lb = "0", ub = "15", extensible = true)]
pub struct ERabId(u8);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 2)]
pub struct ERabInformationItem {
    pub e_rab_id: ERabId,
    #[asn(optional_idx = 0)]
    pub dl_forwarding: Option<DlForwarding>,
    #[asn(optional_idx = 1)]
    pub ie_extensions: Option<ERabInformationItemiEExtensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "256")]
pub struct ERabInformationList(Vec<ERabInformationItem>);

#[derive(Debug, AperCodec)]
#[asn(type = "ENUMERATED", extensible = true, lb = "0", ub = "0")]
pub struct EdtSession(u8);
impl EdtSession {
    const TRUE: u8 = 0u8;
}

#[derive(Debug, AperCodec)]
#[asn(type = "OCTET-STRING")]
pub struct EnDcsonConfigurationTransfer(Vec<u8>);

#[derive(Debug, AperCodec)]
#[asn(type = "CHOICE", lb = "0", ub = "4", extensible = false)]
pub enum EnbId {
    #[asn(key = 0, extended = false)]
    MacroEnbId(BitString18),
    #[asn(key = 1, extended = false)]
    HomeEnbId(BitString19),
    #[asn(key = 2, extended = false)]
    ShortMacroEnbId(BitString20),
    #[asn(key = 3, extended = false)]
    LongMacroEnbId(BitString21),
    #[asn(key = 4, extended = false)]
    ChoiceExtensions(EnbIDchoiceExtensions),
}

#[derive(Debug, AperCodec)]
#[asn(type = "OCTET-STRING", sz_extensible = false, sz_lb = "2", sz_ub = "2")]
pub struct EpsTac(Vec<u8>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct EpsTai {
    pub plmn_identity: PlmnIdentity,
    pub eps_tac: EpsTac,
    #[asn(optional_idx = 0)]
    pub ie_extensions: Option<EpsTaIiEExtensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct EutraCgi {
    pub plmn_identity: PlmnIdentity,
    pub eutra_cell_identity: EutraCellIdentity,
    #[asn(optional_idx = 0)]
    pub ie_extensions: Option<EutraCgIiEExtensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "256")]
pub struct EutraCgiList(Vec<EutraCgi>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "65535")]
pub struct EutraCgiListForWarning(Vec<EutraCgi>);

#[derive(Debug, AperCodec)]
#[asn(type = "BITSTRING", sz_extensible = false, sz_lb = "28", sz_ub = "28")]
pub struct EutraCellIdentity(BitVec<Msb0, u8>);

#[derive(Debug, AperCodec)]
#[asn(type = "BITSTRING", sz_extensible = true, sz_lb = "16", sz_ub = "16")]
pub struct EutrAencryptionAlgorithms(BitVec<Msb0, u8>);

#[derive(Debug, AperCodec)]
#[asn(type = "BITSTRING", sz_extensible = true, sz_lb = "16", sz_ub = "16")]
pub struct EutrAintegrityProtectionAlgorithms(BitVec<Msb0, u8>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct EarlyStatusTransferTransparentContainer {
    pub procedure_stage: ProcedureStageChoice,
    #[asn(optional_idx = 0)]
    pub ie_extensions: Option<EarlyStatusTransferTransparentContaineriEExtensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "OCTET-STRING", sz_extensible = false, sz_lb = "3", sz_ub = "3")]
pub struct EmergencyAreaId(Vec<u8>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "65535")]
pub struct EmergencyAreaIdBroadcastEutra(Vec<EmergencyAreaIdBroadcastEutraItem>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct EmergencyAreaIdBroadcastEutraItem {
    pub emergency_area_id: EmergencyAreaId,
    pub completed_cells_in_eai_eutra: CompletedCellsInEaiEutra,
    #[asn(optional_idx = 0)]
    pub ie_extensions: Option<EmergencyAreaIdBroadcastEutraItemiEExtensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "65535")]
pub struct EmergencyAreaIdBroadcastNr(Vec<EmergencyAreaIdBroadcastNrItem>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct EmergencyAreaIdBroadcastNrItem {
    pub emergency_area_id: EmergencyAreaId,
    pub completed_cells_in_eai_nr: CompletedCellsInEaiNr,
    #[asn(optional_idx = 0)]
    pub ie_extensions: Option<EmergencyAreaIdBroadcastNrItemiEExtensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "65535")]
pub struct EmergencyAreaIdCancelledEutra(Vec<EmergencyAreaIdCancelledEutraItem>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct EmergencyAreaIdCancelledEutraItem {
    pub emergency_area_id: EmergencyAreaId,
    pub cancelled_cells_in_eai_eutra: CancelledCellsInEaiEutra,
    #[asn(optional_idx = 0)]
    pub ie_extensions: Option<EmergencyAreaIdCancelledEutraItemiEExtensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "65535")]
pub struct EmergencyAreaIdCancelledNr(Vec<EmergencyAreaIdCancelledNrItem>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct EmergencyAreaIdCancelledNrItem {
    pub emergency_area_id: EmergencyAreaId,
    pub cancelled_cells_in_eai_nr: CancelledCellsInEaiNr,
    #[asn(optional_idx = 0)]
    pub ie_extensions: Option<EmergencyAreaIdCancelledNrItemiEExtensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "65535")]
pub struct EmergencyAreaIdList(Vec<EmergencyAreaId>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "256")]
pub struct EmergencyAreaIdListForRestart(Vec<EmergencyAreaId>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 2)]
pub struct EmergencyFallbackIndicator {
    pub emergency_fallback_request_indicator: EmergencyFallbackRequestIndicator,
    #[asn(optional_idx = 0)]
    pub emergency_service_target_cn: Option<EmergencyServiceTargetCn>,
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
pub struct EmergencyServiceTargetCn(u8);
impl EmergencyServiceTargetCn {
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
pub struct EndpointIpAddressAndPort {
    pub endpoint_ip_address: TransportLayerAddress,
    pub port_number: PortNumber,
    #[asn(optional_idx = 0)]
    pub ie_extensions: Option<EndpointIpAddressAndPortiEExtensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "ENUMERATED", extensible = true, lb = "0", ub = "0")]
pub struct EnhancedCoverageRestriction(u8);
impl EnhancedCoverageRestriction {
    const RESTRICTED: u8 = 0u8;
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "15")]
pub struct EquivalentPlmNs(Vec<PlmnIdentity>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true)]
pub struct ErrorIndication {
    pub protocol_i_es: ErrorIndicationprotocolIEs,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct EventL1LoggedMdtConfig {
    pub l1_threshold: MeasurementThresholdL1LoggedMdt,
    pub hysteresis: Hysteresis,
    pub time_to_trigger: TimeToTrigger,
    #[asn(optional_idx = 0)]
    pub ie_extensions: Option<EventL1LoggedMdtConfigiEExtensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "CHOICE", lb = "0", ub = "2", extensible = false)]
pub enum EventTrigger {
    #[asn(key = 0, extended = false)]
    OutOfCoverage(Enumerated22),
    #[asn(key = 1, extended = false)]
    EventL1LoggedMdtConfig(EventL1LoggedMdtConfig),
    #[asn(key = 2, extended = false)]
    ChoiceExtensions(EventTriggerchoiceExtensions),
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
pub struct ExpectedHoInterval(u8);
impl ExpectedHoInterval {
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
pub struct ExpectedUeActivityBehaviour {
    #[asn(optional_idx = 0)]
    pub expected_activity_period: Option<ExpectedActivityPeriod>,
    #[asn(optional_idx = 1)]
    pub expected_idle_period: Option<ExpectedIdlePeriod>,
    #[asn(optional_idx = 2)]
    pub source_of_ue_activity_behaviour_information:
        Option<SourceOfUeActivityBehaviourInformation>,
    #[asn(optional_idx = 3)]
    pub ie_extensions: Option<ExpectedUeActivityBehaviouriEExtensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 5)]
pub struct ExpectedUeBehaviour {
    #[asn(optional_idx = 0)]
    pub expected_ue_activity_behaviour: Option<ExpectedUeActivityBehaviour>,
    #[asn(optional_idx = 1)]
    pub expected_ho_interval: Option<ExpectedHoInterval>,
    #[asn(optional_idx = 2)]
    pub expected_ue_mobility: Option<ExpectedUeMobility>,
    #[asn(optional_idx = 3)]
    pub expected_ue_moving_trajectory: Option<ExpectedUeMovingTrajectory>,
    #[asn(optional_idx = 4)]
    pub ie_extensions: Option<ExpectedUeBehaviouriEExtensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "ENUMERATED", extensible = true, lb = "0", ub = "1")]
pub struct ExpectedUeMobility(u8);
impl ExpectedUeMobility {
    const STATIONARY: u8 = 0u8;
    const MOBILE: u8 = 1u8;
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "16")]
pub struct ExpectedUeMovingTrajectory(Vec<ExpectedUeMovingTrajectoryItem>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 2)]
pub struct ExpectedUeMovingTrajectoryItem {
    pub ngran_cgi: NgranCgi,
    #[asn(optional_idx = 0)]
    pub time_stayed_in_cell: Option<Integer23>,
    #[asn(optional_idx = 1)]
    pub ie_extensions: Option<ExpectedUeMovingTrajectoryItemiEExtensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 3)]
pub struct ExtendedAmfName {
    #[asn(optional_idx = 0)]
    pub amf_name_visible_string: Option<AmfNameVisibleString>,
    #[asn(optional_idx = 1)]
    pub amf_name_utf8_string: Option<AmfNameUtf8String>,
    #[asn(optional_idx = 2)]
    pub ie_extensions: Option<ExtendedAmfNameiEExtensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "INTEGER", lb = "0", ub = "255")]
pub struct ExtendedConnectedTime(u8);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 3)]
pub struct ExtendedRanNodeName {
    #[asn(optional_idx = 0)]
    pub ran_node_name_visible_string: Option<RanNodeNameVisibleString>,
    #[asn(optional_idx = 1)]
    pub ran_node_name_utf8_string: Option<RanNodeNameUtf8String>,
    #[asn(optional_idx = 2)]
    pub ie_extensions: Option<ExtendedRanNodeNameiEExtensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "INTEGER", lb = "1", ub = "65535", extensible = true)]
pub struct ExtendedPacketDelayBudget(u16);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct ExtendedRatRestrictionInformation {
    pub primary_rat_restriction: BitString24,
    pub secondary_rat_restriction: BitString25,
    #[asn(optional_idx = 0)]
    pub ie_extensions: Option<ExtendedRatRestrictionInformationiEExtensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "INTEGER", lb = "4096", ub = "65535")]
pub struct ExtendedRncId(u16);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "65535")]
pub struct ExtendedSliceSupportList(Vec<SliceSupportItem>);

#[derive(Debug, AperCodec)]
#[asn(type = "BITSTRING", sz_extensible = false, sz_lb = "16", sz_ub = "16")]
pub struct ExtendedUeIdentityIndexValue(BitVec<Msb0, u8>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct FailureIndication {
    pub uerlf_report_container: UerlfReportContainer,
    #[asn(optional_idx = 0)]
    pub ie_extensions: Option<FailureIndicationiEExtensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct FirstDlCount {
    pub dr_bs_subject_to_early_status_transfer: DrBsSubjectToEarlyStatusTransferList,
    #[asn(optional_idx = 0)]
    pub ie_extension: Option<FirstDlCountiEExtension>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct FiveGSTmsi {
    pub amf_set_id: AmfSetId,
    pub amf_pointer: AmfPointer,
    pub five_g_tmsi: FiveGTmsi,
    #[asn(optional_idx = 0)]
    pub ie_extensions: Option<FiveGSTmsIiEExtensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "OCTET-STRING", sz_extensible = false, sz_lb = "4", sz_ub = "4")]
pub struct FiveGTmsi(Vec<u8>);

#[derive(Debug, AperCodec)]
#[asn(type = "INTEGER", lb = "0", ub = "255", extensible = true)]
pub struct FiveQi(u8);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "16")]
pub struct ForbiddenAreaInformation(Vec<ForbiddenAreaInformationItem>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct ForbiddenAreaInformationItem {
    pub plmn_identity: PlmnIdentity,
    pub forbidden_ta_cs: ForbiddenTaCs,
    #[asn(optional_idx = 0)]
    pub ie_extensions: Option<ForbiddenAreaInformationItemiEExtensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "4096")]
pub struct ForbiddenTaCs(Vec<Tac>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false, optional_fields = 1)]
pub struct FromEutraNtoNgran {
    pub sourcee_nbid: IntersystemSoNeNbid,
    pub target_ngra_nnode_id: IntersystemSonngraNnodeId,
    #[asn(optional_idx = 0)]
    pub ie_extensions: Option<FromEutraNtoNgraNiEExtensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false, optional_fields = 1)]
pub struct FromNgraNtoEutran {
    pub source_ngra_nnode_id: IntersystemSonngraNnodeId,
    pub targete_nbid: IntersystemSoNeNbid,
    #[asn(optional_idx = 0)]
    pub ie_extensions: Option<FromNgraNtoEutraNiEExtensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 4)]
pub struct GbrQosInformation {
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
    pub ie_extensions: Option<GbrQosInformationiEExtensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "CHOICE", lb = "0", ub = "1", extensible = false)]
pub enum GnbId {
    #[asn(key = 0, extended = false)]
    GNbId(BitString26),
    #[asn(key = 1, extended = false)]
    ChoiceExtensions(GnbIDchoiceExtensions),
}

#[derive(Debug, AperCodec)]
#[asn(type = "BITSTRING", sz_extensible = false, sz_lb = "22", sz_ub = "22")]
pub struct GnbSetId(BitVec<Msb0, u8>);

#[derive(Debug, AperCodec)]
#[asn(type = "OCTET-STRING", sz_extensible = false, sz_lb = "4", sz_ub = "4")]
pub struct GtpTeid(Vec<u8>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct GtpTunnel {
    pub transport_layer_address: TransportLayerAddress,
    pub gtp_teid: GtpTeid,
    #[asn(optional_idx = 0)]
    pub ie_extensions: Option<GtpTunneliEExtensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct Guami {
    pub plmn_identity: PlmnIdentity,
    pub amf_region_id: AmfRegionId,
    pub amf_set_id: AmfSetId,
    pub amf_pointer: AmfPointer,
    #[asn(optional_idx = 0)]
    pub ie_extensions: Option<GuamIiEExtensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "ENUMERATED", extensible = true, lb = "0", ub = "1")]
pub struct GuamiType(u8);
impl GuamiType {
    const NATIVE: u8 = 0u8;
    const MAPPED: u8 = 1u8;
}

#[derive(Debug, AperCodec)]
#[asn(type = "OCTET-STRING")]
pub struct GlobalCableId(Vec<u8>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct GlobalEnbId {
    pub plm_nidentity: PlmnIdentity,
    pub enb_id: EnbId,
    #[asn(optional_idx = 0)]
    pub ie_extensions: Option<GlobalEnbIDiEExtensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct GlobalGnbId {
    pub plmn_identity: PlmnIdentity,
    pub gnb_id: GnbId,
    #[asn(optional_idx = 0)]
    pub ie_extensions: Option<GlobalGnbIDiEExtensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 2)]
pub struct GlobalLineId {
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
pub struct GlobalN3iwfId {
    pub plmn_identity: PlmnIdentity,
    pub n3iwf_id: N3iwfId,
    #[asn(optional_idx = 0)]
    pub ie_extensions: Option<GlobalN3iwfIDiEExtensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct GlobalNgEnbId {
    pub plmn_identity: PlmnIdentity,
    pub ng_enb_id: NgEnbId,
    #[asn(optional_idx = 0)]
    pub ie_extensions: Option<GlobalNgEnbIDiEExtensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "CHOICE", lb = "0", ub = "3", extensible = false)]
pub enum GlobalRanNodeId {
    #[asn(key = 0, extended = false)]
    GlobalGnbId(GlobalGnbId),
    #[asn(key = 1, extended = false)]
    GlobalNgEnbId(GlobalNgEnbId),
    #[asn(key = 2, extended = false)]
    GlobalN3iwfId(GlobalN3iwfId),
    #[asn(key = 3, extended = false)]
    ChoiceExtensions(GlobalRanNodeIDchoiceExtensions),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct GlobalTngfId {
    pub plmn_identity: PlmnIdentity,
    pub tngf_id: TngfId,
    #[asn(optional_idx = 0)]
    pub ie_extensions: Option<GlobalTngfIDiEExtensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct GlobalTwifId {
    pub plmn_identity: PlmnIdentity,
    pub twif_id: TwifId,
    #[asn(optional_idx = 0)]
    pub ie_extensions: Option<GlobalTwifIDiEExtensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct GlobalWAgfId {
    pub plmn_identity: PlmnIdentity,
    pub w_agf_id: WAgfId,
    #[asn(optional_idx = 0)]
    pub ie_extensions: Option<GlobalWAgfIDiEExtensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "OCTET-STRING")]
pub struct HfcNodeId(Vec<u8>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 6)]
pub struct HoReport {
    pub handover_report_type: Enumerated27,
    pub handover_cause: Cause,
    pub sourcecell_cgi: NgranCgi,
    pub targetcell_cgi: NgranCgi,
    #[asn(optional_idx = 0)]
    pub reestablishmentcell_cgi: Option<NgranCgi>,
    #[asn(optional_idx = 1)]
    pub sourcecell_c_rnti: Option<BitString28>,
    #[asn(optional_idx = 2)]
    pub targetcellin_e_utran: Option<EutraCgi>,
    #[asn(optional_idx = 3)]
    pub mobility_information: Option<MobilityInformation>,
    #[asn(optional_idx = 4)]
    pub uerlf_report_container: Option<UerlfReportContainer>,
    #[asn(optional_idx = 5)]
    pub ie_extensions: Option<HoReportiEExtensions>,
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
    pub dl_forwarding_up_tnl_information: Option<UpTransportLayerInformation>,
    #[asn(optional_idx = 1)]
    pub qos_flow_to_be_forwarded_list: Option<QosFlowToBeForwardedList>,
    #[asn(optional_idx = 2)]
    pub data_forwarding_response_drb_list: Option<DataForwardingResponseDrbList>,
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
    pub dl_ngu_up_tnl_information: UpTransportLayerInformation,
    #[asn(optional_idx = 0)]
    pub dl_forwarding_up_tnl_information: Option<UpTransportLayerInformation>,
    #[asn(optional_idx = 1)]
    pub security_result: Option<SecurityResult>,
    pub qos_flow_setup_response_list: QosFlowListWithDataForwarding,
    #[asn(optional_idx = 2)]
    pub qos_flow_failed_to_setup_list: Option<QosFlowListWithCause>,
    #[asn(optional_idx = 3)]
    pub data_forwarding_response_drb_list: Option<DataForwardingResponseDrbList>,
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
pub struct IabAuthorized(u8);
impl IabAuthorized {
    const AUTHORIZED: u8 = 0u8;
    const NOT_AUTHORIZED: u8 = 1u8;
}

#[derive(Debug, AperCodec)]
#[asn(type = "ENUMERATED", extensible = true, lb = "0", ub = "0")]
pub struct IabSupported(u8);
impl IabSupported {
    const TRUE: u8 = 0u8;
}

#[derive(Debug, AperCodec)]
#[asn(type = "ENUMERATED", extensible = true, lb = "0", ub = "0")]
pub struct IabNodeIndication(u8);
impl IabNodeIndication {
    const TRUE: u8 = 0u8;
}

#[derive(Debug, AperCodec)]
#[asn(type = "ENUMERATED", extensible = true, lb = "0", ub = "1")]
pub struct ImsVoiceSupportIndicator(u8);
impl ImsVoiceSupportIndicator {
    const SUPPORTED: u8 = 0u8;
    const NOT_SUPPORTED: u8 = 1u8;
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 10)]
pub struct ImmediateMdtNr {
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
    pub wlan_measurement_configuration: Option<WlanMeasurementConfiguration>,
    #[asn(optional_idx = 7)]
    pub mdt_location_info: Option<MdtLocationInfo>,
    #[asn(optional_idx = 8)]
    pub sensor_measurement_configuration: Option<SensorMeasurementConfiguration>,
    #[asn(optional_idx = 9)]
    pub ie_extensions: Option<ImmediateMdtNriEExtensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "INTEGER", lb = "1", ub = "256", extensible = true)]
pub struct IndexToRfsp(u16);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct InfoOnRecommendedCellsAndRanNodesForPaging {
    pub recommended_cells_for_paging: RecommendedCellsForPaging,
    pub recommend_ran_nodes_for_paging: RecommendedRanNodesForPaging,
    #[asn(optional_idx = 0)]
    pub ie_extensions: Option<InfoOnRecommendedCellsAndRanNodesForPagingiEExtensions>,
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
pub struct InitialUeMessage {
    pub protocol_i_es: InitialUeMessageprotocolIEs,
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
    pub uerlf_report_container: Option<UerlfReportContainer>,
    #[asn(optional_idx = 1)]
    pub ie_extensions: Option<InterSystemFailureIndicationiEExtensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct InterSystemHoReport {
    pub handover_report_type: InterSystemHandoverReportType,
    #[asn(optional_idx = 0)]
    pub ie_extensions: Option<InterSystemHoReportiEExtensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "CHOICE", lb = "0", ub = "2", extensible = false)]
pub enum InterSystemHandoverReportType {
    #[asn(key = 0, extended = false)]
    TooearlyIntersystemHo(TooearlyIntersystemHo),
    #[asn(key = 1, extended = false)]
    IntersystemUnnecessaryHo(IntersystemUnnecessaryHo),
    #[asn(key = 2, extended = false)]
    ChoiceExtensions(InterSystemHandoverReportTypechoiceExtensions),
}

#[derive(Debug, AperCodec)]
#[asn(type = "BITSTRING", sz_extensible = false, sz_lb = "8", sz_ub = "8")]
pub struct InterfacesToTrace(BitVec<Msb0, u8>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct IntersystemSonConfigurationTransfer {
    pub transfer_type: IntersystemSonTransferType,
    pub intersystem_son_information: IntersystemSonInformation,
    #[asn(optional_idx = 0)]
    pub ie_extensions: Option<IntersystemSonConfigurationTransferiEExtensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "CHOICE", lb = "0", ub = "1", extensible = false)]
pub enum IntersystemSonInformation {
    #[asn(key = 0, extended = false)]
    IntersystemSonInformationReport(IntersystemSonInformationReport),
    #[asn(key = 1, extended = false)]
    ChoiceExtensions(IntersystemSonInformationchoiceExtensions),
}

#[derive(Debug, AperCodec)]
#[asn(type = "CHOICE", lb = "0", ub = "2", extensible = false)]
pub enum IntersystemSonInformationReport {
    #[asn(key = 0, extended = false)]
    HOReportInformation(InterSystemHoReport),
    #[asn(key = 1, extended = false)]
    FailureIndicationInformation(InterSystemFailureIndication),
    #[asn(key = 2, extended = false)]
    ChoiceExtensions(IntersystemSonInformationReportchoiceExtensions),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct IntersystemSonngraNnodeId {
    pub global_ran_node_id: GlobalRanNodeId,
    pub selected_tai: Tai,
    #[asn(optional_idx = 0)]
    pub ie_extensions: Option<IntersystemSonngraNnodeIDiEExtensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "CHOICE", lb = "0", ub = "2", extensible = false)]
pub enum IntersystemSonTransferType {
    #[asn(key = 0, extended = false)]
    FromEutraNtoNgran(FromEutraNtoNgran),
    #[asn(key = 1, extended = false)]
    FromNgraNtoEutran(FromNgraNtoEutran),
    #[asn(key = 2, extended = false)]
    ChoiceExtensions(IntersystemSonTransferTypechoiceExtensions),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct IntersystemSoNeNbid {
    pub globale_nbid: GlobalEnbId,
    pub selected_epstai: EpsTai,
    #[asn(optional_idx = 0)]
    pub ie_extensions: Option<IntersystemSoNeNbiDiEExtensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct IntersystemUnnecessaryHo {
    pub sourcecell_id: NgranCgi,
    pub targetcell_id: EutraCgi,
    pub early_iratho: Enumerated29,
    pub candidate_cell_list: CandidateCellList,
    #[asn(optional_idx = 0)]
    pub ie_extensions: Option<IntersystemUnnecessaryHOiEExtensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "OCTET-STRING", sz_extensible = false, sz_lb = "2", sz_ub = "2")]
pub struct Lac(Vec<u8>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct Lai {
    pub plm_nidentity: PlmnIdentity,
    pub lac: Lac,
    #[asn(optional_idx = 0)]
    pub ie_extensions: Option<LaIiEExtensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "ENUMERATED", extensible = true, lb = "0", ub = "0")]
pub struct LtemIndication(u8);
impl LtemIndication {
    const LTE_M: u8 = 0u8;
}

#[derive(Debug, AperCodec)]
#[asn(type = "OCTET-STRING")]
pub struct LteuerlfReportContainer(Vec<u8>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct LteueSidelinkAggregateMaximumBitrate {
    pub ue_sidelink_aggregate_maximum_bit_rate: BitRate,
    #[asn(optional_idx = 0)]
    pub ie_extensions: Option<LteueSidelinkAggregateMaximumBitrateiEExtensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 3)]
pub struct Ltev2xServicesAuthorized {
    #[asn(optional_idx = 0)]
    pub vehicle_ue: Option<VehicleUe>,
    #[asn(optional_idx = 1)]
    pub pedestrian_ue: Option<PedestrianUe>,
    #[asn(optional_idx = 2)]
    pub ie_extensions: Option<Ltev2xServicesAuthorizediEExtensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "CHOICE", lb = "0", ub = "4", extensible = false)]
pub enum LastVisitedCellInformation {
    #[asn(key = 0, extended = false)]
    NGranCell(LastVisitedNgranCellInformation),
    #[asn(key = 1, extended = false)]
    EUtranCell(LastVisitedEutranCellInformation),
    #[asn(key = 2, extended = false)]
    UTranCell(LastVisitedUtranCellInformation),
    #[asn(key = 3, extended = false)]
    GEranCell(LastVisitedGeranCellInformation),
    #[asn(key = 4, extended = false)]
    ChoiceExtensions(LastVisitedCellInformationchoiceExtensions),
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
pub struct LastVisitedEutranCellInformation(Vec<u8>);

#[derive(Debug, AperCodec)]
#[asn(type = "OCTET-STRING")]
pub struct LastVisitedGeranCellInformation(Vec<u8>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 3)]
pub struct LastVisitedNgranCellInformation {
    pub global_cell_id: NgranCgi,
    pub cell_type: CellType,
    pub time_ue_stayed_in_cell: TimeUeStayedInCell,
    #[asn(optional_idx = 0)]
    pub time_ue_stayed_in_cell_enhanced_granularity:
        Option<TimeUeStayedInCellEnhancedGranularity>,
    #[asn(optional_idx = 1)]
    pub ho_cause_value: Option<Cause>,
    #[asn(optional_idx = 2)]
    pub ie_extensions: Option<LastVisitedNgranCellInformationiEExtensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "OCTET-STRING")]
pub struct LastVisitedUtranCellInformation(Vec<u8>);

#[derive(Debug, AperCodec)]
#[asn(type = "ENUMERATED", extensible = true, lb = "0", ub = "1")]
pub struct LineType(u8);
impl LineType {
    const DSL: u8 = 0u8;
    const PON: u8 = 1u8;
}

#[derive(Debug, AperCodec)]
#[asn(type = "ENUMERATED", extensible = true, lb = "0", ub = "2")]
pub struct LinksToLog(u8);
impl LinksToLog {
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
pub struct LocationReportingReferenceId(u8);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 3)]
pub struct LocationReportingRequestType {
    pub event_type: EventType,
    pub report_area: ReportArea,
    #[asn(optional_idx = 0)]
    pub area_of_interest_list: Option<AreaOfInterestList>,
    #[asn(optional_idx = 1)]
    pub location_reporting_reference_id_to_be_cancelled: Option<LocationReportingReferenceId>,
    #[asn(optional_idx = 2)]
    pub ie_extensions: Option<LocationReportingRequestTypeiEExtensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 5)]
pub struct LoggedMdtNr {
    pub logging_interval: LoggingInterval,
    pub logging_duration: LoggingDuration,
    pub logged_mdt_trigger: LoggedMdtTrigger,
    #[asn(optional_idx = 0)]
    pub bluetooth_measurement_configuration: Option<BluetoothMeasurementConfiguration>,
    #[asn(optional_idx = 1)]
    pub wlan_measurement_configuration: Option<WlanMeasurementConfiguration>,
    #[asn(optional_idx = 2)]
    pub sensor_measurement_configuration: Option<SensorMeasurementConfiguration>,
    #[asn(optional_idx = 3)]
    pub area_scope_of_neigh_cells_list: Option<AreaScopeOfNeighCellsList>,
    #[asn(optional_idx = 4)]
    pub ie_extensions: Option<LoggedMdtNriEExtensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "CHOICE", lb = "0", ub = "2", extensible = false)]
pub enum LoggedMdtTrigger {
    #[asn(key = 0, extended = false)]
    Periodical(Null30),
    #[asn(key = 1, extended = false)]
    EventTrigger(EventTrigger),
    #[asn(key = 2, extended = false)]
    ChoiceExtensions(LoggedMdtTriggerchoiceExtensions),
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
    pub report_interval: ReportIntervalMdt,
    pub report_amount: ReportAmountMdt,
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
    ThresholdRsrp(ThresholdRsrp),
    #[asn(key = 1, extended = false)]
    ThresholdRsrq(ThresholdRsrq),
    #[asn(key = 2, extended = false)]
    ThresholdSinr(ThresholdSinr),
    #[asn(key = 3, extended = false)]
    ChoiceExtensions(M1ThresholdTypechoiceExtensions),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct M4Configuration {
    pub m4period: M4period,
    pub m4_links_to_log: LinksToLog,
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
    pub m5_links_to_log: LinksToLog,
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
    pub m6_links_to_log: LinksToLog,
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
    pub m7_links_to_log: LinksToLog,
    #[asn(optional_idx = 0)]
    pub ie_extensions: Option<M7ConfigurationiEExtensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "INTEGER", lb = "1", ub = "60", extensible = true)]
pub struct M7period(u8);

#[derive(Debug, AperCodec)]
#[asn(type = "ENUMERATED", extensible = true, lb = "0", ub = "2")]
pub struct MdtActivation(u8);
impl MdtActivation {
    const IMMEDIATE_MDT_ONLY: u8 = 0u8;
    const LOGGED_MDT_ONLY: u8 = 1u8;
    const IMMEDIATE_MDT_AND_TRACE: u8 = 2u8;
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 3)]
pub struct MdtConfiguration {
    #[asn(optional_idx = 0)]
    pub mdt_config_nr: Option<MdtConfigurationNr>,
    #[asn(optional_idx = 1)]
    pub mdt_config_eutra: Option<MdtConfigurationEutra>,
    #[asn(optional_idx = 2)]
    pub ie_extensions: Option<MdtConfigurationiEExtensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 2)]
pub struct MdtConfigurationEutra {
    pub mdt_activation: MdtActivation,
    pub area_scope_of_mdt: AreaScopeOfMdtEutra,
    pub mdt_mode: MdtModeEutra,
    #[asn(optional_idx = 0)]
    pub signalling_based_mdtplmn_list: Option<MdtplmnList>,
    #[asn(optional_idx = 1)]
    pub ie_extensions: Option<MdtConfigurationEutrAiEExtensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 2)]
pub struct MdtConfigurationNr {
    pub mdt_activation: MdtActivation,
    pub area_scope_of_mdt: AreaScopeOfMdtNr,
    pub mdt_mode_nr: MdtModeNr,
    #[asn(optional_idx = 0)]
    pub signalling_based_mdtplmn_list: Option<MdtplmnList>,
    #[asn(optional_idx = 1)]
    pub ie_extensions: Option<MdtConfigurationNRiEExtensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct MdtLocationInfo {
    pub mdt_location_information: MdtLocationInformation,
    #[asn(optional_idx = 0)]
    pub ie_extensions: Option<MdtLocationInfoiEExtensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "BITSTRING", sz_extensible = false, sz_lb = "8", sz_ub = "8")]
pub struct MdtLocationInformation(BitVec<Msb0, u8>);

#[derive(Debug, AperCodec)]
#[asn(type = "OCTET-STRING")]
pub struct MdtModeEutra(Vec<u8>);

#[derive(Debug, AperCodec)]
#[asn(type = "CHOICE", lb = "0", ub = "2", extensible = false)]
pub enum MdtModeNr {
    #[asn(key = 0, extended = false)]
    ImmediateMdtNr(ImmediateMdtNr),
    #[asn(key = 1, extended = false)]
    LoggedMdtNr(LoggedMdtNr),
    #[asn(key = 2, extended = false)]
    ChoiceExtensions(MdtModeNrchoiceExtensions),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "16")]
pub struct MdtplmnList(Vec<PlmnIdentity>);

#[derive(Debug, AperCodec)]
#[asn(type = "ENUMERATED", extensible = true, lb = "0", ub = "0")]
pub struct MicoModeIndication(u8);
impl MicoModeIndication {
    const TRUE: u8 = 0u8;
}

#[derive(Debug, AperCodec)]
#[asn(type = "BITSTRING", sz_extensible = false, sz_lb = "64", sz_ub = "64")]
pub struct MaskedImeisv(BitVec<Msb0, u8>);

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
pub enum MeasurementThresholdL1LoggedMdt {
    #[asn(key = 0, extended = false)]
    ThresholdRsrp(ThresholdRsrp),
    #[asn(key = 1, extended = false)]
    ThresholdRsrq(ThresholdRsrq),
    #[asn(key = 2, extended = false)]
    ChoiceExtensions(MeasurementThresholdL1LoggedMdTchoiceExtensions),
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
    pub serving_plmn: PlmnIdentity,
    #[asn(optional_idx = 0)]
    pub equivalent_plm_ns: Option<EquivalentPlmNs>,
    #[asn(optional_idx = 1)]
    pub rat_restrictions: Option<RatRestrictions>,
    #[asn(optional_idx = 2)]
    pub forbidden_area_information: Option<ForbiddenAreaInformation>,
    #[asn(optional_idx = 3)]
    pub service_area_information: Option<ServiceAreaInformation>,
    #[asn(optional_idx = 4)]
    pub ie_extensions: Option<MobilityRestrictionListiEExtensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "CHOICE", lb = "0", ub = "1", extensible = false)]
pub enum N3iwfId {
    #[asn(key = 0, extended = false)]
    N3IwfId(BitString31),
    #[asn(key = 1, extended = false)]
    ChoiceExtensions(N3iwfIDchoiceExtensions),
}

#[derive(Debug, AperCodec)]
#[asn(type = "OCTET-STRING")]
pub struct NasPdu(Vec<u8>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true)]
pub struct NasNonDeliveryIndication {
    pub protocol_i_es: NasNonDeliveryIndicationprotocolIEs,
}

#[derive(Debug, AperCodec)]
#[asn(type = "OCTET-STRING")]
pub struct NasSecurityParametersFromNgran(Vec<u8>);

#[derive(Debug, AperCodec)]
#[asn(type = "ENUMERATED", extensible = true, lb = "0", ub = "3")]
pub struct NbIoTDefaultPagingDrx(u8);
impl NbIoTDefaultPagingDrx {
    const RF128: u8 = 0u8;
    const RF256: u8 = 1u8;
    const RF512: u8 = 2u8;
    const RF1024: u8 = 3u8;
}

#[derive(Debug, AperCodec)]
#[asn(type = "ENUMERATED", extensible = true, lb = "0", ub = "15")]
pub struct NbIoTPagingTimeWindow(u8);
impl NbIoTPagingTimeWindow {
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
pub struct NbIoTPagingEDrxCycle(u8);
impl NbIoTPagingEDrxCycle {
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
pub struct NbIoTPagingEDrxInfo {
    pub nb_io_t_paging_e_drx_cycle: NbIoTPagingEDrxCycle,
    #[asn(optional_idx = 0)]
    pub nb_io_t_paging_time_window: Option<NbIoTPagingTimeWindow>,
    #[asn(optional_idx = 1)]
    pub ie_extensions: Option<NbIoTPagingEDrxInfoiEExtensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "ENUMERATED", extensible = true, lb = "0", ub = "5")]
pub struct NbIoTPagingDrx(u8);
impl NbIoTPagingDrx {
    const RF32: u8 = 0u8;
    const RF64: u8 = 1u8;
    const RF128: u8 = 2u8;
    const RF256: u8 = 3u8;
    const RF512: u8 = 4u8;
    const RF1024: u8 = 5u8;
}

#[derive(Debug, AperCodec)]
#[asn(type = "INTEGER", lb = "0", ub = "255", extensible = true)]
pub struct NbIoTUePriority(u8);

#[derive(Debug, AperCodec)]
#[asn(type = "CHOICE", lb = "0", ub = "2", extensible = true)]
pub enum NgapPdu {
    #[asn(key = 0, extended = false)]
    InitiatingMessage(InitiatingMessage),
    #[asn(key = 1, extended = false)]
    SuccessfulOutcome(SuccessfulOutcome),
    #[asn(key = 2, extended = false)]
    UnsuccessfulOutcome(UnsuccessfulOutcome),
}

#[derive(Debug, AperCodec)]
#[asn(type = "CHOICE", lb = "0", ub = "2", extensible = false)]
pub enum NgranCgi {
    #[asn(key = 0, extended = false)]
    NRCgi(NrCgi),
    #[asn(key = 1, extended = false)]
    EUtraCgi(EutraCgi),
    #[asn(key = 2, extended = false)]
    ChoiceExtensions(NgranCgIchoiceExtensions),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false, optional_fields = 2)]
pub struct NgranTnlAssociationToRemoveItem {
    pub tnl_association_transport_layer_address: CpTransportLayerInformation,
    #[asn(optional_idx = 0)]
    pub tnl_association_transport_layer_address_amf: Option<CpTransportLayerInformation>,
    #[asn(optional_idx = 1)]
    pub ie_extensions: Option<NgranTnlAssociationToRemoveItemiEExtensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "32")]
pub struct NgranTnlAssociationToRemoveList(Vec<NgranTnlAssociationToRemoveItem>);

#[derive(Debug, AperCodec)]
#[asn(type = "OCTET-STRING", sz_extensible = false, sz_lb = "8", sz_ub = "8")]
pub struct NgranTraceId(Vec<u8>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true)]
pub struct NgReset {
    pub protocol_i_es: NgResetprotocolIEs,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true)]
pub struct NgResetAcknowledge {
    pub protocol_i_es: NgResetAcknowledgeprotocolIEs,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true)]
pub struct NgSetupFailure {
    pub protocol_i_es: NgSetupFailureprotocolIEs,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true)]
pub struct NgSetupRequest {
    pub protocol_i_es: NgSetupRequestprotocolIEs,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true)]
pub struct NgSetupResponse {
    pub protocol_i_es: NgSetupResponseprotocolIEs,
}

#[derive(Debug, AperCodec)]
#[asn(type = "BITSTRING", sz_extensible = false, sz_lb = "44", sz_ub = "44")]
pub struct Nid(BitVec<Msb0, u8>);

#[derive(Debug, AperCodec)]
#[asn(type = "CHOICE", lb = "0", ub = "1", extensible = false)]
pub enum NpnAccessInformation {
    #[asn(key = 0, extended = false)]
    PNiNpnAccessInformation(CellCagList),
    #[asn(key = 1, extended = false)]
    ChoiceExtensions(NpnAccessInformationchoiceExtensions),
}

#[derive(Debug, AperCodec)]
#[asn(type = "CHOICE", lb = "0", ub = "2", extensible = false)]
pub enum NpnMobilityInformation {
    #[asn(key = 0, extended = false)]
    SNpnMobilityInformation(SnpnMobilityInformation),
    #[asn(key = 1, extended = false)]
    PNiNpnMobilityInformation(PniNpnMobilityInformation),
    #[asn(key = 2, extended = false)]
    ChoiceExtensions(NpnMobilityInformationchoiceExtensions),
}

#[derive(Debug, AperCodec)]
#[asn(type = "CHOICE", lb = "0", ub = "1", extensible = false)]
pub enum NpnPagingAssistanceInformation {
    #[asn(key = 0, extended = false)]
    PNiNpnPagingAssistance(AllowedPniNpnList),
    #[asn(key = 1, extended = false)]
    ChoiceExtensions(NpnPagingAssistanceInformationchoiceExtensions),
}

#[derive(Debug, AperCodec)]
#[asn(type = "CHOICE", lb = "0", ub = "1", extensible = false)]
pub enum NpnSupport {
    #[asn(key = 0, extended = false)]
    SNpn(Nid),
    #[asn(key = 1, extended = false)]
    ChoiceExtensions(NpnSupportchoiceExtensions),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct NrCgi {
    pub plmn_identity: PlmnIdentity,
    pub nr_cell_identity: NrCellIdentity,
    #[asn(optional_idx = 0)]
    pub ie_extensions: Option<NrCgIiEExtensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "16384")]
pub struct NrCgiList(Vec<NrCgi>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "65535")]
pub struct NrCgiListForWarning(Vec<NrCgi>);

#[derive(Debug, AperCodec)]
#[asn(type = "INTEGER", lb = "0", ub = "1007", extensible = true)]
pub struct NrPci(u16);

#[derive(Debug, AperCodec)]
#[asn(type = "INTEGER", lb = "0", ub = "3279165")]
pub struct Nrarfcn(u32);

#[derive(Debug, AperCodec)]
#[asn(type = "BITSTRING", sz_extensible = false, sz_lb = "36", sz_ub = "36")]
pub struct NrCellIdentity(BitVec<Msb0, u8>);

#[derive(Debug, AperCodec)]
#[asn(type = "INTEGER", lb = "1", ub = "1024", extensible = true)]
pub struct NrFrequencyBand(u16);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "32")]
pub struct NrFrequencyBandList(Vec<NrFrequencyBandItem>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct NrFrequencyBandItem {
    pub nr_frequency_band: NrFrequencyBand,
    #[asn(optional_idx = 0)]
    pub ie_extension: Option<NrFrequencyBandItemiEExtension>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct NrFrequencyInfo {
    pub nr_arfcn: Nrarfcn,
    pub frequency_band_list: NrFrequencyBandList,
    #[asn(optional_idx = 0)]
    pub ie_extension: Option<NrFrequencyInfoiEExtension>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "OCTET-STRING")]
pub struct NrMobilityHistoryReport(Vec<u8>);

#[derive(Debug, AperCodec)]
#[asn(type = "OCTET-STRING")]
pub struct NrpPaPdu(Vec<u8>);

#[derive(Debug, AperCodec)]
#[asn(type = "OCTET-STRING")]
pub struct NruerlfReportContainer(Vec<u8>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct NrueSidelinkAggregateMaximumBitrate {
    pub ue_sidelink_aggregate_maximum_bit_rate: BitRate,
    #[asn(optional_idx = 0)]
    pub ie_extensions: Option<NrueSidelinkAggregateMaximumBitrateiEExtensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 3)]
pub struct Nrv2xServicesAuthorized {
    #[asn(optional_idx = 0)]
    pub vehicle_ue: Option<VehicleUe>,
    #[asn(optional_idx = 1)]
    pub pedestrian_ue: Option<PedestrianUe>,
    #[asn(optional_idx = 2)]
    pub ie_extensions: Option<Nrv2xServicesAuthorizediEExtensions>,
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
pub enum NgEnbId {
    #[asn(key = 0, extended = false)]
    MacroNgEnbId(BitString32),
    #[asn(key = 1, extended = false)]
    ShortMacroNgEnbId(BitString33),
    #[asn(key = 2, extended = false)]
    LongMacroNgEnbId(BitString34),
    #[asn(key = 3, extended = false)]
    ChoiceExtensions(NgEnbIDchoiceExtensions),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 4)]
pub struct NonDynamic5QiDescriptor {
    pub five_qi: FiveQi,
    #[asn(optional_idx = 0)]
    pub priority_level_qos: Option<PriorityLevelQos>,
    #[asn(optional_idx = 1)]
    pub averaging_window: Option<AveragingWindow>,
    #[asn(optional_idx = 2)]
    pub maximum_data_burst_volume: Option<MaximumDataBurstVolume>,
    #[asn(optional_idx = 3)]
    pub ie_extensions: Option<NonDynamic5QiDescriptoriEExtensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "16")]
pub struct NotAllowedTaCs(Vec<Tac>);

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
pub struct NotifySourceNgranNode(u8);
impl NotifySourceNgranNode {
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
    ChoiceExtensions(OverloadResponsechoiceExtensions),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true)]
pub struct OverloadStart {
    pub protocol_i_es: OverloadStartprotocolIEs,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 3)]
pub struct OverloadStartNssaiItem {
    pub slice_overload_list: SliceOverloadList,
    #[asn(optional_idx = 0)]
    pub slice_overload_response: Option<OverloadResponse>,
    #[asn(optional_idx = 1)]
    pub slice_traffic_load_reduction_indication: Option<TrafficLoadReductionIndication>,
    #[asn(optional_idx = 2)]
    pub ie_extensions: Option<OverloadStartNssaiItemiEExtensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "1024")]
pub struct OverloadStartNssaiList(Vec<OverloadStartNssaiItem>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true)]
pub struct OverloadStop {
    pub protocol_i_es: OverloadStopprotocolIEs,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct Pc5FlowBitRates {
    pub guaranteed_flow_bit_rate: BitRate,
    pub maximum_flow_bit_rate: BitRate,
    #[asn(optional_idx = 0)]
    pub ie_extensions: Option<Pc5FlowBitRatesiEExtensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 3)]
pub struct Pc5QoSFlowItem {
    pub pqi: FiveQi,
    #[asn(optional_idx = 0)]
    pub pc5_flow_bit_rates: Option<Pc5FlowBitRates>,
    #[asn(optional_idx = 1)]
    pub range: Option<Range>,
    #[asn(optional_idx = 2)]
    pub ie_extensions: Option<Pc5QoSFlowItemiEExtensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "2048")]
pub struct Pc5QoSFlowList(Vec<Pc5QoSFlowItem>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 2)]
pub struct Pc5QoSParameters {
    pub pc5_qo_s_flow_list: Pc5QoSFlowList,
    #[asn(optional_idx = 0)]
    pub pc5_link_aggregate_bit_rates: Option<BitRate>,
    #[asn(optional_idx = 1)]
    pub ie_extensions: Option<Pc5QoSParametersiEExtensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "32")]
pub struct PciListForMdt(Vec<NrPci>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct PduSessionAggregateMaximumBitRate {
    pub pdu_session_aggregate_maximum_bit_rate_dl: BitRate,
    pub pdu_session_aggregate_maximum_bit_rate_ul: BitRate,
    #[asn(optional_idx = 0)]
    pub ie_extensions: Option<PduSessionAggregateMaximumBitRateiEExtensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "INTEGER", lb = "0", ub = "255")]
pub struct PduSessionId(u8);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct PduSessionResourceAdmittedItem {
    pub pdu_session_id: PduSessionId,
    pub handover_request_acknowledge_transfer: OctetString35,
    #[asn(optional_idx = 0)]
    pub ie_extensions: Option<PduSessionResourceAdmittedItemiEExtensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "256")]
pub struct PduSessionResourceAdmittedList(Vec<PduSessionResourceAdmittedItem>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct PduSessionResourceFailedToModifyItemModCfm {
    pub pdu_session_id: PduSessionId,
    pub pdu_session_resource_modify_indication_unsuccessful_transfer: OctetString36,
    #[asn(optional_idx = 0)]
    pub ie_extensions: Option<PduSessionResourceFailedToModifyItemModCfmiEExtensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct PduSessionResourceFailedToModifyItemModRes {
    pub pdu_session_id: PduSessionId,
    pub pdu_session_resource_modify_unsuccessful_transfer: OctetString37,
    #[asn(optional_idx = 0)]
    pub ie_extensions: Option<PduSessionResourceFailedToModifyItemModResiEExtensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "256")]
pub struct PduSessionResourceFailedToModifyListModCfm(Vec<PduSessionResourceFailedToModifyItemModCfm>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "256")]
pub struct PduSessionResourceFailedToModifyListModRes(Vec<PduSessionResourceFailedToModifyItemModRes>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct PduSessionResourceFailedToResumeItemResReq {
    pub pdu_session_id: PduSessionId,
    pub cause: Cause,
    #[asn(optional_idx = 0)]
    pub ie_extensions: Option<PduSessionResourceFailedToResumeItemResReqiEExtensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct PduSessionResourceFailedToResumeItemResRes {
    pub pdu_session_id: PduSessionId,
    pub cause: Cause,
    #[asn(optional_idx = 0)]
    pub ie_extensions: Option<PduSessionResourceFailedToResumeItemResResiEExtensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "256")]
pub struct PduSessionResourceFailedToResumeListResReq(Vec<PduSessionResourceFailedToResumeItemResReq>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "256")]
pub struct PduSessionResourceFailedToResumeListResRes(Vec<PduSessionResourceFailedToResumeItemResRes>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct PduSessionResourceFailedToSetupItemCxtFail {
    pub pdu_session_id: PduSessionId,
    pub pdu_session_resource_setup_unsuccessful_transfer: OctetString38,
    #[asn(optional_idx = 0)]
    pub ie_extensions: Option<PduSessionResourceFailedToSetupItemCxtFailiEExtensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct PduSessionResourceFailedToSetupItemCxtRes {
    pub pdu_session_id: PduSessionId,
    pub pdu_session_resource_setup_unsuccessful_transfer: OctetString39,
    #[asn(optional_idx = 0)]
    pub ie_extensions: Option<PduSessionResourceFailedToSetupItemCxtResiEExtensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct PduSessionResourceFailedToSetupItemHoAck {
    pub pdu_session_id: PduSessionId,
    pub handover_resource_allocation_unsuccessful_transfer: OctetString40,
    #[asn(optional_idx = 0)]
    pub ie_extensions: Option<PduSessionResourceFailedToSetupItemHoAckiEExtensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct PduSessionResourceFailedToSetupItemPsReq {
    pub pdu_session_id: PduSessionId,
    pub path_switch_request_setup_failed_transfer: OctetString41,
    #[asn(optional_idx = 0)]
    pub ie_extensions: Option<PduSessionResourceFailedToSetupItemPsReqiEExtensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct PduSessionResourceFailedToSetupItemSuRes {
    pub pdu_session_id: PduSessionId,
    pub pdu_session_resource_setup_unsuccessful_transfer: OctetString42,
    #[asn(optional_idx = 0)]
    pub ie_extensions: Option<PduSessionResourceFailedToSetupItemSuResiEExtensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "256")]
pub struct PduSessionResourceFailedToSetupListCxtFail(Vec<PduSessionResourceFailedToSetupItemCxtFail>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "256")]
pub struct PduSessionResourceFailedToSetupListCxtRes(Vec<PduSessionResourceFailedToSetupItemCxtRes>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "256")]
pub struct PduSessionResourceFailedToSetupListHoAck(Vec<PduSessionResourceFailedToSetupItemHoAck>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "256")]
pub struct PduSessionResourceFailedToSetupListPsReq(Vec<PduSessionResourceFailedToSetupItemPsReq>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "256")]
pub struct PduSessionResourceFailedToSetupListSuRes(Vec<PduSessionResourceFailedToSetupItemSuRes>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct PduSessionResourceHandoverItem {
    pub pdu_session_id: PduSessionId,
    pub handover_command_transfer: OctetString43,
    #[asn(optional_idx = 0)]
    pub ie_extensions: Option<PduSessionResourceHandoverItemiEExtensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "256")]
pub struct PduSessionResourceHandoverList(Vec<PduSessionResourceHandoverItem>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 2)]
pub struct PduSessionResourceInformationItem {
    pub pdu_session_id: PduSessionId,
    pub qos_flow_information_list: QosFlowInformationList,
    #[asn(optional_idx = 0)]
    pub dr_bs_to_qos_flows_mapping_list: Option<DrBsToQosFlowsMappingList>,
    #[asn(optional_idx = 1)]
    pub ie_extensions: Option<PduSessionResourceInformationItemiEExtensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "256")]
pub struct PduSessionResourceInformationList(Vec<PduSessionResourceInformationItem>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct PduSessionResourceItemCxtRelCpl {
    pub pdu_session_id: PduSessionId,
    #[asn(optional_idx = 0)]
    pub ie_extensions: Option<PduSessionResourceItemCxtRelCpliEExtensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct PduSessionResourceItemCxtRelReq {
    pub pdu_session_id: PduSessionId,
    #[asn(optional_idx = 0)]
    pub ie_extensions: Option<PduSessionResourceItemCxtRelReqiEExtensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct PduSessionResourceItemHoRqd {
    pub pdu_session_id: PduSessionId,
    pub handover_required_transfer: OctetString44,
    #[asn(optional_idx = 0)]
    pub ie_extensions: Option<PduSessionResourceItemHoRqdiEExtensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "256")]
pub struct PduSessionResourceListCxtRelCpl(Vec<PduSessionResourceItemCxtRelCpl>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "256")]
pub struct PduSessionResourceListCxtRelReq(Vec<PduSessionResourceItemCxtRelReq>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "256")]
pub struct PduSessionResourceListHoRqd(Vec<PduSessionResourceItemHoRqd>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true)]
pub struct PduSessionResourceModifyConfirm {
    pub protocol_i_es: PduSessionResourceModifyConfirmprotocolIEs,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 3)]
pub struct PduSessionResourceModifyConfirmTransfer {
    pub qos_flow_modify_confirm_list: QosFlowModifyConfirmList,
    pub ulngu_up_tnl_information: UpTransportLayerInformation,
    #[asn(optional_idx = 0)]
    pub additional_ng_uuptnl_information: Option<UpTransportLayerInformationPairList>,
    #[asn(optional_idx = 1)]
    pub qos_flow_failed_to_modify_list: Option<QosFlowListWithCause>,
    #[asn(optional_idx = 2)]
    pub ie_extensions: Option<PduSessionResourceModifyConfirmTransferiEExtensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true)]
pub struct PduSessionResourceModifyIndication {
    pub protocol_i_es: PduSessionResourceModifyIndicationprotocolIEs,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 2)]
pub struct PduSessionResourceModifyIndicationTransfer {
    pub dl_qos_flow_per_tnl_information: QosFlowPerTnlInformation,
    #[asn(optional_idx = 0)]
    pub additional_dl_qos_flow_per_tnl_information: Option<QosFlowPerTnlInformationList>,
    #[asn(optional_idx = 1)]
    pub ie_extensions: Option<PduSessionResourceModifyIndicationTransferiEExtensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct PduSessionResourceModifyIndicationUnsuccessfulTransfer {
    pub cause: Cause,
    #[asn(optional_idx = 0)]
    pub ie_extensions:
        Option<PduSessionResourceModifyIndicationUnsuccessfulTransferiEExtensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct PduSessionResourceModifyItemModCfm {
    pub pdu_session_id: PduSessionId,
    pub pdu_session_resource_modify_confirm_transfer: OctetString45,
    #[asn(optional_idx = 0)]
    pub ie_extensions: Option<PduSessionResourceModifyItemModCfmiEExtensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct PduSessionResourceModifyItemModInd {
    pub pdu_session_id: PduSessionId,
    pub pdu_session_resource_modify_indication_transfer: OctetString46,
    #[asn(optional_idx = 0)]
    pub ie_extensions: Option<PduSessionResourceModifyItemModIndiEExtensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 2)]
pub struct PduSessionResourceModifyItemModReq {
    pub pdu_session_id: PduSessionId,
    #[asn(optional_idx = 0)]
    pub nas_pdu: Option<NasPdu>,
    pub pdu_session_resource_modify_request_transfer: OctetString47,
    #[asn(optional_idx = 1)]
    pub ie_extensions: Option<PduSessionResourceModifyItemModReqiEExtensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct PduSessionResourceModifyItemModRes {
    pub pdu_session_id: PduSessionId,
    pub pdu_session_resource_modify_response_transfer: OctetString48,
    #[asn(optional_idx = 0)]
    pub ie_extensions: Option<PduSessionResourceModifyItemModResiEExtensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "256")]
pub struct PduSessionResourceModifyListModCfm(Vec<PduSessionResourceModifyItemModCfm>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "256")]
pub struct PduSessionResourceModifyListModInd(Vec<PduSessionResourceModifyItemModInd>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "256")]
pub struct PduSessionResourceModifyListModReq(Vec<PduSessionResourceModifyItemModReq>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "256")]
pub struct PduSessionResourceModifyListModRes(Vec<PduSessionResourceModifyItemModRes>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true)]
pub struct PduSessionResourceModifyRequest {
    pub protocol_i_es: PduSessionResourceModifyRequestprotocolIEs,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true)]
pub struct PduSessionResourceModifyRequestTransfer {
    pub protocol_i_es: PduSessionResourceModifyRequestTransferprotocolIEs,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true)]
pub struct PduSessionResourceModifyResponse {
    pub protocol_i_es: PduSessionResourceModifyResponseprotocolIEs,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 6)]
pub struct PduSessionResourceModifyResponseTransfer {
    #[asn(optional_idx = 0)]
    pub dl_ngu_up_tnl_information: Option<UpTransportLayerInformation>,
    #[asn(optional_idx = 1)]
    pub ul_ngu_up_tnl_information: Option<UpTransportLayerInformation>,
    #[asn(optional_idx = 2)]
    pub qos_flow_add_or_modify_response_list: Option<QosFlowAddOrModifyResponseList>,
    #[asn(optional_idx = 3)]
    pub additional_dl_qos_flow_per_tnl_information: Option<QosFlowPerTnlInformationList>,
    #[asn(optional_idx = 4)]
    pub qos_flow_failed_to_add_or_modify_list: Option<QosFlowListWithCause>,
    #[asn(optional_idx = 5)]
    pub ie_extensions: Option<PduSessionResourceModifyResponseTransferiEExtensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 2)]
pub struct PduSessionResourceModifyUnsuccessfulTransfer {
    pub cause: Cause,
    #[asn(optional_idx = 0)]
    pub criticality_diagnostics: Option<CriticalityDiagnostics>,
    #[asn(optional_idx = 1)]
    pub ie_extensions: Option<PduSessionResourceModifyUnsuccessfulTransferiEExtensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true)]
pub struct PduSessionResourceNotify {
    pub protocol_i_es: PduSessionResourceNotifyprotocolIEs,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct PduSessionResourceNotifyItem {
    pub pdu_session_id: PduSessionId,
    pub pdu_session_resource_notify_transfer: OctetString49,
    #[asn(optional_idx = 0)]
    pub ie_extensions: Option<PduSessionResourceNotifyItemiEExtensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "256")]
pub struct PduSessionResourceNotifyList(Vec<PduSessionResourceNotifyItem>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct PduSessionResourceNotifyReleasedTransfer {
    pub cause: Cause,
    #[asn(optional_idx = 0)]
    pub ie_extensions: Option<PduSessionResourceNotifyReleasedTransferiEExtensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 3)]
pub struct PduSessionResourceNotifyTransfer {
    #[asn(optional_idx = 0)]
    pub qos_flow_notify_list: Option<QosFlowNotifyList>,
    #[asn(optional_idx = 1)]
    pub qos_flow_released_list: Option<QosFlowListWithCause>,
    #[asn(optional_idx = 2)]
    pub ie_extensions: Option<PduSessionResourceNotifyTransferiEExtensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true)]
pub struct PduSessionResourceReleaseCommand {
    pub protocol_i_es: PduSessionResourceReleaseCommandprotocolIEs,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct PduSessionResourceReleaseCommandTransfer {
    pub cause: Cause,
    #[asn(optional_idx = 0)]
    pub ie_extensions: Option<PduSessionResourceReleaseCommandTransferiEExtensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true)]
pub struct PduSessionResourceReleaseResponse {
    pub protocol_i_es: PduSessionResourceReleaseResponseprotocolIEs,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct PduSessionResourceReleaseResponseTransfer {
    #[asn(optional_idx = 0)]
    pub ie_extensions: Option<PduSessionResourceReleaseResponseTransferiEExtensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct PduSessionResourceReleasedItemNot {
    pub pdu_session_id: PduSessionId,
    pub pdu_session_resource_notify_released_transfer: OctetString50,
    #[asn(optional_idx = 0)]
    pub ie_extensions: Option<PduSessionResourceReleasedItemNotiEExtensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct PduSessionResourceReleasedItemPsAck {
    pub pdu_session_id: PduSessionId,
    pub path_switch_request_unsuccessful_transfer: OctetString51,
    #[asn(optional_idx = 0)]
    pub ie_extensions: Option<PduSessionResourceReleasedItemPsAckiEExtensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct PduSessionResourceReleasedItemPsFail {
    pub pdu_session_id: PduSessionId,
    pub path_switch_request_unsuccessful_transfer: OctetString52,
    #[asn(optional_idx = 0)]
    pub ie_extensions: Option<PduSessionResourceReleasedItemPsFailiEExtensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct PduSessionResourceReleasedItemRelRes {
    pub pdu_session_id: PduSessionId,
    pub pdu_session_resource_release_response_transfer: OctetString53,
    #[asn(optional_idx = 0)]
    pub ie_extensions: Option<PduSessionResourceReleasedItemRelResiEExtensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "256")]
pub struct PduSessionResourceReleasedListNot(Vec<PduSessionResourceReleasedItemNot>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "256")]
pub struct PduSessionResourceReleasedListPsAck(Vec<PduSessionResourceReleasedItemPsAck>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "256")]
pub struct PduSessionResourceReleasedListPsFail(Vec<PduSessionResourceReleasedItemPsFail>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "256")]
pub struct PduSessionResourceReleasedListRelRes(Vec<PduSessionResourceReleasedItemRelRes>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct PduSessionResourceResumeItemResReq {
    pub pdu_session_id: PduSessionId,
    pub ue_context_resume_request_transfer: OctetString54,
    #[asn(optional_idx = 0)]
    pub ie_extensions: Option<PduSessionResourceResumeItemResReqiEExtensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct PduSessionResourceResumeItemResRes {
    pub pdu_session_id: PduSessionId,
    pub ue_context_resume_response_transfer: OctetString55,
    #[asn(optional_idx = 0)]
    pub ie_extensions: Option<PduSessionResourceResumeItemResResiEExtensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "256")]
pub struct PduSessionResourceResumeListResReq(Vec<PduSessionResourceResumeItemResReq>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "256")]
pub struct PduSessionResourceResumeListResRes(Vec<PduSessionResourceResumeItemResRes>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct PduSessionResourceSecondaryRatUsageItem {
    pub pdu_session_id: PduSessionId,
    pub secondary_rat_data_usage_report_transfer: OctetString56,
    #[asn(optional_idx = 0)]
    pub ie_extensions: Option<PduSessionResourceSecondaryRatUsageItemiEExtensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "256")]
pub struct PduSessionResourceSecondaryRatUsageList(Vec<PduSessionResourceSecondaryRatUsageItem>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 2)]
pub struct PduSessionResourceSetupItemCxtReq {
    pub pdu_session_id: PduSessionId,
    #[asn(optional_idx = 0)]
    pub nas_pdu: Option<NasPdu>,
    pub s_nssai: SNssai,
    pub pdu_session_resource_setup_request_transfer: OctetString57,
    #[asn(optional_idx = 1)]
    pub ie_extensions: Option<PduSessionResourceSetupItemCxtReqiEExtensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct PduSessionResourceSetupItemCxtRes {
    pub pdu_session_id: PduSessionId,
    pub pdu_session_resource_setup_response_transfer: OctetString58,
    #[asn(optional_idx = 0)]
    pub ie_extensions: Option<PduSessionResourceSetupItemCxtResiEExtensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct PduSessionResourceSetupItemHoReq {
    pub pdu_session_id: PduSessionId,
    pub s_nssai: SNssai,
    pub handover_request_transfer: OctetString59,
    #[asn(optional_idx = 0)]
    pub ie_extensions: Option<PduSessionResourceSetupItemHoReqiEExtensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 2)]
pub struct PduSessionResourceSetupItemSuReq {
    pub pdu_session_id: PduSessionId,
    #[asn(optional_idx = 0)]
    pub pdu_session_nas_pdu: Option<NasPdu>,
    pub s_nssai: SNssai,
    pub pdu_session_resource_setup_request_transfer: OctetString60,
    #[asn(optional_idx = 1)]
    pub ie_extensions: Option<PduSessionResourceSetupItemSuReqiEExtensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct PduSessionResourceSetupItemSuRes {
    pub pdu_session_id: PduSessionId,
    pub pdu_session_resource_setup_response_transfer: OctetString61,
    #[asn(optional_idx = 0)]
    pub ie_extensions: Option<PduSessionResourceSetupItemSuResiEExtensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "256")]
pub struct PduSessionResourceSetupListCxtReq(Vec<PduSessionResourceSetupItemCxtReq>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "256")]
pub struct PduSessionResourceSetupListCxtRes(Vec<PduSessionResourceSetupItemCxtRes>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "256")]
pub struct PduSessionResourceSetupListHoReq(Vec<PduSessionResourceSetupItemHoReq>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "256")]
pub struct PduSessionResourceSetupListSuReq(Vec<PduSessionResourceSetupItemSuReq>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "256")]
pub struct PduSessionResourceSetupListSuRes(Vec<PduSessionResourceSetupItemSuRes>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true)]
pub struct PduSessionResourceSetupRequest {
    pub protocol_i_es: PduSessionResourceSetupRequestprotocolIEs,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true)]
pub struct PduSessionResourceSetupRequestTransfer {
    pub protocol_i_es: PduSessionResourceSetupRequestTransferprotocolIEs,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true)]
pub struct PduSessionResourceSetupResponse {
    pub protocol_i_es: PduSessionResourceSetupResponseprotocolIEs,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 4)]
pub struct PduSessionResourceSetupResponseTransfer {
    pub dl_qos_flow_per_tnl_information: QosFlowPerTnlInformation,
    #[asn(optional_idx = 0)]
    pub additional_dl_qos_flow_per_tnl_information: Option<QosFlowPerTnlInformationList>,
    #[asn(optional_idx = 1)]
    pub security_result: Option<SecurityResult>,
    #[asn(optional_idx = 2)]
    pub qos_flow_failed_to_setup_list: Option<QosFlowListWithCause>,
    #[asn(optional_idx = 3)]
    pub ie_extensions: Option<PduSessionResourceSetupResponseTransferiEExtensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 2)]
pub struct PduSessionResourceSetupUnsuccessfulTransfer {
    pub cause: Cause,
    #[asn(optional_idx = 0)]
    pub criticality_diagnostics: Option<CriticalityDiagnostics>,
    #[asn(optional_idx = 1)]
    pub ie_extensions: Option<PduSessionResourceSetupUnsuccessfulTransferiEExtensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct PduSessionResourceSuspendItemSusReq {
    pub pdu_session_id: PduSessionId,
    pub ue_context_suspend_request_transfer: OctetString62,
    #[asn(optional_idx = 0)]
    pub ie_extensions: Option<PduSessionResourceSuspendItemSusReqiEExtensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "256")]
pub struct PduSessionResourceSuspendListSusReq(Vec<PduSessionResourceSuspendItemSusReq>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct PduSessionResourceSwitchedItem {
    pub pdu_session_id: PduSessionId,
    pub path_switch_request_acknowledge_transfer: OctetString63,
    #[asn(optional_idx = 0)]
    pub ie_extensions: Option<PduSessionResourceSwitchedItemiEExtensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "256")]
pub struct PduSessionResourceSwitchedList(Vec<PduSessionResourceSwitchedItem>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct PduSessionResourceToBeSwitchedDlItem {
    pub pdu_session_id: PduSessionId,
    pub path_switch_request_transfer: OctetString64,
    #[asn(optional_idx = 0)]
    pub ie_extensions: Option<PduSessionResourceToBeSwitchedDlItemiEExtensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "256")]
pub struct PduSessionResourceToBeSwitchedDlList(Vec<PduSessionResourceToBeSwitchedDlItem>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct PduSessionResourceToReleaseItemHoCmd {
    pub pdu_session_id: PduSessionId,
    pub handover_preparation_unsuccessful_transfer: OctetString65,
    #[asn(optional_idx = 0)]
    pub ie_extensions: Option<PduSessionResourceToReleaseItemHoCmdiEExtensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct PduSessionResourceToReleaseItemRelCmd {
    pub pdu_session_id: PduSessionId,
    pub pdu_session_resource_release_command_transfer: OctetString66,
    #[asn(optional_idx = 0)]
    pub ie_extensions: Option<PduSessionResourceToReleaseItemRelCmdiEExtensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "256")]
pub struct PduSessionResourceToReleaseListHoCmd(Vec<PduSessionResourceToReleaseItemHoCmd>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "256")]
pub struct PduSessionResourceToReleaseListRelCmd(Vec<PduSessionResourceToReleaseItemRelCmd>);

#[derive(Debug, AperCodec)]
#[asn(type = "ENUMERATED", extensible = true, lb = "0", ub = "4")]
pub struct PduSessionType(u8);
impl PduSessionType {
    const IPV4: u8 = 0u8;
    const IPV6: u8 = 1u8;
    const IPV4V6: u8 = 2u8;
    const ETHERNET: u8 = 3u8;
    const UNSTRUCTURED: u8 = 4u8;
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct PduSessionUsageReport {
    pub rat_type: Enumerated67,
    pub pdu_session_timed_report_list: VolumeTimedReportList,
    #[asn(optional_idx = 0)]
    pub ie_extensions: Option<PduSessionUsageReportiEExtensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "OCTET-STRING", sz_extensible = false, sz_lb = "3", sz_ub = "3")]
pub struct PlmnIdentity(Vec<u8>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct PlmnSupportItem {
    pub plmn_identity: PlmnIdentity,
    pub slice_support_list: SliceSupportList,
    #[asn(optional_idx = 0)]
    pub ie_extensions: Option<PlmnSupportItemiEExtensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "12")]
pub struct PlmnSupportList(Vec<PlmnSupportItem>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct PniNpnMobilityInformation {
    pub allowed_pni_npi_list: AllowedPniNpnList,
    #[asn(optional_idx = 0)]
    pub ie_extensions: Option<PniNpnMobilityInformationiEExtensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true)]
pub struct PwsCancelRequest {
    pub protocol_i_es: PwsCancelRequestprotocolIEs,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true)]
pub struct PwsCancelResponse {
    pub protocol_i_es: PwsCancelResponseprotocolIEs,
}

#[derive(Debug, AperCodec)]
#[asn(type = "CHOICE", lb = "0", ub = "2", extensible = false)]
pub enum PwsFailedCellIdList {
    #[asn(key = 0, extended = false)]
    EUtraCgiPwsFailedList(EutraCgiList),
    #[asn(key = 1, extended = false)]
    NRCgiPwsFailedList(NrCgiList),
    #[asn(key = 2, extended = false)]
    ChoiceExtensions(PwsFailedCellIdListchoiceExtensions),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true)]
pub struct PwsFailureIndication {
    pub protocol_i_es: PwsFailureIndicationprotocolIEs,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true)]
pub struct PwsRestartIndication {
    pub protocol_i_es: PwsRestartIndicationprotocolIEs,
}

#[derive(Debug, AperCodec)]
#[asn(type = "INTEGER", lb = "0", ub = "1023", extensible = true)]
pub struct PacketDelayBudget(u16);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct PacketErrorRate {
    pub per_scalar: Integer68,
    pub per_exponent: Integer69,
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
pub struct PagingEDrxCycle(u8);
impl PagingEDrxCycle {
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
pub struct PagingAssisDataforCEcapabUe {
    pub eutra_cgi: EutraCgi,
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
pub struct PagingDrx(u8);
impl PagingDrx {
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
pub struct PagingeDrxInformation {
    pub paging_e_drx_cycle: PagingEDrxCycle,
    #[asn(optional_idx = 0)]
    pub paging_time_window: Option<PagingTimeWindow>,
    #[asn(optional_idx = 1)]
    pub ie_extensions: Option<PagingeDrxInformationiEExtensions>,
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
    pub ul_ngu_up_tnl_information: Option<UpTransportLayerInformation>,
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
    pub dl_ngu_up_tnl_information: UpTransportLayerInformation,
    #[asn(optional_idx = 0)]
    pub dl_ngu_tnl_information_reused: Option<DlNguTnlInformationReused>,
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
pub struct PedestrianUe(u8);
impl PedestrianUe {
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
pub struct PreEmptionCapability(u8);
impl PreEmptionCapability {
    const SHALL_NOT_TRIGGER_PRE_EMPTION: u8 = 0u8;
    const MAY_TRIGGER_PRE_EMPTION: u8 = 1u8;
}

#[derive(Debug, AperCodec)]
#[asn(type = "ENUMERATED", extensible = true, lb = "0", ub = "1")]
pub struct PreEmptionVulnerability(u8);
impl PreEmptionVulnerability {
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
pub struct PriorityLevelArp(u8);

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
pub enum PrivateIeId {
    #[asn(key = 0, extended = false)]
    Local(Integer70),
    #[asn(key = 1, extended = false)]
    Global(ObjectIdentifier71),
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
    FirstDlCount(FirstDlCount),
    #[asn(key = 1, extended = false)]
    ChoiceExtensions(ProcedureStageChoicechoiceExtensions),
}

#[derive(Debug, AperCodec)]
#[asn(type = "INTEGER", lb = "0", ub = "65535")]
pub struct ProtocolExtensionId(u16);

#[derive(Debug, AperCodec)]
#[asn(type = "INTEGER", lb = "0", ub = "65535")]
pub struct ProtocolIeId(u16);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct QoSFlowsUsageReportItem {
    pub qos_flow_identifier: QosFlowIdentifier,
    pub rat_type: Enumerated72,
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
    NonDynamic5Qi(NonDynamic5QiDescriptor),
    #[asn(key = 1, extended = false)]
    Dynamic5Qi(Dynamic5QiDescriptor),
    #[asn(key = 2, extended = false)]
    ChoiceExtensions(QosCharacteristicschoiceExtensions),
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
    pub e_rab_id: Option<ERabId>,
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
    pub dl_forwarding: Option<DlForwarding>,
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
    pub gbr_qos_information: Option<GbrQosInformation>,
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
pub struct QosFlowPerTnlInformation {
    pub up_transport_layer_information: UpTransportLayerInformation,
    pub associated_qos_flow_list: AssociatedQosFlowList,
    #[asn(optional_idx = 0)]
    pub ie_extensions: Option<QosFlowPerTnlInformationiEExtensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct QosFlowPerTnlInformationItem {
    pub qos_flow_per_tnl_information: QosFlowPerTnlInformation,
    #[asn(optional_idx = 0)]
    pub ie_extensions: Option<QosFlowPerTnlInformationItemiEExtensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "3")]
pub struct QosFlowPerTnlInformationList(Vec<QosFlowPerTnlInformationItem>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 2)]
pub struct QosFlowSetupRequestItem {
    pub qos_flow_identifier: QosFlowIdentifier,
    pub qos_flow_level_qos_parameters: QosFlowLevelQosParameters,
    #[asn(optional_idx = 0)]
    pub e_rab_id: Option<ERabId>,
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
pub struct RanUeNgapId(u32);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true)]
pub struct RancpRelocationIndication {
    pub protocol_i_es: RancpRelocationIndicationprotocolIEs,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true)]
pub struct RanConfigurationUpdate {
    pub protocol_i_es: RanConfigurationUpdateprotocolIEs,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true)]
pub struct RanConfigurationUpdateAcknowledge {
    pub protocol_i_es: RanConfigurationUpdateAcknowledgeprotocolIEs,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true)]
pub struct RanConfigurationUpdateFailure {
    pub protocol_i_es: RanConfigurationUpdateFailureprotocolIEs,
}

#[derive(Debug, AperCodec)]
#[asn(type = "PrintableString", sz_extensible = true, sz_lb = "1", sz_ub = "150")]
pub struct RanNodeName(String);

#[derive(Debug, AperCodec)]
#[asn(type = "UTF8String", sz_extensible = true, sz_lb = "1", sz_ub = "150")]
pub struct RanNodeNameUtf8String(String);

#[derive(Debug, AperCodec)]
#[asn(type = "VisibleString", sz_extensible = true, sz_lb = "1", sz_ub = "150")]
pub struct RanNodeNameVisibleString(String);

#[derive(Debug, AperCodec)]
#[asn(type = "INTEGER", lb = "1", ub = "256")]
pub struct RanPagingPriority(u16);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct RanStatusTransferTransparentContainer {
    pub dr_bs_subject_to_status_transfer_list: DrBsSubjectToStatusTransferList,
    #[asn(optional_idx = 0)]
    pub ie_extensions: Option<RanStatusTransferTransparentContaineriEExtensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "ENUMERATED", extensible = true, lb = "0", ub = "1")]
pub struct RatInformation(u8);
impl RatInformation {
    const UNLICENSED: u8 = 0u8;
    const NB_IO_T: u8 = 1u8;
}

#[derive(Debug, AperCodec)]
#[asn(type = "BITSTRING", sz_extensible = true, sz_lb = "8", sz_ub = "8")]
pub struct RatRestrictionInformation(BitVec<Msb0, u8>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "16")]
pub struct RatRestrictions(Vec<RatRestrictionsItem>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct RatRestrictionsItem {
    pub plmn_identity: PlmnIdentity,
    pub rat_restriction_information: RatRestrictionInformation,
    #[asn(optional_idx = 0)]
    pub ie_extensions: Option<RatRestrictionsItemiEExtensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "OCTET-STRING")]
pub struct RgLevelWirelineAccessCharacteristics(Vec<u8>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct RimInformation {
    pub targetg_nb_set_id: GnbSetId,
    pub rim_rs_detection: Enumerated73,
    #[asn(optional_idx = 0)]
    pub ie_extensions: Option<RimInformationiEExtensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct RimInformationTransfer {
    pub target_ran_node_id: TargetRanNodeId,
    pub source_ran_node_id: SourceRanNodeId,
    pub rim_information: RimInformation,
    #[asn(optional_idx = 0)]
    pub ie_extensions: Option<RimInformationTransferiEExtensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "INTEGER", lb = "0", ub = "4095")]
pub struct RncId(u16);

#[derive(Debug, AperCodec)]
#[asn(type = "OCTET-STRING")]
pub struct RrcContainer(Vec<u8>);

#[derive(Debug, AperCodec)]
#[asn(type = "ENUMERATED", extensible = true, lb = "0", ub = "9")]
pub struct RrcEstablishmentCause(u8);
impl RrcEstablishmentCause {
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
pub struct RrcInactiveTransitionReport {
    pub protocol_i_es: RrcInactiveTransitionReportprotocolIEs,
}

#[derive(Debug, AperCodec)]
#[asn(type = "ENUMERATED", extensible = true, lb = "0", ub = "2")]
pub struct RrcInactiveTransitionReportRequest(u8);
impl RrcInactiveTransitionReportRequest {
    const SUBSEQUENT_STATE_TRANSITION_REPORT: u8 = 0u8;
    const SINGLE_RRC_CONNECTED_STATE_REPORT: u8 = 1u8;
    const CANCEL_REPORT: u8 = 2u8;
}

#[derive(Debug, AperCodec)]
#[asn(type = "ENUMERATED", extensible = true, lb = "0", ub = "1")]
pub struct RrcState(u8);
impl RrcState {
    const INACTIVE: u8 = 0u8;
    const CONNECTED: u8 = 1u8;
}

#[derive(Debug, AperCodec)]
#[asn(type = "ENUMERATED", extensible = true, lb = "0", ub = "1")]
pub struct Rsn(u8);
impl Rsn {
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
    pub ngran_cgi: NgranCgi,
    #[asn(optional_idx = 0)]
    pub time_stayed_in_cell: Option<Integer74>,
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
pub struct RecommendedRanNodeItem {
    pub amf_paging_target: AmfPagingTarget,
    #[asn(optional_idx = 0)]
    pub ie_extensions: Option<RecommendedRanNodeItemiEExtensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "16")]
pub struct RecommendedRanNodeList(Vec<RecommendedRanNodeItem>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct RecommendedRanNodesForPaging {
    pub recommended_ran_node_list: RecommendedRanNodeList,
    #[asn(optional_idx = 0)]
    pub ie_extensions: Option<RecommendedRanNodesForPagingiEExtensions>,
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
pub struct RedundantPduSessionInformation {
    pub rsn: Rsn,
    #[asn(optional_idx = 0)]
    pub ie_extensions: Option<RedundantPduSessionInformationiEExtensions>,
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
pub struct RejectedNssaIinPlmn(Vec<u8>);

#[derive(Debug, AperCodec)]
#[asn(type = "OCTET-STRING", sz_extensible = false, sz_lb = "32", sz_ub = "32")]
pub struct RejectedNssaIinTa(Vec<u8>);

#[derive(Debug, AperCodec)]
#[asn(type = "INTEGER", lb = "0", ub = "255")]
pub struct RelativeAmfCapacity(u8);

#[derive(Debug, AperCodec)]
#[asn(type = "INTEGER", lb = "0", ub = "131071")]
pub struct RepetitionPeriod(u32);

#[derive(Debug, AperCodec)]
#[asn(type = "ENUMERATED", lb = "0", ub = "7")]
pub struct ReportAmountMdt(u8);
impl ReportAmountMdt {
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
pub struct ReportIntervalMdt(u8);
impl ReportIntervalMdt {
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
pub struct RerouteNasRequest {
    pub protocol_i_es: RerouteNasRequestprotocolIEs,
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
    NGInterface(ResetAll),
    #[asn(key = 1, extended = false)]
    PartOfNgInterface(UeAssociatedLogicalNgConnectionList),
    #[asn(key = 2, extended = false)]
    ChoiceExtensions(ResetTypechoiceExtensions),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true)]
pub struct RetrieveUeInformation {
    pub protocol_i_es: RetrieveUeInformationprotocolIEs,
}

#[derive(Debug, AperCodec)]
#[asn(type = "OCTET-STRING")]
pub struct RoutingId(Vec<u8>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 2)]
pub struct SNssai {
    pub sst: Sst,
    #[asn(optional_idx = 0)]
    pub sd: Option<Sd>,
    #[asn(optional_idx = 1)]
    pub ie_extensions: Option<SNssaIiEExtensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "2")]
pub struct SctpTlAs(Vec<TransportLayerAddress>);

#[derive(Debug, AperCodec)]
#[asn(type = "OCTET-STRING", sz_extensible = false, sz_lb = "3", sz_ub = "3")]
pub struct Sd(Vec<u8>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct SnpnMobilityInformation {
    pub serving_nid: Nid,
    #[asn(optional_idx = 0)]
    pub ie_extensions: Option<SnpnMobilityInformationiEExtensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 2)]
pub struct SonConfigurationTransfer {
    pub target_ran_node_id: TargetRanNodeId,
    pub source_ran_node_id: SourceRanNodeId,
    pub son_information: SonInformation,
    #[asn(optional_idx = 0)]
    pub xn_tnl_configuration_info: Option<XnTnlConfigurationInfo>,
    #[asn(optional_idx = 1)]
    pub ie_extensions: Option<SonConfigurationTransferiEExtensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "CHOICE", lb = "0", ub = "2", extensible = false)]
pub enum SonInformation {
    #[asn(key = 0, extended = false)]
    SOnInformationRequest(SonInformationRequest),
    #[asn(key = 1, extended = false)]
    SOnInformationReply(SonInformationReply),
    #[asn(key = 2, extended = false)]
    ChoiceExtensions(SonInformationchoiceExtensions),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 2)]
pub struct SonInformationReply {
    #[asn(optional_idx = 0)]
    pub xn_tnl_configuration_info: Option<XnTnlConfigurationInfo>,
    #[asn(optional_idx = 1)]
    pub ie_extensions: Option<SonInformationReplyiEExtensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "CHOICE", lb = "0", ub = "2", extensible = false)]
pub enum SonInformationReport {
    #[asn(key = 0, extended = false)]
    FailureIndicationInformation(FailureIndication),
    #[asn(key = 1, extended = false)]
    HOReportInformation(HoReport),
    #[asn(key = 2, extended = false)]
    ChoiceExtensions(SonInformationReportchoiceExtensions),
}

#[derive(Debug, AperCodec)]
#[asn(type = "ENUMERATED", extensible = true, lb = "0", ub = "0")]
pub struct SonInformationRequest(u8);
impl SonInformationRequest {
    const XN_TNL_CONFIGURATION_INFO: u8 = 0u8;
}

#[derive(Debug, AperCodec)]
#[asn(type = "ENUMERATED", extensible = true, lb = "0", ub = "1")]
pub struct SrvccOperationPossible(u8);
impl SrvccOperationPossible {
    const POSSIBLE: u8 = 0u8;
    const NOT_POSSIBLE: u8 = 1u8;
}

#[derive(Debug, AperCodec)]
#[asn(type = "OCTET-STRING", sz_extensible = false, sz_lb = "1", sz_ub = "1")]
pub struct Sst(Vec<u8>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 4)]
pub struct ScheduledCommunicationTime {
    #[asn(optional_idx = 0)]
    pub dayof_week: Option<BitString75>,
    #[asn(optional_idx = 1)]
    pub timeof_day_start: Option<Integer76>,
    #[asn(optional_idx = 2)]
    pub timeof_day_end: Option<Integer77>,
    #[asn(optional_idx = 3)]
    pub ie_extensions: Option<ScheduledCommunicationTimeiEExtensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true)]
pub struct SecondaryRatDataUsageReport {
    pub protocol_i_es: SecondaryRatDataUsageReportprotocolIEs,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 2)]
pub struct SecondaryRatDataUsageReportTransfer {
    #[asn(optional_idx = 0)]
    pub secondary_rat_usage_information: Option<SecondaryRatUsageInformation>,
    #[asn(optional_idx = 1)]
    pub ie_extensions: Option<SecondaryRatDataUsageReportTransferiEExtensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 3)]
pub struct SecondaryRatUsageInformation {
    #[asn(optional_idx = 0)]
    pub pdu_session_usage_report: Option<PduSessionUsageReport>,
    #[asn(optional_idx = 1)]
    pub qos_flows_usage_report_list: Option<QoSFlowsUsageReportList>,
    #[asn(optional_idx = 2)]
    pub ie_extension: Option<SecondaryRatUsageInformationiEExtension>,
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
    UncompensatedBarometricConfig(Enumerated78),
    #[asn(key = 1, extended = false)]
    UeSpeedConfig(Enumerated79),
    #[asn(key = 2, extended = false)]
    UeOrientationConfig(Enumerated80),
    #[asn(key = 3, extended = false)]
    ChoiceExtensions(SensorNameConfigchoiceExtensions),
}

#[derive(Debug, AperCodec)]
#[asn(type = "BITSTRING", sz_extensible = false, sz_lb = "16", sz_ub = "16")]
pub struct SerialNumber(BitVec<Msb0, u8>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 2)]
pub struct ServedGuamiItem {
    pub guami: Guami,
    #[asn(optional_idx = 0)]
    pub backup_amf_name: Option<AmfName>,
    #[asn(optional_idx = 1)]
    pub ie_extensions: Option<ServedGuamiItemiEExtensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "256")]
pub struct ServedGuamiList(Vec<ServedGuamiItem>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "16")]
pub struct ServiceAreaInformation(Vec<ServiceAreaInformationItem>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 3)]
pub struct ServiceAreaInformationItem {
    pub plmn_identity: PlmnIdentity,
    #[asn(optional_idx = 0)]
    pub allowed_ta_cs: Option<AllowedTaCs>,
    #[asn(optional_idx = 1)]
    pub not_allowed_ta_cs: Option<NotAllowedTaCs>,
    #[asn(optional_idx = 2)]
    pub ie_extensions: Option<ServiceAreaInformationItemiEExtensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "INTEGER", lb = "0", ub = "4294967295")]
pub struct SgNbUeX2apId(u32);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct SliceOverloadItem {
    pub s_nssai: SNssai,
    #[asn(optional_idx = 0)]
    pub ie_extensions: Option<SliceOverloadItemiEExtensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "1024")]
pub struct SliceOverloadList(Vec<SliceOverloadItem>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct SliceSupportItem {
    pub s_nssai: SNssai,
    #[asn(optional_idx = 0)]
    pub ie_extensions: Option<SliceSupportItemiEExtensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "1024")]
pub struct SliceSupportList(Vec<SliceSupportItem>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 4)]
pub struct SourceNgranNodeToTargetNgranNodeTransparentContainer {
    pub rrc_container: RrcContainer,
    #[asn(optional_idx = 0)]
    pub pdu_session_resource_information_list: Option<PduSessionResourceInformationList>,
    #[asn(optional_idx = 1)]
    pub e_rab_information_list: Option<ERabInformationList>,
    pub target_cell_id: NgranCgi,
    #[asn(optional_idx = 2)]
    pub index_to_rfsp: Option<IndexToRfsp>,
    pub ue_history_information: UeHistoryInformation,
    #[asn(optional_idx = 3)]
    pub ie_extensions: Option<SourceNgranNodeToTargetNgranNodeTransparentContaineriEExtensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "ENUMERATED", extensible = true, lb = "0", ub = "1")]
pub struct SourceOfUeActivityBehaviourInformation(u8);
impl SourceOfUeActivityBehaviourInformation {
    const SUBSCRIPTION_INFORMATION: u8 = 0u8;
    const STATISTICS: u8 = 1u8;
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct SourceRanNodeId {
    pub global_ran_node_id: GlobalRanNodeId,
    pub selected_tai: Tai,
    #[asn(optional_idx = 0)]
    pub ie_extensions: Option<SourceRanNodeIDiEExtensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 4)]
pub struct SourceToTargetAmfInformationReroute {
    #[asn(optional_idx = 0)]
    pub configured_nssai: Option<ConfiguredNssai>,
    #[asn(optional_idx = 1)]
    pub rejected_nssa_iin_plmn: Option<RejectedNssaIinPlmn>,
    #[asn(optional_idx = 2)]
    pub rejected_nssa_iin_ta: Option<RejectedNssaIinTa>,
    #[asn(optional_idx = 3)]
    pub ie_extensions: Option<SourceToTargetAmfInformationRerouteiEExtensions>,
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
pub struct SupportedTaItem {
    pub tac: Tac,
    pub broadcast_plmn_list: BroadcastPlmnList,
    #[asn(optional_idx = 0)]
    pub ie_extensions: Option<SupportedTaItemiEExtensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "256")]
pub struct SupportedTaList(Vec<SupportedTaItem>);

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
pub struct TaBasedMdt {
    pub ta_listfor_mdt: TaListforMdt,
    #[asn(optional_idx = 0)]
    pub ie_extensions: Option<TaBasedMdTiEExtensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "OCTET-STRING", sz_extensible = false, sz_lb = "3", sz_ub = "3")]
pub struct Tac(Vec<u8>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct Tai {
    pub plmn_identity: PlmnIdentity,
    pub tac: Tac,
    #[asn(optional_idx = 0)]
    pub ie_extensions: Option<TaIiEExtensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct TaiBasedMdt {
    pub tai_listfor_mdt: TaiListforMdt,
    #[asn(optional_idx = 0)]
    pub ie_extensions: Option<TaiBasedMdTiEExtensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "65535")]
pub struct TaiBroadcastEutra(Vec<TaiBroadcastEutraItem>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct TaiBroadcastEutraItem {
    pub tai: Tai,
    pub completed_cells_in_tai_eutra: CompletedCellsInTaiEutra,
    #[asn(optional_idx = 0)]
    pub ie_extensions: Option<TaiBroadcastEutraItemiEExtensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "65535")]
pub struct TaiBroadcastNr(Vec<TaiBroadcastNrItem>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct TaiBroadcastNrItem {
    pub tai: Tai,
    pub completed_cells_in_tai_nr: CompletedCellsInTaiNr,
    #[asn(optional_idx = 0)]
    pub ie_extensions: Option<TaiBroadcastNrItemiEExtensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "65535")]
pub struct TaiCancelledEutra(Vec<TaiCancelledEutraItem>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct TaiCancelledEutraItem {
    pub tai: Tai,
    pub cancelled_cells_in_tai_eutra: CancelledCellsInTaiEutra,
    #[asn(optional_idx = 0)]
    pub ie_extensions: Option<TaiCancelledEutraItemiEExtensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "65535")]
pub struct TaiCancelledNr(Vec<TaiCancelledNrItem>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct TaiCancelledNrItem {
    pub tai: Tai,
    pub cancelled_cells_in_tai_nr: CancelledCellsInTaiNr,
    #[asn(optional_idx = 0)]
    pub ie_extensions: Option<TaiCancelledNrItemiEExtensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "16")]
pub struct TaiListForInactive(Vec<TaiListForInactiveItem>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct TaiListForInactiveItem {
    pub tai: Tai,
    #[asn(optional_idx = 0)]
    pub ie_extensions: Option<TaiListForInactiveItemiEExtensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "16")]
pub struct TaiListForPaging(Vec<TaiListForPagingItem>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct TaiListForPagingItem {
    pub tai: Tai,
    #[asn(optional_idx = 0)]
    pub ie_extensions: Option<TaiListForPagingItemiEExtensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "2048")]
pub struct TaiListForRestart(Vec<Tai>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "65535")]
pub struct TaiListForWarning(Vec<Tai>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "8")]
pub struct TaiListforMdt(Vec<Tai>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "8")]
pub struct TaListforMdt(Vec<Tac>);

#[derive(Debug, AperCodec)]
#[asn(type = "OCTET-STRING")]
pub struct TnapId(Vec<u8>);

#[derive(Debug, AperCodec)]
#[asn(type = "CHOICE", lb = "0", ub = "1", extensible = false)]
pub enum TngfId {
    #[asn(key = 0, extended = false)]
    TNgfId(BitString81),
    #[asn(key = 1, extended = false)]
    ChoiceExtensions(TngfIDchoiceExtensions),
}

#[derive(Debug, AperCodec)]
#[asn(type = "INTEGER", lb = "0", ub = "255")]
pub struct TnlAddressWeightFactor(u8);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct TnlAssociationItem {
    pub tnl_association_address: CpTransportLayerInformation,
    pub cause: Cause,
    #[asn(optional_idx = 0)]
    pub ie_extensions: Option<TnlAssociationItemiEExtensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "32")]
pub struct TnlAssociationList(Vec<TnlAssociationItem>);

#[derive(Debug, AperCodec)]
#[asn(type = "ENUMERATED", extensible = true, lb = "0", ub = "2")]
pub struct TnlAssociationUsage(u8);
impl TnlAssociationUsage {
    const UE: u8 = 0u8;
    const NON_UE: u8 = 1u8;
    const BOTH: u8 = 2u8;
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 2)]
pub struct TscAssistanceInformation {
    pub periodicity: Periodicity,
    #[asn(optional_idx = 0)]
    pub burst_arrival_time: Option<BurstArrivalTime>,
    #[asn(optional_idx = 1)]
    pub ie_extensions: Option<TscAssistanceInformationiEExtensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 3)]
pub struct TscTrafficCharacteristics {
    #[asn(optional_idx = 0)]
    pub tsc_assistance_information_dl: Option<TscAssistanceInformation>,
    #[asn(optional_idx = 1)]
    pub tsc_assistance_information_ul: Option<TscAssistanceInformation>,
    #[asn(optional_idx = 2)]
    pub ie_extensions: Option<TscTrafficCharacteristicsiEExtensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "OCTET-STRING")]
pub struct TwapId(Vec<u8>);

#[derive(Debug, AperCodec)]
#[asn(type = "CHOICE", lb = "0", ub = "1", extensible = false)]
pub enum TwifId {
    #[asn(key = 0, extended = false)]
    TWifId(BitString82),
    #[asn(key = 1, extended = false)]
    ChoiceExtensions(TwifIDchoiceExtensions),
}

#[derive(Debug, AperCodec)]
#[asn(type = "CHOICE", lb = "0", ub = "2", extensible = false)]
pub enum TargetId {
    #[asn(key = 0, extended = false)]
    TargetRanNodeId(TargetRanNodeId),
    #[asn(key = 1, extended = false)]
    TargeteNbId(TargeteNbId),
    #[asn(key = 2, extended = false)]
    ChoiceExtensions(TargetIDchoiceExtensions),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct TargetNgranNodeToSourceNgranNodeFailureTransparentContainer {
    pub cell_cag_information: CellCagInformation,
    #[asn(optional_idx = 0)]
    pub ie_extensions:
        Option<TargetNgranNodeToSourceNgranNodeFailureTransparentContaineriEExtensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct TargetNgranNodeToSourceNgranNodeTransparentContainer {
    pub rrc_container: RrcContainer,
    #[asn(optional_idx = 0)]
    pub ie_extensions: Option<TargetNgranNodeToSourceNgranNodeTransparentContaineriEExtensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct TargetRanNodeId {
    pub global_ran_node_id: GlobalRanNodeId,
    pub selected_tai: Tai,
    #[asn(optional_idx = 0)]
    pub ie_extensions: Option<TargetRanNodeIDiEExtensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 2)]
pub struct TargetRncId {
    pub lai: Lai,
    pub rnc_id: RncId,
    #[asn(optional_idx = 0)]
    pub extended_rnc_id: Option<ExtendedRncId>,
    #[asn(optional_idx = 1)]
    pub ie_extensions: Option<TargetRncIDiEExtensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "OCTET-STRING")]
pub struct TargetToSourceTransparentContainer(Vec<u8>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct TargeteNbId {
    pub global_enb_id: GlobalNgEnbId,
    pub selected_eps_tai: EpsTai,
    #[asn(optional_idx = 0)]
    pub ie_extensions: Option<TargeteNbIDiEExtensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "OCTET-STRING")]
pub struct TargettoSourceFailureTransparentContainer(Vec<u8>);

#[derive(Debug, AperCodec)]
#[asn(type = "INTEGER", lb = "0", ub = "127")]
pub struct ThresholdRsrp(u8);

#[derive(Debug, AperCodec)]
#[asn(type = "INTEGER", lb = "0", ub = "127")]
pub struct ThresholdRsrq(u8);

#[derive(Debug, AperCodec)]
#[asn(type = "INTEGER", lb = "0", ub = "127")]
pub struct ThresholdSinr(u8);

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
pub struct TimeUeStayedInCell(u16);

#[derive(Debug, AperCodec)]
#[asn(type = "INTEGER", lb = "0", ub = "40950")]
pub struct TimeUeStayedInCellEnhancedGranularity(u16);

#[derive(Debug, AperCodec)]
#[asn(type = "ENUMERATED", extensible = true, lb = "0", ub = "0")]
pub struct TimerApproachForGuamiRemoval(u8);
impl TimerApproachForGuamiRemoval {
    const APPLY_TIMER: u8 = 0u8;
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 2)]
pub struct TooearlyIntersystemHo {
    pub sourcecell_id: EutraCgi,
    pub failurecell_id: NgranCgi,
    #[asn(optional_idx = 0)]
    pub uerlf_report_container: Option<UerlfReportContainer>,
    #[asn(optional_idx = 1)]
    pub ie_extensions: Option<TooearlyIntersystemHOiEExtensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct TraceActivation {
    pub ngran_trace_id: NgranTraceId,
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
pub struct UeDifferentiationInfo {
    #[asn(optional_idx = 0)]
    pub periodic_communication_indicator: Option<Enumerated83>,
    #[asn(optional_idx = 1)]
    pub periodic_time: Option<Integer84>,
    #[asn(optional_idx = 2)]
    pub scheduled_communication_time: Option<ScheduledCommunicationTime>,
    #[asn(optional_idx = 3)]
    pub stationary_indication: Option<Enumerated85>,
    #[asn(optional_idx = 4)]
    pub traffic_profile: Option<Enumerated86>,
    #[asn(optional_idx = 5)]
    pub battery_indication: Option<Enumerated87>,
    #[asn(optional_idx = 6)]
    pub ie_extensions: Option<UeDifferentiationInfoiEExtensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct UeNgapIdPair {
    pub amf_ue_ngap_id: AmfUeNgapId,
    pub ran_ue_ngap_id: RanUeNgapId,
    #[asn(optional_idx = 0)]
    pub ie_extensions: Option<UeNgapIdPairiEExtensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "CHOICE", lb = "0", ub = "2", extensible = false)]
pub enum UeNgapIDs {
    #[asn(key = 0, extended = false)]
    UENgapIdPair(UeNgapIdPair),
    #[asn(key = 1, extended = false)]
    AMfUeNgapId(AmfUeNgapId),
    #[asn(key = 2, extended = false)]
    ChoiceExtensions(UeNgapIDschoiceExtensions),
}

#[derive(Debug, AperCodec)]
#[asn(type = "ENUMERATED", extensible = true, lb = "0", ub = "0")]
pub struct UeUpCIoTSupport(u8);
impl UeUpCIoTSupport {
    const SUPPORTED: u8 = 0u8;
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 3)]
pub struct UeAssociatedLogicalNgConnectionItem {
    #[asn(optional_idx = 0)]
    pub amf_ue_ngap_id: Option<AmfUeNgapId>,
    #[asn(optional_idx = 1)]
    pub ran_ue_ngap_id: Option<RanUeNgapId>,
    #[asn(optional_idx = 2)]
    pub ie_extensions: Option<UeAssociatedLogicalNgConnectionItemiEExtensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "65536")]
pub struct UeAssociatedLogicalNgConnectionList(Vec<UeAssociatedLogicalNgConnectionItem>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct UeAggregateMaximumBitRate {
    pub ue_aggregate_maximum_bit_rate_dl: BitRate,
    pub ue_aggregate_maximum_bit_rate_ul: BitRate,
    #[asn(optional_idx = 0)]
    pub ie_extensions: Option<UeAggregateMaximumBitRateiEExtensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "ENUMERATED", extensible = true, lb = "0", ub = "0")]
pub struct UeCapabilityInfoRequest(u8);
impl UeCapabilityInfoRequest {
    const REQUESTED: u8 = 0u8;
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true)]
pub struct UeContextModificationFailure {
    pub protocol_i_es: UeContextModificationFailureprotocolIEs,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true)]
pub struct UeContextModificationRequest {
    pub protocol_i_es: UeContextModificationRequestprotocolIEs,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true)]
pub struct UeContextModificationResponse {
    pub protocol_i_es: UeContextModificationResponseprotocolIEs,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true)]
pub struct UeContextReleaseCommand {
    pub protocol_i_es: UeContextReleaseCommandprotocolIEs,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true)]
pub struct UeContextReleaseComplete {
    pub protocol_i_es: UeContextReleaseCompleteprotocolIEs,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true)]
pub struct UeContextReleaseRequest {
    pub protocol_i_es: UeContextReleaseRequestprotocolIEs,
}

#[derive(Debug, AperCodec)]
#[asn(type = "ENUMERATED", extensible = true, lb = "0", ub = "0")]
pub struct UeContextRequest(u8);
impl UeContextRequest {
    const REQUESTED: u8 = 0u8;
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true)]
pub struct UeContextResumeFailure {
    pub protocol_i_es: UeContextResumeFailureprotocolIEs,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true)]
pub struct UeContextResumeRequest {
    pub protocol_i_es: UeContextResumeRequestprotocolIEs,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 2)]
pub struct UeContextResumeRequestTransfer {
    #[asn(optional_idx = 0)]
    pub qos_flow_failed_to_resume_list: Option<QosFlowListWithCause>,
    #[asn(optional_idx = 1)]
    pub ie_extensions: Option<UeContextResumeRequestTransferiEExtensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true)]
pub struct UeContextResumeResponse {
    pub protocol_i_es: UeContextResumeResponseprotocolIEs,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 2)]
pub struct UeContextResumeResponseTransfer {
    #[asn(optional_idx = 0)]
    pub qos_flow_failed_to_resume_list: Option<QosFlowListWithCause>,
    #[asn(optional_idx = 1)]
    pub ie_extensions: Option<UeContextResumeResponseTransferiEExtensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true)]
pub struct UeContextSuspendFailure {
    pub protocol_i_es: UeContextSuspendFailureprotocolIEs,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true)]
pub struct UeContextSuspendRequest {
    pub protocol_i_es: UeContextSuspendRequestprotocolIEs,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 2)]
pub struct UeContextSuspendRequestTransfer {
    #[asn(optional_idx = 0)]
    pub suspend_indicator: Option<SuspendIndicator>,
    #[asn(optional_idx = 1)]
    pub ie_extensions: Option<UeContextSuspendRequestTransferiEExtensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true)]
pub struct UeContextSuspendResponse {
    pub protocol_i_es: UeContextSuspendResponseprotocolIEs,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "16")]
pub struct UeHistoryInformation(Vec<LastVisitedCellItem>);

#[derive(Debug, AperCodec)]
#[asn(type = "CHOICE", lb = "0", ub = "1", extensible = false)]
pub enum UeHistoryInformationFromTheUe {
    #[asn(key = 0, extended = false)]
    NR(NrMobilityHistoryReport),
    #[asn(key = 1, extended = false)]
    ChoiceExtensions(UeHistoryInformationFromTheUEchoiceExtensions),
}

#[derive(Debug, AperCodec)]
#[asn(type = "CHOICE", lb = "0", ub = "1", extensible = false)]
pub enum UeIdentityIndexValue {
    #[asn(key = 0, extended = false)]
    IndexLength10(BitString88),
    #[asn(key = 1, extended = false)]
    ChoiceExtensions(UeIdentityIndexValuechoiceExtensions),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true)]
pub struct UeInformationTransfer {
    pub protocol_i_es: UeInformationTransferprotocolIEs,
}

#[derive(Debug, AperCodec)]
#[asn(type = "CHOICE", lb = "0", ub = "1", extensible = false)]
pub enum UePagingIdentity {
    #[asn(key = 0, extended = false)]
    FiveGSTmsi(FiveGSTmsi),
    #[asn(key = 1, extended = false)]
    ChoiceExtensions(UePagingIdentitychoiceExtensions),
}

#[derive(Debug, AperCodec)]
#[asn(type = "ENUMERATED", extensible = true, lb = "0", ub = "2")]
pub struct UePresence(u8);
impl UePresence {
    const IN: u8 = 0u8;
    const OUT: u8 = 1u8;
    const UNKNOWN: u8 = 2u8;
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct UePresenceInAreaOfInterestItem {
    pub location_reporting_reference_id: LocationReportingReferenceId,
    pub ue_presence: UePresence,
    #[asn(optional_idx = 0)]
    pub ie_extensions: Option<UePresenceInAreaOfInterestItemiEExtensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "64")]
pub struct UePresenceInAreaOfInterestList(Vec<UePresenceInAreaOfInterestItem>);

#[derive(Debug, AperCodec)]
#[asn(type = "CHOICE", lb = "0", ub = "2", extensible = false)]
pub enum UerlfReportContainer {
    #[asn(key = 0, extended = false)]
    NR(NruerlfReportContainer),
    #[asn(key = 1, extended = false)]
    LTe(LteuerlfReportContainer),
    #[asn(key = 2, extended = false)]
    ChoiceExtensions(UerlfReportContainerchoiceExtensions),
}

#[derive(Debug, AperCodec)]
#[asn(type = "OCTET-STRING")]
pub struct UeRadioCapability(Vec<u8>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true)]
pub struct UeRadioCapabilityCheckRequest {
    pub protocol_i_es: UeRadioCapabilityCheckRequestprotocolIEs,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true)]
pub struct UeRadioCapabilityCheckResponse {
    pub protocol_i_es: UeRadioCapabilityCheckResponseprotocolIEs,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 3)]
pub struct UeRadioCapabilityForPaging {
    #[asn(optional_idx = 0)]
    pub ue_radio_capability_for_paging_of_nr: Option<UeRadioCapabilityForPagingOfNr>,
    #[asn(optional_idx = 1)]
    pub ue_radio_capability_for_paging_of_eutra: Option<UeRadioCapabilityForPagingOfEutra>,
    #[asn(optional_idx = 2)]
    pub ie_extensions: Option<UeRadioCapabilityForPagingiEExtensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "OCTET-STRING")]
pub struct UeRadioCapabilityForPagingOfEutra(Vec<u8>);

#[derive(Debug, AperCodec)]
#[asn(type = "OCTET-STRING")]
pub struct UeRadioCapabilityForPagingOfNbIoT(Vec<u8>);

#[derive(Debug, AperCodec)]
#[asn(type = "OCTET-STRING")]
pub struct UeRadioCapabilityForPagingOfNr(Vec<u8>);

#[derive(Debug, AperCodec)]
#[asn(type = "OCTET-STRING")]
pub struct UeRadioCapabilityId(Vec<u8>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true)]
pub struct UeRadioCapabilityIdMappingRequest {
    pub protocol_i_es: UeRadioCapabilityIdMappingRequestprotocolIEs,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true)]
pub struct UeRadioCapabilityIdMappingResponse {
    pub protocol_i_es: UeRadioCapabilityIdMappingResponseprotocolIEs,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true)]
pub struct UeRadioCapabilityInfoIndication {
    pub protocol_i_es: UeRadioCapabilityInfoIndicationprotocolIEs,
}

#[derive(Debug, AperCodec)]
#[asn(type = "ENUMERATED", extensible = true, lb = "0", ub = "0")]
pub struct UeRetentionInformation(u8);
impl UeRetentionInformation {
    const UES_RETAINED: u8 = 0u8;
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct UeSecurityCapabilities {
    pub n_rencryption_algorithms: NRencryptionAlgorithms,
    pub n_rintegrity_protection_algorithms: NRintegrityProtectionAlgorithms,
    pub eutr_aencryption_algorithms: EutrAencryptionAlgorithms,
    pub eutr_aintegrity_protection_algorithms: EutrAintegrityProtectionAlgorithms,
    #[asn(optional_idx = 0)]
    pub ie_extensions: Option<UeSecurityCapabilitiesiEExtensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true)]
pub struct UetnlaBindingReleaseRequest {
    pub protocol_i_es: UetnlaBindingReleaseRequestprotocolIEs,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct UlCpSecurityInformation {
    pub ul_nas_mac: UlNasMac,
    pub ul_nas_count: UlNasCount,
    #[asn(optional_idx = 0)]
    pub ie_extensions: Option<UlCpSecurityInformationiEExtensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "BITSTRING", sz_extensible = false, sz_lb = "5", sz_ub = "5")]
pub struct UlNasCount(BitVec<Msb0, u8>);

#[derive(Debug, AperCodec)]
#[asn(type = "BITSTRING", sz_extensible = false, sz_lb = "16", sz_ub = "16")]
pub struct UlNasMac(BitVec<Msb0, u8>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct UlNguUpTnlModifyItem {
    pub ul_ngu_up_tnl_information: UpTransportLayerInformation,
    pub dl_ngu_up_tnl_information: UpTransportLayerInformation,
    #[asn(optional_idx = 0)]
    pub ie_extensions: Option<UlNguUpTnlModifyItemiEExtensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "4")]
pub struct UlNguUpTnlModifyList(Vec<UlNguUpTnlModifyItem>);

#[derive(Debug, AperCodec)]
#[asn(type = "ENUMERATED", extensible = true, lb = "0", ub = "0")]
pub struct UlForwarding(u8);
impl UlForwarding {
    const UL_FORWARDING_PROPOSED: u8 = 0u8;
}

#[derive(Debug, AperCodec)]
#[asn(type = "CHOICE", lb = "0", ub = "1", extensible = false)]
pub enum UpTransportLayerInformation {
    #[asn(key = 0, extended = false)]
    GTpTunnel(GtpTunnel),
    #[asn(key = 1, extended = false)]
    ChoiceExtensions(UpTransportLayerInformationchoiceExtensions),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct UpTransportLayerInformationItem {
    pub ngu_up_tnl_information: UpTransportLayerInformation,
    #[asn(optional_idx = 0)]
    pub ie_extensions: Option<UpTransportLayerInformationItemiEExtensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "3")]
pub struct UpTransportLayerInformationList(Vec<UpTransportLayerInformationItem>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct UpTransportLayerInformationPairItem {
    pub ul_ngu_up_tnl_information: UpTransportLayerInformation,
    pub dl_ngu_up_tnl_information: UpTransportLayerInformation,
    #[asn(optional_idx = 0)]
    pub ie_extensions: Option<UpTransportLayerInformationPairItemiEExtensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "3")]
pub struct UpTransportLayerInformationPairList(Vec<UpTransportLayerInformationPairItem>);

#[derive(Debug, AperCodec)]
#[asn(type = "VisibleString")]
pub struct UriAddress(String);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 3)]
pub struct UnavailableGuamiItem {
    pub guami: Guami,
    #[asn(optional_idx = 0)]
    pub timer_approach_for_guami_removal: Option<TimerApproachForGuamiRemoval>,
    #[asn(optional_idx = 1)]
    pub backup_amf_name: Option<AmfName>,
    #[asn(optional_idx = 2)]
    pub ie_extensions: Option<UnavailableGuamiItemiEExtensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "256")]
pub struct UnavailableGuamiList(Vec<UnavailableGuamiItem>);

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
pub struct UplinkNasTransport {
    pub protocol_i_es: UplinkNasTransportprotocolIEs,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true)]
pub struct UplinkNonUeAssociatedNrpPaTransport {
    pub protocol_i_es: UplinkNonUeAssociatedNrpPaTransportprotocolIEs,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true)]
pub struct UplinkRanConfigurationTransfer {
    pub protocol_i_es: UplinkRanConfigurationTransferprotocolIEs,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true)]
pub struct UplinkRanEarlyStatusTransfer {
    pub protocol_i_es: UplinkRanEarlyStatusTransferprotocolIEs,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true)]
pub struct UplinkRanStatusTransfer {
    pub protocol_i_es: UplinkRanStatusTransferprotocolIEs,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true)]
pub struct UplinkRimInformationTransfer {
    pub protocol_i_es: UplinkRimInformationTransferprotocolIEs,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true)]
pub struct UplinkUeAssociatedNrpPaTransport {
    pub protocol_i_es: UplinkUeAssociatedNrpPaTransportprotocolIEs,
}

#[derive(Debug, AperCodec)]
#[asn(type = "CHOICE", lb = "0", ub = "3", extensible = false)]
pub enum UserLocationInformation {
    #[asn(key = 0, extended = false)]
    UserLocationInformationEutra(UserLocationInformationEutra),
    #[asn(key = 1, extended = false)]
    UserLocationInformationNr(UserLocationInformationNr),
    #[asn(key = 2, extended = false)]
    UserLocationInformationN3iwf(UserLocationInformationN3iwf),
    #[asn(key = 3, extended = false)]
    ChoiceExtensions(UserLocationInformationchoiceExtensions),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 2)]
pub struct UserLocationInformationEutra {
    pub eutra_cgi: EutraCgi,
    pub tai: Tai,
    #[asn(optional_idx = 0)]
    pub time_stamp: Option<TimeStamp>,
    #[asn(optional_idx = 1)]
    pub ie_extensions: Option<UserLocationInformationEutrAiEExtensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct UserLocationInformationN3iwf {
    pub ip_address: TransportLayerAddress,
    pub port_number: PortNumber,
    #[asn(optional_idx = 0)]
    pub ie_extensions: Option<UserLocationInformationN3iwFiEExtensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 2)]
pub struct UserLocationInformationNr {
    pub nr_cgi: NrCgi,
    pub tai: Tai,
    #[asn(optional_idx = 0)]
    pub time_stamp: Option<TimeStamp>,
    #[asn(optional_idx = 1)]
    pub ie_extensions: Option<UserLocationInformationNRiEExtensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 2)]
pub struct UserLocationInformationTngf {
    pub tnap_id: TnapId,
    pub ip_address: TransportLayerAddress,
    #[asn(optional_idx = 0)]
    pub port_number: Option<PortNumber>,
    #[asn(optional_idx = 1)]
    pub ie_extensions: Option<UserLocationInformationTngFiEExtensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 2)]
pub struct UserLocationInformationTwif {
    pub twap_id: TwapId,
    pub ip_address: TransportLayerAddress,
    #[asn(optional_idx = 0)]
    pub port_number: Option<PortNumber>,
    #[asn(optional_idx = 1)]
    pub ie_extensions: Option<UserLocationInformationTwiFiEExtensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "CHOICE", lb = "0", ub = "2", extensible = false)]
pub enum UserLocationInformationWAgf {
    #[asn(key = 0, extended = false)]
    GlobalLineId(GlobalLineId),
    #[asn(key = 1, extended = false)]
    HFcNodeId(HfcNodeId),
    #[asn(key = 2, extended = false)]
    ChoiceExtensions(UserLocationInformationWAgFchoiceExtensions),
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
pub struct VehicleUe(u8);
impl VehicleUe {
    const AUTHORIZED: u8 = 0u8;
    const NOT_AUTHORIZED: u8 = 1u8;
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct VolumeTimedReportItem {
    pub start_time_stamp: OctetString89,
    pub end_time_stamp: OctetString90,
    pub usage_count_ul: Integer91,
    pub usage_count_dl: Integer92,
    #[asn(optional_idx = 0)]
    pub ie_extensions: Option<VolumeTimedReportItemiEExtensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "2")]
pub struct VolumeTimedReportList(Vec<VolumeTimedReportItem>);

#[derive(Debug, AperCodec)]
#[asn(type = "CHOICE", lb = "0", ub = "1", extensible = false)]
pub enum WAgfId {
    #[asn(key = 0, extended = false)]
    WAgfId(BitString93),
    #[asn(key = 1, extended = false)]
    ChoiceExtensions(WAgfIDchoiceExtensions),
}

#[derive(Debug, AperCodec)]
#[asn(type = "ENUMERATED", extensible = true, lb = "0", ub = "0")]
pub struct WlanMeasConfig(u8);
impl WlanMeasConfig {
    const SETUP: u8 = 0u8;
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct WlanMeasConfigNameItem {
    pub wlan_name: WlanName,
    #[asn(optional_idx = 0)]
    pub ie_extensions: Option<WlanMeasConfigNameItemiEExtensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "4")]
pub struct WlanMeasConfigNameList(Vec<WlanMeasConfigNameItem>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 4)]
pub struct WlanMeasurementConfiguration {
    pub wlan_meas_config: WlanMeasConfig,
    #[asn(optional_idx = 0)]
    pub wlan_meas_config_name_list: Option<WlanMeasConfigNameList>,
    #[asn(optional_idx = 1)]
    pub wlan_rssi: Option<Enumerated94>,
    #[asn(optional_idx = 2)]
    pub wlan_rtt: Option<Enumerated95>,
    #[asn(optional_idx = 3)]
    pub ie_extensions: Option<WlanMeasurementConfigurationiEExtensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "OCTET-STRING", sz_extensible = false, sz_lb = "1", sz_ub = "32")]
pub struct WlanName(Vec<u8>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct WusAssistanceInformation {
    pub paging_probability_information: PagingProbabilityInformation,
    #[asn(optional_idx = 0)]
    pub ie_extensions: Option<WusAssistanceInformationiEExtensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "OCTET-STRING", sz_extensible = false, sz_lb = "1", sz_ub = "1024")]
pub struct WarningAreaCoordinates(Vec<u8>);

#[derive(Debug, AperCodec)]
#[asn(type = "CHOICE", lb = "0", ub = "4", extensible = false)]
pub enum WarningAreaList {
    #[asn(key = 0, extended = false)]
    EUtraCgiListForWarning(EutraCgiListForWarning),
    #[asn(key = 1, extended = false)]
    NRCgiListForWarning(NrCgiListForWarning),
    #[asn(key = 2, extended = false)]
    TAiListForWarning(TaiListForWarning),
    #[asn(key = 3, extended = false)]
    EmergencyAreaIdList(EmergencyAreaIdList),
    #[asn(key = 4, extended = false)]
    ChoiceExtensions(WarningAreaListchoiceExtensions),
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
pub struct XnExtTlaItem {
    #[asn(optional_idx = 0)]
    pub i_psec_tla: Option<TransportLayerAddress>,
    #[asn(optional_idx = 1)]
    pub gtp_tl_as: Option<XnGtpTlAs>,
    #[asn(optional_idx = 2)]
    pub ie_extensions: Option<XnExtTlaItemiEExtensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "16")]
pub struct XnExtTlAs(Vec<XnExtTlaItem>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "16")]
pub struct XnGtpTlAs(Vec<TransportLayerAddress>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "2")]
pub struct XnTlAs(Vec<TransportLayerAddress>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 2)]
pub struct XnTnlConfigurationInfo {
    pub xn_transport_layer_addresses: XnTlAs,
    #[asn(optional_idx = 0)]
    pub xn_extended_transport_layer_addresses: Option<XnExtTlAs>,
    #[asn(optional_idx = 1)]
    pub ie_extensions: Option<XnTnlConfigurationInfoiEExtensions>,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct AmfTnlAssociationSetupItemiEExtensionsItem {}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "65535")]
pub struct AmfTnlAssociationSetupItemiEExtensions(Vec<AmfTnlAssociationSetupItemiEExtensionsItem>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct AmfTnlAssociationToAddItemiEExtensionsItem {}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "65535")]
pub struct AmfTnlAssociationToAddItemiEExtensions(Vec<AmfTnlAssociationToAddItemiEExtensionsItem>);

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum AmfTnlAssociationToRemoveItemiEExtensionsItemextensionValue {
    #[asn(key = 168)]
    IdTnlAssociationTransportLayerAddressNgran(CpTransportLayerInformation),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct AmfTnlAssociationToRemoveItemiEExtensionsItem {
    #[asn(key_field = true)]
    pub id: ProtocolExtensionId,
    pub criticality: Criticality,
    pub extension_value: AmfTnlAssociationToRemoveItemiEExtensionsItemextensionValue,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "65535")]
pub struct AmfTnlAssociationToRemoveItemiEExtensions(Vec<AmfTnlAssociationToRemoveItemiEExtensionsItem>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct AmfTnlAssociationToUpdateItemiEExtensionsItem {}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "65535")]
pub struct AmfTnlAssociationToUpdateItemiEExtensions(Vec<AmfTnlAssociationToUpdateItemiEExtensionsItem>);

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum AmfcpRelocationIndicationprotocolIEsItemvalue {
    #[asn(key = 10)]
    IdAmfUeNgapId(AmfUeNgapId),
    #[asn(key = 0)]
    IdAllowedNssai(AllowedNssai),
    #[asn(key = 85)]
    IdRanUeNgapId(RanUeNgapId),
    #[asn(key = 148)]
    IdSNssai(SNssai),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct AmfcpRelocationIndicationprotocolIEsItem {
    #[asn(key_field = true)]
    pub id: ProtocolIeId,
    pub criticality: Criticality,
    pub value: AmfcpRelocationIndicationprotocolIEsItemvalue,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "0", sz_ub = "65535")]
pub struct AmfcpRelocationIndicationprotocolIEs(Vec<AmfcpRelocationIndicationprotocolIEsItem>);

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum AmfConfigurationUpdateprotocolIEsItemvalue {
    #[asn(key = 6)]
    IdAmfTnlAssociationToAddList(AmfTnlAssociationToAddList),
    #[asn(key = 7)]
    IdAmfTnlAssociationToRemoveList(AmfTnlAssociationToRemoveList),
    #[asn(key = 8)]
    IdAmfTnlAssociationToUpdateList(AmfTnlAssociationToUpdateList),
    #[asn(key = 1)]
    IdAmfName(AmfName),
    #[asn(key = 274)]
    IdExtendedAmfName(ExtendedAmfName),
    #[asn(key = 80)]
    IdPlmnSupportList(PlmnSupportList),
    #[asn(key = 86)]
    IdRelativeAmfCapacity(RelativeAmfCapacity),
    #[asn(key = 96)]
    IdServedGuamiList(ServedGuamiList),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct AmfConfigurationUpdateprotocolIEsItem {
    #[asn(key_field = true)]
    pub id: ProtocolIeId,
    pub criticality: Criticality,
    pub value: AmfConfigurationUpdateprotocolIEsItemvalue,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "0", sz_ub = "65535")]
pub struct AmfConfigurationUpdateprotocolIEs(Vec<AmfConfigurationUpdateprotocolIEsItem>);

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum AmfConfigurationUpdateAcknowledgeprotocolIEsItemvalue {
    #[asn(key = 4)]
    IdAmfTnlAssociationFailedToSetupList(TnlAssociationList),
    #[asn(key = 5)]
    IdAmfTnlAssociationSetupList(AmfTnlAssociationSetupList),
    #[asn(key = 19)]
    IdCriticalityDiagnostics(CriticalityDiagnostics),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct AmfConfigurationUpdateAcknowledgeprotocolIEsItem {
    #[asn(key_field = true)]
    pub id: ProtocolIeId,
    pub criticality: Criticality,
    pub value: AmfConfigurationUpdateAcknowledgeprotocolIEsItemvalue,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "0", sz_ub = "65535")]
pub struct AmfConfigurationUpdateAcknowledgeprotocolIEs(Vec<AmfConfigurationUpdateAcknowledgeprotocolIEsItem>);

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum AmfConfigurationUpdateFailureprotocolIEsItemvalue {
    #[asn(key = 15)]
    IdCause(Cause),
    #[asn(key = 19)]
    IdCriticalityDiagnostics(CriticalityDiagnostics),
    #[asn(key = 107)]
    IdTimeToWait(TimeToWait),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct AmfConfigurationUpdateFailureprotocolIEsItem {
    #[asn(key_field = true)]
    pub id: ProtocolIeId,
    pub criticality: Criticality,
    pub value: AmfConfigurationUpdateFailureprotocolIEsItemvalue,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "0", sz_ub = "65535")]
pub struct AmfConfigurationUpdateFailureprotocolIEs(Vec<AmfConfigurationUpdateFailureprotocolIEsItem>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct AmfPagingTargetchoiceExtensions {}

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum AmfStatusIndicationprotocolIEsItemvalue {
    #[asn(key = 120)]
    IdUnavailableGuamiList(UnavailableGuamiList),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct AmfStatusIndicationprotocolIEsItem {
    #[asn(key_field = true)]
    pub id: ProtocolIeId,
    pub criticality: Criticality,
    pub value: AmfStatusIndicationprotocolIEsItemvalue,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "0", sz_ub = "65535")]
pub struct AmfStatusIndicationprotocolIEs(Vec<AmfStatusIndicationprotocolIEsItem>);

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum AdditionalDluptnlInformationForHoItemiEExtensionsItemextensionValue {
    #[asn(key = 183)]
    IdAdditionalRedundantDlNguUpTnlInformation(UpTransportLayerInformation),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct AdditionalDluptnlInformationForHoItemiEExtensionsItem {
    #[asn(key_field = true)]
    pub id: ProtocolExtensionId,
    pub criticality: Criticality,
    pub extension_value: AdditionalDluptnlInformationForHoItemiEExtensionsItemextensionValue,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "65535")]
pub struct AdditionalDluptnlInformationForHoItemiEExtensions(Vec<AdditionalDluptnlInformationForHoItemiEExtensionsItem>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct AllocationAndRetentionPriorityiEExtensionsItem {}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "65535")]
pub struct AllocationAndRetentionPriorityiEExtensions(Vec<AllocationAndRetentionPriorityiEExtensionsItem>);

#[derive(Debug, AperCodec)]
#[asn(type = "ENUMERATED", extensible = true, lb = "0", ub = "1")]
pub struct Enumerated2(u8);
impl Enumerated2 {
    const RESTRICTED: u8 = 0u8;
    const NOT_RESTRICTED: u8 = 1u8;
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct AllowedPniNpnItemiEExtensionsItem {}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "65535")]
pub struct AllowedPniNpnItemiEExtensions(Vec<AllowedPniNpnItemiEExtensionsItem>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct AllowedNssaiItemiEExtensionsItem {}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "65535")]
pub struct AllowedNssaiItemiEExtensions(Vec<AllowedNssaiItemiEExtensionsItem>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct AlternativeQoSParaSetItemiEExtensionsItem {}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "65535")]
pub struct AlternativeQoSParaSetItemiEExtensions(Vec<AlternativeQoSParaSetItemiEExtensionsItem>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct AreaOfInterestiEExtensionsItem {}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "65535")]
pub struct AreaOfInterestiEExtensions(Vec<AreaOfInterestiEExtensionsItem>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct AreaOfInterestCellItemiEExtensionsItem {}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "65535")]
pub struct AreaOfInterestCellItemiEExtensions(Vec<AreaOfInterestCellItemiEExtensionsItem>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct AreaOfInterestItemiEExtensionsItem {}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "65535")]
pub struct AreaOfInterestItemiEExtensions(Vec<AreaOfInterestItemiEExtensionsItem>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct AreaOfInterestRanNodeItemiEExtensionsItem {}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "65535")]
pub struct AreaOfInterestRanNodeItemiEExtensions(Vec<AreaOfInterestRanNodeItemiEExtensionsItem>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct AreaOfInterestTaiItemiEExtensionsItem {}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "65535")]
pub struct AreaOfInterestTaiItemiEExtensions(Vec<AreaOfInterestTaiItemiEExtensionsItem>);

#[derive(Debug, AperCodec)]
#[asn(type = "NULL")]
pub struct Null3;

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct AreaScopeOfMdtEutrAchoiceExtensions {}

#[derive(Debug, AperCodec)]
#[asn(type = "NULL")]
pub struct Null4;

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct AreaScopeOfMdtNRchoiceExtensions {}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct AreaScopeOfNeighCellsItemiEExtensionsItem {}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "65535")]
pub struct AreaScopeOfNeighCellsItemiEExtensions(Vec<AreaScopeOfNeighCellsItemiEExtensionsItem>);

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum AssistanceDataForPagingiEExtensionsItemextensionValue {
    #[asn(key = 260)]
    IdNpnPagingAssistanceInformation(NpnPagingAssistanceInformation),
    #[asn(key = 207)]
    IdPagingAssisDataforCEcapabUe(PagingAssisDataforCEcapabUe),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct AssistanceDataForPagingiEExtensionsItem {
    #[asn(key_field = true)]
    pub id: ProtocolExtensionId,
    pub criticality: Criticality,
    pub extension_value: AssistanceDataForPagingiEExtensionsItemextensionValue,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "65535")]
pub struct AssistanceDataForPagingiEExtensions(Vec<AssistanceDataForPagingiEExtensionsItem>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct AssistanceDataForRecommendedCellsiEExtensionsItem {}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "65535")]
pub struct AssistanceDataForRecommendedCellsiEExtensions(Vec<AssistanceDataForRecommendedCellsiEExtensionsItem>);

#[derive(Debug, AperCodec)]
#[asn(type = "ENUMERATED", extensible = true, lb = "0", ub = "1")]
pub struct Enumerated5(u8);
impl Enumerated5 {
    const UL: u8 = 0u8;
    const DL: u8 = 1u8;
}

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum AssociatedQosFlowItemiEExtensionsItemextensionValue {
    #[asn(key = 221)]
    IdCurrentQoSParaSetIndex(AlternativeQoSParaSetIndex),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct AssociatedQosFlowItemiEExtensionsItem {
    #[asn(key_field = true)]
    pub id: ProtocolExtensionId,
    pub criticality: Criticality,
    pub extension_value: AssociatedQosFlowItemiEExtensionsItemextensionValue,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "65535")]
pub struct AssociatedQosFlowItemiEExtensions(Vec<AssociatedQosFlowItemiEExtensionsItem>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct BluetoothMeasConfigNameItemiEExtensionsItem {}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "65535")]
pub struct BluetoothMeasConfigNameItemiEExtensions(Vec<BluetoothMeasConfigNameItemiEExtensionsItem>);

#[derive(Debug, AperCodec)]
#[asn(type = "ENUMERATED", extensible = true, lb = "0", ub = "0")]
pub struct Enumerated6(u8);
impl Enumerated6 {
    const TRUE: u8 = 0u8;
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct BluetoothMeasurementConfigurationiEExtensionsItem {}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "65535")]
pub struct BluetoothMeasurementConfigurationiEExtensions(Vec<BluetoothMeasurementConfigurationiEExtensionsItem>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct BroadcastCancelledAreaListchoiceExtensions {}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct BroadcastCompletedAreaListchoiceExtensions {}

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum BroadcastPlmnItemiEExtensionsItemextensionValue {
    #[asn(key = 271)]
    IdExtendedTaiSliceSupportList(ExtendedSliceSupportList),
    #[asn(key = 258)]
    IdNpnSupport(NpnSupport),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct BroadcastPlmnItemiEExtensionsItem {
    #[asn(key_field = true)]
    pub id: ProtocolExtensionId,
    pub criticality: Criticality,
    pub extension_value: BroadcastPlmnItemiEExtensionsItemextensionValue,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "65535")]
pub struct BroadcastPlmnItemiEExtensions(Vec<BroadcastPlmnItemiEExtensionsItem>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct CnAssistedRanTuningiEExtensionsItem {}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "65535")]
pub struct CnAssistedRanTuningiEExtensions(Vec<CnAssistedRanTuningiEExtensionsItem>);

#[derive(Debug, AperCodec)]
#[asn(type = "ENUMERATED", extensible = true, lb = "0", ub = "1")]
pub struct Enumerated7(u8);
impl Enumerated7 {
    const EPC_FORBIDDEN: u8 = 0u8;
    const FIVE_GC_FORBIDDEN: u8 = 1u8;
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct CnTypeRestrictionsForEquivalentItemiEExtensionsItem {}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "65535")]
pub struct CnTypeRestrictionsForEquivalentItemiEExtensions(Vec<CnTypeRestrictionsForEquivalentItemiEExtensionsItem>);

#[derive(Debug, AperCodec)]
#[asn(type = "INTEGER", lb = "0", ub = "4095")]
pub struct Integer8(u16);

#[derive(Debug, AperCodec)]
#[asn(type = "INTEGER", lb = "0", ub = "1048575")]
pub struct Integer9(u32);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct CountValueForPdcpSn12iEExtensionsItem {}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "65535")]
pub struct CountValueForPdcpSn12iEExtensions(Vec<CountValueForPdcpSn12iEExtensionsItem>);

#[derive(Debug, AperCodec)]
#[asn(type = "INTEGER", lb = "0", ub = "262143")]
pub struct Integer10(u32);

#[derive(Debug, AperCodec)]
#[asn(type = "INTEGER", lb = "0", ub = "16383")]
pub struct Integer11(u16);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct CountValueForPdcpSn18iEExtensionsItem {}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "65535")]
pub struct CountValueForPdcpSn18iEExtensions(Vec<CountValueForPdcpSn18iEExtensionsItem>);

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum CpTransportLayerInformationchoiceExtensionsvalue {
    #[asn(key = 169)]
    IdEndpointIpAddressAndPort(EndpointIpAddressAndPort),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct CpTransportLayerInformationchoiceExtensions {
    #[asn(key_field = true)]
    pub id: ProtocolIeId,
    pub criticality: Criticality,
    pub value: CpTransportLayerInformationchoiceExtensionsvalue,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct CancelledCellsInEaiEutraItemiEExtensionsItem {}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "65535")]
pub struct CancelledCellsInEaiEutraItemiEExtensions(Vec<CancelledCellsInEaiEutraItemiEExtensionsItem>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct CancelledCellsInEaiNrItemiEExtensionsItem {}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "65535")]
pub struct CancelledCellsInEaiNrItemiEExtensions(Vec<CancelledCellsInEaiNrItemiEExtensionsItem>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct CancelledCellsInTaiEutraItemiEExtensionsItem {}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "65535")]
pub struct CancelledCellsInTaiEutraItemiEExtensions(Vec<CancelledCellsInTaiEutraItemiEExtensionsItem>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct CancelledCellsInTaiNrItemiEExtensionsItem {}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "65535")]
pub struct CancelledCellsInTaiNrItemiEExtensions(Vec<CancelledCellsInTaiNrItemiEExtensionsItem>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct CandidateCellchoiceExtensions {}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct CandidateCellIDiEExtensionsItem {}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "65535")]
pub struct CandidateCellIDiEExtensions(Vec<CandidateCellIDiEExtensionsItem>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct CandidateCellItemiEExtensionsItem {}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "65535")]
pub struct CandidateCellItemiEExtensions(Vec<CandidateCellItemiEExtensionsItem>);

#[derive(Debug, AperCodec)]
#[asn(type = "INTEGER", lb = "0", ub = "1007", extensible = true)]
pub struct Integer12(u16);

#[derive(Debug, AperCodec)]
#[asn(type = "INTEGER", lb = "0", ub = "3279165")]
pub struct Integer13(u32);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct CandidatePcIiEExtensionsItem {}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "65535")]
pub struct CandidatePcIiEExtensions(Vec<CandidatePcIiEExtensionsItem>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct CausechoiceExtensions {}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct CellCagInformationiEExtensionsItem {}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "65535")]
pub struct CellCagInformationiEExtensions(Vec<CellCagInformationiEExtensionsItem>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct CellBasedMdtEutrAiEExtensionsItem {}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "65535")]
pub struct CellBasedMdtEutrAiEExtensions(Vec<CellBasedMdtEutrAiEExtensionsItem>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct CellBasedMdtNRiEExtensionsItem {}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "65535")]
pub struct CellBasedMdtNRiEExtensions(Vec<CellBasedMdtNRiEExtensionsItem>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct CellIdBroadcastEutraItemiEExtensionsItem {}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "65535")]
pub struct CellIdBroadcastEutraItemiEExtensions(Vec<CellIdBroadcastEutraItemiEExtensionsItem>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct CellIdBroadcastNrItemiEExtensionsItem {}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "65535")]
pub struct CellIdBroadcastNrItemiEExtensions(Vec<CellIdBroadcastNrItemiEExtensionsItem>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct CellIdCancelledEutraItemiEExtensionsItem {}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "65535")]
pub struct CellIdCancelledEutraItemiEExtensions(Vec<CellIdCancelledEutraItemiEExtensionsItem>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct CellIdCancelledNrItemiEExtensionsItem {}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "65535")]
pub struct CellIdCancelledNrItemiEExtensions(Vec<CellIdCancelledNrItemiEExtensionsItem>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct CellIdListForRestartchoiceExtensions {}

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum CellTrafficTraceprotocolIEsItemvalue {
    #[asn(key = 10)]
    IdAmfUeNgapId(AmfUeNgapId),
    #[asn(key = 43)]
    IdNgranCgi(NgranCgi),
    #[asn(key = 44)]
    IdNgranTraceId(NgranTraceId),
    #[asn(key = 256)]
    IdPrivacyIndicator(PrivacyIndicator),
    #[asn(key = 85)]
    IdRanUeNgapId(RanUeNgapId),
    #[asn(key = 109)]
    IdTraceCollectionEntityIpAddress(TransportLayerAddress),
    #[asn(key = 257)]
    IdTraceCollectionEntityUri(UriAddress),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct CellTrafficTraceprotocolIEsItem {
    #[asn(key_field = true)]
    pub id: ProtocolIeId,
    pub criticality: Criticality,
    pub value: CellTrafficTraceprotocolIEsItemvalue,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "0", sz_ub = "65535")]
pub struct CellTrafficTraceprotocolIEs(Vec<CellTrafficTraceprotocolIEsItem>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct CellTypeiEExtensionsItem {}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "65535")]
pub struct CellTypeiEExtensions(Vec<CellTypeiEExtensionsItem>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct CompletedCellsInEaiEutraItemiEExtensionsItem {}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "65535")]
pub struct CompletedCellsInEaiEutraItemiEExtensions(Vec<CompletedCellsInEaiEutraItemiEExtensionsItem>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct CompletedCellsInEaiNrItemiEExtensionsItem {}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "65535")]
pub struct CompletedCellsInEaiNrItemiEExtensions(Vec<CompletedCellsInEaiNrItemiEExtensionsItem>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct CompletedCellsInTaiEutraItemiEExtensionsItem {}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "65535")]
pub struct CompletedCellsInTaiEutraItemiEExtensions(Vec<CompletedCellsInTaiEutraItemiEExtensionsItem>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct CompletedCellsInTaiNrItemiEExtensionsItem {}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "65535")]
pub struct CompletedCellsInTaiNrItemiEExtensions(Vec<CompletedCellsInTaiNrItemiEExtensionsItem>);

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum ConnectionEstablishmentIndicationprotocolIEsItemvalue {
    #[asn(key = 10)]
    IdAmfUeNgapId(AmfUeNgapId),
    #[asn(key = 0)]
    IdAllowedNssai(AllowedNssai),
    #[asn(key = 222)]
    IdCEmodeBrestricted(CEmodeBrestricted),
    #[asn(key = 212)]
    IdDlCpSecurityInformation(DlCpSecurityInformation),
    #[asn(key = 226)]
    IdEndIndication(EndIndication),
    #[asn(key = 205)]
    IdEnhancedCoverageRestriction(EnhancedCoverageRestriction),
    #[asn(key = 210)]
    IdNbIoTUePriority(NbIoTUePriority),
    #[asn(key = 85)]
    IdRanUeNgapId(RanUeNgapId),
    #[asn(key = 148)]
    IdSNssai(SNssai),
    #[asn(key = 209)]
    IdUeDifferentiationInfo(UeDifferentiationInfo),
    #[asn(key = 117)]
    IdUeRadioCapability(UeRadioCapability),
    #[asn(key = 264)]
    IdUeRadioCapabilityId(UeRadioCapabilityId),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct ConnectionEstablishmentIndicationprotocolIEsItem {
    #[asn(key_field = true)]
    pub id: ProtocolIeId,
    pub criticality: Criticality,
    pub value: ConnectionEstablishmentIndicationprotocolIEsItemvalue,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "0", sz_ub = "65535")]
pub struct ConnectionEstablishmentIndicationprotocolIEs(Vec<ConnectionEstablishmentIndicationprotocolIEsItem>);

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum CoreNetworkAssistanceInformationForInactiveiEExtensionsItemextensionValue {
    #[asn(key = 280)]
    IdExtendedUeIdentityIndexValue(ExtendedUeIdentityIndexValue),
    #[asn(key = 223)]
    IdPagingeDrxInformation(PagingeDrxInformation),
    #[asn(key = 118)]
    IdUeRadioCapabilityForPaging(UeRadioCapabilityForPaging),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct CoreNetworkAssistanceInformationForInactiveiEExtensionsItem {
    #[asn(key_field = true)]
    pub id: ProtocolExtensionId,
    pub criticality: Criticality,
    pub extension_value: CoreNetworkAssistanceInformationForInactiveiEExtensionsItemextensionValue,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "65535")]
pub struct CoreNetworkAssistanceInformationForInactiveiEExtensions(Vec<CoreNetworkAssistanceInformationForInactiveiEExtensionsItem>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct CriticalityDiagnosticsiEExtensionsItem {}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "65535")]
pub struct CriticalityDiagnosticsiEExtensions(Vec<CriticalityDiagnosticsiEExtensionsItem>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct CriticalityDiagnosticsIeItemiEExtensionsItem {}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "65535")]
pub struct CriticalityDiagnosticsIeItemiEExtensions(Vec<CriticalityDiagnosticsIeItemiEExtensionsItem>);

#[derive(Debug, AperCodec)]
#[asn(type = "ENUMERATED", extensible = true, lb = "0", ub = "0")]
pub struct Enumerated14(u8);
impl Enumerated14 {
    const DAPS_HO_REQUIRED: u8 = 0u8;
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct DapsRequestInfoiEExtensionsItem {}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "65535")]
pub struct DapsRequestInfoiEExtensions(Vec<DapsRequestInfoiEExtensionsItem>);

#[derive(Debug, AperCodec)]
#[asn(type = "ENUMERATED", extensible = true, lb = "0", ub = "1")]
pub struct Enumerated15(u8);
impl Enumerated15 {
    const DAPS_HO_ACCEPTED: u8 = 0u8;
    const DAPS_HO_NOT_ACCEPTED: u8 = 1u8;
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct DapsResponseInfoiEExtensionsItem {}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "65535")]
pub struct DapsResponseInfoiEExtensions(Vec<DapsResponseInfoiEExtensionsItem>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct DapsResponseInfoItemiEExtensionItem {}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "65535")]
pub struct DapsResponseInfoItemiEExtension(Vec<DapsResponseInfoItemiEExtensionItem>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct DlCpSecurityInformationiEExtensionsItem {}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "65535")]
pub struct DlCpSecurityInformationiEExtensions(Vec<DlCpSecurityInformationiEExtensionsItem>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct DrbStatusDLchoiceExtensions {}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct DrbStatusDl12iEExtensionItem {}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "65535")]
pub struct DrbStatusDl12iEExtension(Vec<DrbStatusDl12iEExtensionItem>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct DrbStatusDl18iEExtensionItem {}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "65535")]
pub struct DrbStatusDl18iEExtension(Vec<DrbStatusDl18iEExtensionItem>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct DrbStatusULchoiceExtensions {}

#[derive(Debug, AperCodec)]
#[asn(type = "BITSTRING", sz_extensible = false, sz_lb = "1", sz_ub = "2048")]
pub struct BitString16(BitVec<Msb0, u8>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct DrbStatusUl12iEExtensionItem {}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "65535")]
pub struct DrbStatusUl12iEExtension(Vec<DrbStatusUl12iEExtensionItem>);

#[derive(Debug, AperCodec)]
#[asn(type = "BITSTRING", sz_extensible = false, sz_lb = "1", sz_ub = "131072")]
pub struct BitString17(BitVec<Msb0, u8>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct DrbStatusUl18iEExtensionItem {}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "65535")]
pub struct DrbStatusUl18iEExtension(Vec<DrbStatusUl18iEExtensionItem>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct DrBsSubjectToEarlyStatusTransferItemiEExtensionItem {}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "65535")]
pub struct DrBsSubjectToEarlyStatusTransferItemiEExtension(Vec<DrBsSubjectToEarlyStatusTransferItemiEExtensionItem>);

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum DrBsSubjectToStatusTransferItemiEExtensionItemextensionValue {
    #[asn(key = 159)]
    IdOldAssociatedQosFlowListULendmarkerexpected(AssociatedQosFlowList),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct DrBsSubjectToStatusTransferItemiEExtensionItem {
    #[asn(key_field = true)]
    pub id: ProtocolExtensionId,
    pub criticality: Criticality,
    pub extension_value: DrBsSubjectToStatusTransferItemiEExtensionItemextensionValue,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "65535")]
pub struct DrBsSubjectToStatusTransferItemiEExtension(Vec<DrBsSubjectToStatusTransferItemiEExtensionItem>);

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum DrBsToQosFlowsMappingItemiEExtensionsItemextensionValue {
    #[asn(key = 266)]
    IdDapsRequestInfo(DapsRequestInfo),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct DrBsToQosFlowsMappingItemiEExtensionsItem {
    #[asn(key_field = true)]
    pub id: ProtocolExtensionId,
    pub criticality: Criticality,
    pub extension_value: DrBsToQosFlowsMappingItemiEExtensionsItemextensionValue,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "65535")]
pub struct DrBsToQosFlowsMappingItemiEExtensions(Vec<DrBsToQosFlowsMappingItemiEExtensionsItem>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct DataForwardingResponseDrbItemiEExtensionsItem {}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "65535")]
pub struct DataForwardingResponseDrbItemiEExtensions(Vec<DataForwardingResponseDrbItemiEExtensionsItem>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct DataForwardingResponseErabListItemiEExtensionsItem {}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "65535")]
pub struct DataForwardingResponseErabListItemiEExtensions(Vec<DataForwardingResponseErabListItemiEExtensionsItem>);

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum DeactivateTraceprotocolIEsItemvalue {
    #[asn(key = 10)]
    IdAmfUeNgapId(AmfUeNgapId),
    #[asn(key = 44)]
    IdNgranTraceId(NgranTraceId),
    #[asn(key = 85)]
    IdRanUeNgapId(RanUeNgapId),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct DeactivateTraceprotocolIEsItem {
    #[asn(key_field = true)]
    pub id: ProtocolIeId,
    pub criticality: Criticality,
    pub value: DeactivateTraceprotocolIEsItemvalue,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "0", sz_ub = "65535")]
pub struct DeactivateTraceprotocolIEs(Vec<DeactivateTraceprotocolIEsItem>);

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum DownlinkNasTransportprotocolIEsItemvalue {
    #[asn(key = 10)]
    IdAmfUeNgapId(AmfUeNgapId),
    #[asn(key = 0)]
    IdAllowedNssai(AllowedNssai),
    #[asn(key = 222)]
    IdCEmodeBrestricted(CEmodeBrestricted),
    #[asn(key = 226)]
    IdEndIndication(EndIndication),
    #[asn(key = 205)]
    IdEnhancedCoverageRestriction(EnhancedCoverageRestriction),
    #[asn(key = 206)]
    IdExtendedConnectedTime(ExtendedConnectedTime),
    #[asn(key = 31)]
    IdIndexToRfsp(IndexToRfsp),
    #[asn(key = 36)]
    IdMobilityRestrictionList(MobilityRestrictionList),
    #[asn(key = 38)]
    IdNasPdu(NasPdu),
    #[asn(key = 48)]
    IdOldAmf(AmfName),
    #[asn(key = 85)]
    IdRanUeNgapId(RanUeNgapId),
    #[asn(key = 83)]
    IdRanPagingPriority(RanPagingPriority),
    #[asn(key = 177)]
    IdSrvccOperationPossible(SrvccOperationPossible),
    #[asn(key = 209)]
    IdUeDifferentiationInfo(UeDifferentiationInfo),
    #[asn(key = 110)]
    IdUeAggregateMaximumBitRate(UeAggregateMaximumBitRate),
    #[asn(key = 228)]
    IdUeCapabilityInfoRequest(UeCapabilityInfoRequest),
    #[asn(key = 117)]
    IdUeRadioCapability(UeRadioCapability),
    #[asn(key = 264)]
    IdUeRadioCapabilityId(UeRadioCapabilityId),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct DownlinkNasTransportprotocolIEsItem {
    #[asn(key_field = true)]
    pub id: ProtocolIeId,
    pub criticality: Criticality,
    pub value: DownlinkNasTransportprotocolIEsItemvalue,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "0", sz_ub = "65535")]
pub struct DownlinkNasTransportprotocolIEs(Vec<DownlinkNasTransportprotocolIEsItem>);

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum DownlinkNonUeAssociatedNrpPaTransportprotocolIEsItemvalue {
    #[asn(key = 46)]
    IdNrpPaPdu(NrpPaPdu),
    #[asn(key = 89)]
    IdRoutingId(RoutingId),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct DownlinkNonUeAssociatedNrpPaTransportprotocolIEsItem {
    #[asn(key_field = true)]
    pub id: ProtocolIeId,
    pub criticality: Criticality,
    pub value: DownlinkNonUeAssociatedNrpPaTransportprotocolIEsItemvalue,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "0", sz_ub = "65535")]
pub struct DownlinkNonUeAssociatedNrpPaTransportprotocolIEs(Vec<DownlinkNonUeAssociatedNrpPaTransportprotocolIEsItem>);

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum DownlinkRanConfigurationTransferprotocolIEsItemvalue {
    #[asn(key = 157)]
    IdEndcSonConfigurationTransferDl(EnDcsonConfigurationTransfer),
    #[asn(key = 250)]
    IdIntersystemSonConfigurationTransferDl(IntersystemSonConfigurationTransfer),
    #[asn(key = 98)]
    IdSonConfigurationTransferDl(SonConfigurationTransfer),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct DownlinkRanConfigurationTransferprotocolIEsItem {
    #[asn(key_field = true)]
    pub id: ProtocolIeId,
    pub criticality: Criticality,
    pub value: DownlinkRanConfigurationTransferprotocolIEsItemvalue,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "0", sz_ub = "65535")]
pub struct DownlinkRanConfigurationTransferprotocolIEs(Vec<DownlinkRanConfigurationTransferprotocolIEsItem>);

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum DownlinkRanEarlyStatusTransferprotocolIEsItemvalue {
    #[asn(key = 10)]
    IdAmfUeNgapId(AmfUeNgapId),
    #[asn(key = 268)]
    IdEarlyStatusTransferTransparentContainer(EarlyStatusTransferTransparentContainer),
    #[asn(key = 85)]
    IdRanUeNgapId(RanUeNgapId),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct DownlinkRanEarlyStatusTransferprotocolIEsItem {
    #[asn(key_field = true)]
    pub id: ProtocolIeId,
    pub criticality: Criticality,
    pub value: DownlinkRanEarlyStatusTransferprotocolIEsItemvalue,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "0", sz_ub = "65535")]
pub struct DownlinkRanEarlyStatusTransferprotocolIEs(Vec<DownlinkRanEarlyStatusTransferprotocolIEsItem>);

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum DownlinkRanStatusTransferprotocolIEsItemvalue {
    #[asn(key = 10)]
    IdAmfUeNgapId(AmfUeNgapId),
    #[asn(key = 85)]
    IdRanUeNgapId(RanUeNgapId),
    #[asn(key = 84)]
    IdRanStatusTransferTransparentContainer(RanStatusTransferTransparentContainer),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct DownlinkRanStatusTransferprotocolIEsItem {
    #[asn(key_field = true)]
    pub id: ProtocolIeId,
    pub criticality: Criticality,
    pub value: DownlinkRanStatusTransferprotocolIEsItemvalue,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "0", sz_ub = "65535")]
pub struct DownlinkRanStatusTransferprotocolIEs(Vec<DownlinkRanStatusTransferprotocolIEsItem>);

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum DownlinkRimInformationTransferprotocolIEsItemvalue {
    #[asn(key = 175)]
    IdRimInformationTransfer(RimInformationTransfer),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct DownlinkRimInformationTransferprotocolIEsItem {
    #[asn(key_field = true)]
    pub id: ProtocolIeId,
    pub criticality: Criticality,
    pub value: DownlinkRimInformationTransferprotocolIEsItemvalue,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "0", sz_ub = "65535")]
pub struct DownlinkRimInformationTransferprotocolIEs(Vec<DownlinkRimInformationTransferprotocolIEsItem>);

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum DownlinkUeAssociatedNrpPaTransportprotocolIEsItemvalue {
    #[asn(key = 10)]
    IdAmfUeNgapId(AmfUeNgapId),
    #[asn(key = 46)]
    IdNrpPaPdu(NrpPaPdu),
    #[asn(key = 85)]
    IdRanUeNgapId(RanUeNgapId),
    #[asn(key = 89)]
    IdRoutingId(RoutingId),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct DownlinkUeAssociatedNrpPaTransportprotocolIEsItem {
    #[asn(key_field = true)]
    pub id: ProtocolIeId,
    pub criticality: Criticality,
    pub value: DownlinkUeAssociatedNrpPaTransportprotocolIEsItemvalue,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "0", sz_ub = "65535")]
pub struct DownlinkUeAssociatedNrpPaTransportprotocolIEs(Vec<DownlinkUeAssociatedNrpPaTransportprotocolIEsItem>);

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum Dynamic5QiDescriptoriEExtensionsItemextensionValue {
    #[asn(key = 187)]
    IdCnPacketDelayBudgetDl(ExtendedPacketDelayBudget),
    #[asn(key = 188)]
    IdCnPacketDelayBudgetUl(ExtendedPacketDelayBudget),
    #[asn(key = 189)]
    IdExtendedPacketDelayBudget(ExtendedPacketDelayBudget),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct Dynamic5QiDescriptoriEExtensionsItem {
    #[asn(key_field = true)]
    pub id: ProtocolExtensionId,
    pub criticality: Criticality,
    pub extension_value: Dynamic5QiDescriptoriEExtensionsItemextensionValue,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "65535")]
pub struct Dynamic5QiDescriptoriEExtensions(Vec<Dynamic5QiDescriptoriEExtensionsItem>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct ERabInformationItemiEExtensionsItem {}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "65535")]
pub struct ERabInformationItemiEExtensions(Vec<ERabInformationItemiEExtensionsItem>);

#[derive(Debug, AperCodec)]
#[asn(type = "BITSTRING", sz_extensible = false, sz_lb = "20", sz_ub = "20")]
pub struct BitString18(BitVec<Msb0, u8>);

#[derive(Debug, AperCodec)]
#[asn(type = "BITSTRING", sz_extensible = false, sz_lb = "28", sz_ub = "28")]
pub struct BitString19(BitVec<Msb0, u8>);

#[derive(Debug, AperCodec)]
#[asn(type = "BITSTRING", sz_extensible = false, sz_lb = "18", sz_ub = "18")]
pub struct BitString20(BitVec<Msb0, u8>);

#[derive(Debug, AperCodec)]
#[asn(type = "BITSTRING", sz_extensible = false, sz_lb = "21", sz_ub = "21")]
pub struct BitString21(BitVec<Msb0, u8>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct EnbIDchoiceExtensions {}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct EpsTaIiEExtensionsItem {}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "65535")]
pub struct EpsTaIiEExtensions(Vec<EpsTaIiEExtensionsItem>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct EutraCgIiEExtensionsItem {}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "65535")]
pub struct EutraCgIiEExtensions(Vec<EutraCgIiEExtensionsItem>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct EarlyStatusTransferTransparentContaineriEExtensionsItem {}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "65535")]
pub struct EarlyStatusTransferTransparentContaineriEExtensions(Vec<EarlyStatusTransferTransparentContaineriEExtensionsItem>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct EmergencyAreaIdBroadcastEutraItemiEExtensionsItem {}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "65535")]
pub struct EmergencyAreaIdBroadcastEutraItemiEExtensions(Vec<EmergencyAreaIdBroadcastEutraItemiEExtensionsItem>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct EmergencyAreaIdBroadcastNrItemiEExtensionsItem {}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "65535")]
pub struct EmergencyAreaIdBroadcastNrItemiEExtensions(Vec<EmergencyAreaIdBroadcastNrItemiEExtensionsItem>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct EmergencyAreaIdCancelledEutraItemiEExtensionsItem {}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "65535")]
pub struct EmergencyAreaIdCancelledEutraItemiEExtensions(Vec<EmergencyAreaIdCancelledEutraItemiEExtensionsItem>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct EmergencyAreaIdCancelledNrItemiEExtensionsItem {}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "65535")]
pub struct EmergencyAreaIdCancelledNrItemiEExtensions(Vec<EmergencyAreaIdCancelledNrItemiEExtensionsItem>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct EmergencyFallbackIndicatoriEExtensionsItem {}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "65535")]
pub struct EmergencyFallbackIndicatoriEExtensions(Vec<EmergencyFallbackIndicatoriEExtensionsItem>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct EndpointIpAddressAndPortiEExtensionsItem {}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "65535")]
pub struct EndpointIpAddressAndPortiEExtensions(Vec<EndpointIpAddressAndPortiEExtensionsItem>);

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum ErrorIndicationprotocolIEsItemvalue {
    #[asn(key = 10)]
    IdAmfUeNgapId(AmfUeNgapId),
    #[asn(key = 15)]
    IdCause(Cause),
    #[asn(key = 19)]
    IdCriticalityDiagnostics(CriticalityDiagnostics),
    #[asn(key = 26)]
    IdFiveGSTmsi(FiveGSTmsi),
    #[asn(key = 85)]
    IdRanUeNgapId(RanUeNgapId),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct ErrorIndicationprotocolIEsItem {
    #[asn(key_field = true)]
    pub id: ProtocolIeId,
    pub criticality: Criticality,
    pub value: ErrorIndicationprotocolIEsItemvalue,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "0", sz_ub = "65535")]
pub struct ErrorIndicationprotocolIEs(Vec<ErrorIndicationprotocolIEsItem>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct EventL1LoggedMdtConfigiEExtensionsItem {}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "65535")]
pub struct EventL1LoggedMdtConfigiEExtensions(Vec<EventL1LoggedMdtConfigiEExtensionsItem>);

#[derive(Debug, AperCodec)]
#[asn(type = "ENUMERATED", extensible = true, lb = "0", ub = "0")]
pub struct Enumerated22(u8);
impl Enumerated22 {
    const TRUE: u8 = 0u8;
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct EventTriggerchoiceExtensions {}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct ExpectedUeActivityBehaviouriEExtensionsItem {}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "65535")]
pub struct ExpectedUeActivityBehaviouriEExtensions(Vec<ExpectedUeActivityBehaviouriEExtensionsItem>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct ExpectedUeBehaviouriEExtensionsItem {}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "65535")]
pub struct ExpectedUeBehaviouriEExtensions(Vec<ExpectedUeBehaviouriEExtensionsItem>);

#[derive(Debug, AperCodec)]
#[asn(type = "INTEGER", lb = "0", ub = "4095")]
pub struct Integer23(u16);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct ExpectedUeMovingTrajectoryItemiEExtensionsItem {}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "65535")]
pub struct ExpectedUeMovingTrajectoryItemiEExtensions(Vec<ExpectedUeMovingTrajectoryItemiEExtensionsItem>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct ExtendedAmfNameiEExtensionsItem {}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "65535")]
pub struct ExtendedAmfNameiEExtensions(Vec<ExtendedAmfNameiEExtensionsItem>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct ExtendedRanNodeNameiEExtensionsItem {}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "65535")]
pub struct ExtendedRanNodeNameiEExtensions(Vec<ExtendedRanNodeNameiEExtensionsItem>);

#[derive(Debug, AperCodec)]
#[asn(type = "BITSTRING", sz_extensible = true, sz_lb = "8", sz_ub = "8")]
pub struct BitString24(BitVec<Msb0, u8>);

#[derive(Debug, AperCodec)]
#[asn(type = "BITSTRING", sz_extensible = true, sz_lb = "8", sz_ub = "8")]
pub struct BitString25(BitVec<Msb0, u8>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct ExtendedRatRestrictionInformationiEExtensionsItem {}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "65535")]
pub struct ExtendedRatRestrictionInformationiEExtensions(Vec<ExtendedRatRestrictionInformationiEExtensionsItem>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct FailureIndicationiEExtensionsItem {}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "65535")]
pub struct FailureIndicationiEExtensions(Vec<FailureIndicationiEExtensionsItem>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct FirstDlCountiEExtensionItem {}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "65535")]
pub struct FirstDlCountiEExtension(Vec<FirstDlCountiEExtensionItem>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct FiveGSTmsIiEExtensionsItem {}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "65535")]
pub struct FiveGSTmsIiEExtensions(Vec<FiveGSTmsIiEExtensionsItem>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct ForbiddenAreaInformationItemiEExtensionsItem {}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "65535")]
pub struct ForbiddenAreaInformationItemiEExtensions(Vec<ForbiddenAreaInformationItemiEExtensionsItem>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct FromEutraNtoNgraNiEExtensionsItem {}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "65535")]
pub struct FromEutraNtoNgraNiEExtensions(Vec<FromEutraNtoNgraNiEExtensionsItem>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct FromNgraNtoEutraNiEExtensionsItem {}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "65535")]
pub struct FromNgraNtoEutraNiEExtensions(Vec<FromNgraNtoEutraNiEExtensionsItem>);

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum GbrQosInformationiEExtensionsItemextensionValue {
    #[asn(key = 220)]
    IdAlternativeQoSParaSetList(AlternativeQoSParaSetList),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct GbrQosInformationiEExtensionsItem {
    #[asn(key_field = true)]
    pub id: ProtocolExtensionId,
    pub criticality: Criticality,
    pub extension_value: GbrQosInformationiEExtensionsItemextensionValue,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "65535")]
pub struct GbrQosInformationiEExtensions(Vec<GbrQosInformationiEExtensionsItem>);

#[derive(Debug, AperCodec)]
#[asn(type = "BITSTRING", sz_extensible = false, sz_lb = "22", sz_ub = "32")]
pub struct BitString26(BitVec<Msb0, u8>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct GnbIDchoiceExtensions {}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct GtpTunneliEExtensionsItem {}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "65535")]
pub struct GtpTunneliEExtensions(Vec<GtpTunneliEExtensionsItem>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct GuamIiEExtensionsItem {}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "65535")]
pub struct GuamIiEExtensions(Vec<GuamIiEExtensionsItem>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct GlobalEnbIDiEExtensionsItem {}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "65535")]
pub struct GlobalEnbIDiEExtensions(Vec<GlobalEnbIDiEExtensionsItem>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct GlobalGnbIDiEExtensionsItem {}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "65535")]
pub struct GlobalGnbIDiEExtensions(Vec<GlobalGnbIDiEExtensionsItem>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct GlobalLineIDiEExtensionsItem {}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "65535")]
pub struct GlobalLineIDiEExtensions(Vec<GlobalLineIDiEExtensionsItem>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct GlobalN3iwfIDiEExtensionsItem {}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "65535")]
pub struct GlobalN3iwfIDiEExtensions(Vec<GlobalN3iwfIDiEExtensionsItem>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct GlobalNgEnbIDiEExtensionsItem {}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "65535")]
pub struct GlobalNgEnbIDiEExtensions(Vec<GlobalNgEnbIDiEExtensionsItem>);

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum GlobalRanNodeIDchoiceExtensionsvalue {
    #[asn(key = 240)]
    IdGlobalTngfId(GlobalTngfId),
    #[asn(key = 241)]
    IdGlobalTwifId(GlobalTwifId),
    #[asn(key = 242)]
    IdGlobalWAgfId(GlobalWAgfId),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct GlobalRanNodeIDchoiceExtensions {
    #[asn(key_field = true)]
    pub id: ProtocolIeId,
    pub criticality: Criticality,
    pub value: GlobalRanNodeIDchoiceExtensionsvalue,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct GlobalTngfIDiEExtensionsItem {}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "65535")]
pub struct GlobalTngfIDiEExtensions(Vec<GlobalTngfIDiEExtensionsItem>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct GlobalTwifIDiEExtensionsItem {}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "65535")]
pub struct GlobalTwifIDiEExtensions(Vec<GlobalTwifIDiEExtensionsItem>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct GlobalWAgfIDiEExtensionsItem {}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "65535")]
pub struct GlobalWAgfIDiEExtensions(Vec<GlobalWAgfIDiEExtensionsItem>);

#[derive(Debug, AperCodec)]
#[asn(type = "ENUMERATED", extensible = true, lb = "0", ub = "2")]
pub struct Enumerated27(u8);
impl Enumerated27 {
    const HO_TOO_EARLY: u8 = 0u8;
    const HO_TO_WRONG_CELL: u8 = 1u8;
    const INTERSYSTEM_PING_PONG: u8 = 2u8;
}

#[derive(Debug, AperCodec)]
#[asn(type = "BITSTRING", sz_extensible = false, sz_lb = "16", sz_ub = "16")]
pub struct BitString28(BitVec<Msb0, u8>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct HoReportiEExtensionsItem {}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "65535")]
pub struct HoReportiEExtensions(Vec<HoReportiEExtensionsItem>);

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum HandoverCancelprotocolIEsItemvalue {
    #[asn(key = 10)]
    IdAmfUeNgapId(AmfUeNgapId),
    #[asn(key = 15)]
    IdCause(Cause),
    #[asn(key = 85)]
    IdRanUeNgapId(RanUeNgapId),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct HandoverCancelprotocolIEsItem {
    #[asn(key_field = true)]
    pub id: ProtocolIeId,
    pub criticality: Criticality,
    pub value: HandoverCancelprotocolIEsItemvalue,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "0", sz_ub = "65535")]
pub struct HandoverCancelprotocolIEs(Vec<HandoverCancelprotocolIEsItem>);

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum HandoverCancelAcknowledgeprotocolIEsItemvalue {
    #[asn(key = 10)]
    IdAmfUeNgapId(AmfUeNgapId),
    #[asn(key = 19)]
    IdCriticalityDiagnostics(CriticalityDiagnostics),
    #[asn(key = 85)]
    IdRanUeNgapId(RanUeNgapId),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct HandoverCancelAcknowledgeprotocolIEsItem {
    #[asn(key_field = true)]
    pub id: ProtocolIeId,
    pub criticality: Criticality,
    pub value: HandoverCancelAcknowledgeprotocolIEsItemvalue,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "0", sz_ub = "65535")]
pub struct HandoverCancelAcknowledgeprotocolIEs(Vec<HandoverCancelAcknowledgeprotocolIEsItem>);

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum HandoverCommandprotocolIEsItemvalue {
    #[asn(key = 10)]
    IdAmfUeNgapId(AmfUeNgapId),
    #[asn(key = 19)]
    IdCriticalityDiagnostics(CriticalityDiagnostics),
    #[asn(key = 29)]
    IdHandoverType(HandoverType),
    #[asn(key = 39)]
    IdNasSecurityParametersFromNgran(NasSecurityParametersFromNgran),
    #[asn(key = 59)]
    IdPduSessionResourceHandoverList(PduSessionResourceHandoverList),
    #[asn(key = 78)]
    IdPduSessionResourceToReleaseListHoCmd(PduSessionResourceToReleaseListHoCmd),
    #[asn(key = 85)]
    IdRanUeNgapId(RanUeNgapId),
    #[asn(key = 106)]
    IdTargetToSourceTransparentContainer(TargetToSourceTransparentContainer),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct HandoverCommandprotocolIEsItem {
    #[asn(key_field = true)]
    pub id: ProtocolIeId,
    pub criticality: Criticality,
    pub value: HandoverCommandprotocolIEsItemvalue,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "0", sz_ub = "65535")]
pub struct HandoverCommandprotocolIEs(Vec<HandoverCommandprotocolIEsItem>);

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum HandoverCommandTransferiEExtensionsItemextensionValue {
    #[asn(key = 152)]
    IdAdditionalDlForwardingUptnlInformation(QosFlowPerTnlInformationList),
    #[asn(key = 172)]
    IdAdditionalUlForwardingUptnlInformation(UpTransportLayerInformationList),
    #[asn(key = 249)]
    IdDataForwardingResponseErabList(DataForwardingResponseErabList),
    #[asn(key = 164)]
    IdUlForwardingUpTnlInformation(UpTransportLayerInformation),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct HandoverCommandTransferiEExtensionsItem {
    #[asn(key_field = true)]
    pub id: ProtocolExtensionId,
    pub criticality: Criticality,
    pub extension_value: HandoverCommandTransferiEExtensionsItemextensionValue,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "65535")]
pub struct HandoverCommandTransferiEExtensions(Vec<HandoverCommandTransferiEExtensionsItem>);

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum HandoverFailureprotocolIEsItemvalue {
    #[asn(key = 10)]
    IdAmfUeNgapId(AmfUeNgapId),
    #[asn(key = 15)]
    IdCause(Cause),
    #[asn(key = 19)]
    IdCriticalityDiagnostics(CriticalityDiagnostics),
    #[asn(key = 262)]
    IdTargettoSourceFailureTransparentContainer(TargettoSourceFailureTransparentContainer),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct HandoverFailureprotocolIEsItem {
    #[asn(key_field = true)]
    pub id: ProtocolIeId,
    pub criticality: Criticality,
    pub value: HandoverFailureprotocolIEsItemvalue,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "0", sz_ub = "65535")]
pub struct HandoverFailureprotocolIEs(Vec<HandoverFailureprotocolIEsItem>);

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum HandoverNotifyprotocolIEsItemvalue {
    #[asn(key = 10)]
    IdAmfUeNgapId(AmfUeNgapId),
    #[asn(key = 269)]
    IdNotifySourceNgranNode(NotifySourceNgranNode),
    #[asn(key = 85)]
    IdRanUeNgapId(RanUeNgapId),
    #[asn(key = 121)]
    IdUserLocationInformation(UserLocationInformation),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct HandoverNotifyprotocolIEsItem {
    #[asn(key_field = true)]
    pub id: ProtocolIeId,
    pub criticality: Criticality,
    pub value: HandoverNotifyprotocolIEsItemvalue,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "0", sz_ub = "65535")]
pub struct HandoverNotifyprotocolIEs(Vec<HandoverNotifyprotocolIEsItem>);

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum HandoverPreparationFailureprotocolIEsItemvalue {
    #[asn(key = 10)]
    IdAmfUeNgapId(AmfUeNgapId),
    #[asn(key = 15)]
    IdCause(Cause),
    #[asn(key = 19)]
    IdCriticalityDiagnostics(CriticalityDiagnostics),
    #[asn(key = 85)]
    IdRanUeNgapId(RanUeNgapId),
    #[asn(key = 262)]
    IdTargettoSourceFailureTransparentContainer(TargettoSourceFailureTransparentContainer),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct HandoverPreparationFailureprotocolIEsItem {
    #[asn(key_field = true)]
    pub id: ProtocolIeId,
    pub criticality: Criticality,
    pub value: HandoverPreparationFailureprotocolIEsItemvalue,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "0", sz_ub = "65535")]
pub struct HandoverPreparationFailureprotocolIEs(Vec<HandoverPreparationFailureprotocolIEsItem>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct HandoverPreparationUnsuccessfulTransferiEExtensionsItem {}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "65535")]
pub struct HandoverPreparationUnsuccessfulTransferiEExtensions(Vec<HandoverPreparationUnsuccessfulTransferiEExtensionsItem>);

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum HandoverRequestprotocolIEsItemvalue {
    #[asn(key = 10)]
    IdAmfUeNgapId(AmfUeNgapId),
    #[asn(key = 0)]
    IdAllowedNssai(AllowedNssai),
    #[asn(key = 222)]
    IdCEmodeBrestricted(CEmodeBrestricted),
    #[asn(key = 165)]
    IdCnAssistedRanTuning(CnAssistedRanTuning),
    #[asn(key = 15)]
    IdCause(Cause),
    #[asn(key = 18)]
    IdCoreNetworkAssistanceInformationForInactive(CoreNetworkAssistanceInformationForInactive),
    #[asn(key = 205)]
    IdEnhancedCoverageRestriction(EnhancedCoverageRestriction),
    #[asn(key = 206)]
    IdExtendedConnectedTime(ExtendedConnectedTime),
    #[asn(key = 28)]
    IdGuami(Guami),
    #[asn(key = 29)]
    IdHandoverType(HandoverType),
    #[asn(key = 199)]
    IdIabAuthorized(IabAuthorized),
    #[asn(key = 217)]
    IdLteueSidelinkAggregateMaximumBitrate(LteueSidelinkAggregateMaximumBitrate),
    #[asn(key = 215)]
    IdLtev2xServicesAuthorized(Ltev2xServicesAuthorized),
    #[asn(key = 33)]
    IdLocationReportingRequestType(LocationReportingRequestType),
    #[asn(key = 254)]
    IdManagementBasedMdtplmnList(MdtplmnList),
    #[asn(key = 34)]
    IdMaskedImeisv(MaskedImeisv),
    #[asn(key = 36)]
    IdMobilityRestrictionList(MobilityRestrictionList),
    #[asn(key = 37)]
    IdNasc(NasPdu),
    #[asn(key = 218)]
    IdNrueSidelinkAggregateMaximumBitrate(NrueSidelinkAggregateMaximumBitrate),
    #[asn(key = 216)]
    IdNrv2xServicesAuthorized(Nrv2xServicesAuthorized),
    #[asn(key = 41)]
    IdNewSecurityContextInd(NewSecurityContextInd),
    #[asn(key = 219)]
    IdPc5QoSParameters(Pc5QoSParameters),
    #[asn(key = 73)]
    IdPduSessionResourceSetupListHoReq(PduSessionResourceSetupListHoReq),
    #[asn(key = 91)]
    IdRrcInactiveTransitionReportRequest(RrcInactiveTransitionReportRequest),
    #[asn(key = 146)]
    IdRedirectionVoiceFallback(RedirectionVoiceFallback),
    #[asn(key = 177)]
    IdSrvccOperationPossible(SrvccOperationPossible),
    #[asn(key = 93)]
    IdSecurityContext(SecurityContext),
    #[asn(key = 101)]
    IdSourceToTargetTransparentContainer(SourceToTargetTransparentContainer),
    #[asn(key = 108)]
    IdTraceActivation(TraceActivation),
    #[asn(key = 209)]
    IdUeDifferentiationInfo(UeDifferentiationInfo),
    #[asn(key = 234)]
    IdUeUpCIoTSupport(UeUpCIoTSupport),
    #[asn(key = 110)]
    IdUeAggregateMaximumBitRate(UeAggregateMaximumBitRate),
    #[asn(key = 264)]
    IdUeRadioCapabilityId(UeRadioCapabilityId),
    #[asn(key = 119)]
    IdUeSecurityCapabilities(UeSecurityCapabilities),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct HandoverRequestprotocolIEsItem {
    #[asn(key_field = true)]
    pub id: ProtocolIeId,
    pub criticality: Criticality,
    pub value: HandoverRequestprotocolIEsItemvalue,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "0", sz_ub = "65535")]
pub struct HandoverRequestprotocolIEs(Vec<HandoverRequestprotocolIEsItem>);

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum HandoverRequestAcknowledgeprotocolIEsItemvalue {
    #[asn(key = 10)]
    IdAmfUeNgapId(AmfUeNgapId),
    #[asn(key = 19)]
    IdCriticalityDiagnostics(CriticalityDiagnostics),
    #[asn(key = 53)]
    IdPduSessionResourceAdmittedList(PduSessionResourceAdmittedList),
    #[asn(key = 56)]
    IdPduSessionResourceFailedToSetupListHoAck(PduSessionResourceFailedToSetupListHoAck),
    #[asn(key = 85)]
    IdRanUeNgapId(RanUeNgapId),
    #[asn(key = 106)]
    IdTargetToSourceTransparentContainer(TargetToSourceTransparentContainer),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct HandoverRequestAcknowledgeprotocolIEsItem {
    #[asn(key_field = true)]
    pub id: ProtocolIeId,
    pub criticality: Criticality,
    pub value: HandoverRequestAcknowledgeprotocolIEsItemvalue,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "0", sz_ub = "65535")]
pub struct HandoverRequestAcknowledgeprotocolIEs(Vec<HandoverRequestAcknowledgeprotocolIEsItem>);

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum HandoverRequestAcknowledgeTransferiEExtensionsItemextensionValue {
    #[asn(key = 153)]
    IdAdditionalDluptnlInformationForHoList(AdditionalDluptnlInformationForHoList),
    #[asn(key = 172)]
    IdAdditionalUlForwardingUptnlInformation(UpTransportLayerInformationList),
    #[asn(key = 249)]
    IdDataForwardingResponseErabList(DataForwardingResponseErabList),
    #[asn(key = 27)]
    IdGlobalRanNodeId(GlobalRanNodeId),
    #[asn(key = 192)]
    IdRedundantDlNguUpTnlInformation(UpTransportLayerInformation),
    #[asn(key = 164)]
    IdUlForwardingUpTnlInformation(UpTransportLayerInformation),
    #[asn(key = 198)]
    IdUsedRsnInformation(RedundantPduSessionInformation),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct HandoverRequestAcknowledgeTransferiEExtensionsItem {
    #[asn(key_field = true)]
    pub id: ProtocolExtensionId,
    pub criticality: Criticality,
    pub extension_value: HandoverRequestAcknowledgeTransferiEExtensionsItemextensionValue,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "65535")]
pub struct HandoverRequestAcknowledgeTransferiEExtensions(Vec<HandoverRequestAcknowledgeTransferiEExtensionsItem>);

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum HandoverRequiredprotocolIEsItemvalue {
    #[asn(key = 10)]
    IdAmfUeNgapId(AmfUeNgapId),
    #[asn(key = 15)]
    IdCause(Cause),
    #[asn(key = 22)]
    IdDirectForwardingPathAvailability(DirectForwardingPathAvailability),
    #[asn(key = 29)]
    IdHandoverType(HandoverType),
    #[asn(key = 61)]
    IdPduSessionResourceListHoRqd(PduSessionResourceListHoRqd),
    #[asn(key = 85)]
    IdRanUeNgapId(RanUeNgapId),
    #[asn(key = 101)]
    IdSourceToTargetTransparentContainer(SourceToTargetTransparentContainer),
    #[asn(key = 105)]
    IdTargetId(TargetId),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct HandoverRequiredprotocolIEsItem {
    #[asn(key_field = true)]
    pub id: ProtocolIeId,
    pub criticality: Criticality,
    pub value: HandoverRequiredprotocolIEsItemvalue,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "0", sz_ub = "65535")]
pub struct HandoverRequiredprotocolIEs(Vec<HandoverRequiredprotocolIEsItem>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct HandoverRequiredTransferiEExtensionsItem {}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "65535")]
pub struct HandoverRequiredTransferiEExtensions(Vec<HandoverRequiredTransferiEExtensionsItem>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct HandoverResourceAllocationUnsuccessfulTransferiEExtensionsItem {}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "65535")]
pub struct HandoverResourceAllocationUnsuccessfulTransferiEExtensions(Vec<HandoverResourceAllocationUnsuccessfulTransferiEExtensionsItem>);

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum HandoverSuccessprotocolIEsItemvalue {
    #[asn(key = 10)]
    IdAmfUeNgapId(AmfUeNgapId),
    #[asn(key = 85)]
    IdRanUeNgapId(RanUeNgapId),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct HandoverSuccessprotocolIEsItem {
    #[asn(key_field = true)]
    pub id: ProtocolIeId,
    pub criticality: Criticality,
    pub value: HandoverSuccessprotocolIEsItemvalue,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "0", sz_ub = "65535")]
pub struct HandoverSuccessprotocolIEs(Vec<HandoverSuccessprotocolIEsItem>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct ImmediateMdtNriEExtensionsItem {}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "65535")]
pub struct ImmediateMdtNriEExtensions(Vec<ImmediateMdtNriEExtensionsItem>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct InfoOnRecommendedCellsAndRanNodesForPagingiEExtensionsItem {}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "65535")]
pub struct InfoOnRecommendedCellsAndRanNodesForPagingiEExtensions(Vec<InfoOnRecommendedCellsAndRanNodesForPagingiEExtensionsItem>);

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum InitialContextSetupFailureprotocolIEsItemvalue {
    #[asn(key = 10)]
    IdAmfUeNgapId(AmfUeNgapId),
    #[asn(key = 15)]
    IdCause(Cause),
    #[asn(key = 19)]
    IdCriticalityDiagnostics(CriticalityDiagnostics),
    #[asn(key = 132)]
    IdPduSessionResourceFailedToSetupListCxtFail(PduSessionResourceFailedToSetupListCxtFail),
    #[asn(key = 85)]
    IdRanUeNgapId(RanUeNgapId),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct InitialContextSetupFailureprotocolIEsItem {
    #[asn(key_field = true)]
    pub id: ProtocolIeId,
    pub criticality: Criticality,
    pub value: InitialContextSetupFailureprotocolIEsItemvalue,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "0", sz_ub = "65535")]
pub struct InitialContextSetupFailureprotocolIEs(Vec<InitialContextSetupFailureprotocolIEsItem>);

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum InitialContextSetupRequestprotocolIEsItemvalue {
    #[asn(key = 10)]
    IdAmfUeNgapId(AmfUeNgapId),
    #[asn(key = 0)]
    IdAllowedNssai(AllowedNssai),
    #[asn(key = 222)]
    IdCEmodeBrestricted(CEmodeBrestricted),
    #[asn(key = 165)]
    IdCnAssistedRanTuning(CnAssistedRanTuning),
    #[asn(key = 18)]
    IdCoreNetworkAssistanceInformationForInactive(CoreNetworkAssistanceInformationForInactive),
    #[asn(key = 24)]
    IdEmergencyFallbackIndicator(EmergencyFallbackIndicator),
    #[asn(key = 205)]
    IdEnhancedCoverageRestriction(EnhancedCoverageRestriction),
    #[asn(key = 206)]
    IdExtendedConnectedTime(ExtendedConnectedTime),
    #[asn(key = 28)]
    IdGuami(Guami),
    #[asn(key = 199)]
    IdIabAuthorized(IabAuthorized),
    #[asn(key = 31)]
    IdIndexToRfsp(IndexToRfsp),
    #[asn(key = 217)]
    IdLteueSidelinkAggregateMaximumBitrate(LteueSidelinkAggregateMaximumBitrate),
    #[asn(key = 215)]
    IdLtev2xServicesAuthorized(Ltev2xServicesAuthorized),
    #[asn(key = 33)]
    IdLocationReportingRequestType(LocationReportingRequestType),
    #[asn(key = 254)]
    IdManagementBasedMdtplmnList(MdtplmnList),
    #[asn(key = 34)]
    IdMaskedImeisv(MaskedImeisv),
    #[asn(key = 36)]
    IdMobilityRestrictionList(MobilityRestrictionList),
    #[asn(key = 38)]
    IdNasPdu(NasPdu),
    #[asn(key = 218)]
    IdNrueSidelinkAggregateMaximumBitrate(NrueSidelinkAggregateMaximumBitrate),
    #[asn(key = 216)]
    IdNrv2xServicesAuthorized(Nrv2xServicesAuthorized),
    #[asn(key = 48)]
    IdOldAmf(AmfName),
    #[asn(key = 219)]
    IdPc5QoSParameters(Pc5QoSParameters),
    #[asn(key = 71)]
    IdPduSessionResourceSetupListCxtReq(PduSessionResourceSetupListCxtReq),
    #[asn(key = 85)]
    IdRanUeNgapId(RanUeNgapId),
    #[asn(key = 238)]
    IdRgLevelWirelineAccessCharacteristics(RgLevelWirelineAccessCharacteristics),
    #[asn(key = 91)]
    IdRrcInactiveTransitionReportRequest(RrcInactiveTransitionReportRequest),
    #[asn(key = 146)]
    IdRedirectionVoiceFallback(RedirectionVoiceFallback),
    #[asn(key = 177)]
    IdSrvccOperationPossible(SrvccOperationPossible),
    #[asn(key = 94)]
    IdSecurityKey(SecurityKey),
    #[asn(key = 108)]
    IdTraceActivation(TraceActivation),
    #[asn(key = 209)]
    IdUeDifferentiationInfo(UeDifferentiationInfo),
    #[asn(key = 234)]
    IdUeUpCIoTSupport(UeUpCIoTSupport),
    #[asn(key = 110)]
    IdUeAggregateMaximumBitRate(UeAggregateMaximumBitRate),
    #[asn(key = 117)]
    IdUeRadioCapability(UeRadioCapability),
    #[asn(key = 118)]
    IdUeRadioCapabilityForPaging(UeRadioCapabilityForPaging),
    #[asn(key = 264)]
    IdUeRadioCapabilityId(UeRadioCapabilityId),
    #[asn(key = 119)]
    IdUeSecurityCapabilities(UeSecurityCapabilities),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct InitialContextSetupRequestprotocolIEsItem {
    #[asn(key_field = true)]
    pub id: ProtocolIeId,
    pub criticality: Criticality,
    pub value: InitialContextSetupRequestprotocolIEsItemvalue,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "0", sz_ub = "65535")]
pub struct InitialContextSetupRequestprotocolIEs(Vec<InitialContextSetupRequestprotocolIEsItem>);

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum InitialContextSetupResponseprotocolIEsItemvalue {
    #[asn(key = 10)]
    IdAmfUeNgapId(AmfUeNgapId),
    #[asn(key = 19)]
    IdCriticalityDiagnostics(CriticalityDiagnostics),
    #[asn(key = 55)]
    IdPduSessionResourceFailedToSetupListCxtRes(PduSessionResourceFailedToSetupListCxtRes),
    #[asn(key = 72)]
    IdPduSessionResourceSetupListCxtRes(PduSessionResourceSetupListCxtRes),
    #[asn(key = 85)]
    IdRanUeNgapId(RanUeNgapId),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct InitialContextSetupResponseprotocolIEsItem {
    #[asn(key_field = true)]
    pub id: ProtocolIeId,
    pub criticality: Criticality,
    pub value: InitialContextSetupResponseprotocolIEsItemvalue,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "0", sz_ub = "65535")]
pub struct InitialContextSetupResponseprotocolIEs(Vec<InitialContextSetupResponseprotocolIEsItem>);

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum InitialUeMessageprotocolIEsItemvalue {
    #[asn(key = 3)]
    IdAmfSetId(AmfSetId),
    #[asn(key = 0)]
    IdAllowedNssai(AllowedNssai),
    #[asn(key = 245)]
    IdAuthenticatedIndication(AuthenticatedIndication),
    #[asn(key = 224)]
    IdCEmodeBSupportIndicator(CEmodeBSupportIndicator),
    #[asn(key = 227)]
    IdEdtSession(EdtSession),
    #[asn(key = 26)]
    IdFiveGSTmsi(FiveGSTmsi),
    #[asn(key = 201)]
    IdIabNodeIndication(IabNodeIndication),
    #[asn(key = 225)]
    IdLtemIndication(LtemIndication),
    #[asn(key = 38)]
    IdNasPdu(NasPdu),
    #[asn(key = 259)]
    IdNpnAccessInformation(NpnAccessInformation),
    #[asn(key = 85)]
    IdRanUeNgapId(RanUeNgapId),
    #[asn(key = 90)]
    IdRrcEstablishmentCause(RrcEstablishmentCause),
    #[asn(key = 174)]
    IdSelectedPlmnIdentity(PlmnIdentity),
    #[asn(key = 171)]
    IdSourceToTargetAmfInformationReroute(SourceToTargetAmfInformationReroute),
    #[asn(key = 112)]
    IdUeContextRequest(UeContextRequest),
    #[asn(key = 121)]
    IdUserLocationInformation(UserLocationInformation),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct InitialUeMessageprotocolIEsItem {
    #[asn(key_field = true)]
    pub id: ProtocolIeId,
    pub criticality: Criticality,
    pub value: InitialUeMessageprotocolIEsItemvalue,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "0", sz_ub = "65535")]
pub struct InitialUeMessageprotocolIEs(Vec<InitialUeMessageprotocolIEsItem>);

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum InitiatingMessagevalue {
    #[asn(key = 64)]
    IdAmfcpRelocationIndication(AmfcpRelocationIndication),
    #[asn(key = 0)]
    IdAmfConfigurationUpdate(AmfConfigurationUpdate),
    #[asn(key = 1)]
    IdAmfStatusIndication(AmfStatusIndication),
    #[asn(key = 2)]
    IdCellTrafficTrace(CellTrafficTrace),
    #[asn(key = 65)]
    IdConnectionEstablishmentIndication(ConnectionEstablishmentIndication),
    #[asn(key = 3)]
    IdDeactivateTrace(DeactivateTrace),
    #[asn(key = 4)]
    IdDownlinkNasTransport(DownlinkNasTransport),
    #[asn(key = 5)]
    IdDownlinkNonUeAssociatedNrpPaTransport(DownlinkNonUeAssociatedNrpPaTransport),
    #[asn(key = 6)]
    IdDownlinkRanConfigurationTransfer(DownlinkRanConfigurationTransfer),
    #[asn(key = 63)]
    IdDownlinkRanEarlyStatusTransfer(DownlinkRanEarlyStatusTransfer),
    #[asn(key = 7)]
    IdDownlinkRanStatusTransfer(DownlinkRanStatusTransfer),
    #[asn(key = 54)]
    IdDownlinkRimInformationTransfer(DownlinkRimInformationTransfer),
    #[asn(key = 8)]
    IdDownlinkUeAssociatedNrpPaTransport(DownlinkUeAssociatedNrpPaTransport),
    #[asn(key = 9)]
    IdErrorIndication(ErrorIndication),
    #[asn(key = 10)]
    IdHandoverCancel(HandoverCancel),
    #[asn(key = 11)]
    IdHandoverNotification(HandoverNotify),
    #[asn(key = 12)]
    IdHandoverPreparation(HandoverRequired),
    #[asn(key = 13)]
    IdHandoverResourceAllocation(HandoverRequest),
    #[asn(key = 61)]
    IdHandoverSuccess(HandoverSuccess),
    #[asn(key = 14)]
    IdInitialContextSetup(InitialContextSetupRequest),
    #[asn(key = 15)]
    IdInitialUeMessage(InitialUeMessage),
    #[asn(key = 18)]
    IdLocationReport(LocationReport),
    #[asn(key = 16)]
    IdLocationReportingControl(LocationReportingControl),
    #[asn(key = 17)]
    IdLocationReportingFailureIndication(LocationReportingFailureIndication),
    #[asn(key = 19)]
    IdNasNonDeliveryIndication(NasNonDeliveryIndication),
    #[asn(key = 20)]
    IdNgReset(NgReset),
    #[asn(key = 21)]
    IdNgSetup(NgSetupRequest),
    #[asn(key = 22)]
    IdOverloadStart(OverloadStart),
    #[asn(key = 23)]
    IdOverloadStop(OverloadStop),
    #[asn(key = 26)]
    IdPduSessionResourceModify(PduSessionResourceModifyRequest),
    #[asn(key = 27)]
    IdPduSessionResourceModifyIndication(PduSessionResourceModifyIndication),
    #[asn(key = 30)]
    IdPduSessionResourceNotify(PduSessionResourceNotify),
    #[asn(key = 28)]
    IdPduSessionResourceRelease(PduSessionResourceReleaseCommand),
    #[asn(key = 29)]
    IdPduSessionResourceSetup(PduSessionResourceSetupRequest),
    #[asn(key = 32)]
    IdPwsCancel(PwsCancelRequest),
    #[asn(key = 33)]
    IdPwsFailureIndication(PwsFailureIndication),
    #[asn(key = 34)]
    IdPwsRestartIndication(PwsRestartIndication),
    #[asn(key = 24)]
    IdPaging(Paging),
    #[asn(key = 25)]
    IdPathSwitchRequest(PathSwitchRequest),
    #[asn(key = 31)]
    IdPrivateMessage(PrivateMessage),
    #[asn(key = 57)]
    IdRancpRelocationIndication(RancpRelocationIndication),
    #[asn(key = 35)]
    IdRanConfigurationUpdate(RanConfigurationUpdate),
    #[asn(key = 37)]
    IdRrcInactiveTransitionReport(RrcInactiveTransitionReport),
    #[asn(key = 36)]
    IdRerouteNasRequest(RerouteNasRequest),
    #[asn(key = 55)]
    IdRetrieveUeInformation(RetrieveUeInformation),
    #[asn(key = 52)]
    IdSecondaryRatDataUsageReport(SecondaryRatDataUsageReport),
    #[asn(key = 38)]
    IdTraceFailureIndication(TraceFailureIndication),
    #[asn(key = 39)]
    IdTraceStart(TraceStart),
    #[asn(key = 40)]
    IdUeContextModification(UeContextModificationRequest),
    #[asn(key = 41)]
    IdUeContextRelease(UeContextReleaseCommand),
    #[asn(key = 42)]
    IdUeContextReleaseRequest(UeContextReleaseRequest),
    #[asn(key = 58)]
    IdUeContextResume(UeContextResumeRequest),
    #[asn(key = 59)]
    IdUeContextSuspend(UeContextSuspendRequest),
    #[asn(key = 56)]
    IdUeInformationTransfer(UeInformationTransfer),
    #[asn(key = 43)]
    IdUeRadioCapabilityCheck(UeRadioCapabilityCheckRequest),
    #[asn(key = 60)]
    IdUeRadioCapabilityIdMapping(UeRadioCapabilityIdMappingRequest),
    #[asn(key = 44)]
    IdUeRadioCapabilityInfoIndication(UeRadioCapabilityInfoIndication),
    #[asn(key = 45)]
    IdUetnlaBindingRelease(UetnlaBindingReleaseRequest),
    #[asn(key = 46)]
    IdUplinkNasTransport(UplinkNasTransport),
    #[asn(key = 47)]
    IdUplinkNonUeAssociatedNrpPaTransport(UplinkNonUeAssociatedNrpPaTransport),
    #[asn(key = 48)]
    IdUplinkRanConfigurationTransfer(UplinkRanConfigurationTransfer),
    #[asn(key = 62)]
    IdUplinkRanEarlyStatusTransfer(UplinkRanEarlyStatusTransfer),
    #[asn(key = 49)]
    IdUplinkRanStatusTransfer(UplinkRanStatusTransfer),
    #[asn(key = 53)]
    IdUplinkRimInformationTransfer(UplinkRimInformationTransfer),
    #[asn(key = 50)]
    IdUplinkUeAssociatedNrpPaTransport(UplinkUeAssociatedNrpPaTransport),
    #[asn(key = 51)]
    IdWriteReplaceWarning(WriteReplaceWarningRequest),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct InterSystemFailureIndicationiEExtensionsItem {}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "65535")]
pub struct InterSystemFailureIndicationiEExtensions(Vec<InterSystemFailureIndicationiEExtensionsItem>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct InterSystemHoReportiEExtensionsItem {}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "65535")]
pub struct InterSystemHoReportiEExtensions(Vec<InterSystemHoReportiEExtensionsItem>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct InterSystemHandoverReportTypechoiceExtensions {}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct IntersystemSonConfigurationTransferiEExtensionsItem {}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "65535")]
pub struct IntersystemSonConfigurationTransferiEExtensions(Vec<IntersystemSonConfigurationTransferiEExtensionsItem>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct IntersystemSonInformationchoiceExtensions {}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct IntersystemSonInformationReportchoiceExtensions {}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct IntersystemSonngraNnodeIDiEExtensionsItem {}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "65535")]
pub struct IntersystemSonngraNnodeIDiEExtensions(Vec<IntersystemSonngraNnodeIDiEExtensionsItem>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct IntersystemSonTransferTypechoiceExtensions {}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct IntersystemSoNeNbiDiEExtensionsItem {}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "65535")]
pub struct IntersystemSoNeNbiDiEExtensions(Vec<IntersystemSoNeNbiDiEExtensionsItem>);

#[derive(Debug, AperCodec)]
#[asn(type = "ENUMERATED", extensible = true, lb = "0", ub = "1")]
pub struct Enumerated29(u8);
impl Enumerated29 {
    const TRUE: u8 = 0u8;
    const FALSE: u8 = 1u8;
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct IntersystemUnnecessaryHOiEExtensionsItem {}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "65535")]
pub struct IntersystemUnnecessaryHOiEExtensions(Vec<IntersystemUnnecessaryHOiEExtensionsItem>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct LaIiEExtensionsItem {}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "65535")]
pub struct LaIiEExtensions(Vec<LaIiEExtensionsItem>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct LteueSidelinkAggregateMaximumBitrateiEExtensionsItem {}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "65535")]
pub struct LteueSidelinkAggregateMaximumBitrateiEExtensions(Vec<LteueSidelinkAggregateMaximumBitrateiEExtensionsItem>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct Ltev2xServicesAuthorizediEExtensionsItem {}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "65535")]
pub struct Ltev2xServicesAuthorizediEExtensions(Vec<Ltev2xServicesAuthorizediEExtensionsItem>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct LastVisitedCellInformationchoiceExtensions {}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct LastVisitedCellItemiEExtensionsItem {}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "65535")]
pub struct LastVisitedCellItemiEExtensions(Vec<LastVisitedCellItemiEExtensionsItem>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct LastVisitedNgranCellInformationiEExtensionsItem {}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "65535")]
pub struct LastVisitedNgranCellInformationiEExtensions(Vec<LastVisitedNgranCellInformationiEExtensionsItem>);

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum LocationReportprotocolIEsItemvalue {
    #[asn(key = 10)]
    IdAmfUeNgapId(AmfUeNgapId),
    #[asn(key = 33)]
    IdLocationReportingRequestType(LocationReportingRequestType),
    #[asn(key = 85)]
    IdRanUeNgapId(RanUeNgapId),
    #[asn(key = 116)]
    IdUePresenceInAreaOfInterestList(UePresenceInAreaOfInterestList),
    #[asn(key = 121)]
    IdUserLocationInformation(UserLocationInformation),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct LocationReportprotocolIEsItem {
    #[asn(key_field = true)]
    pub id: ProtocolIeId,
    pub criticality: Criticality,
    pub value: LocationReportprotocolIEsItemvalue,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "0", sz_ub = "65535")]
pub struct LocationReportprotocolIEs(Vec<LocationReportprotocolIEsItem>);

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum LocationReportingControlprotocolIEsItemvalue {
    #[asn(key = 10)]
    IdAmfUeNgapId(AmfUeNgapId),
    #[asn(key = 33)]
    IdLocationReportingRequestType(LocationReportingRequestType),
    #[asn(key = 85)]
    IdRanUeNgapId(RanUeNgapId),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct LocationReportingControlprotocolIEsItem {
    #[asn(key_field = true)]
    pub id: ProtocolIeId,
    pub criticality: Criticality,
    pub value: LocationReportingControlprotocolIEsItemvalue,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "0", sz_ub = "65535")]
pub struct LocationReportingControlprotocolIEs(Vec<LocationReportingControlprotocolIEsItem>);

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum LocationReportingFailureIndicationprotocolIEsItemvalue {
    #[asn(key = 10)]
    IdAmfUeNgapId(AmfUeNgapId),
    #[asn(key = 15)]
    IdCause(Cause),
    #[asn(key = 85)]
    IdRanUeNgapId(RanUeNgapId),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct LocationReportingFailureIndicationprotocolIEsItem {
    #[asn(key_field = true)]
    pub id: ProtocolIeId,
    pub criticality: Criticality,
    pub value: LocationReportingFailureIndicationprotocolIEsItemvalue,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "0", sz_ub = "65535")]
pub struct LocationReportingFailureIndicationprotocolIEs(Vec<LocationReportingFailureIndicationprotocolIEsItem>);

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum LocationReportingRequestTypeiEExtensionsItemextensionValue {
    #[asn(key = 170)]
    IdLocationReportingAdditionalInfo(LocationReportingAdditionalInfo),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct LocationReportingRequestTypeiEExtensionsItem {
    #[asn(key_field = true)]
    pub id: ProtocolExtensionId,
    pub criticality: Criticality,
    pub extension_value: LocationReportingRequestTypeiEExtensionsItemextensionValue,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "65535")]
pub struct LocationReportingRequestTypeiEExtensions(Vec<LocationReportingRequestTypeiEExtensionsItem>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct LoggedMdtNriEExtensionsItem {}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "65535")]
pub struct LoggedMdtNriEExtensions(Vec<LoggedMdtNriEExtensionsItem>);

#[derive(Debug, AperCodec)]
#[asn(type = "NULL")]
pub struct Null30;

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct LoggedMdtTriggerchoiceExtensions {}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct M1ConfigurationiEExtensionsItem {}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "65535")]
pub struct M1ConfigurationiEExtensions(Vec<M1ConfigurationiEExtensionsItem>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct M1PeriodicReportingiEExtensionsItem {}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "65535")]
pub struct M1PeriodicReportingiEExtensions(Vec<M1PeriodicReportingiEExtensionsItem>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct M1ThresholdEventA2iEExtensionsItem {}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "65535")]
pub struct M1ThresholdEventA2iEExtensions(Vec<M1ThresholdEventA2iEExtensionsItem>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct M1ThresholdTypechoiceExtensions {}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct M4ConfigurationiEExtensionsItem {}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "65535")]
pub struct M4ConfigurationiEExtensions(Vec<M4ConfigurationiEExtensionsItem>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct M5ConfigurationiEExtensionsItem {}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "65535")]
pub struct M5ConfigurationiEExtensions(Vec<M5ConfigurationiEExtensionsItem>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct M6ConfigurationiEExtensionsItem {}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "65535")]
pub struct M6ConfigurationiEExtensions(Vec<M6ConfigurationiEExtensionsItem>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct M7ConfigurationiEExtensionsItem {}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "65535")]
pub struct M7ConfigurationiEExtensions(Vec<M7ConfigurationiEExtensionsItem>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct MdtConfigurationiEExtensionsItem {}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "65535")]
pub struct MdtConfigurationiEExtensions(Vec<MdtConfigurationiEExtensionsItem>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct MdtConfigurationEutrAiEExtensionsItem {}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "65535")]
pub struct MdtConfigurationEutrAiEExtensions(Vec<MdtConfigurationEutrAiEExtensionsItem>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct MdtConfigurationNRiEExtensionsItem {}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "65535")]
pub struct MdtConfigurationNRiEExtensions(Vec<MdtConfigurationNRiEExtensionsItem>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct MdtLocationInfoiEExtensionsItem {}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "65535")]
pub struct MdtLocationInfoiEExtensions(Vec<MdtLocationInfoiEExtensionsItem>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct MdtModeNrchoiceExtensions {}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct MeasurementThresholdL1LoggedMdTchoiceExtensions {}

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum MobilityRestrictionListiEExtensionsItemextensionValue {
    #[asn(key = 160)]
    IdCnTypeRestrictionsForEquivalent(CnTypeRestrictionsForEquivalent),
    #[asn(key = 161)]
    IdCnTypeRestrictionsForServing(CnTypeRestrictionsForServing),
    #[asn(key = 150)]
    IdLastEutranPlmnIdentity(PlmnIdentity),
    #[asn(key = 261)]
    IdNpnMobilityInformation(NpnMobilityInformation),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct MobilityRestrictionListiEExtensionsItem {
    #[asn(key_field = true)]
    pub id: ProtocolExtensionId,
    pub criticality: Criticality,
    pub extension_value: MobilityRestrictionListiEExtensionsItemextensionValue,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "65535")]
pub struct MobilityRestrictionListiEExtensions(Vec<MobilityRestrictionListiEExtensionsItem>);

#[derive(Debug, AperCodec)]
#[asn(type = "BITSTRING", sz_extensible = false, sz_lb = "16", sz_ub = "16")]
pub struct BitString31(BitVec<Msb0, u8>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct N3iwfIDchoiceExtensions {}

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum NasNonDeliveryIndicationprotocolIEsItemvalue {
    #[asn(key = 10)]
    IdAmfUeNgapId(AmfUeNgapId),
    #[asn(key = 15)]
    IdCause(Cause),
    #[asn(key = 38)]
    IdNasPdu(NasPdu),
    #[asn(key = 85)]
    IdRanUeNgapId(RanUeNgapId),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct NasNonDeliveryIndicationprotocolIEsItem {
    #[asn(key_field = true)]
    pub id: ProtocolIeId,
    pub criticality: Criticality,
    pub value: NasNonDeliveryIndicationprotocolIEsItemvalue,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "0", sz_ub = "65535")]
pub struct NasNonDeliveryIndicationprotocolIEs(Vec<NasNonDeliveryIndicationprotocolIEsItem>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct NbIoTPagingEDrxInfoiEExtensionsItem {}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "65535")]
pub struct NbIoTPagingEDrxInfoiEExtensions(Vec<NbIoTPagingEDrxInfoiEExtensionsItem>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct NgranCgIchoiceExtensions {}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct NgranTnlAssociationToRemoveItemiEExtensionsItem {}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "65535")]
pub struct NgranTnlAssociationToRemoveItemiEExtensions(Vec<NgranTnlAssociationToRemoveItemiEExtensionsItem>);

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum NgResetprotocolIEsItemvalue {
    #[asn(key = 15)]
    IdCause(Cause),
    #[asn(key = 88)]
    IdResetType(ResetType),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct NgResetprotocolIEsItem {
    #[asn(key_field = true)]
    pub id: ProtocolIeId,
    pub criticality: Criticality,
    pub value: NgResetprotocolIEsItemvalue,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "0", sz_ub = "65535")]
pub struct NgResetprotocolIEs(Vec<NgResetprotocolIEsItem>);

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum NgResetAcknowledgeprotocolIEsItemvalue {
    #[asn(key = 19)]
    IdCriticalityDiagnostics(CriticalityDiagnostics),
    #[asn(key = 111)]
    IdUeAssociatedLogicalNgConnectionList(UeAssociatedLogicalNgConnectionList),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct NgResetAcknowledgeprotocolIEsItem {
    #[asn(key_field = true)]
    pub id: ProtocolIeId,
    pub criticality: Criticality,
    pub value: NgResetAcknowledgeprotocolIEsItemvalue,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "0", sz_ub = "65535")]
pub struct NgResetAcknowledgeprotocolIEs(Vec<NgResetAcknowledgeprotocolIEsItem>);

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum NgSetupFailureprotocolIEsItemvalue {
    #[asn(key = 15)]
    IdCause(Cause),
    #[asn(key = 19)]
    IdCriticalityDiagnostics(CriticalityDiagnostics),
    #[asn(key = 107)]
    IdTimeToWait(TimeToWait),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct NgSetupFailureprotocolIEsItem {
    #[asn(key_field = true)]
    pub id: ProtocolIeId,
    pub criticality: Criticality,
    pub value: NgSetupFailureprotocolIEsItemvalue,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "0", sz_ub = "65535")]
pub struct NgSetupFailureprotocolIEs(Vec<NgSetupFailureprotocolIEsItem>);

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum NgSetupRequestprotocolIEsItemvalue {
    #[asn(key = 21)]
    IdDefaultPagingDrx(PagingDrx),
    #[asn(key = 273)]
    IdExtendedRanNodeName(ExtendedRanNodeName),
    #[asn(key = 27)]
    IdGlobalRanNodeId(GlobalRanNodeId),
    #[asn(key = 204)]
    IdNbIoTDefaultPagingDrx(NbIoTDefaultPagingDrx),
    #[asn(key = 82)]
    IdRanNodeName(RanNodeName),
    #[asn(key = 102)]
    IdSupportedTaList(SupportedTaList),
    #[asn(key = 147)]
    IdUeRetentionInformation(UeRetentionInformation),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct NgSetupRequestprotocolIEsItem {
    #[asn(key_field = true)]
    pub id: ProtocolIeId,
    pub criticality: Criticality,
    pub value: NgSetupRequestprotocolIEsItemvalue,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "0", sz_ub = "65535")]
pub struct NgSetupRequestprotocolIEs(Vec<NgSetupRequestprotocolIEsItem>);

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum NgSetupResponseprotocolIEsItemvalue {
    #[asn(key = 1)]
    IdAmfName(AmfName),
    #[asn(key = 19)]
    IdCriticalityDiagnostics(CriticalityDiagnostics),
    #[asn(key = 274)]
    IdExtendedAmfName(ExtendedAmfName),
    #[asn(key = 200)]
    IdIabSupported(IabSupported),
    #[asn(key = 80)]
    IdPlmnSupportList(PlmnSupportList),
    #[asn(key = 86)]
    IdRelativeAmfCapacity(RelativeAmfCapacity),
    #[asn(key = 96)]
    IdServedGuamiList(ServedGuamiList),
    #[asn(key = 147)]
    IdUeRetentionInformation(UeRetentionInformation),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct NgSetupResponseprotocolIEsItem {
    #[asn(key_field = true)]
    pub id: ProtocolIeId,
    pub criticality: Criticality,
    pub value: NgSetupResponseprotocolIEsItemvalue,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "0", sz_ub = "65535")]
pub struct NgSetupResponseprotocolIEs(Vec<NgSetupResponseprotocolIEsItem>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct NpnAccessInformationchoiceExtensions {}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct NpnMobilityInformationchoiceExtensions {}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct NpnPagingAssistanceInformationchoiceExtensions {}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct NpnSupportchoiceExtensions {}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct NrCgIiEExtensionsItem {}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "65535")]
pub struct NrCgIiEExtensions(Vec<NrCgIiEExtensionsItem>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct NrFrequencyBandItemiEExtensionItem {}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "65535")]
pub struct NrFrequencyBandItemiEExtension(Vec<NrFrequencyBandItemiEExtensionItem>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct NrFrequencyInfoiEExtensionItem {}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "65535")]
pub struct NrFrequencyInfoiEExtension(Vec<NrFrequencyInfoiEExtensionItem>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct NrueSidelinkAggregateMaximumBitrateiEExtensionsItem {}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "65535")]
pub struct NrueSidelinkAggregateMaximumBitrateiEExtensions(Vec<NrueSidelinkAggregateMaximumBitrateiEExtensionsItem>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct Nrv2xServicesAuthorizediEExtensionsItem {}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "65535")]
pub struct Nrv2xServicesAuthorizediEExtensions(Vec<Nrv2xServicesAuthorizediEExtensionsItem>);

#[derive(Debug, AperCodec)]
#[asn(type = "BITSTRING", sz_extensible = false, sz_lb = "20", sz_ub = "20")]
pub struct BitString32(BitVec<Msb0, u8>);

#[derive(Debug, AperCodec)]
#[asn(type = "BITSTRING", sz_extensible = false, sz_lb = "18", sz_ub = "18")]
pub struct BitString33(BitVec<Msb0, u8>);

#[derive(Debug, AperCodec)]
#[asn(type = "BITSTRING", sz_extensible = false, sz_lb = "21", sz_ub = "21")]
pub struct BitString34(BitVec<Msb0, u8>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct NgEnbIDchoiceExtensions {}

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum NonDynamic5QiDescriptoriEExtensionsItemextensionValue {
    #[asn(key = 187)]
    IdCnPacketDelayBudgetDl(ExtendedPacketDelayBudget),
    #[asn(key = 188)]
    IdCnPacketDelayBudgetUl(ExtendedPacketDelayBudget),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct NonDynamic5QiDescriptoriEExtensionsItem {
    #[asn(key_field = true)]
    pub id: ProtocolExtensionId,
    pub criticality: Criticality,
    pub extension_value: NonDynamic5QiDescriptoriEExtensionsItemextensionValue,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "65535")]
pub struct NonDynamic5QiDescriptoriEExtensions(Vec<NonDynamic5QiDescriptoriEExtensionsItem>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct OverloadResponsechoiceExtensions {}

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum OverloadStartprotocolIEsItemvalue {
    #[asn(key = 2)]
    IdAmfOverloadResponse(OverloadResponse),
    #[asn(key = 9)]
    IdAmfTrafficLoadReductionIndication(TrafficLoadReductionIndication),
    #[asn(key = 49)]
    IdOverloadStartNssaiList(OverloadStartNssaiList),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct OverloadStartprotocolIEsItem {
    #[asn(key_field = true)]
    pub id: ProtocolIeId,
    pub criticality: Criticality,
    pub value: OverloadStartprotocolIEsItemvalue,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "0", sz_ub = "65535")]
pub struct OverloadStartprotocolIEs(Vec<OverloadStartprotocolIEsItem>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct OverloadStartNssaiItemiEExtensionsItem {}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "65535")]
pub struct OverloadStartNssaiItemiEExtensions(Vec<OverloadStartNssaiItemiEExtensionsItem>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct OverloadStopprotocolIEsItem {}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "0", sz_ub = "65535")]
pub struct OverloadStopprotocolIEs(Vec<OverloadStopprotocolIEsItem>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct Pc5FlowBitRatesiEExtensionsItem {}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "65535")]
pub struct Pc5FlowBitRatesiEExtensions(Vec<Pc5FlowBitRatesiEExtensionsItem>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct Pc5QoSFlowItemiEExtensionsItem {}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "65535")]
pub struct Pc5QoSFlowItemiEExtensions(Vec<Pc5QoSFlowItemiEExtensionsItem>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct Pc5QoSParametersiEExtensionsItem {}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "65535")]
pub struct Pc5QoSParametersiEExtensions(Vec<Pc5QoSParametersiEExtensionsItem>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct PduSessionAggregateMaximumBitRateiEExtensionsItem {}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "65535")]
pub struct PduSessionAggregateMaximumBitRateiEExtensions(Vec<PduSessionAggregateMaximumBitRateiEExtensionsItem>);

#[derive(Debug, AperCodec)]
#[asn(type = "OCTET-STRING")]
pub struct OctetString35(Vec<u8>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct PduSessionResourceAdmittedItemiEExtensionsItem {}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "65535")]
pub struct PduSessionResourceAdmittedItemiEExtensions(Vec<PduSessionResourceAdmittedItemiEExtensionsItem>);

#[derive(Debug, AperCodec)]
#[asn(type = "OCTET-STRING")]
pub struct OctetString36(Vec<u8>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct PduSessionResourceFailedToModifyItemModCfmiEExtensionsItem {}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "65535")]
pub struct PduSessionResourceFailedToModifyItemModCfmiEExtensions(Vec<PduSessionResourceFailedToModifyItemModCfmiEExtensionsItem>);

#[derive(Debug, AperCodec)]
#[asn(type = "OCTET-STRING")]
pub struct OctetString37(Vec<u8>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct PduSessionResourceFailedToModifyItemModResiEExtensionsItem {}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "65535")]
pub struct PduSessionResourceFailedToModifyItemModResiEExtensions(Vec<PduSessionResourceFailedToModifyItemModResiEExtensionsItem>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct PduSessionResourceFailedToResumeItemResReqiEExtensionsItem {}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "65535")]
pub struct PduSessionResourceFailedToResumeItemResReqiEExtensions(Vec<PduSessionResourceFailedToResumeItemResReqiEExtensionsItem>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct PduSessionResourceFailedToResumeItemResResiEExtensionsItem {}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "65535")]
pub struct PduSessionResourceFailedToResumeItemResResiEExtensions(Vec<PduSessionResourceFailedToResumeItemResResiEExtensionsItem>);

#[derive(Debug, AperCodec)]
#[asn(type = "OCTET-STRING")]
pub struct OctetString38(Vec<u8>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct PduSessionResourceFailedToSetupItemCxtFailiEExtensionsItem {}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "65535")]
pub struct PduSessionResourceFailedToSetupItemCxtFailiEExtensions(Vec<PduSessionResourceFailedToSetupItemCxtFailiEExtensionsItem>);

#[derive(Debug, AperCodec)]
#[asn(type = "OCTET-STRING")]
pub struct OctetString39(Vec<u8>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct PduSessionResourceFailedToSetupItemCxtResiEExtensionsItem {}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "65535")]
pub struct PduSessionResourceFailedToSetupItemCxtResiEExtensions(Vec<PduSessionResourceFailedToSetupItemCxtResiEExtensionsItem>);

#[derive(Debug, AperCodec)]
#[asn(type = "OCTET-STRING")]
pub struct OctetString40(Vec<u8>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct PduSessionResourceFailedToSetupItemHoAckiEExtensionsItem {}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "65535")]
pub struct PduSessionResourceFailedToSetupItemHoAckiEExtensions(Vec<PduSessionResourceFailedToSetupItemHoAckiEExtensionsItem>);

#[derive(Debug, AperCodec)]
#[asn(type = "OCTET-STRING")]
pub struct OctetString41(Vec<u8>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct PduSessionResourceFailedToSetupItemPsReqiEExtensionsItem {}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "65535")]
pub struct PduSessionResourceFailedToSetupItemPsReqiEExtensions(Vec<PduSessionResourceFailedToSetupItemPsReqiEExtensionsItem>);

#[derive(Debug, AperCodec)]
#[asn(type = "OCTET-STRING")]
pub struct OctetString42(Vec<u8>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct PduSessionResourceFailedToSetupItemSuResiEExtensionsItem {}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "65535")]
pub struct PduSessionResourceFailedToSetupItemSuResiEExtensions(Vec<PduSessionResourceFailedToSetupItemSuResiEExtensionsItem>);

#[derive(Debug, AperCodec)]
#[asn(type = "OCTET-STRING")]
pub struct OctetString43(Vec<u8>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct PduSessionResourceHandoverItemiEExtensionsItem {}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "65535")]
pub struct PduSessionResourceHandoverItemiEExtensions(Vec<PduSessionResourceHandoverItemiEExtensionsItem>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct PduSessionResourceInformationItemiEExtensionsItem {}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "65535")]
pub struct PduSessionResourceInformationItemiEExtensions(Vec<PduSessionResourceInformationItemiEExtensionsItem>);

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum PduSessionResourceItemCxtRelCpliEExtensionsItemextensionValue { }

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct PduSessionResourceItemCxtRelCpliEExtensionsItem {
    #[asn(key_field = true)]
    pub id: ProtocolExtensionId,
    pub criticality: Criticality,
    pub extension_value: PduSessionResourceItemCxtRelCpliEExtensionsItemextensionValue,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "65535")]
pub struct PduSessionResourceItemCxtRelCpliEExtensions(Vec<PduSessionResourceItemCxtRelCpliEExtensionsItem>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct PduSessionResourceItemCxtRelReqiEExtensionsItem {}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "65535")]
pub struct PduSessionResourceItemCxtRelReqiEExtensions(Vec<PduSessionResourceItemCxtRelReqiEExtensionsItem>);

#[derive(Debug, AperCodec)]
#[asn(type = "OCTET-STRING")]
pub struct OctetString44(Vec<u8>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct PduSessionResourceItemHoRqdiEExtensionsItem {}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "65535")]
pub struct PduSessionResourceItemHoRqdiEExtensions(Vec<PduSessionResourceItemHoRqdiEExtensionsItem>);

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum PduSessionResourceModifyConfirmprotocolIEsItemvalue {
    #[asn(key = 10)]
    IdAmfUeNgapId(AmfUeNgapId),
    #[asn(key = 19)]
    IdCriticalityDiagnostics(CriticalityDiagnostics),
    #[asn(key = 131)]
    IdPduSessionResourceFailedToModifyListModCfm(PduSessionResourceFailedToModifyListModCfm),
    #[asn(key = 62)]
    IdPduSessionResourceModifyListModCfm(PduSessionResourceModifyListModCfm),
    #[asn(key = 85)]
    IdRanUeNgapId(RanUeNgapId),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct PduSessionResourceModifyConfirmprotocolIEsItem {
    #[asn(key_field = true)]
    pub id: ProtocolIeId,
    pub criticality: Criticality,
    pub value: PduSessionResourceModifyConfirmprotocolIEsItemvalue,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "0", sz_ub = "65535")]
pub struct PduSessionResourceModifyConfirmprotocolIEs(Vec<PduSessionResourceModifyConfirmprotocolIEsItem>);

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum PduSessionResourceModifyConfirmTransferiEExtensionsItemextensionValue {
    #[asn(key = 185)]
    IdAdditionalRedundantNguUpTnlInformation(UpTransportLayerInformationPairList),
    #[asn(key = 195)]
    IdRedundantUlNguUpTnlInformation(UpTransportLayerInformation),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct PduSessionResourceModifyConfirmTransferiEExtensionsItem {
    #[asn(key_field = true)]
    pub id: ProtocolExtensionId,
    pub criticality: Criticality,
    pub extension_value: PduSessionResourceModifyConfirmTransferiEExtensionsItemextensionValue,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "65535")]
pub struct PduSessionResourceModifyConfirmTransferiEExtensions(Vec<PduSessionResourceModifyConfirmTransferiEExtensionsItem>);

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum PduSessionResourceModifyIndicationprotocolIEsItemvalue {
    #[asn(key = 10)]
    IdAmfUeNgapId(AmfUeNgapId),
    #[asn(key = 63)]
    IdPduSessionResourceModifyListModInd(PduSessionResourceModifyListModInd),
    #[asn(key = 85)]
    IdRanUeNgapId(RanUeNgapId),
    #[asn(key = 121)]
    IdUserLocationInformation(UserLocationInformation),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct PduSessionResourceModifyIndicationprotocolIEsItem {
    #[asn(key_field = true)]
    pub id: ProtocolIeId,
    pub criticality: Criticality,
    pub value: PduSessionResourceModifyIndicationprotocolIEsItemvalue,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "0", sz_ub = "65535")]
pub struct PduSessionResourceModifyIndicationprotocolIEs(Vec<PduSessionResourceModifyIndicationprotocolIEsItem>);

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum PduSessionResourceModifyIndicationTransferiEExtensionsItemextensionValue {
    #[asn(key = 184)]
    IdAdditionalRedundantDlQosFlowPerTnlInformation(QosFlowPerTnlInformationList),
    #[asn(key = 27)]
    IdGlobalRanNodeId(GlobalRanNodeId),
    #[asn(key = 193)]
    IdRedundantDlQosFlowPerTnlInformation(QosFlowPerTnlInformation),
    #[asn(key = 144)]
    IdSecondaryRatUsageInformation(SecondaryRatUsageInformation),
    #[asn(key = 156)]
    IdSecurityResult(SecurityResult),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct PduSessionResourceModifyIndicationTransferiEExtensionsItem {
    #[asn(key_field = true)]
    pub id: ProtocolExtensionId,
    pub criticality: Criticality,
    pub extension_value: PduSessionResourceModifyIndicationTransferiEExtensionsItemextensionValue,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "65535")]
pub struct PduSessionResourceModifyIndicationTransferiEExtensions(Vec<PduSessionResourceModifyIndicationTransferiEExtensionsItem>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct PduSessionResourceModifyIndicationUnsuccessfulTransferiEExtensionsItem {}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "65535")]
pub struct PduSessionResourceModifyIndicationUnsuccessfulTransferiEExtensions(Vec<PduSessionResourceModifyIndicationUnsuccessfulTransferiEExtensionsItem>);

#[derive(Debug, AperCodec)]
#[asn(type = "OCTET-STRING")]
pub struct OctetString45(Vec<u8>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct PduSessionResourceModifyItemModCfmiEExtensionsItem {}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "65535")]
pub struct PduSessionResourceModifyItemModCfmiEExtensions(Vec<PduSessionResourceModifyItemModCfmiEExtensionsItem>);

#[derive(Debug, AperCodec)]
#[asn(type = "OCTET-STRING")]
pub struct OctetString46(Vec<u8>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct PduSessionResourceModifyItemModIndiEExtensionsItem {}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "65535")]
pub struct PduSessionResourceModifyItemModIndiEExtensions(Vec<PduSessionResourceModifyItemModIndiEExtensionsItem>);

#[derive(Debug, AperCodec)]
#[asn(type = "OCTET-STRING")]
pub struct OctetString47(Vec<u8>);

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum PduSessionResourceModifyItemModReqiEExtensionsItemextensionValue {
    #[asn(key = 148)]
    IdSNssai(SNssai),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct PduSessionResourceModifyItemModReqiEExtensionsItem {
    #[asn(key_field = true)]
    pub id: ProtocolExtensionId,
    pub criticality: Criticality,
    pub extension_value: PduSessionResourceModifyItemModReqiEExtensionsItemextensionValue,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "65535")]
pub struct PduSessionResourceModifyItemModReqiEExtensions(Vec<PduSessionResourceModifyItemModReqiEExtensionsItem>);

#[derive(Debug, AperCodec)]
#[asn(type = "OCTET-STRING")]
pub struct OctetString48(Vec<u8>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct PduSessionResourceModifyItemModResiEExtensionsItem {}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "65535")]
pub struct PduSessionResourceModifyItemModResiEExtensions(Vec<PduSessionResourceModifyItemModResiEExtensionsItem>);

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum PduSessionResourceModifyRequestprotocolIEsItemvalue {
    #[asn(key = 10)]
    IdAmfUeNgapId(AmfUeNgapId),
    #[asn(key = 64)]
    IdPduSessionResourceModifyListModReq(PduSessionResourceModifyListModReq),
    #[asn(key = 85)]
    IdRanUeNgapId(RanUeNgapId),
    #[asn(key = 83)]
    IdRanPagingPriority(RanPagingPriority),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct PduSessionResourceModifyRequestprotocolIEsItem {
    #[asn(key_field = true)]
    pub id: ProtocolIeId,
    pub criticality: Criticality,
    pub value: PduSessionResourceModifyRequestprotocolIEsItemvalue,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "0", sz_ub = "65535")]
pub struct PduSessionResourceModifyRequestprotocolIEs(Vec<PduSessionResourceModifyRequestprotocolIEsItem>);

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum PduSessionResourceModifyRequestTransferprotocolIEsItemvalue {
    #[asn(key = 186)]
    IdAdditionalRedundantUlNguUpTnlInformation(UpTransportLayerInformationList),
    #[asn(key = 126)]
    IdAdditionalUlNguUpTnlInformation(UpTransportLayerInformationList),
    #[asn(key = 166)]
    IdCommonNetworkInstance(CommonNetworkInstance),
    #[asn(key = 129)]
    IdNetworkInstance(NetworkInstance),
    #[asn(key = 130)]
    IdPduSessionAggregateMaximumBitRate(PduSessionAggregateMaximumBitRate),
    #[asn(key = 135)]
    IdQosFlowAddOrModifyRequestList(QosFlowAddOrModifyRequestList),
    #[asn(key = 137)]
    IdQosFlowToReleaseList(QosFlowListWithCause),
    #[asn(key = 190)]
    IdRedundantCommonNetworkInstance(CommonNetworkInstance),
    #[asn(key = 195)]
    IdRedundantUlNguUpTnlInformation(UpTransportLayerInformation),
    #[asn(key = 138)]
    IdSecurityIndication(SecurityIndication),
    #[asn(key = 140)]
    IdUlNguUpTnlModifyList(UlNguUpTnlModifyList),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct PduSessionResourceModifyRequestTransferprotocolIEsItem {
    #[asn(key_field = true)]
    pub id: ProtocolIeId,
    pub criticality: Criticality,
    pub value: PduSessionResourceModifyRequestTransferprotocolIEsItemvalue,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "0", sz_ub = "65535")]
pub struct PduSessionResourceModifyRequestTransferprotocolIEs(Vec<PduSessionResourceModifyRequestTransferprotocolIEsItem>);

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum PduSessionResourceModifyResponseprotocolIEsItemvalue {
    #[asn(key = 10)]
    IdAmfUeNgapId(AmfUeNgapId),
    #[asn(key = 19)]
    IdCriticalityDiagnostics(CriticalityDiagnostics),
    #[asn(key = 54)]
    IdPduSessionResourceFailedToModifyListModRes(PduSessionResourceFailedToModifyListModRes),
    #[asn(key = 65)]
    IdPduSessionResourceModifyListModRes(PduSessionResourceModifyListModRes),
    #[asn(key = 85)]
    IdRanUeNgapId(RanUeNgapId),
    #[asn(key = 121)]
    IdUserLocationInformation(UserLocationInformation),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct PduSessionResourceModifyResponseprotocolIEsItem {
    #[asn(key_field = true)]
    pub id: ProtocolIeId,
    pub criticality: Criticality,
    pub value: PduSessionResourceModifyResponseprotocolIEsItemvalue,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "0", sz_ub = "65535")]
pub struct PduSessionResourceModifyResponseprotocolIEs(Vec<PduSessionResourceModifyResponseprotocolIEsItem>);

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum PduSessionResourceModifyResponseTransferiEExtensionsItemextensionValue {
    #[asn(key = 154)]
    IdAdditionalNguUpTnlInformation(UpTransportLayerInformationPairList),
    #[asn(key = 184)]
    IdAdditionalRedundantDlQosFlowPerTnlInformation(QosFlowPerTnlInformationList),
    #[asn(key = 185)]
    IdAdditionalRedundantNguUpTnlInformation(UpTransportLayerInformationPairList),
    #[asn(key = 192)]
    IdRedundantDlNguUpTnlInformation(UpTransportLayerInformation),
    #[asn(key = 195)]
    IdRedundantUlNguUpTnlInformation(UpTransportLayerInformation),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct PduSessionResourceModifyResponseTransferiEExtensionsItem {
    #[asn(key_field = true)]
    pub id: ProtocolExtensionId,
    pub criticality: Criticality,
    pub extension_value: PduSessionResourceModifyResponseTransferiEExtensionsItemextensionValue,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "65535")]
pub struct PduSessionResourceModifyResponseTransferiEExtensions(Vec<PduSessionResourceModifyResponseTransferiEExtensionsItem>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct PduSessionResourceModifyUnsuccessfulTransferiEExtensionsItem {}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "65535")]
pub struct PduSessionResourceModifyUnsuccessfulTransferiEExtensions(Vec<PduSessionResourceModifyUnsuccessfulTransferiEExtensionsItem>);

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum PduSessionResourceNotifyprotocolIEsItemvalue {
    #[asn(key = 10)]
    IdAmfUeNgapId(AmfUeNgapId),
    #[asn(key = 66)]
    IdPduSessionResourceNotifyList(PduSessionResourceNotifyList),
    #[asn(key = 67)]
    IdPduSessionResourceReleasedListNot(PduSessionResourceReleasedListNot),
    #[asn(key = 85)]
    IdRanUeNgapId(RanUeNgapId),
    #[asn(key = 121)]
    IdUserLocationInformation(UserLocationInformation),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct PduSessionResourceNotifyprotocolIEsItem {
    #[asn(key_field = true)]
    pub id: ProtocolIeId,
    pub criticality: Criticality,
    pub value: PduSessionResourceNotifyprotocolIEsItemvalue,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "0", sz_ub = "65535")]
pub struct PduSessionResourceNotifyprotocolIEs(Vec<PduSessionResourceNotifyprotocolIEsItem>);

#[derive(Debug, AperCodec)]
#[asn(type = "OCTET-STRING")]
pub struct OctetString49(Vec<u8>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct PduSessionResourceNotifyItemiEExtensionsItem {}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "65535")]
pub struct PduSessionResourceNotifyItemiEExtensions(Vec<PduSessionResourceNotifyItemiEExtensionsItem>);

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum PduSessionResourceNotifyReleasedTransferiEExtensionsItemextensionValue {
    #[asn(key = 144)]
    IdSecondaryRatUsageInformation(SecondaryRatUsageInformation),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct PduSessionResourceNotifyReleasedTransferiEExtensionsItem {
    #[asn(key_field = true)]
    pub id: ProtocolExtensionId,
    pub criticality: Criticality,
    pub extension_value: PduSessionResourceNotifyReleasedTransferiEExtensionsItemextensionValue,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "65535")]
pub struct PduSessionResourceNotifyReleasedTransferiEExtensions(Vec<PduSessionResourceNotifyReleasedTransferiEExtensionsItem>);

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum PduSessionResourceNotifyTransferiEExtensionsItemextensionValue {
    #[asn(key = 278)]
    IdQosFlowFeedbackList(QosFlowFeedbackList),
    #[asn(key = 144)]
    IdSecondaryRatUsageInformation(SecondaryRatUsageInformation),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct PduSessionResourceNotifyTransferiEExtensionsItem {
    #[asn(key_field = true)]
    pub id: ProtocolExtensionId,
    pub criticality: Criticality,
    pub extension_value: PduSessionResourceNotifyTransferiEExtensionsItemextensionValue,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "65535")]
pub struct PduSessionResourceNotifyTransferiEExtensions(Vec<PduSessionResourceNotifyTransferiEExtensionsItem>);

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum PduSessionResourceReleaseCommandprotocolIEsItemvalue {
    #[asn(key = 10)]
    IdAmfUeNgapId(AmfUeNgapId),
    #[asn(key = 38)]
    IdNasPdu(NasPdu),
    #[asn(key = 79)]
    IdPduSessionResourceToReleaseListRelCmd(PduSessionResourceToReleaseListRelCmd),
    #[asn(key = 85)]
    IdRanUeNgapId(RanUeNgapId),
    #[asn(key = 83)]
    IdRanPagingPriority(RanPagingPriority),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct PduSessionResourceReleaseCommandprotocolIEsItem {
    #[asn(key_field = true)]
    pub id: ProtocolIeId,
    pub criticality: Criticality,
    pub value: PduSessionResourceReleaseCommandprotocolIEsItemvalue,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "0", sz_ub = "65535")]
pub struct PduSessionResourceReleaseCommandprotocolIEs(Vec<PduSessionResourceReleaseCommandprotocolIEsItem>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct PduSessionResourceReleaseCommandTransferiEExtensionsItem {}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "65535")]
pub struct PduSessionResourceReleaseCommandTransferiEExtensions(Vec<PduSessionResourceReleaseCommandTransferiEExtensionsItem>);

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum PduSessionResourceReleaseResponseprotocolIEsItemvalue {
    #[asn(key = 10)]
    IdAmfUeNgapId(AmfUeNgapId),
    #[asn(key = 19)]
    IdCriticalityDiagnostics(CriticalityDiagnostics),
    #[asn(key = 70)]
    IdPduSessionResourceReleasedListRelRes(PduSessionResourceReleasedListRelRes),
    #[asn(key = 85)]
    IdRanUeNgapId(RanUeNgapId),
    #[asn(key = 121)]
    IdUserLocationInformation(UserLocationInformation),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct PduSessionResourceReleaseResponseprotocolIEsItem {
    #[asn(key_field = true)]
    pub id: ProtocolIeId,
    pub criticality: Criticality,
    pub value: PduSessionResourceReleaseResponseprotocolIEsItemvalue,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "0", sz_ub = "65535")]
pub struct PduSessionResourceReleaseResponseprotocolIEs(Vec<PduSessionResourceReleaseResponseprotocolIEsItem>);

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum PduSessionResourceReleaseResponseTransferiEExtensionsItemextensionValue {
    #[asn(key = 144)]
    IdSecondaryRatUsageInformation(SecondaryRatUsageInformation),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct PduSessionResourceReleaseResponseTransferiEExtensionsItem {
    #[asn(key_field = true)]
    pub id: ProtocolExtensionId,
    pub criticality: Criticality,
    pub extension_value: PduSessionResourceReleaseResponseTransferiEExtensionsItemextensionValue,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "65535")]
pub struct PduSessionResourceReleaseResponseTransferiEExtensions(Vec<PduSessionResourceReleaseResponseTransferiEExtensionsItem>);

#[derive(Debug, AperCodec)]
#[asn(type = "OCTET-STRING")]
pub struct OctetString50(Vec<u8>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct PduSessionResourceReleasedItemNotiEExtensionsItem {}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "65535")]
pub struct PduSessionResourceReleasedItemNotiEExtensions(Vec<PduSessionResourceReleasedItemNotiEExtensionsItem>);

#[derive(Debug, AperCodec)]
#[asn(type = "OCTET-STRING")]
pub struct OctetString51(Vec<u8>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct PduSessionResourceReleasedItemPsAckiEExtensionsItem {}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "65535")]
pub struct PduSessionResourceReleasedItemPsAckiEExtensions(Vec<PduSessionResourceReleasedItemPsAckiEExtensionsItem>);

#[derive(Debug, AperCodec)]
#[asn(type = "OCTET-STRING")]
pub struct OctetString52(Vec<u8>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct PduSessionResourceReleasedItemPsFailiEExtensionsItem {}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "65535")]
pub struct PduSessionResourceReleasedItemPsFailiEExtensions(Vec<PduSessionResourceReleasedItemPsFailiEExtensionsItem>);

#[derive(Debug, AperCodec)]
#[asn(type = "OCTET-STRING")]
pub struct OctetString53(Vec<u8>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct PduSessionResourceReleasedItemRelResiEExtensionsItem {}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "65535")]
pub struct PduSessionResourceReleasedItemRelResiEExtensions(Vec<PduSessionResourceReleasedItemRelResiEExtensionsItem>);

#[derive(Debug, AperCodec)]
#[asn(type = "OCTET-STRING")]
pub struct OctetString54(Vec<u8>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct PduSessionResourceResumeItemResReqiEExtensionsItem {}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "65535")]
pub struct PduSessionResourceResumeItemResReqiEExtensions(Vec<PduSessionResourceResumeItemResReqiEExtensionsItem>);

#[derive(Debug, AperCodec)]
#[asn(type = "OCTET-STRING")]
pub struct OctetString55(Vec<u8>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct PduSessionResourceResumeItemResResiEExtensionsItem {}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "65535")]
pub struct PduSessionResourceResumeItemResResiEExtensions(Vec<PduSessionResourceResumeItemResResiEExtensionsItem>);

#[derive(Debug, AperCodec)]
#[asn(type = "OCTET-STRING")]
pub struct OctetString56(Vec<u8>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct PduSessionResourceSecondaryRatUsageItemiEExtensionsItem {}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "65535")]
pub struct PduSessionResourceSecondaryRatUsageItemiEExtensions(Vec<PduSessionResourceSecondaryRatUsageItemiEExtensionsItem>);

#[derive(Debug, AperCodec)]
#[asn(type = "OCTET-STRING")]
pub struct OctetString57(Vec<u8>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct PduSessionResourceSetupItemCxtReqiEExtensionsItem {}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "65535")]
pub struct PduSessionResourceSetupItemCxtReqiEExtensions(Vec<PduSessionResourceSetupItemCxtReqiEExtensionsItem>);

#[derive(Debug, AperCodec)]
#[asn(type = "OCTET-STRING")]
pub struct OctetString58(Vec<u8>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct PduSessionResourceSetupItemCxtResiEExtensionsItem {}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "65535")]
pub struct PduSessionResourceSetupItemCxtResiEExtensions(Vec<PduSessionResourceSetupItemCxtResiEExtensionsItem>);

#[derive(Debug, AperCodec)]
#[asn(type = "OCTET-STRING")]
pub struct OctetString59(Vec<u8>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct PduSessionResourceSetupItemHoReqiEExtensionsItem {}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "65535")]
pub struct PduSessionResourceSetupItemHoReqiEExtensions(Vec<PduSessionResourceSetupItemHoReqiEExtensionsItem>);

#[derive(Debug, AperCodec)]
#[asn(type = "OCTET-STRING")]
pub struct OctetString60(Vec<u8>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct PduSessionResourceSetupItemSuReqiEExtensionsItem {}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "65535")]
pub struct PduSessionResourceSetupItemSuReqiEExtensions(Vec<PduSessionResourceSetupItemSuReqiEExtensionsItem>);

#[derive(Debug, AperCodec)]
#[asn(type = "OCTET-STRING")]
pub struct OctetString61(Vec<u8>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct PduSessionResourceSetupItemSuResiEExtensionsItem {}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "65535")]
pub struct PduSessionResourceSetupItemSuResiEExtensions(Vec<PduSessionResourceSetupItemSuResiEExtensionsItem>);

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum PduSessionResourceSetupRequestprotocolIEsItemvalue {
    #[asn(key = 10)]
    IdAmfUeNgapId(AmfUeNgapId),
    #[asn(key = 38)]
    IdNasPdu(NasPdu),
    #[asn(key = 74)]
    IdPduSessionResourceSetupListSuReq(PduSessionResourceSetupListSuReq),
    #[asn(key = 85)]
    IdRanUeNgapId(RanUeNgapId),
    #[asn(key = 83)]
    IdRanPagingPriority(RanPagingPriority),
    #[asn(key = 110)]
    IdUeAggregateMaximumBitRate(UeAggregateMaximumBitRate),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct PduSessionResourceSetupRequestprotocolIEsItem {
    #[asn(key_field = true)]
    pub id: ProtocolIeId,
    pub criticality: Criticality,
    pub value: PduSessionResourceSetupRequestprotocolIEsItemvalue,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "0", sz_ub = "65535")]
pub struct PduSessionResourceSetupRequestprotocolIEs(Vec<PduSessionResourceSetupRequestprotocolIEsItem>);

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum PduSessionResourceSetupRequestTransferprotocolIEsItemvalue {
    #[asn(key = 186)]
    IdAdditionalRedundantUlNguUpTnlInformation(UpTransportLayerInformationList),
    #[asn(key = 126)]
    IdAdditionalUlNguUpTnlInformation(UpTransportLayerInformationList),
    #[asn(key = 166)]
    IdCommonNetworkInstance(CommonNetworkInstance),
    #[asn(key = 127)]
    IdDataForwardingNotPossible(DataForwardingNotPossible),
    #[asn(key = 22)]
    IdDirectForwardingPathAvailability(DirectForwardingPathAvailability),
    #[asn(key = 129)]
    IdNetworkInstance(NetworkInstance),
    #[asn(key = 130)]
    IdPduSessionAggregateMaximumBitRate(PduSessionAggregateMaximumBitRate),
    #[asn(key = 134)]
    IdPduSessionType(PduSessionType),
    #[asn(key = 136)]
    IdQosFlowSetupRequestList(QosFlowSetupRequestList),
    #[asn(key = 190)]
    IdRedundantCommonNetworkInstance(CommonNetworkInstance),
    #[asn(key = 197)]
    IdRedundantPduSessionInformation(RedundantPduSessionInformation),
    #[asn(key = 195)]
    IdRedundantUlNguUpTnlInformation(UpTransportLayerInformation),
    #[asn(key = 138)]
    IdSecurityIndication(SecurityIndication),
    #[asn(key = 139)]
    IdUlNguUpTnlInformation(UpTransportLayerInformation),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct PduSessionResourceSetupRequestTransferprotocolIEsItem {
    #[asn(key_field = true)]
    pub id: ProtocolIeId,
    pub criticality: Criticality,
    pub value: PduSessionResourceSetupRequestTransferprotocolIEsItemvalue,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "0", sz_ub = "65535")]
pub struct PduSessionResourceSetupRequestTransferprotocolIEs(Vec<PduSessionResourceSetupRequestTransferprotocolIEsItem>);

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum PduSessionResourceSetupResponseprotocolIEsItemvalue {
    #[asn(key = 10)]
    IdAmfUeNgapId(AmfUeNgapId),
    #[asn(key = 19)]
    IdCriticalityDiagnostics(CriticalityDiagnostics),
    #[asn(key = 58)]
    IdPduSessionResourceFailedToSetupListSuRes(PduSessionResourceFailedToSetupListSuRes),
    #[asn(key = 75)]
    IdPduSessionResourceSetupListSuRes(PduSessionResourceSetupListSuRes),
    #[asn(key = 85)]
    IdRanUeNgapId(RanUeNgapId),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct PduSessionResourceSetupResponseprotocolIEsItem {
    #[asn(key_field = true)]
    pub id: ProtocolIeId,
    pub criticality: Criticality,
    pub value: PduSessionResourceSetupResponseprotocolIEsItemvalue,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "0", sz_ub = "65535")]
pub struct PduSessionResourceSetupResponseprotocolIEs(Vec<PduSessionResourceSetupResponseprotocolIEsItem>);

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum PduSessionResourceSetupResponseTransferiEExtensionsItemextensionValue {
    #[asn(key = 184)]
    IdAdditionalRedundantDlQosFlowPerTnlInformation(QosFlowPerTnlInformationList),
    #[asn(key = 27)]
    IdGlobalRanNodeId(GlobalRanNodeId),
    #[asn(key = 193)]
    IdRedundantDlQosFlowPerTnlInformation(QosFlowPerTnlInformation),
    #[asn(key = 198)]
    IdUsedRsnInformation(RedundantPduSessionInformation),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct PduSessionResourceSetupResponseTransferiEExtensionsItem {
    #[asn(key_field = true)]
    pub id: ProtocolExtensionId,
    pub criticality: Criticality,
    pub extension_value: PduSessionResourceSetupResponseTransferiEExtensionsItemextensionValue,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "65535")]
pub struct PduSessionResourceSetupResponseTransferiEExtensions(Vec<PduSessionResourceSetupResponseTransferiEExtensionsItem>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct PduSessionResourceSetupUnsuccessfulTransferiEExtensionsItem {}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "65535")]
pub struct PduSessionResourceSetupUnsuccessfulTransferiEExtensions(Vec<PduSessionResourceSetupUnsuccessfulTransferiEExtensionsItem>);

#[derive(Debug, AperCodec)]
#[asn(type = "OCTET-STRING")]
pub struct OctetString62(Vec<u8>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct PduSessionResourceSuspendItemSusReqiEExtensionsItem {}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "65535")]
pub struct PduSessionResourceSuspendItemSusReqiEExtensions(Vec<PduSessionResourceSuspendItemSusReqiEExtensionsItem>);

#[derive(Debug, AperCodec)]
#[asn(type = "OCTET-STRING")]
pub struct OctetString63(Vec<u8>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct PduSessionResourceSwitchedItemiEExtensionsItem {}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "65535")]
pub struct PduSessionResourceSwitchedItemiEExtensions(Vec<PduSessionResourceSwitchedItemiEExtensionsItem>);

#[derive(Debug, AperCodec)]
#[asn(type = "OCTET-STRING")]
pub struct OctetString64(Vec<u8>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct PduSessionResourceToBeSwitchedDlItemiEExtensionsItem {}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "65535")]
pub struct PduSessionResourceToBeSwitchedDlItemiEExtensions(Vec<PduSessionResourceToBeSwitchedDlItemiEExtensionsItem>);

#[derive(Debug, AperCodec)]
#[asn(type = "OCTET-STRING")]
pub struct OctetString65(Vec<u8>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct PduSessionResourceToReleaseItemHoCmdiEExtensionsItem {}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "65535")]
pub struct PduSessionResourceToReleaseItemHoCmdiEExtensions(Vec<PduSessionResourceToReleaseItemHoCmdiEExtensionsItem>);

#[derive(Debug, AperCodec)]
#[asn(type = "OCTET-STRING")]
pub struct OctetString66(Vec<u8>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct PduSessionResourceToReleaseItemRelCmdiEExtensionsItem {}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "65535")]
pub struct PduSessionResourceToReleaseItemRelCmdiEExtensions(Vec<PduSessionResourceToReleaseItemRelCmdiEExtensionsItem>);

#[derive(Debug, AperCodec)]
#[asn(type = "ENUMERATED", extensible = true, lb = "0", ub = "1")]
pub struct Enumerated67(u8);
impl Enumerated67 {
    const NR: u8 = 0u8;
    const EUTRA: u8 = 1u8;
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct PduSessionUsageReportiEExtensionsItem {}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "65535")]
pub struct PduSessionUsageReportiEExtensions(Vec<PduSessionUsageReportiEExtensionsItem>);

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum PlmnSupportItemiEExtensionsItemextensionValue {
    #[asn(key = 270)]
    IdExtendedSliceSupportList(ExtendedSliceSupportList),
    #[asn(key = 258)]
    IdNpnSupport(NpnSupport),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct PlmnSupportItemiEExtensionsItem {
    #[asn(key_field = true)]
    pub id: ProtocolExtensionId,
    pub criticality: Criticality,
    pub extension_value: PlmnSupportItemiEExtensionsItemextensionValue,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "65535")]
pub struct PlmnSupportItemiEExtensions(Vec<PlmnSupportItemiEExtensionsItem>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct PniNpnMobilityInformationiEExtensionsItem {}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "65535")]
pub struct PniNpnMobilityInformationiEExtensions(Vec<PniNpnMobilityInformationiEExtensionsItem>);

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum PwsCancelRequestprotocolIEsItemvalue {
    #[asn(key = 14)]
    IdCancelAllWarningMessages(CancelAllWarningMessages),
    #[asn(key = 35)]
    IdMessageIdentifier(MessageIdentifier),
    #[asn(key = 95)]
    IdSerialNumber(SerialNumber),
    #[asn(key = 122)]
    IdWarningAreaList(WarningAreaList),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct PwsCancelRequestprotocolIEsItem {
    #[asn(key_field = true)]
    pub id: ProtocolIeId,
    pub criticality: Criticality,
    pub value: PwsCancelRequestprotocolIEsItemvalue,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "0", sz_ub = "65535")]
pub struct PwsCancelRequestprotocolIEs(Vec<PwsCancelRequestprotocolIEsItem>);

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum PwsCancelResponseprotocolIEsItemvalue {
    #[asn(key = 12)]
    IdBroadcastCancelledAreaList(BroadcastCancelledAreaList),
    #[asn(key = 19)]
    IdCriticalityDiagnostics(CriticalityDiagnostics),
    #[asn(key = 35)]
    IdMessageIdentifier(MessageIdentifier),
    #[asn(key = 95)]
    IdSerialNumber(SerialNumber),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct PwsCancelResponseprotocolIEsItem {
    #[asn(key_field = true)]
    pub id: ProtocolIeId,
    pub criticality: Criticality,
    pub value: PwsCancelResponseprotocolIEsItemvalue,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "0", sz_ub = "65535")]
pub struct PwsCancelResponseprotocolIEs(Vec<PwsCancelResponseprotocolIEsItem>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct PwsFailedCellIdListchoiceExtensions {}

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum PwsFailureIndicationprotocolIEsItemvalue {
    #[asn(key = 27)]
    IdGlobalRanNodeId(GlobalRanNodeId),
    #[asn(key = 81)]
    IdPwsFailedCellIdList(PwsFailedCellIdList),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct PwsFailureIndicationprotocolIEsItem {
    #[asn(key_field = true)]
    pub id: ProtocolIeId,
    pub criticality: Criticality,
    pub value: PwsFailureIndicationprotocolIEsItemvalue,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "0", sz_ub = "65535")]
pub struct PwsFailureIndicationprotocolIEs(Vec<PwsFailureIndicationprotocolIEsItem>);

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum PwsRestartIndicationprotocolIEsItemvalue {
    #[asn(key = 16)]
    IdCellIdListForRestart(CellIdListForRestart),
    #[asn(key = 23)]
    IdEmergencyAreaIdListForRestart(EmergencyAreaIdListForRestart),
    #[asn(key = 27)]
    IdGlobalRanNodeId(GlobalRanNodeId),
    #[asn(key = 104)]
    IdTaiListForRestart(TaiListForRestart),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct PwsRestartIndicationprotocolIEsItem {
    #[asn(key_field = true)]
    pub id: ProtocolIeId,
    pub criticality: Criticality,
    pub value: PwsRestartIndicationprotocolIEsItemvalue,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "0", sz_ub = "65535")]
pub struct PwsRestartIndicationprotocolIEs(Vec<PwsRestartIndicationprotocolIEsItem>);

#[derive(Debug, AperCodec)]
#[asn(type = "INTEGER", lb = "0", ub = "9", extensible = true)]
pub struct Integer68(u8);

#[derive(Debug, AperCodec)]
#[asn(type = "INTEGER", lb = "0", ub = "9", extensible = true)]
pub struct Integer69(u8);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct PacketErrorRateiEExtensionsItem {}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "65535")]
pub struct PacketErrorRateiEExtensions(Vec<PacketErrorRateiEExtensionsItem>);

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum PagingprotocolIEsItemvalue {
    #[asn(key = 11)]
    IdAssistanceDataForPaging(AssistanceDataForPaging),
    #[asn(key = 222)]
    IdCEmodeBrestricted(CEmodeBrestricted),
    #[asn(key = 205)]
    IdEnhancedCoverageRestriction(EnhancedCoverageRestriction),
    #[asn(key = 203)]
    IdNbIoTPagingEDrxInfo(NbIoTPagingEDrxInfo),
    #[asn(key = 202)]
    IdNbIoTPagingDrx(NbIoTPagingDrx),
    #[asn(key = 50)]
    IdPagingDrx(PagingDrx),
    #[asn(key = 51)]
    IdPagingOrigin(PagingOrigin),
    #[asn(key = 52)]
    IdPagingPriority(PagingPriority),
    #[asn(key = 223)]
    IdPagingeDrxInformation(PagingeDrxInformation),
    #[asn(key = 103)]
    IdTaiListForPaging(TaiListForPaging),
    #[asn(key = 115)]
    IdUePagingIdentity(UePagingIdentity),
    #[asn(key = 118)]
    IdUeRadioCapabilityForPaging(UeRadioCapabilityForPaging),
    #[asn(key = 208)]
    IdWusAssistanceInformation(WusAssistanceInformation),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct PagingprotocolIEsItem {
    #[asn(key_field = true)]
    pub id: ProtocolIeId,
    pub criticality: Criticality,
    pub value: PagingprotocolIEsItemvalue,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "0", sz_ub = "65535")]
pub struct PagingprotocolIEs(Vec<PagingprotocolIEsItem>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct PagingAssisDataforCEcapabUEiEExtensionsItem {}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "65535")]
pub struct PagingAssisDataforCEcapabUEiEExtensions(Vec<PagingAssisDataforCEcapabUEiEExtensionsItem>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct PagingAttemptInformationiEExtensionsItem {}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "65535")]
pub struct PagingAttemptInformationiEExtensions(Vec<PagingAttemptInformationiEExtensionsItem>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct PagingeDrxInformationiEExtensionsItem {}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "65535")]
pub struct PagingeDrxInformationiEExtensions(Vec<PagingeDrxInformationiEExtensionsItem>);

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum PathSwitchRequestprotocolIEsItemvalue {
    #[asn(key = 57)]
    IdPduSessionResourceFailedToSetupListPsReq(PduSessionResourceFailedToSetupListPsReq),
    #[asn(key = 76)]
    IdPduSessionResourceToBeSwitchedDlList(PduSessionResourceToBeSwitchedDlList),
    #[asn(key = 85)]
    IdRanUeNgapId(RanUeNgapId),
    #[asn(key = 237)]
    IdRrcResumeCause(RrcEstablishmentCause),
    #[asn(key = 100)]
    IdSourceAmfUeNgapId(AmfUeNgapId),
    #[asn(key = 119)]
    IdUeSecurityCapabilities(UeSecurityCapabilities),
    #[asn(key = 121)]
    IdUserLocationInformation(UserLocationInformation),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct PathSwitchRequestprotocolIEsItem {
    #[asn(key_field = true)]
    pub id: ProtocolIeId,
    pub criticality: Criticality,
    pub value: PathSwitchRequestprotocolIEsItemvalue,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "0", sz_ub = "65535")]
pub struct PathSwitchRequestprotocolIEs(Vec<PathSwitchRequestprotocolIEsItem>);

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum PathSwitchRequestAcknowledgeprotocolIEsItemvalue {
    #[asn(key = 10)]
    IdAmfUeNgapId(AmfUeNgapId),
    #[asn(key = 0)]
    IdAllowedNssai(AllowedNssai),
    #[asn(key = 222)]
    IdCEmodeBrestricted(CEmodeBrestricted),
    #[asn(key = 165)]
    IdCnAssistedRanTuning(CnAssistedRanTuning),
    #[asn(key = 18)]
    IdCoreNetworkAssistanceInformationForInactive(CoreNetworkAssistanceInformationForInactive),
    #[asn(key = 19)]
    IdCriticalityDiagnostics(CriticalityDiagnostics),
    #[asn(key = 205)]
    IdEnhancedCoverageRestriction(EnhancedCoverageRestriction),
    #[asn(key = 206)]
    IdExtendedConnectedTime(ExtendedConnectedTime),
    #[asn(key = 217)]
    IdLteueSidelinkAggregateMaximumBitrate(LteueSidelinkAggregateMaximumBitrate),
    #[asn(key = 215)]
    IdLtev2xServicesAuthorized(Ltev2xServicesAuthorized),
    #[asn(key = 218)]
    IdNrueSidelinkAggregateMaximumBitrate(NrueSidelinkAggregateMaximumBitrate),
    #[asn(key = 216)]
    IdNrv2xServicesAuthorized(Nrv2xServicesAuthorized),
    #[asn(key = 41)]
    IdNewSecurityContextInd(NewSecurityContextInd),
    #[asn(key = 219)]
    IdPc5QoSParameters(Pc5QoSParameters),
    #[asn(key = 68)]
    IdPduSessionResourceReleasedListPsAck(PduSessionResourceReleasedListPsAck),
    #[asn(key = 77)]
    IdPduSessionResourceSwitchedList(PduSessionResourceSwitchedList),
    #[asn(key = 85)]
    IdRanUeNgapId(RanUeNgapId),
    #[asn(key = 91)]
    IdRrcInactiveTransitionReportRequest(RrcInactiveTransitionReportRequest),
    #[asn(key = 146)]
    IdRedirectionVoiceFallback(RedirectionVoiceFallback),
    #[asn(key = 177)]
    IdSrvccOperationPossible(SrvccOperationPossible),
    #[asn(key = 93)]
    IdSecurityContext(SecurityContext),
    #[asn(key = 209)]
    IdUeDifferentiationInfo(UeDifferentiationInfo),
    #[asn(key = 234)]
    IdUeUpCIoTSupport(UeUpCIoTSupport),
    #[asn(key = 264)]
    IdUeRadioCapabilityId(UeRadioCapabilityId),
    #[asn(key = 119)]
    IdUeSecurityCapabilities(UeSecurityCapabilities),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct PathSwitchRequestAcknowledgeprotocolIEsItem {
    #[asn(key_field = true)]
    pub id: ProtocolIeId,
    pub criticality: Criticality,
    pub value: PathSwitchRequestAcknowledgeprotocolIEsItemvalue,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "0", sz_ub = "65535")]
pub struct PathSwitchRequestAcknowledgeprotocolIEs(Vec<PathSwitchRequestAcknowledgeprotocolIEsItem>);

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum PathSwitchRequestAcknowledgeTransferiEExtensionsItemextensionValue {
    #[asn(key = 154)]
    IdAdditionalNguUpTnlInformation(UpTransportLayerInformationPairList),
    #[asn(key = 185)]
    IdAdditionalRedundantNguUpTnlInformation(UpTransportLayerInformationPairList),
    #[asn(key = 277)]
    IdQosFlowParametersList(QosFlowParametersList),
    #[asn(key = 195)]
    IdRedundantUlNguUpTnlInformation(UpTransportLayerInformation),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct PathSwitchRequestAcknowledgeTransferiEExtensionsItem {
    #[asn(key_field = true)]
    pub id: ProtocolExtensionId,
    pub criticality: Criticality,
    pub extension_value: PathSwitchRequestAcknowledgeTransferiEExtensionsItemextensionValue,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "65535")]
pub struct PathSwitchRequestAcknowledgeTransferiEExtensions(Vec<PathSwitchRequestAcknowledgeTransferiEExtensionsItem>);

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum PathSwitchRequestFailureprotocolIEsItemvalue {
    #[asn(key = 10)]
    IdAmfUeNgapId(AmfUeNgapId),
    #[asn(key = 19)]
    IdCriticalityDiagnostics(CriticalityDiagnostics),
    #[asn(key = 69)]
    IdPduSessionResourceReleasedListPsFail(PduSessionResourceReleasedListPsFail),
    #[asn(key = 85)]
    IdRanUeNgapId(RanUeNgapId),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct PathSwitchRequestFailureprotocolIEsItem {
    #[asn(key_field = true)]
    pub id: ProtocolIeId,
    pub criticality: Criticality,
    pub value: PathSwitchRequestFailureprotocolIEsItemvalue,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "0", sz_ub = "65535")]
pub struct PathSwitchRequestFailureprotocolIEs(Vec<PathSwitchRequestFailureprotocolIEsItem>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct PathSwitchRequestSetupFailedTransferiEExtensionsItem {}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "65535")]
pub struct PathSwitchRequestSetupFailedTransferiEExtensions(Vec<PathSwitchRequestSetupFailedTransferiEExtensionsItem>);

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum PathSwitchRequestTransferiEExtensionsItemextensionValue {
    #[asn(key = 155)]
    IdAdditionalDlQosFlowPerTnlInformation(QosFlowPerTnlInformationList),
    #[asn(key = 184)]
    IdAdditionalRedundantDlQosFlowPerTnlInformation(QosFlowPerTnlInformationList),
    #[asn(key = 27)]
    IdGlobalRanNodeId(GlobalRanNodeId),
    #[asn(key = 191)]
    IdRedundantDlNguTnlInformationReused(DlNguTnlInformationReused),
    #[asn(key = 192)]
    IdRedundantDlNguUpTnlInformation(UpTransportLayerInformation),
    #[asn(key = 198)]
    IdUsedRsnInformation(RedundantPduSessionInformation),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct PathSwitchRequestTransferiEExtensionsItem {
    #[asn(key_field = true)]
    pub id: ProtocolExtensionId,
    pub criticality: Criticality,
    pub extension_value: PathSwitchRequestTransferiEExtensionsItemextensionValue,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "65535")]
pub struct PathSwitchRequestTransferiEExtensions(Vec<PathSwitchRequestTransferiEExtensionsItem>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct PathSwitchRequestUnsuccessfulTransferiEExtensionsItem {}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "65535")]
pub struct PathSwitchRequestUnsuccessfulTransferiEExtensions(Vec<PathSwitchRequestUnsuccessfulTransferiEExtensionsItem>);

#[derive(Debug, AperCodec)]
#[asn(type = "INTEGER", lb = "0", ub = "65535")]
pub struct Integer70(u16);

#[derive(Debug, AperCodec)]
#[asn(type = "OBJECT-IDENTIFIER")]
pub struct ObjectIdentifier71;

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct PrivateMessageprivateIEsItem {}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "65535")]
pub struct PrivateMessageprivateIEs(Vec<PrivateMessageprivateIEsItem>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct ProcedureStageChoicechoiceExtensions {}

#[derive(Debug, AperCodec)]
#[asn(type = "ENUMERATED", extensible = true, lb = "0", ub = "1")]
pub struct Enumerated72(u8);
impl Enumerated72 {
    const NR: u8 = 0u8;
    const EUTRA: u8 = 1u8;
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct QoSFlowsUsageReportItemiEExtensionsItem {}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "65535")]
pub struct QoSFlowsUsageReportItemiEExtensions(Vec<QoSFlowsUsageReportItemiEExtensionsItem>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct QosCharacteristicschoiceExtensions {}

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum QosFlowAcceptedItemiEExtensionsItemextensionValue {
    #[asn(key = 221)]
    IdCurrentQoSParaSetIndex(AlternativeQoSParaSetIndex),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct QosFlowAcceptedItemiEExtensionsItem {
    #[asn(key_field = true)]
    pub id: ProtocolExtensionId,
    pub criticality: Criticality,
    pub extension_value: QosFlowAcceptedItemiEExtensionsItemextensionValue,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "65535")]
pub struct QosFlowAcceptedItemiEExtensions(Vec<QosFlowAcceptedItemiEExtensionsItem>);

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum QosFlowAddOrModifyRequestItemiEExtensionsItemextensionValue {
    #[asn(key = 194)]
    IdRedundantQosFlowIndicator(RedundantQosFlowIndicator),
    #[asn(key = 196)]
    IdTscTrafficCharacteristics(TscTrafficCharacteristics),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct QosFlowAddOrModifyRequestItemiEExtensionsItem {
    #[asn(key_field = true)]
    pub id: ProtocolExtensionId,
    pub criticality: Criticality,
    pub extension_value: QosFlowAddOrModifyRequestItemiEExtensionsItemextensionValue,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "65535")]
pub struct QosFlowAddOrModifyRequestItemiEExtensions(Vec<QosFlowAddOrModifyRequestItemiEExtensionsItem>);

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum QosFlowAddOrModifyResponseItemiEExtensionsItemextensionValue {
    #[asn(key = 221)]
    IdCurrentQoSParaSetIndex(AlternativeQoSParaSetIndex),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct QosFlowAddOrModifyResponseItemiEExtensionsItem {
    #[asn(key_field = true)]
    pub id: ProtocolExtensionId,
    pub criticality: Criticality,
    pub extension_value: QosFlowAddOrModifyResponseItemiEExtensionsItemextensionValue,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "65535")]
pub struct QosFlowAddOrModifyResponseItemiEExtensions(Vec<QosFlowAddOrModifyResponseItemiEExtensionsItem>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct QosFlowFeedbackItemiEExtensionsItem {}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "65535")]
pub struct QosFlowFeedbackItemiEExtensions(Vec<QosFlowFeedbackItemiEExtensionsItem>);

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum QosFlowInformationItemiEExtensionsItemextensionValue {
    #[asn(key = 163)]
    IdUlForwarding(UlForwarding),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct QosFlowInformationItemiEExtensionsItem {
    #[asn(key_field = true)]
    pub id: ProtocolExtensionId,
    pub criticality: Criticality,
    pub extension_value: QosFlowInformationItemiEExtensionsItemextensionValue,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "65535")]
pub struct QosFlowInformationItemiEExtensions(Vec<QosFlowInformationItemiEExtensionsItem>);

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum QosFlowItemWithDataForwardingiEExtensionsItemextensionValue {
    #[asn(key = 221)]
    IdCurrentQoSParaSetIndex(AlternativeQoSParaSetIndex),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct QosFlowItemWithDataForwardingiEExtensionsItem {
    #[asn(key_field = true)]
    pub id: ProtocolExtensionId,
    pub criticality: Criticality,
    pub extension_value: QosFlowItemWithDataForwardingiEExtensionsItemextensionValue,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "65535")]
pub struct QosFlowItemWithDataForwardingiEExtensions(Vec<QosFlowItemWithDataForwardingiEExtensionsItem>);

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum QosFlowLevelQosParametersiEExtensionsItemextensionValue {
    #[asn(key = 276)]
    IdQosMonitoringReportingFrequency(QosMonitoringReportingFrequency),
    #[asn(key = 181)]
    IdQosMonitoringRequest(QosMonitoringRequest),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct QosFlowLevelQosParametersiEExtensionsItem {
    #[asn(key_field = true)]
    pub id: ProtocolExtensionId,
    pub criticality: Criticality,
    pub extension_value: QosFlowLevelQosParametersiEExtensionsItemextensionValue,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "65535")]
pub struct QosFlowLevelQosParametersiEExtensions(Vec<QosFlowLevelQosParametersiEExtensionsItem>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct QosFlowModifyConfirmItemiEExtensionsItem {}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "65535")]
pub struct QosFlowModifyConfirmItemiEExtensions(Vec<QosFlowModifyConfirmItemiEExtensionsItem>);

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum QosFlowNotifyItemiEExtensionsItemextensionValue {
    #[asn(key = 221)]
    IdCurrentQoSParaSetIndex(AlternativeQoSParaSetNotifyIndex),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct QosFlowNotifyItemiEExtensionsItem {
    #[asn(key_field = true)]
    pub id: ProtocolExtensionId,
    pub criticality: Criticality,
    pub extension_value: QosFlowNotifyItemiEExtensionsItemextensionValue,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "65535")]
pub struct QosFlowNotifyItemiEExtensions(Vec<QosFlowNotifyItemiEExtensionsItem>);

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum QosFlowParametersItemiEExtensionsItemextensionValue {
    #[asn(key = 279)]
    IdBurstArrivalTimeDownlink(BurstArrivalTime),
    #[asn(key = 187)]
    IdCnPacketDelayBudgetDl(ExtendedPacketDelayBudget),
    #[asn(key = 188)]
    IdCnPacketDelayBudgetUl(ExtendedPacketDelayBudget),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct QosFlowParametersItemiEExtensionsItem {
    #[asn(key_field = true)]
    pub id: ProtocolExtensionId,
    pub criticality: Criticality,
    pub extension_value: QosFlowParametersItemiEExtensionsItemextensionValue,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "65535")]
pub struct QosFlowParametersItemiEExtensions(Vec<QosFlowParametersItemiEExtensionsItem>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct QosFlowPerTnlInformationiEExtensionsItem {}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "65535")]
pub struct QosFlowPerTnlInformationiEExtensions(Vec<QosFlowPerTnlInformationiEExtensionsItem>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct QosFlowPerTnlInformationItemiEExtensionsItem {}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "65535")]
pub struct QosFlowPerTnlInformationItemiEExtensions(Vec<QosFlowPerTnlInformationItemiEExtensionsItem>);

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum QosFlowSetupRequestItemiEExtensionsItemextensionValue {
    #[asn(key = 194)]
    IdRedundantQosFlowIndicator(RedundantQosFlowIndicator),
    #[asn(key = 196)]
    IdTscTrafficCharacteristics(TscTrafficCharacteristics),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct QosFlowSetupRequestItemiEExtensionsItem {
    #[asn(key_field = true)]
    pub id: ProtocolExtensionId,
    pub criticality: Criticality,
    pub extension_value: QosFlowSetupRequestItemiEExtensionsItemextensionValue,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "65535")]
pub struct QosFlowSetupRequestItemiEExtensions(Vec<QosFlowSetupRequestItemiEExtensionsItem>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct QosFlowToBeForwardedItemiEExtensionsItem {}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "65535")]
pub struct QosFlowToBeForwardedItemiEExtensions(Vec<QosFlowToBeForwardedItemiEExtensionsItem>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct QosFlowWithCauseItemiEExtensionsItem {}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "65535")]
pub struct QosFlowWithCauseItemiEExtensions(Vec<QosFlowWithCauseItemiEExtensionsItem>);

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum RancpRelocationIndicationprotocolIEsItemvalue {
    #[asn(key = 25)]
    IdEutraCgi(EutraCgi),
    #[asn(key = 26)]
    IdFiveGSTmsi(FiveGSTmsi),
    #[asn(key = 85)]
    IdRanUeNgapId(RanUeNgapId),
    #[asn(key = 213)]
    IdTai(Tai),
    #[asn(key = 211)]
    IdUlCpSecurityInformation(UlCpSecurityInformation),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct RancpRelocationIndicationprotocolIEsItem {
    #[asn(key_field = true)]
    pub id: ProtocolIeId,
    pub criticality: Criticality,
    pub value: RancpRelocationIndicationprotocolIEsItemvalue,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "0", sz_ub = "65535")]
pub struct RancpRelocationIndicationprotocolIEs(Vec<RancpRelocationIndicationprotocolIEsItem>);

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum RanConfigurationUpdateprotocolIEsItemvalue {
    #[asn(key = 21)]
    IdDefaultPagingDrx(PagingDrx),
    #[asn(key = 273)]
    IdExtendedRanNodeName(ExtendedRanNodeName),
    #[asn(key = 27)]
    IdGlobalRanNodeId(GlobalRanNodeId),
    #[asn(key = 204)]
    IdNbIoTDefaultPagingDrx(NbIoTDefaultPagingDrx),
    #[asn(key = 167)]
    IdNgranTnlAssociationToRemoveList(NgranTnlAssociationToRemoveList),
    #[asn(key = 82)]
    IdRanNodeName(RanNodeName),
    #[asn(key = 102)]
    IdSupportedTaList(SupportedTaList),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct RanConfigurationUpdateprotocolIEsItem {
    #[asn(key_field = true)]
    pub id: ProtocolIeId,
    pub criticality: Criticality,
    pub value: RanConfigurationUpdateprotocolIEsItemvalue,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "0", sz_ub = "65535")]
pub struct RanConfigurationUpdateprotocolIEs(Vec<RanConfigurationUpdateprotocolIEsItem>);

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum RanConfigurationUpdateAcknowledgeprotocolIEsItemvalue {
    #[asn(key = 19)]
    IdCriticalityDiagnostics(CriticalityDiagnostics),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct RanConfigurationUpdateAcknowledgeprotocolIEsItem {
    #[asn(key_field = true)]
    pub id: ProtocolIeId,
    pub criticality: Criticality,
    pub value: RanConfigurationUpdateAcknowledgeprotocolIEsItemvalue,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "0", sz_ub = "65535")]
pub struct RanConfigurationUpdateAcknowledgeprotocolIEs(Vec<RanConfigurationUpdateAcknowledgeprotocolIEsItem>);

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum RanConfigurationUpdateFailureprotocolIEsItemvalue {
    #[asn(key = 15)]
    IdCause(Cause),
    #[asn(key = 19)]
    IdCriticalityDiagnostics(CriticalityDiagnostics),
    #[asn(key = 107)]
    IdTimeToWait(TimeToWait),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct RanConfigurationUpdateFailureprotocolIEsItem {
    #[asn(key_field = true)]
    pub id: ProtocolIeId,
    pub criticality: Criticality,
    pub value: RanConfigurationUpdateFailureprotocolIEsItemvalue,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "0", sz_ub = "65535")]
pub struct RanConfigurationUpdateFailureprotocolIEs(Vec<RanConfigurationUpdateFailureprotocolIEsItem>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct RanStatusTransferTransparentContaineriEExtensionsItem {}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "65535")]
pub struct RanStatusTransferTransparentContaineriEExtensions(Vec<RanStatusTransferTransparentContaineriEExtensionsItem>);

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum RatRestrictionsItemiEExtensionsItemextensionValue {
    #[asn(key = 180)]
    IdExtendedRatRestrictionInformation(ExtendedRatRestrictionInformation),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct RatRestrictionsItemiEExtensionsItem {
    #[asn(key_field = true)]
    pub id: ProtocolExtensionId,
    pub criticality: Criticality,
    pub extension_value: RatRestrictionsItemiEExtensionsItemextensionValue,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "65535")]
pub struct RatRestrictionsItemiEExtensions(Vec<RatRestrictionsItemiEExtensionsItem>);

#[derive(Debug, AperCodec)]
#[asn(type = "ENUMERATED", extensible = true, lb = "0", ub = "1")]
pub struct Enumerated73(u8);
impl Enumerated73 {
    const RS_DETECTED: u8 = 0u8;
    const RS_DISAPPEARED: u8 = 1u8;
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct RimInformationiEExtensionsItem {}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "65535")]
pub struct RimInformationiEExtensions(Vec<RimInformationiEExtensionsItem>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct RimInformationTransferiEExtensionsItem {}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "65535")]
pub struct RimInformationTransferiEExtensions(Vec<RimInformationTransferiEExtensionsItem>);

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum RrcInactiveTransitionReportprotocolIEsItemvalue {
    #[asn(key = 10)]
    IdAmfUeNgapId(AmfUeNgapId),
    #[asn(key = 85)]
    IdRanUeNgapId(RanUeNgapId),
    #[asn(key = 92)]
    IdRrcState(RrcState),
    #[asn(key = 121)]
    IdUserLocationInformation(UserLocationInformation),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct RrcInactiveTransitionReportprotocolIEsItem {
    #[asn(key_field = true)]
    pub id: ProtocolIeId,
    pub criticality: Criticality,
    pub value: RrcInactiveTransitionReportprotocolIEsItemvalue,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "0", sz_ub = "65535")]
pub struct RrcInactiveTransitionReportprotocolIEs(Vec<RrcInactiveTransitionReportprotocolIEsItem>);

#[derive(Debug, AperCodec)]
#[asn(type = "INTEGER", lb = "0", ub = "4095")]
pub struct Integer74(u16);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct RecommendedCellItemiEExtensionsItem {}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "65535")]
pub struct RecommendedCellItemiEExtensions(Vec<RecommendedCellItemiEExtensionsItem>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct RecommendedCellsForPagingiEExtensionsItem {}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "65535")]
pub struct RecommendedCellsForPagingiEExtensions(Vec<RecommendedCellsForPagingiEExtensionsItem>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct RecommendedRanNodeItemiEExtensionsItem {}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "65535")]
pub struct RecommendedRanNodeItemiEExtensions(Vec<RecommendedRanNodeItemiEExtensionsItem>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct RecommendedRanNodesForPagingiEExtensionsItem {}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "65535")]
pub struct RecommendedRanNodesForPagingiEExtensions(Vec<RecommendedRanNodesForPagingiEExtensionsItem>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct RedundantPduSessionInformationiEExtensionsItem {}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "65535")]
pub struct RedundantPduSessionInformationiEExtensions(Vec<RedundantPduSessionInformationiEExtensionsItem>);

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum RerouteNasRequestprotocolIEsItemvalue {
    #[asn(key = 10)]
    IdAmfUeNgapId(AmfUeNgapId),
    #[asn(key = 3)]
    IdAmfSetId(AmfSetId),
    #[asn(key = 0)]
    IdAllowedNssai(AllowedNssai),
    #[asn(key = 85)]
    IdRanUeNgapId(RanUeNgapId),
    #[asn(key = 171)]
    IdSourceToTargetAmfInformationReroute(SourceToTargetAmfInformationReroute),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct RerouteNasRequestprotocolIEsItem {
    #[asn(key_field = true)]
    pub id: ProtocolIeId,
    pub criticality: Criticality,
    pub value: RerouteNasRequestprotocolIEsItemvalue,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "0", sz_ub = "65535")]
pub struct RerouteNasRequestprotocolIEs(Vec<RerouteNasRequestprotocolIEsItem>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct ResetTypechoiceExtensions {}

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum RetrieveUeInformationprotocolIEsItemvalue {
    #[asn(key = 26)]
    IdFiveGSTmsi(FiveGSTmsi),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct RetrieveUeInformationprotocolIEsItem {
    #[asn(key_field = true)]
    pub id: ProtocolIeId,
    pub criticality: Criticality,
    pub value: RetrieveUeInformationprotocolIEsItemvalue,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "0", sz_ub = "65535")]
pub struct RetrieveUeInformationprotocolIEs(Vec<RetrieveUeInformationprotocolIEsItem>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct SNssaIiEExtensionsItem {}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "65535")]
pub struct SNssaIiEExtensions(Vec<SNssaIiEExtensionsItem>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct SnpnMobilityInformationiEExtensionsItem {}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "65535")]
pub struct SnpnMobilityInformationiEExtensions(Vec<SnpnMobilityInformationiEExtensionsItem>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct SonConfigurationTransferiEExtensionsItem {}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "65535")]
pub struct SonConfigurationTransferiEExtensions(Vec<SonConfigurationTransferiEExtensionsItem>);

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum SonInformationchoiceExtensionsvalue {
    #[asn(key = 252)]
    IdSonInformationReport(SonInformationReport),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct SonInformationchoiceExtensions {
    #[asn(key_field = true)]
    pub id: ProtocolIeId,
    pub criticality: Criticality,
    pub value: SonInformationchoiceExtensionsvalue,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct SonInformationReplyiEExtensionsItem {}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "65535")]
pub struct SonInformationReplyiEExtensions(Vec<SonInformationReplyiEExtensionsItem>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct SonInformationReportchoiceExtensions {}

#[derive(Debug, AperCodec)]
#[asn(type = "BITSTRING", sz_extensible = false, sz_lb = "7", sz_ub = "7")]
pub struct BitString75(BitVec<Msb0, u8>);

#[derive(Debug, AperCodec)]
#[asn(type = "INTEGER", lb = "0", ub = "86399", extensible = true)]
pub struct Integer76(u32);

#[derive(Debug, AperCodec)]
#[asn(type = "INTEGER", lb = "0", ub = "86399", extensible = true)]
pub struct Integer77(u32);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct ScheduledCommunicationTimeiEExtensionsItem {}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "65535")]
pub struct ScheduledCommunicationTimeiEExtensions(Vec<ScheduledCommunicationTimeiEExtensionsItem>);

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum SecondaryRatDataUsageReportprotocolIEsItemvalue {
    #[asn(key = 10)]
    IdAmfUeNgapId(AmfUeNgapId),
    #[asn(key = 143)]
    IdHandoverFlag(HandoverFlag),
    #[asn(key = 142)]
    IdPduSessionResourceSecondaryRatUsageList(PduSessionResourceSecondaryRatUsageList),
    #[asn(key = 85)]
    IdRanUeNgapId(RanUeNgapId),
    #[asn(key = 121)]
    IdUserLocationInformation(UserLocationInformation),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct SecondaryRatDataUsageReportprotocolIEsItem {
    #[asn(key_field = true)]
    pub id: ProtocolIeId,
    pub criticality: Criticality,
    pub value: SecondaryRatDataUsageReportprotocolIEsItemvalue,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "0", sz_ub = "65535")]
pub struct SecondaryRatDataUsageReportprotocolIEs(Vec<SecondaryRatDataUsageReportprotocolIEsItem>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct SecondaryRatDataUsageReportTransferiEExtensionsItem {}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "65535")]
pub struct SecondaryRatDataUsageReportTransferiEExtensions(Vec<SecondaryRatDataUsageReportTransferiEExtensionsItem>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct SecondaryRatUsageInformationiEExtensionItem {}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "65535")]
pub struct SecondaryRatUsageInformationiEExtension(Vec<SecondaryRatUsageInformationiEExtensionItem>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct SecurityContextiEExtensionsItem {}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "65535")]
pub struct SecurityContextiEExtensions(Vec<SecurityContextiEExtensionsItem>);

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum SecurityIndicationiEExtensionsItemextensionValue {
    #[asn(key = 151)]
    IdMaximumIntegrityProtectedDataRateDl(MaximumIntegrityProtectedDataRate),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct SecurityIndicationiEExtensionsItem {
    #[asn(key_field = true)]
    pub id: ProtocolExtensionId,
    pub criticality: Criticality,
    pub extension_value: SecurityIndicationiEExtensionsItemextensionValue,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "65535")]
pub struct SecurityIndicationiEExtensions(Vec<SecurityIndicationiEExtensionsItem>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct SecurityResultiEExtensionsItem {}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "65535")]
pub struct SecurityResultiEExtensions(Vec<SecurityResultiEExtensionsItem>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct SensorMeasConfigNameItemiEExtensionsItem {}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "65535")]
pub struct SensorMeasConfigNameItemiEExtensions(Vec<SensorMeasConfigNameItemiEExtensionsItem>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct SensorMeasurementConfigurationiEExtensionsItem {}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "65535")]
pub struct SensorMeasurementConfigurationiEExtensions(Vec<SensorMeasurementConfigurationiEExtensionsItem>);

#[derive(Debug, AperCodec)]
#[asn(type = "ENUMERATED", extensible = true, lb = "0", ub = "0")]
pub struct Enumerated78(u8);
impl Enumerated78 {
    const TRUE: u8 = 0u8;
}

#[derive(Debug, AperCodec)]
#[asn(type = "ENUMERATED", extensible = true, lb = "0", ub = "0")]
pub struct Enumerated79(u8);
impl Enumerated79 {
    const TRUE: u8 = 0u8;
}

#[derive(Debug, AperCodec)]
#[asn(type = "ENUMERATED", extensible = true, lb = "0", ub = "0")]
pub struct Enumerated80(u8);
impl Enumerated80 {
    const TRUE: u8 = 0u8;
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct SensorNameConfigchoiceExtensions {}

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum ServedGuamiItemiEExtensionsItemextensionValue {
    #[asn(key = 176)]
    IdGuamiType(GuamiType),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct ServedGuamiItemiEExtensionsItem {
    #[asn(key_field = true)]
    pub id: ProtocolExtensionId,
    pub criticality: Criticality,
    pub extension_value: ServedGuamiItemiEExtensionsItemextensionValue,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "65535")]
pub struct ServedGuamiItemiEExtensions(Vec<ServedGuamiItemiEExtensionsItem>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct ServiceAreaInformationItemiEExtensionsItem {}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "65535")]
pub struct ServiceAreaInformationItemiEExtensions(Vec<ServiceAreaInformationItemiEExtensionsItem>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct SliceOverloadItemiEExtensionsItem {}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "65535")]
pub struct SliceOverloadItemiEExtensions(Vec<SliceOverloadItemiEExtensionsItem>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct SliceSupportItemiEExtensionsItem {}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "65535")]
pub struct SliceSupportItemiEExtensions(Vec<SliceSupportItemiEExtensionsItem>);

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum SourceNgranNodeToTargetNgranNodeTransparentContaineriEExtensionsItemextensionValue {
    #[asn(key = 182)]
    IdSgNbUeX2apId(SgNbUeX2apId),
    #[asn(key = 253)]
    IdUeHistoryInformationFromTheUe(UeHistoryInformationFromTheUe),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct SourceNgranNodeToTargetNgranNodeTransparentContaineriEExtensionsItem {
    #[asn(key_field = true)]
    pub id: ProtocolExtensionId,
    pub criticality: Criticality,
    pub extension_value:
        SourceNgranNodeToTargetNgranNodeTransparentContaineriEExtensionsItemextensionValue,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "65535")]
pub struct SourceNgranNodeToTargetNgranNodeTransparentContaineriEExtensions(Vec<SourceNgranNodeToTargetNgranNodeTransparentContaineriEExtensionsItem>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct SourceRanNodeIDiEExtensionsItem {}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "65535")]
pub struct SourceRanNodeIDiEExtensions(Vec<SourceRanNodeIDiEExtensionsItem>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct SourceToTargetAmfInformationRerouteiEExtensionsItem {}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "65535")]
pub struct SourceToTargetAmfInformationRerouteiEExtensions(Vec<SourceToTargetAmfInformationRerouteiEExtensionsItem>);

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum SuccessfulOutcomevalue {
    #[asn(key = 0)]
    IdAmfConfigurationUpdate(AmfConfigurationUpdateAcknowledge),
    #[asn(key = 10)]
    IdHandoverCancel(HandoverCancelAcknowledge),
    #[asn(key = 12)]
    IdHandoverPreparation(HandoverCommand),
    #[asn(key = 13)]
    IdHandoverResourceAllocation(HandoverRequestAcknowledge),
    #[asn(key = 14)]
    IdInitialContextSetup(InitialContextSetupResponse),
    #[asn(key = 20)]
    IdNgReset(NgResetAcknowledge),
    #[asn(key = 21)]
    IdNgSetup(NgSetupResponse),
    #[asn(key = 26)]
    IdPduSessionResourceModify(PduSessionResourceModifyResponse),
    #[asn(key = 27)]
    IdPduSessionResourceModifyIndication(PduSessionResourceModifyConfirm),
    #[asn(key = 28)]
    IdPduSessionResourceRelease(PduSessionResourceReleaseResponse),
    #[asn(key = 29)]
    IdPduSessionResourceSetup(PduSessionResourceSetupResponse),
    #[asn(key = 32)]
    IdPwsCancel(PwsCancelResponse),
    #[asn(key = 25)]
    IdPathSwitchRequest(PathSwitchRequestAcknowledge),
    #[asn(key = 35)]
    IdRanConfigurationUpdate(RanConfigurationUpdateAcknowledge),
    #[asn(key = 40)]
    IdUeContextModification(UeContextModificationResponse),
    #[asn(key = 41)]
    IdUeContextRelease(UeContextReleaseComplete),
    #[asn(key = 58)]
    IdUeContextResume(UeContextResumeResponse),
    #[asn(key = 59)]
    IdUeContextSuspend(UeContextSuspendResponse),
    #[asn(key = 43)]
    IdUeRadioCapabilityCheck(UeRadioCapabilityCheckResponse),
    #[asn(key = 60)]
    IdUeRadioCapabilityIdMapping(UeRadioCapabilityIdMappingResponse),
    #[asn(key = 51)]
    IdWriteReplaceWarning(WriteReplaceWarningResponse),
}

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum SupportedTaItemiEExtensionsItemextensionValue {
    #[asn(key = 272)]
    IdConfiguredTacIndication(ConfiguredTacIndication),
    #[asn(key = 179)]
    IdRatInformation(RatInformation),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct SupportedTaItemiEExtensionsItem {
    #[asn(key_field = true)]
    pub id: ProtocolExtensionId,
    pub criticality: Criticality,
    pub extension_value: SupportedTaItemiEExtensionsItemextensionValue,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "65535")]
pub struct SupportedTaItemiEExtensions(Vec<SupportedTaItemiEExtensionsItem>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct TaBasedMdTiEExtensionsItem {}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "65535")]
pub struct TaBasedMdTiEExtensions(Vec<TaBasedMdTiEExtensionsItem>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct TaIiEExtensionsItem {}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "65535")]
pub struct TaIiEExtensions(Vec<TaIiEExtensionsItem>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct TaiBasedMdTiEExtensionsItem {}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "65535")]
pub struct TaiBasedMdTiEExtensions(Vec<TaiBasedMdTiEExtensionsItem>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct TaiBroadcastEutraItemiEExtensionsItem {}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "65535")]
pub struct TaiBroadcastEutraItemiEExtensions(Vec<TaiBroadcastEutraItemiEExtensionsItem>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct TaiBroadcastNrItemiEExtensionsItem {}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "65535")]
pub struct TaiBroadcastNrItemiEExtensions(Vec<TaiBroadcastNrItemiEExtensionsItem>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct TaiCancelledEutraItemiEExtensionsItem {}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "65535")]
pub struct TaiCancelledEutraItemiEExtensions(Vec<TaiCancelledEutraItemiEExtensionsItem>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct TaiCancelledNrItemiEExtensionsItem {}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "65535")]
pub struct TaiCancelledNrItemiEExtensions(Vec<TaiCancelledNrItemiEExtensionsItem>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct TaiListForInactiveItemiEExtensionsItem {}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "65535")]
pub struct TaiListForInactiveItemiEExtensions(Vec<TaiListForInactiveItemiEExtensionsItem>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct TaiListForPagingItemiEExtensionsItem {}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "65535")]
pub struct TaiListForPagingItemiEExtensions(Vec<TaiListForPagingItemiEExtensionsItem>);

#[derive(Debug, AperCodec)]
#[asn(type = "BITSTRING", sz_extensible = true, sz_lb = "32", sz_ub = "32")]
pub struct BitString81(BitVec<Msb0, u8>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct TngfIDchoiceExtensions {}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct TnlAssociationItemiEExtensionsItem {}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "65535")]
pub struct TnlAssociationItemiEExtensions(Vec<TnlAssociationItemiEExtensionsItem>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct TscAssistanceInformationiEExtensionsItem {}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "65535")]
pub struct TscAssistanceInformationiEExtensions(Vec<TscAssistanceInformationiEExtensionsItem>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct TscTrafficCharacteristicsiEExtensionsItem {}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "65535")]
pub struct TscTrafficCharacteristicsiEExtensions(Vec<TscTrafficCharacteristicsiEExtensionsItem>);

#[derive(Debug, AperCodec)]
#[asn(type = "BITSTRING", sz_extensible = true, sz_lb = "32", sz_ub = "32")]
pub struct BitString82(BitVec<Msb0, u8>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct TwifIDchoiceExtensions {}

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum TargetIDchoiceExtensionsvalue {
    #[asn(key = 178)]
    IdTargetRncId(TargetRncId),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct TargetIDchoiceExtensions {
    #[asn(key_field = true)]
    pub id: ProtocolIeId,
    pub criticality: Criticality,
    pub value: TargetIDchoiceExtensionsvalue,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct TargetNgranNodeToSourceNgranNodeFailureTransparentContaineriEExtensionsItem {}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "65535")]
pub struct TargetNgranNodeToSourceNgranNodeFailureTransparentContaineriEExtensions(Vec<TargetNgranNodeToSourceNgranNodeFailureTransparentContaineriEExtensionsItem>);

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum TargetNgranNodeToSourceNgranNodeTransparentContaineriEExtensionsItemextensionValue {
    #[asn(key = 267)]
    IdDapsResponseInfoList(DapsResponseInfoList),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct TargetNgranNodeToSourceNgranNodeTransparentContaineriEExtensionsItem {
    #[asn(key_field = true)]
    pub id: ProtocolExtensionId,
    pub criticality: Criticality,
    pub extension_value:
        TargetNgranNodeToSourceNgranNodeTransparentContaineriEExtensionsItemextensionValue,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "65535")]
pub struct TargetNgranNodeToSourceNgranNodeTransparentContaineriEExtensions(Vec<TargetNgranNodeToSourceNgranNodeTransparentContaineriEExtensionsItem>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct TargetRanNodeIDiEExtensionsItem {}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "65535")]
pub struct TargetRanNodeIDiEExtensions(Vec<TargetRanNodeIDiEExtensionsItem>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct TargetRncIDiEExtensionsItem {}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "65535")]
pub struct TargetRncIDiEExtensions(Vec<TargetRncIDiEExtensionsItem>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct TargeteNbIDiEExtensionsItem {}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "65535")]
pub struct TargeteNbIDiEExtensions(Vec<TargeteNbIDiEExtensionsItem>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct TooearlyIntersystemHOiEExtensionsItem {}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "65535")]
pub struct TooearlyIntersystemHOiEExtensions(Vec<TooearlyIntersystemHOiEExtensionsItem>);

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum TraceActivationiEExtensionsItemextensionValue {
    #[asn(key = 255)]
    IdMdtConfiguration(MdtConfiguration),
    #[asn(key = 257)]
    IdTraceCollectionEntityUri(UriAddress),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct TraceActivationiEExtensionsItem {
    #[asn(key_field = true)]
    pub id: ProtocolExtensionId,
    pub criticality: Criticality,
    pub extension_value: TraceActivationiEExtensionsItemextensionValue,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "65535")]
pub struct TraceActivationiEExtensions(Vec<TraceActivationiEExtensionsItem>);

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum TraceFailureIndicationprotocolIEsItemvalue {
    #[asn(key = 10)]
    IdAmfUeNgapId(AmfUeNgapId),
    #[asn(key = 15)]
    IdCause(Cause),
    #[asn(key = 44)]
    IdNgranTraceId(NgranTraceId),
    #[asn(key = 85)]
    IdRanUeNgapId(RanUeNgapId),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct TraceFailureIndicationprotocolIEsItem {
    #[asn(key_field = true)]
    pub id: ProtocolIeId,
    pub criticality: Criticality,
    pub value: TraceFailureIndicationprotocolIEsItemvalue,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "0", sz_ub = "65535")]
pub struct TraceFailureIndicationprotocolIEs(Vec<TraceFailureIndicationprotocolIEsItem>);

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum TraceStartprotocolIEsItemvalue {
    #[asn(key = 10)]
    IdAmfUeNgapId(AmfUeNgapId),
    #[asn(key = 85)]
    IdRanUeNgapId(RanUeNgapId),
    #[asn(key = 108)]
    IdTraceActivation(TraceActivation),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct TraceStartprotocolIEsItem {
    #[asn(key_field = true)]
    pub id: ProtocolIeId,
    pub criticality: Criticality,
    pub value: TraceStartprotocolIEsItemvalue,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "0", sz_ub = "65535")]
pub struct TraceStartprotocolIEs(Vec<TraceStartprotocolIEsItem>);

#[derive(Debug, AperCodec)]
#[asn(type = "ENUMERATED", extensible = true, lb = "0", ub = "1")]
pub struct Enumerated83(u8);
impl Enumerated83 {
    const PERIODICALLY: u8 = 0u8;
    const ONDEMAND: u8 = 1u8;
}

#[derive(Debug, AperCodec)]
#[asn(type = "INTEGER", lb = "1", ub = "3600", extensible = true)]
pub struct Integer84(u16);

#[derive(Debug, AperCodec)]
#[asn(type = "ENUMERATED", extensible = true, lb = "0", ub = "1")]
pub struct Enumerated85(u8);
impl Enumerated85 {
    const STATIONARY: u8 = 0u8;
    const MOBILE: u8 = 1u8;
}

#[derive(Debug, AperCodec)]
#[asn(type = "ENUMERATED", extensible = true, lb = "0", ub = "2")]
pub struct Enumerated86(u8);
impl Enumerated86 {
    const SINGLE_PACKET: u8 = 0u8;
    const DUAL_PACKETS: u8 = 1u8;
    const MULTIPLE_PACKETS: u8 = 2u8;
}

#[derive(Debug, AperCodec)]
#[asn(type = "ENUMERATED", extensible = true, lb = "0", ub = "2")]
pub struct Enumerated87(u8);
impl Enumerated87 {
    const BATTERY_POWERED: u8 = 0u8;
    const BATTERY_POWERED_NOT_RECHARGEABLE_OR_REPLACEABLE: u8 = 1u8;
    const NOT_BATTERY_POWERED: u8 = 2u8;
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct UeDifferentiationInfoiEExtensionsItem {}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "65535")]
pub struct UeDifferentiationInfoiEExtensions(Vec<UeDifferentiationInfoiEExtensionsItem>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct UeNgapIdPairiEExtensionsItem {}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "65535")]
pub struct UeNgapIdPairiEExtensions(Vec<UeNgapIdPairiEExtensionsItem>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct UeNgapIDschoiceExtensions {}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct UeAssociatedLogicalNgConnectionItemiEExtensionsItem {}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "65535")]
pub struct UeAssociatedLogicalNgConnectionItemiEExtensions(Vec<UeAssociatedLogicalNgConnectionItemiEExtensionsItem>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct UeAggregateMaximumBitRateiEExtensionsItem {}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "65535")]
pub struct UeAggregateMaximumBitRateiEExtensions(Vec<UeAggregateMaximumBitRateiEExtensionsItem>);

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum UeContextModificationFailureprotocolIEsItemvalue {
    #[asn(key = 10)]
    IdAmfUeNgapId(AmfUeNgapId),
    #[asn(key = 15)]
    IdCause(Cause),
    #[asn(key = 19)]
    IdCriticalityDiagnostics(CriticalityDiagnostics),
    #[asn(key = 85)]
    IdRanUeNgapId(RanUeNgapId),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct UeContextModificationFailureprotocolIEsItem {
    #[asn(key_field = true)]
    pub id: ProtocolIeId,
    pub criticality: Criticality,
    pub value: UeContextModificationFailureprotocolIEsItemvalue,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "0", sz_ub = "65535")]
pub struct UeContextModificationFailureprotocolIEs(Vec<UeContextModificationFailureprotocolIEsItem>);

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum UeContextModificationRequestprotocolIEsItemvalue {
    #[asn(key = 10)]
    IdAmfUeNgapId(AmfUeNgapId),
    #[asn(key = 165)]
    IdCnAssistedRanTuning(CnAssistedRanTuning),
    #[asn(key = 18)]
    IdCoreNetworkAssistanceInformationForInactive(CoreNetworkAssistanceInformationForInactive),
    #[asn(key = 24)]
    IdEmergencyFallbackIndicator(EmergencyFallbackIndicator),
    #[asn(key = 199)]
    IdIabAuthorized(IabAuthorized),
    #[asn(key = 31)]
    IdIndexToRfsp(IndexToRfsp),
    #[asn(key = 217)]
    IdLteueSidelinkAggregateMaximumBitrate(LteueSidelinkAggregateMaximumBitrate),
    #[asn(key = 215)]
    IdLtev2xServicesAuthorized(Ltev2xServicesAuthorized),
    #[asn(key = 218)]
    IdNrueSidelinkAggregateMaximumBitrate(NrueSidelinkAggregateMaximumBitrate),
    #[asn(key = 216)]
    IdNrv2xServicesAuthorized(Nrv2xServicesAuthorized),
    #[asn(key = 40)]
    IdNewAmfUeNgapId(AmfUeNgapId),
    #[asn(key = 162)]
    IdNewGuami(Guami),
    #[asn(key = 219)]
    IdPc5QoSParameters(Pc5QoSParameters),
    #[asn(key = 85)]
    IdRanUeNgapId(RanUeNgapId),
    #[asn(key = 83)]
    IdRanPagingPriority(RanPagingPriority),
    #[asn(key = 238)]
    IdRgLevelWirelineAccessCharacteristics(RgLevelWirelineAccessCharacteristics),
    #[asn(key = 91)]
    IdRrcInactiveTransitionReportRequest(RrcInactiveTransitionReportRequest),
    #[asn(key = 177)]
    IdSrvccOperationPossible(SrvccOperationPossible),
    #[asn(key = 94)]
    IdSecurityKey(SecurityKey),
    #[asn(key = 110)]
    IdUeAggregateMaximumBitRate(UeAggregateMaximumBitRate),
    #[asn(key = 264)]
    IdUeRadioCapabilityId(UeRadioCapabilityId),
    #[asn(key = 119)]
    IdUeSecurityCapabilities(UeSecurityCapabilities),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct UeContextModificationRequestprotocolIEsItem {
    #[asn(key_field = true)]
    pub id: ProtocolIeId,
    pub criticality: Criticality,
    pub value: UeContextModificationRequestprotocolIEsItemvalue,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "0", sz_ub = "65535")]
pub struct UeContextModificationRequestprotocolIEs(Vec<UeContextModificationRequestprotocolIEsItem>);

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum UeContextModificationResponseprotocolIEsItemvalue {
    #[asn(key = 10)]
    IdAmfUeNgapId(AmfUeNgapId),
    #[asn(key = 19)]
    IdCriticalityDiagnostics(CriticalityDiagnostics),
    #[asn(key = 85)]
    IdRanUeNgapId(RanUeNgapId),
    #[asn(key = 92)]
    IdRrcState(RrcState),
    #[asn(key = 121)]
    IdUserLocationInformation(UserLocationInformation),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct UeContextModificationResponseprotocolIEsItem {
    #[asn(key_field = true)]
    pub id: ProtocolIeId,
    pub criticality: Criticality,
    pub value: UeContextModificationResponseprotocolIEsItemvalue,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "0", sz_ub = "65535")]
pub struct UeContextModificationResponseprotocolIEs(Vec<UeContextModificationResponseprotocolIEsItem>);

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum UeContextReleaseCommandprotocolIEsItemvalue {
    #[asn(key = 15)]
    IdCause(Cause),
    #[asn(key = 114)]
    IdUeNgapIDs(UeNgapIDs),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct UeContextReleaseCommandprotocolIEsItem {
    #[asn(key_field = true)]
    pub id: ProtocolIeId,
    pub criticality: Criticality,
    pub value: UeContextReleaseCommandprotocolIEsItemvalue,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "0", sz_ub = "65535")]
pub struct UeContextReleaseCommandprotocolIEs(Vec<UeContextReleaseCommandprotocolIEsItem>);

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum UeContextReleaseCompleteprotocolIEsItemvalue {
    #[asn(key = 10)]
    IdAmfUeNgapId(AmfUeNgapId),
    #[asn(key = 19)]
    IdCriticalityDiagnostics(CriticalityDiagnostics),
    #[asn(key = 32)]
    IdInfoOnRecommendedCellsAndRanNodesForPaging(InfoOnRecommendedCellsAndRanNodesForPaging),
    #[asn(key = 60)]
    IdPduSessionResourceListCxtRelCpl(PduSessionResourceListCxtRelCpl),
    #[asn(key = 207)]
    IdPagingAssisDataforCEcapabUe(PagingAssisDataforCEcapabUe),
    #[asn(key = 85)]
    IdRanUeNgapId(RanUeNgapId),
    #[asn(key = 121)]
    IdUserLocationInformation(UserLocationInformation),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct UeContextReleaseCompleteprotocolIEsItem {
    #[asn(key_field = true)]
    pub id: ProtocolIeId,
    pub criticality: Criticality,
    pub value: UeContextReleaseCompleteprotocolIEsItemvalue,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "0", sz_ub = "65535")]
pub struct UeContextReleaseCompleteprotocolIEs(Vec<UeContextReleaseCompleteprotocolIEsItem>);

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum UeContextReleaseRequestprotocolIEsItemvalue {
    #[asn(key = 10)]
    IdAmfUeNgapId(AmfUeNgapId),
    #[asn(key = 15)]
    IdCause(Cause),
    #[asn(key = 133)]
    IdPduSessionResourceListCxtRelReq(PduSessionResourceListCxtRelReq),
    #[asn(key = 85)]
    IdRanUeNgapId(RanUeNgapId),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct UeContextReleaseRequestprotocolIEsItem {
    #[asn(key_field = true)]
    pub id: ProtocolIeId,
    pub criticality: Criticality,
    pub value: UeContextReleaseRequestprotocolIEsItemvalue,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "0", sz_ub = "65535")]
pub struct UeContextReleaseRequestprotocolIEs(Vec<UeContextReleaseRequestprotocolIEsItem>);

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum UeContextResumeFailureprotocolIEsItemvalue {
    #[asn(key = 10)]
    IdAmfUeNgapId(AmfUeNgapId),
    #[asn(key = 15)]
    IdCause(Cause),
    #[asn(key = 19)]
    IdCriticalityDiagnostics(CriticalityDiagnostics),
    #[asn(key = 85)]
    IdRanUeNgapId(RanUeNgapId),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct UeContextResumeFailureprotocolIEsItem {
    #[asn(key_field = true)]
    pub id: ProtocolIeId,
    pub criticality: Criticality,
    pub value: UeContextResumeFailureprotocolIEsItemvalue,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "0", sz_ub = "65535")]
pub struct UeContextResumeFailureprotocolIEs(Vec<UeContextResumeFailureprotocolIEsItem>);

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum UeContextResumeRequestprotocolIEsItemvalue {
    #[asn(key = 10)]
    IdAmfUeNgapId(AmfUeNgapId),
    #[asn(key = 32)]
    IdInfoOnRecommendedCellsAndRanNodesForPaging(InfoOnRecommendedCellsAndRanNodesForPaging),
    #[asn(key = 229)]
    IdPduSessionResourceFailedToResumeListResReq(PduSessionResourceFailedToResumeListResReq),
    #[asn(key = 232)]
    IdPduSessionResourceResumeListResReq(PduSessionResourceResumeListResReq),
    #[asn(key = 207)]
    IdPagingAssisDataforCEcapabUe(PagingAssisDataforCEcapabUe),
    #[asn(key = 85)]
    IdRanUeNgapId(RanUeNgapId),
    #[asn(key = 237)]
    IdRrcResumeCause(RrcEstablishmentCause),
    #[asn(key = 235)]
    IdSuspendRequestIndication(SuspendRequestIndication),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct UeContextResumeRequestprotocolIEsItem {
    #[asn(key_field = true)]
    pub id: ProtocolIeId,
    pub criticality: Criticality,
    pub value: UeContextResumeRequestprotocolIEsItemvalue,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "0", sz_ub = "65535")]
pub struct UeContextResumeRequestprotocolIEs(Vec<UeContextResumeRequestprotocolIEsItem>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct UeContextResumeRequestTransferiEExtensionsItem {}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "65535")]
pub struct UeContextResumeRequestTransferiEExtensions(Vec<UeContextResumeRequestTransferiEExtensionsItem>);

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum UeContextResumeResponseprotocolIEsItemvalue {
    #[asn(key = 10)]
    IdAmfUeNgapId(AmfUeNgapId),
    #[asn(key = 19)]
    IdCriticalityDiagnostics(CriticalityDiagnostics),
    #[asn(key = 206)]
    IdExtendedConnectedTime(ExtendedConnectedTime),
    #[asn(key = 230)]
    IdPduSessionResourceFailedToResumeListResRes(PduSessionResourceFailedToResumeListResRes),
    #[asn(key = 233)]
    IdPduSessionResourceResumeListResRes(PduSessionResourceResumeListResRes),
    #[asn(key = 85)]
    IdRanUeNgapId(RanUeNgapId),
    #[asn(key = 93)]
    IdSecurityContext(SecurityContext),
    #[asn(key = 236)]
    IdSuspendResponseIndication(SuspendResponseIndication),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct UeContextResumeResponseprotocolIEsItem {
    #[asn(key_field = true)]
    pub id: ProtocolIeId,
    pub criticality: Criticality,
    pub value: UeContextResumeResponseprotocolIEsItemvalue,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "0", sz_ub = "65535")]
pub struct UeContextResumeResponseprotocolIEs(Vec<UeContextResumeResponseprotocolIEsItem>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct UeContextResumeResponseTransferiEExtensionsItem {}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "65535")]
pub struct UeContextResumeResponseTransferiEExtensions(Vec<UeContextResumeResponseTransferiEExtensionsItem>);

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum UeContextSuspendFailureprotocolIEsItemvalue {
    #[asn(key = 10)]
    IdAmfUeNgapId(AmfUeNgapId),
    #[asn(key = 15)]
    IdCause(Cause),
    #[asn(key = 19)]
    IdCriticalityDiagnostics(CriticalityDiagnostics),
    #[asn(key = 85)]
    IdRanUeNgapId(RanUeNgapId),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct UeContextSuspendFailureprotocolIEsItem {
    #[asn(key_field = true)]
    pub id: ProtocolIeId,
    pub criticality: Criticality,
    pub value: UeContextSuspendFailureprotocolIEsItemvalue,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "0", sz_ub = "65535")]
pub struct UeContextSuspendFailureprotocolIEs(Vec<UeContextSuspendFailureprotocolIEsItem>);

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum UeContextSuspendRequestprotocolIEsItemvalue {
    #[asn(key = 10)]
    IdAmfUeNgapId(AmfUeNgapId),
    #[asn(key = 32)]
    IdInfoOnRecommendedCellsAndRanNodesForPaging(InfoOnRecommendedCellsAndRanNodesForPaging),
    #[asn(key = 231)]
    IdPduSessionResourceSuspendListSusReq(PduSessionResourceSuspendListSusReq),
    #[asn(key = 207)]
    IdPagingAssisDataforCEcapabUe(PagingAssisDataforCEcapabUe),
    #[asn(key = 85)]
    IdRanUeNgapId(RanUeNgapId),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct UeContextSuspendRequestprotocolIEsItem {
    #[asn(key_field = true)]
    pub id: ProtocolIeId,
    pub criticality: Criticality,
    pub value: UeContextSuspendRequestprotocolIEsItemvalue,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "0", sz_ub = "65535")]
pub struct UeContextSuspendRequestprotocolIEs(Vec<UeContextSuspendRequestprotocolIEsItem>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct UeContextSuspendRequestTransferiEExtensionsItem {}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "65535")]
pub struct UeContextSuspendRequestTransferiEExtensions(Vec<UeContextSuspendRequestTransferiEExtensionsItem>);

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum UeContextSuspendResponseprotocolIEsItemvalue {
    #[asn(key = 10)]
    IdAmfUeNgapId(AmfUeNgapId),
    #[asn(key = 19)]
    IdCriticalityDiagnostics(CriticalityDiagnostics),
    #[asn(key = 85)]
    IdRanUeNgapId(RanUeNgapId),
    #[asn(key = 93)]
    IdSecurityContext(SecurityContext),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct UeContextSuspendResponseprotocolIEsItem {
    #[asn(key_field = true)]
    pub id: ProtocolIeId,
    pub criticality: Criticality,
    pub value: UeContextSuspendResponseprotocolIEsItemvalue,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "0", sz_ub = "65535")]
pub struct UeContextSuspendResponseprotocolIEs(Vec<UeContextSuspendResponseprotocolIEsItem>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct UeHistoryInformationFromTheUEchoiceExtensions {}

#[derive(Debug, AperCodec)]
#[asn(type = "BITSTRING", sz_extensible = false, sz_lb = "10", sz_ub = "10")]
pub struct BitString88(BitVec<Msb0, u8>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct UeIdentityIndexValuechoiceExtensions {}

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum UeInformationTransferprotocolIEsItemvalue {
    #[asn(key = 0)]
    IdAllowedNssai(AllowedNssai),
    #[asn(key = 26)]
    IdFiveGSTmsi(FiveGSTmsi),
    #[asn(key = 210)]
    IdNbIoTUePriority(NbIoTUePriority),
    #[asn(key = 148)]
    IdSNssai(SNssai),
    #[asn(key = 209)]
    IdUeDifferentiationInfo(UeDifferentiationInfo),
    #[asn(key = 117)]
    IdUeRadioCapability(UeRadioCapability),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct UeInformationTransferprotocolIEsItem {
    #[asn(key_field = true)]
    pub id: ProtocolIeId,
    pub criticality: Criticality,
    pub value: UeInformationTransferprotocolIEsItemvalue,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "0", sz_ub = "65535")]
pub struct UeInformationTransferprotocolIEs(Vec<UeInformationTransferprotocolIEsItem>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct UePagingIdentitychoiceExtensions {}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct UePresenceInAreaOfInterestItemiEExtensionsItem {}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "65535")]
pub struct UePresenceInAreaOfInterestItemiEExtensions(Vec<UePresenceInAreaOfInterestItemiEExtensionsItem>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct UerlfReportContainerchoiceExtensions {}

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum UeRadioCapabilityCheckRequestprotocolIEsItemvalue {
    #[asn(key = 10)]
    IdAmfUeNgapId(AmfUeNgapId),
    #[asn(key = 85)]
    IdRanUeNgapId(RanUeNgapId),
    #[asn(key = 117)]
    IdUeRadioCapability(UeRadioCapability),
    #[asn(key = 264)]
    IdUeRadioCapabilityId(UeRadioCapabilityId),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct UeRadioCapabilityCheckRequestprotocolIEsItem {
    #[asn(key_field = true)]
    pub id: ProtocolIeId,
    pub criticality: Criticality,
    pub value: UeRadioCapabilityCheckRequestprotocolIEsItemvalue,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "0", sz_ub = "65535")]
pub struct UeRadioCapabilityCheckRequestprotocolIEs(Vec<UeRadioCapabilityCheckRequestprotocolIEsItem>);

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum UeRadioCapabilityCheckResponseprotocolIEsItemvalue {
    #[asn(key = 10)]
    IdAmfUeNgapId(AmfUeNgapId),
    #[asn(key = 19)]
    IdCriticalityDiagnostics(CriticalityDiagnostics),
    #[asn(key = 30)]
    IdImsVoiceSupportIndicator(ImsVoiceSupportIndicator),
    #[asn(key = 85)]
    IdRanUeNgapId(RanUeNgapId),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct UeRadioCapabilityCheckResponseprotocolIEsItem {
    #[asn(key_field = true)]
    pub id: ProtocolIeId,
    pub criticality: Criticality,
    pub value: UeRadioCapabilityCheckResponseprotocolIEsItemvalue,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "0", sz_ub = "65535")]
pub struct UeRadioCapabilityCheckResponseprotocolIEs(Vec<UeRadioCapabilityCheckResponseprotocolIEsItem>);

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum UeRadioCapabilityForPagingiEExtensionsItemextensionValue {
    #[asn(key = 214)]
    IdUeRadioCapabilityForPagingOfNbIoT(UeRadioCapabilityForPagingOfNbIoT),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct UeRadioCapabilityForPagingiEExtensionsItem {
    #[asn(key_field = true)]
    pub id: ProtocolExtensionId,
    pub criticality: Criticality,
    pub extension_value: UeRadioCapabilityForPagingiEExtensionsItemextensionValue,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "65535")]
pub struct UeRadioCapabilityForPagingiEExtensions(Vec<UeRadioCapabilityForPagingiEExtensionsItem>);

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum UeRadioCapabilityIdMappingRequestprotocolIEsItemvalue {
    #[asn(key = 264)]
    IdUeRadioCapabilityId(UeRadioCapabilityId),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct UeRadioCapabilityIdMappingRequestprotocolIEsItem {
    #[asn(key_field = true)]
    pub id: ProtocolIeId,
    pub criticality: Criticality,
    pub value: UeRadioCapabilityIdMappingRequestprotocolIEsItemvalue,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "0", sz_ub = "65535")]
pub struct UeRadioCapabilityIdMappingRequestprotocolIEs(Vec<UeRadioCapabilityIdMappingRequestprotocolIEsItem>);

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum UeRadioCapabilityIdMappingResponseprotocolIEsItemvalue {
    #[asn(key = 19)]
    IdCriticalityDiagnostics(CriticalityDiagnostics),
    #[asn(key = 117)]
    IdUeRadioCapability(UeRadioCapability),
    #[asn(key = 264)]
    IdUeRadioCapabilityId(UeRadioCapabilityId),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct UeRadioCapabilityIdMappingResponseprotocolIEsItem {
    #[asn(key_field = true)]
    pub id: ProtocolIeId,
    pub criticality: Criticality,
    pub value: UeRadioCapabilityIdMappingResponseprotocolIEsItemvalue,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "0", sz_ub = "65535")]
pub struct UeRadioCapabilityIdMappingResponseprotocolIEs(Vec<UeRadioCapabilityIdMappingResponseprotocolIEsItem>);

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum UeRadioCapabilityInfoIndicationprotocolIEsItemvalue {
    #[asn(key = 10)]
    IdAmfUeNgapId(AmfUeNgapId),
    #[asn(key = 85)]
    IdRanUeNgapId(RanUeNgapId),
    #[asn(key = 117)]
    IdUeRadioCapability(UeRadioCapability),
    #[asn(key = 265)]
    IdUeRadioCapabilityEutraFormat(UeRadioCapability),
    #[asn(key = 118)]
    IdUeRadioCapabilityForPaging(UeRadioCapabilityForPaging),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct UeRadioCapabilityInfoIndicationprotocolIEsItem {
    #[asn(key_field = true)]
    pub id: ProtocolIeId,
    pub criticality: Criticality,
    pub value: UeRadioCapabilityInfoIndicationprotocolIEsItemvalue,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "0", sz_ub = "65535")]
pub struct UeRadioCapabilityInfoIndicationprotocolIEs(Vec<UeRadioCapabilityInfoIndicationprotocolIEsItem>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct UeSecurityCapabilitiesiEExtensionsItem {}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "65535")]
pub struct UeSecurityCapabilitiesiEExtensions(Vec<UeSecurityCapabilitiesiEExtensionsItem>);

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum UetnlaBindingReleaseRequestprotocolIEsItemvalue {
    #[asn(key = 10)]
    IdAmfUeNgapId(AmfUeNgapId),
    #[asn(key = 85)]
    IdRanUeNgapId(RanUeNgapId),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct UetnlaBindingReleaseRequestprotocolIEsItem {
    #[asn(key_field = true)]
    pub id: ProtocolIeId,
    pub criticality: Criticality,
    pub value: UetnlaBindingReleaseRequestprotocolIEsItemvalue,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "0", sz_ub = "65535")]
pub struct UetnlaBindingReleaseRequestprotocolIEs(Vec<UetnlaBindingReleaseRequestprotocolIEsItem>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct UlCpSecurityInformationiEExtensionsItem {}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "65535")]
pub struct UlCpSecurityInformationiEExtensions(Vec<UlCpSecurityInformationiEExtensionsItem>);

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum UlNguUpTnlModifyItemiEExtensionsItemextensionValue {
    #[asn(key = 192)]
    IdRedundantDlNguUpTnlInformation(UpTransportLayerInformation),
    #[asn(key = 195)]
    IdRedundantUlNguUpTnlInformation(UpTransportLayerInformation),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct UlNguUpTnlModifyItemiEExtensionsItem {
    #[asn(key_field = true)]
    pub id: ProtocolExtensionId,
    pub criticality: Criticality,
    pub extension_value: UlNguUpTnlModifyItemiEExtensionsItemextensionValue,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "65535")]
pub struct UlNguUpTnlModifyItemiEExtensions(Vec<UlNguUpTnlModifyItemiEExtensionsItem>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct UpTransportLayerInformationchoiceExtensions {}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct UpTransportLayerInformationItemiEExtensionsItem {}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "65535")]
pub struct UpTransportLayerInformationItemiEExtensions(Vec<UpTransportLayerInformationItemiEExtensionsItem>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct UpTransportLayerInformationPairItemiEExtensionsItem {}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "65535")]
pub struct UpTransportLayerInformationPairItemiEExtensions(Vec<UpTransportLayerInformationPairItemiEExtensionsItem>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct UnavailableGuamiItemiEExtensionsItem {}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "65535")]
pub struct UnavailableGuamiItemiEExtensions(Vec<UnavailableGuamiItemiEExtensionsItem>);

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum UnsuccessfulOutcomevalue {
    #[asn(key = 0)]
    IdAmfConfigurationUpdate(AmfConfigurationUpdateFailure),
    #[asn(key = 12)]
    IdHandoverPreparation(HandoverPreparationFailure),
    #[asn(key = 13)]
    IdHandoverResourceAllocation(HandoverFailure),
    #[asn(key = 14)]
    IdInitialContextSetup(InitialContextSetupFailure),
    #[asn(key = 21)]
    IdNgSetup(NgSetupFailure),
    #[asn(key = 25)]
    IdPathSwitchRequest(PathSwitchRequestFailure),
    #[asn(key = 35)]
    IdRanConfigurationUpdate(RanConfigurationUpdateFailure),
    #[asn(key = 40)]
    IdUeContextModification(UeContextModificationFailure),
    #[asn(key = 58)]
    IdUeContextResume(UeContextResumeFailure),
    #[asn(key = 59)]
    IdUeContextSuspend(UeContextSuspendFailure),
}

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum UplinkNasTransportprotocolIEsItemvalue {
    #[asn(key = 10)]
    IdAmfUeNgapId(AmfUeNgapId),
    #[asn(key = 38)]
    IdNasPdu(NasPdu),
    #[asn(key = 85)]
    IdRanUeNgapId(RanUeNgapId),
    #[asn(key = 121)]
    IdUserLocationInformation(UserLocationInformation),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct UplinkNasTransportprotocolIEsItem {
    #[asn(key_field = true)]
    pub id: ProtocolIeId,
    pub criticality: Criticality,
    pub value: UplinkNasTransportprotocolIEsItemvalue,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "0", sz_ub = "65535")]
pub struct UplinkNasTransportprotocolIEs(Vec<UplinkNasTransportprotocolIEsItem>);

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum UplinkNonUeAssociatedNrpPaTransportprotocolIEsItemvalue {
    #[asn(key = 46)]
    IdNrpPaPdu(NrpPaPdu),
    #[asn(key = 89)]
    IdRoutingId(RoutingId),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct UplinkNonUeAssociatedNrpPaTransportprotocolIEsItem {
    #[asn(key_field = true)]
    pub id: ProtocolIeId,
    pub criticality: Criticality,
    pub value: UplinkNonUeAssociatedNrpPaTransportprotocolIEsItemvalue,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "0", sz_ub = "65535")]
pub struct UplinkNonUeAssociatedNrpPaTransportprotocolIEs(Vec<UplinkNonUeAssociatedNrpPaTransportprotocolIEsItem>);

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum UplinkRanConfigurationTransferprotocolIEsItemvalue {
    #[asn(key = 158)]
    IdEndcSonConfigurationTransferUl(EnDcsonConfigurationTransfer),
    #[asn(key = 251)]
    IdIntersystemSonConfigurationTransferUl(IntersystemSonConfigurationTransfer),
    #[asn(key = 99)]
    IdSonConfigurationTransferUl(SonConfigurationTransfer),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct UplinkRanConfigurationTransferprotocolIEsItem {
    #[asn(key_field = true)]
    pub id: ProtocolIeId,
    pub criticality: Criticality,
    pub value: UplinkRanConfigurationTransferprotocolIEsItemvalue,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "0", sz_ub = "65535")]
pub struct UplinkRanConfigurationTransferprotocolIEs(Vec<UplinkRanConfigurationTransferprotocolIEsItem>);

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum UplinkRanEarlyStatusTransferprotocolIEsItemvalue {
    #[asn(key = 10)]
    IdAmfUeNgapId(AmfUeNgapId),
    #[asn(key = 268)]
    IdEarlyStatusTransferTransparentContainer(EarlyStatusTransferTransparentContainer),
    #[asn(key = 85)]
    IdRanUeNgapId(RanUeNgapId),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct UplinkRanEarlyStatusTransferprotocolIEsItem {
    #[asn(key_field = true)]
    pub id: ProtocolIeId,
    pub criticality: Criticality,
    pub value: UplinkRanEarlyStatusTransferprotocolIEsItemvalue,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "0", sz_ub = "65535")]
pub struct UplinkRanEarlyStatusTransferprotocolIEs(Vec<UplinkRanEarlyStatusTransferprotocolIEsItem>);

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum UplinkRanStatusTransferprotocolIEsItemvalue {
    #[asn(key = 10)]
    IdAmfUeNgapId(AmfUeNgapId),
    #[asn(key = 85)]
    IdRanUeNgapId(RanUeNgapId),
    #[asn(key = 84)]
    IdRanStatusTransferTransparentContainer(RanStatusTransferTransparentContainer),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct UplinkRanStatusTransferprotocolIEsItem {
    #[asn(key_field = true)]
    pub id: ProtocolIeId,
    pub criticality: Criticality,
    pub value: UplinkRanStatusTransferprotocolIEsItemvalue,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "0", sz_ub = "65535")]
pub struct UplinkRanStatusTransferprotocolIEs(Vec<UplinkRanStatusTransferprotocolIEsItem>);

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum UplinkRimInformationTransferprotocolIEsItemvalue {
    #[asn(key = 175)]
    IdRimInformationTransfer(RimInformationTransfer),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct UplinkRimInformationTransferprotocolIEsItem {
    #[asn(key_field = true)]
    pub id: ProtocolIeId,
    pub criticality: Criticality,
    pub value: UplinkRimInformationTransferprotocolIEsItemvalue,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "0", sz_ub = "65535")]
pub struct UplinkRimInformationTransferprotocolIEs(Vec<UplinkRimInformationTransferprotocolIEsItem>);

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum UplinkUeAssociatedNrpPaTransportprotocolIEsItemvalue {
    #[asn(key = 10)]
    IdAmfUeNgapId(AmfUeNgapId),
    #[asn(key = 46)]
    IdNrpPaPdu(NrpPaPdu),
    #[asn(key = 85)]
    IdRanUeNgapId(RanUeNgapId),
    #[asn(key = 89)]
    IdRoutingId(RoutingId),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct UplinkUeAssociatedNrpPaTransportprotocolIEsItem {
    #[asn(key_field = true)]
    pub id: ProtocolIeId,
    pub criticality: Criticality,
    pub value: UplinkUeAssociatedNrpPaTransportprotocolIEsItemvalue,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "0", sz_ub = "65535")]
pub struct UplinkUeAssociatedNrpPaTransportprotocolIEs(Vec<UplinkUeAssociatedNrpPaTransportprotocolIEsItem>);

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum UserLocationInformationchoiceExtensionsvalue {
    #[asn(key = 244)]
    IdUserLocationInformationTngf(UserLocationInformationTngf),
    #[asn(key = 248)]
    IdUserLocationInformationTwif(UserLocationInformationTwif),
    #[asn(key = 243)]
    IdUserLocationInformationWAgf(UserLocationInformationWAgf),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct UserLocationInformationchoiceExtensions {
    #[asn(key_field = true)]
    pub id: ProtocolIeId,
    pub criticality: Criticality,
    pub value: UserLocationInformationchoiceExtensionsvalue,
}

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum UserLocationInformationEutrAiEExtensionsItemextensionValue {
    #[asn(key = 149)]
    IdPsCellInformation(NgranCgi),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct UserLocationInformationEutrAiEExtensionsItem {
    #[asn(key_field = true)]
    pub id: ProtocolExtensionId,
    pub criticality: Criticality,
    pub extension_value: UserLocationInformationEutrAiEExtensionsItemextensionValue,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "65535")]
pub struct UserLocationInformationEutrAiEExtensions(Vec<UserLocationInformationEutrAiEExtensionsItem>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct UserLocationInformationN3iwFiEExtensionsItem {}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "65535")]
pub struct UserLocationInformationN3iwFiEExtensions(Vec<UserLocationInformationN3iwFiEExtensionsItem>);

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum UserLocationInformationNRiEExtensionsItemextensionValue {
    #[asn(key = 263)]
    IdNid(Nid),
    #[asn(key = 149)]
    IdPsCellInformation(NgranCgi),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct UserLocationInformationNRiEExtensionsItem {
    #[asn(key_field = true)]
    pub id: ProtocolExtensionId,
    pub criticality: Criticality,
    pub extension_value: UserLocationInformationNRiEExtensionsItemextensionValue,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "65535")]
pub struct UserLocationInformationNRiEExtensions(Vec<UserLocationInformationNRiEExtensionsItem>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct UserLocationInformationTngFiEExtensionsItem {}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "65535")]
pub struct UserLocationInformationTngFiEExtensions(Vec<UserLocationInformationTngFiEExtensionsItem>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct UserLocationInformationTwiFiEExtensionsItem {}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "65535")]
pub struct UserLocationInformationTwiFiEExtensions(Vec<UserLocationInformationTwiFiEExtensionsItem>);

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum UserLocationInformationWAgFchoiceExtensionsvalue {
    #[asn(key = 275)]
    IdGlobalCableId(GlobalCableId),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct UserLocationInformationWAgFchoiceExtensions {
    #[asn(key_field = true)]
    pub id: ProtocolIeId,
    pub criticality: Criticality,
    pub value: UserLocationInformationWAgFchoiceExtensionsvalue,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct UserPlaneSecurityInformationiEExtensionsItem {}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "65535")]
pub struct UserPlaneSecurityInformationiEExtensions(Vec<UserPlaneSecurityInformationiEExtensionsItem>);

#[derive(Debug, AperCodec)]
#[asn(type = "OCTET-STRING", sz_extensible = false, sz_lb = "4", sz_ub = "4")]
pub struct OctetString89(Vec<u8>);

#[derive(Debug, AperCodec)]
#[asn(type = "OCTET-STRING", sz_extensible = false, sz_lb = "4", sz_ub = "4")]
pub struct OctetString90(Vec<u8>);

#[derive(Debug, AperCodec)]
#[asn(type = "INTEGER", lb = "0", ub = "18446744073709551615")]
pub struct Integer91(u64);

#[derive(Debug, AperCodec)]
#[asn(type = "INTEGER", lb = "0", ub = "18446744073709551615")]
pub struct Integer92(u64);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct VolumeTimedReportItemiEExtensionsItem {}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "65535")]
pub struct VolumeTimedReportItemiEExtensions(Vec<VolumeTimedReportItemiEExtensionsItem>);

#[derive(Debug, AperCodec)]
#[asn(type = "BITSTRING", sz_extensible = true, sz_lb = "16", sz_ub = "16")]
pub struct BitString93(BitVec<Msb0, u8>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct WAgfIDchoiceExtensions {}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct WlanMeasConfigNameItemiEExtensionsItem {}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "65535")]
pub struct WlanMeasConfigNameItemiEExtensions(Vec<WlanMeasConfigNameItemiEExtensionsItem>);

#[derive(Debug, AperCodec)]
#[asn(type = "ENUMERATED", extensible = true, lb = "0", ub = "0")]
pub struct Enumerated94(u8);
impl Enumerated94 {
    const TRUE: u8 = 0u8;
}

#[derive(Debug, AperCodec)]
#[asn(type = "ENUMERATED", extensible = true, lb = "0", ub = "0")]
pub struct Enumerated95(u8);
impl Enumerated95 {
    const TRUE: u8 = 0u8;
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct WlanMeasurementConfigurationiEExtensionsItem {}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "65535")]
pub struct WlanMeasurementConfigurationiEExtensions(Vec<WlanMeasurementConfigurationiEExtensionsItem>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct WusAssistanceInformationiEExtensionsItem {}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "65535")]
pub struct WusAssistanceInformationiEExtensions(Vec<WusAssistanceInformationiEExtensionsItem>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct WarningAreaListchoiceExtensions {}

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum WriteReplaceWarningRequestprotocolIEsItemvalue {
    #[asn(key = 17)]
    IdConcurrentWarningMessageInd(ConcurrentWarningMessageInd),
    #[asn(key = 20)]
    IdDataCodingScheme(DataCodingScheme),
    #[asn(key = 35)]
    IdMessageIdentifier(MessageIdentifier),
    #[asn(key = 47)]
    IdNumberOfBroadcastsRequested(NumberOfBroadcastsRequested),
    #[asn(key = 87)]
    IdRepetitionPeriod(RepetitionPeriod),
    #[asn(key = 95)]
    IdSerialNumber(SerialNumber),
    #[asn(key = 141)]
    IdWarningAreaCoordinates(WarningAreaCoordinates),
    #[asn(key = 122)]
    IdWarningAreaList(WarningAreaList),
    #[asn(key = 123)]
    IdWarningMessageContents(WarningMessageContents),
    #[asn(key = 124)]
    IdWarningSecurityInfo(WarningSecurityInfo),
    #[asn(key = 125)]
    IdWarningType(WarningType),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct WriteReplaceWarningRequestprotocolIEsItem {
    #[asn(key_field = true)]
    pub id: ProtocolIeId,
    pub criticality: Criticality,
    pub value: WriteReplaceWarningRequestprotocolIEsItemvalue,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "0", sz_ub = "65535")]
pub struct WriteReplaceWarningRequestprotocolIEs(Vec<WriteReplaceWarningRequestprotocolIEsItem>);

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum WriteReplaceWarningResponseprotocolIEsItemvalue {
    #[asn(key = 13)]
    IdBroadcastCompletedAreaList(BroadcastCompletedAreaList),
    #[asn(key = 19)]
    IdCriticalityDiagnostics(CriticalityDiagnostics),
    #[asn(key = 35)]
    IdMessageIdentifier(MessageIdentifier),
    #[asn(key = 95)]
    IdSerialNumber(SerialNumber),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct WriteReplaceWarningResponseprotocolIEsItem {
    #[asn(key_field = true)]
    pub id: ProtocolIeId,
    pub criticality: Criticality,
    pub value: WriteReplaceWarningResponseprotocolIEsItemvalue,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "0", sz_ub = "65535")]
pub struct WriteReplaceWarningResponseprotocolIEs(Vec<WriteReplaceWarningResponseprotocolIEsItem>);

#[derive(Debug, AperCodec)]
#[asn(type = "OPEN")]
pub enum XnExtTlaItemiEExtensionsItemextensionValue {
    #[asn(key = 173)]
    IdSctpTlAs(SctpTlAs),
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct XnExtTlaItemiEExtensionsItem {
    #[asn(key_field = true)]
    pub id: ProtocolExtensionId,
    pub criticality: Criticality,
    pub extension_value: XnExtTlaItemiEExtensionsItemextensionValue,
}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "65535")]
pub struct XnExtTlaItemiEExtensions(Vec<XnExtTlaItemiEExtensionsItem>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct XnTnlConfigurationInfoiEExtensionsItem {}

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "65535")]
pub struct XnTnlConfigurationInfoiEExtensions(Vec<XnTnlConfigurationInfoiEExtensionsItem>);

