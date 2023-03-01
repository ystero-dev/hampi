#![cfg(test)]

fn init() {
    let _ = env_logger::builder()
        .filter_level(log::LevelFilter::max())
        .is_test(true)
        .try_init();
}

fn get_module_header(module_name: &str, test_no: u16) -> String {
    init();

    format!(
        r#"{module_name}
        {{ iso org(3) dod(6) internet (1) private(4) enterprise(1)
                hyphenOs(9363) software(1) hampi(5) test(1) {test_no} }}"#,
        module_name = module_name,
        test_no = test_no
    )
}

fn get_module_definitions(definitions: &str) -> String {
    format!(
        r#"

DEFINITIONS ::=
BEGIN

{definitions}

END
"#,
        definitions = definitions
    )
}

mod tests {
    use asn1_compiler::generator::{Codec, Derive, Visibility};
    use asn1_compiler::Asn1Compiler;

    fn get_dev_null_compiler() -> Asn1Compiler {
        Asn1Compiler::new(
            "/dev/null",
            &Visibility::Public,
            vec![Codec::Aper],
            vec![Derive::Debug, Derive::Serialize, Derive::Deserialize],
        )
    }

    #[test]
    fn failing_enumerated_empty_named_root_49() {
        let module_name = "EmptyNamedRootEnumeratedFailing";
        let test_no = 1;

        let module_header = super::get_module_header(module_name, test_no);

        let definitions = "NotificationResponse ::= ENUMERATED {allowed(0), notAllowed(1), ...}";
        let definitions = super::get_module_definitions(definitions);

        let module_str = format!("{} {}", module_header, definitions);

        let mut compiler = get_dev_null_compiler();
        let result = compiler.compile_string(&module_str, false);

        assert!(result.is_ok(), "{:?}", result.err().unwrap());
    }

    #[test]
    fn fix_support_exports_definitions_46() {
        let module_name = "ModuleExportsDefinitions";
        let test_no = 2;

        let module_header = super::get_module_header(module_name, test_no);

        let definitions = "EXPORTS ExportedA; ExportedA ::= SEQUENCE { a INTEGER }";
        let definitions = super::get_module_definitions(definitions);

        let module_str = format!("{} {}", module_header, definitions);

        let mut compiler = get_dev_null_compiler();
        let result = compiler.compile_string(&module_str, false);

        assert!(result.is_ok(), "{:?}", result.err().unwrap());
    }

    #[test]
    fn compile_example_from_x691_spec() {
        let module_name = "ExampleFromX691";
        let test_no = 3;
        let module_header = super::get_module_header(module_name, test_no);

        let definitions = r#"
PersonnelRecord ::=  SET {
        name Name,
        title VisibleString,
        number EmployeeNumber,
        dateOfHire Date,
        nameOfSpouse Name,
        children SEQUENCE OF ChildInformation DEFAULT {}
}
ChildInformation ::= SET {
    name Name,
    dateOfBirth Date
}

Name ::= SEQUENCE {
    givenName VisibleString,
    initial VisibleString,
    familyName VisibleString
}
EmployeeNumber ::= INTEGER
Date ::= VisibleString -- YYYYMMDD"#;

        let definitions = super::get_module_definitions(definitions);

        let module_str = format!("{} {}", module_header, definitions);

        let mut compiler = get_dev_null_compiler();
        let result = compiler.compile_string(&module_str, false);

        assert!(result.is_ok(), "{:#?}", result.err().unwrap());
    }

    #[test]
    fn compile_example_from_rrc_spec() {
        let module_name = "ExampleFrom36331RRCSpec";
        let test_no = 4;
        let module_header = super::get_module_header(module_name, test_no);

        let definitions = r#"
AS-Config ::=                           SEQUENCE {
        sourceMeasConfig                                        MeasConfig,
        sourceRadioResourceConfig                       RadioResourceConfigDedicated,
        sourceSecurityAlgorithmConfig           SecurityAlgorithmConfig,
        sourceUE-Identity                                       C-RNTI,
        sourceMasterInformationBlock            MasterInformationBlock,
        sourceSystemInformationBlockType1       SystemInformationBlockType1(WITH COMPONENTS
                                                                                        {..., nonCriticalExtension ABSENT}),
        sourceSystemInformationBlockType2       SystemInformationBlockType2,
        antennaInfoCommon                                       AntennaInfoCommon,
        sourceDl-CarrierFreq                            ARFCN-ValueEUTRA,
        ...,
        [[      sourceSystemInformationBlockType1Ext    OCTET STRING (CONTAINING
                                                                                                SystemInformationBlockType1-v890-IEs)   OPTIONAL,
                sourceOtherConfig-r9                            OtherConfig-r9
        -- sourceOtherConfig-r9 should have been optional. A target eNB compliant with this transfer
        -- syntax should support receiving an AS-Config not including this extension addition group
        -- e.g. from a legacy source eNB
        ]],
        [[      sourceSCellConfigList-r10                       SCellToAddModList-r10                   OPTIONAL
        ]],
        [[      sourceConfigSCG-r12                                     SCG-Config-r12          OPTIONAL
        ]],
        [[      as-ConfigNR-r15                                         AS-ConfigNR-r15                                 OPTIONAL
        ]],
        [[      as-Config-v1550                                         AS-Config-v1550                                 OPTIONAL
        ]],
        [[      as-ConfigNR-v1570                                       AS-ConfigNR-v1570                               OPTIONAL
        ]],
        [[      as-ConfigNR-v1620                                       AS-ConfigNR-v1620                               OPTIONAL
        ]]
}
"#;
        let definitions = super::get_module_definitions(definitions);

        let module_str = format!("{} {}", module_header, definitions);

        let mut compiler = get_dev_null_compiler();
        let result = compiler.compile_string(&module_str, true);

        assert!(result.is_ok(), "{:#?}", result.err().unwrap());
    }
}
