#[test]
fn route_contract_type_mismatches_do_not_compile() {
    fn assert_serializable<Value>()
    where
        Value: serde::Serialize,
    {
    }
    assert_serializable::<String>();
    assert_eq!(
        size_of::<frontend_contract::PublicTransport>(),
        usize_constants::ZERO
    );
    let cases = trybuild::TestCases::new();
    cases.compile_fail(str_constants::TRYBUILD_ROUTE_CONTRACT_ASTERISK_RS);
}
