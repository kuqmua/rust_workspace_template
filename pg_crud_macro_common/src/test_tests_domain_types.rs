#[test]
fn test_domain_type_tests() {
    assert_eq!(
        crate::import::Import::Crate.sc_str().as_ref(),
        constants_str::CRATE
    );
}
