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

#[test]
fn test_wire_enum_serialization_does_not_require_copy() {
    #[derive(
        proc_macro_optimal_memory_layout::OptimalMemoryLayout,
        proc_macro_newtype_wire_enum::WireEnum,
    )]
    #[wire_enum(
        ref_type = frontend_contract::form_value_ref::FormValueRef,
        error_message = constants_str::WIRE_ENUM_REQUIRES_ATTRIBUTE,
    )]
    enum BorrowedWireEnumFixture {
        #[wire("Item")]
        Item,
    }

    let wire_value = BorrowedWireEnumFixture::try_from(constants_str::TEST_OPENAPI_SCHEMA)
        .map(BorrowedWireEnumFixture::as_str);
    assert!(matches!(
        wire_value,
        Ok(value) if value.as_ref() == constants_str::TEST_OPENAPI_SCHEMA
    ));
    assert_eq!(BorrowedWireEnumFixture::ALL.len(), constants_usize::ONE);
}

#[test]
fn test_wire_enum_preserves_const_generics() {
    #[derive(
        proc_macro_optimal_memory_layout::OptimalMemoryLayout,
        proc_macro_newtype_wire_enum::WireEnum,
    )]
    #[wire_enum(
        ref_type = frontend_contract::form_value_ref::FormValueRef,
        error_message = constants_str::WIRE_ENUM_REQUIRES_ATTRIBUTE,
    )]
    enum ConstGenericWireFixture<const COUNT: usize>
    where
        [(); COUNT]: Sized,
    {
        #[wire("Item")]
        Item,
    }
    let wire_value =
        ConstGenericWireFixture::<0usize>::try_from(constants_str::TEST_OPENAPI_SCHEMA)
            .map(ConstGenericWireFixture::as_str);
    assert!(
        matches!(wire_value, Ok(value) if value.as_ref() == constants_str::TEST_OPENAPI_SCHEMA)
    );
    assert_eq!(
        ConstGenericWireFixture::<1usize>::ALL.len(),
        constants_usize::ONE
    );
    assert!(matches!(
        ConstGenericWireFixture::<1usize>::try_from(constants_str::BAD),
        Err(ConstGenericWireFixtureTryFromStrError)
    ));
}
#[test]
fn test_constructor_input_policy_rejects_invalid_shapes_and_attributes() {
    let cases = trybuild::TestCases::new();
    cases.compile_fail(format!(
        "{}{}{}{}{}{}{}",
        stringify!(trybuild),
        constants_str::SLASH,
        constants_str::CONSTRUCTOR_ATTRIBUTE,
        constants_str::UNDERSCORE,
        constants_str::ASTERISK,
        constants_str::DOT,
        constants_str::RS,
    ));
}
#[test]
fn test_optimal_memory_layout_input_policy_rejects_misalignment_and_unknown_attributes() {
    let cases = trybuild::TestCases::new();
    cases.compile_fail(format!(
        "{}{}{}{}{}",
        stringify!(trybuild),
        constants_str::SLASH,
        stringify!(optimal_memory_layout_rejection),
        constants_str::DOT,
        constants_str::RS,
    ));
    cases.pass(format!(
        "{}{}{}{}{}",
        stringify!(trybuild),
        constants_str::SLASH,
        stringify!(optimal_memory_layout_acceptance),
        constants_str::DOT,
        constants_str::RS,
    ));
}
