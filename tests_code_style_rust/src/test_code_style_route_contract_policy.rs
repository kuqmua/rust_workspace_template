#[test]
fn test_route_contract_policy() {
    fn assert_serializable<Value>()
    where
        Value: serde::Serialize,
    {
        let _: std::marker::PhantomData<Value> = std::marker::PhantomData;
    }
    assert_serializable::<String>();
    assert_eq!(
        size_of::<frontend_contract::public_transport::PublicTransport>(),
        constants_usize::ZERO
    );
    let cases = trybuild::TestCases::new();
    cases.compile_fail(constants_str::TRYBUILD_ROUTE_CONTRACT_ASTERISK_RS);
}
