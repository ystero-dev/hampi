enum Type_CommonID_IEs {
    PermanentNASUEID(PermanentNAS_UE_ID),
}
struct ProtocolIE_Field_CommonID_IEs {
    id: ProtcolIE_ID,
    criticality: Criticality,
    value: Type_CommonID_IEs,
}

struct ProtocolID_Container_CommonID_IEs(Vec<ProtocolIE_Field_CommonID_IEs>);

struct ProtocolExtensionContainer_CommonIDExtensions;

struct SNA_Access_Information {
    authorizedPLMNs: AuthorizedPLMNs,
    iE_Extensions: ProtocolExtensionContainer_SNA_Access_Information_ExtIEs,
}

enum Extension_CommonIDExtensions {
    SNAAccessInformation(SNA_Access_Information),
}

struct ProtocolExtensionContainer_CommonIDExtensions {
    id: ProtcolIE_ID,
    criticality: Criticality,
    extensionValue: Extension_CommonIDExtensions,
}

struct CommonID {
    protocolIEs: ProtocolID_Container_CommonID_IEs,
    protocolExtensions: Option<ProtocolExtensionContainer_CommonIDExtensions>,
}
