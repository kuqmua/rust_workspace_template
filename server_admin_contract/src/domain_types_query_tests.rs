#[test]
fn page_limit_accepts_only_the_contract_range() {
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
fn pagination_values_deserialize_from_url_query_strings() {
    let Err(_zero_error) = serde_json::from_str::<crate::admin_page_limit::AdminPageLimit>(
        constants_str::catalog::VALUE_0,
    ) else {
        panic!("e8fd3a29");
    };
    let Err(_above_maximum_error) = serde_json::from_str::<crate::admin_page_limit::AdminPageLimit>(
        constants_str::catalog::VALUE_101,
    ) else {
        panic!("36f08ad7");
    };
    let limit = <crate::admin_page_limit::AdminPageLimit as serde::Deserialize>::deserialize(
        serde::de::value::StrDeserializer::<serde::de::value::Error>::new(
            constants_str::catalog::VALUE_100,
        ),
    )
    .expect("a6aa5b42 pagination_values_deserialize_from_url_query_strings invariant must hold");
    let offset = <crate::admin_page_offset::AdminPageOffset as serde::Deserialize>::deserialize(
        serde::de::value::StrDeserializer::<serde::de::value::Error>::new(
            constants_str::catalog::VALUE_42,
        ),
    )
    .expect("799e47b0 pagination_values_deserialize_from_url_query_strings invariant must hold");
    assert_eq!(
        u16::from(limit),
        crate::admin_page_limit::AdminPageLimit::MAX
    );
    assert_eq!(u32::from(offset), 42u32);
}
