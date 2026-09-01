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
