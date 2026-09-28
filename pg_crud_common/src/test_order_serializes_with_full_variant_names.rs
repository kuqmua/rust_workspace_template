#[test]
fn test_order_serializes_with_full_variant_names() {
    assert_eq!(
        serde_json::to_value(crate::order::Order::Ascending)
            .expect(constants_str::DIAGNOSTIC_3B565F2D),
        serde_json::json!(stringify!(ascending))
    );
    assert_eq!(
        serde_json::to_value(crate::order::Order::Descending)
            .expect(constants_str::DIAGNOSTIC_CCF4BB5E),
        serde_json::json!(stringify!(descending))
    );
}

#[test]
fn test_order_case_strings_match_fixed_variants() {
    assert!(
        [
            (
                crate::order::Order::Ascending,
                constants_str::ASC_ALT,
                stringify!(Asc),
            ),
            (
                crate::order::Order::Descending,
                constants_str::DESC_ALT,
                stringify!(Desc),
            ),
        ]
        .into_iter()
        .all(|(order, snake_case, upper_camel_case)| {
            order.to_snake_case_str().to_string() == snake_case
                && order.to_upper_camel_case_str().to_string() == upper_camel_case
        })
    );
}
