#[test]
fn page_limit_accepts_only_the_contract_range() {
    assert!(matches!(
        super::AdminPageLimit::try_from(super::AdminPageLimit::MIN),
        Ok(_value)
    ));
    assert!(matches!(
        super::AdminPageLimit::try_from(0u16),
        Err(super::AdminPageLimitError)
    ));
    assert!(matches!(
        super::AdminPageLimit::try_from(super::AdminPageLimit::MAX.saturating_add(1u16)),
        Err(super::AdminPageLimitError)
    ));
}

#[test]
fn pagination_values_deserialize_from_url_query_strings() {
    let Err(_zero_error) = serde_json::from_str::<super::AdminPageLimit>(str_constants::VALUE_0)
    else {
        panic!("e8fd3a29");
    };
    let Err(_above_maximum_error) =
        serde_json::from_str::<super::AdminPageLimit>(str_constants::VALUE_101)
    else {
        panic!("36f08ad7");
    };
    let limit = <super::AdminPageLimit as serde::Deserialize>::deserialize(
        serde::de::value::StrDeserializer::<serde::de::value::Error>::new("100"),
    )
    .expect("a6aa5b42 pagination_values_deserialize_from_url_query_strings invariant must hold");
    let offset = <super::AdminPageOffset as serde::Deserialize>::deserialize(
        serde::de::value::StrDeserializer::<serde::de::value::Error>::new("42"),
    )
    .expect("799e47b0 pagination_values_deserialize_from_url_query_strings invariant must hold");
    assert_eq!(u16::from(limit), super::AdminPageLimit::MAX);
    assert_eq!(u32::from(offset), 42u32);
}
