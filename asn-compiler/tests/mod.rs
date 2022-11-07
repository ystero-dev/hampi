#![cfg(test)]

fn get_module_header(module_name: &str, test_no: u16) -> String {
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
            false,
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
        let result = compiler.compile_string(&module_str);

        assert!(result.is_ok(), "{:?}", result.err());
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
        let result = compiler.compile_string(&module_str);

        assert!(result.is_ok(), "{:?}", result.err());
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

        eprintln!("{}", module_str);

        let mut compiler = get_dev_null_compiler();
        let result = compiler.compile_string(&module_str);

        assert!(result.is_ok(), "{:#?}", result.err().unwrap());
    }
}
