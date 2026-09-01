#[test]
fn test_page_limit_accepts_only_the_contract_range() {
    assert!(matches!(
        crate::admin_page_limit::AdminPageLimit::try_from(
            crate::admin_page_limit::AdminPageLimit::MIN
        ),
        Ok(_value)
    ));
    assert!(matches!(
        crate::admin_page_limit::AdminPageLimit::try_from(constants_u16::ZERO),
        Err(crate::admin_page_limit_error::AdminPageLimitError::OutOfRange)
    ));
    assert!(matches!(
        crate::admin_page_limit::AdminPageLimit::try_from(
            crate::admin_page_limit::AdminPageLimit::MAX.saturating_add(1u16)
        ),
        Err(crate::admin_page_limit_error::AdminPageLimitError::OutOfRange)
    ));
}

#[test]
fn test_pagination_values_deserialize_from_url_query_strings() {
    let Err(_zero_error) =
        serde_json::from_str::<crate::admin_page_limit::AdminPageLimit>(constants_str::VALUE_0)
    else {
        std::panic::panic_any(constants_str::PANIC_E8FD3A29);
    };
    let Err(_above_maximum_error) =
        serde_json::from_str::<crate::admin_page_limit::AdminPageLimit>(constants_str::VALUE_101)
    else {
        std::panic::panic_any(constants_str::PANIC_36F08AD7);
    };
    let limit = <crate::admin_page_limit::AdminPageLimit as serde::Deserialize>::deserialize(
        serde::de::value::StrDeserializer::<serde::de::value::Error>::new(constants_str::VALUE_100),
    )
    .expect(constants_str::DIAGNOSTIC_A6AA5B42);
    let offset = <crate::admin_page_offset::AdminPageOffset as serde::Deserialize>::deserialize(
        serde::de::value::StrDeserializer::<serde::de::value::Error>::new(constants_str::VALUE_42),
    )
    .expect(constants_str::DIAGNOSTIC_799E47B0);
    assert_eq!(
        u16::from(limit),
        crate::admin_page_limit::AdminPageLimit::MAX
    );
    assert_eq!(u32::from(offset), 42u32);
}
