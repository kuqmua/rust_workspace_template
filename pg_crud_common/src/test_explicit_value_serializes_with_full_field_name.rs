#[test]
fn test_explicit_value_serializes_with_full_field_name() {
    let serialized = serde_json::to_value(crate::explicit_value::ExplicitValue::new(7u8))
        .expect(constants_str::DIAGNOSTIC_F22636CE);
    assert_eq!(serialized, serde_json::json!({ (stringify!(value)): 7u8 }));
    assert!(serialized.get(stringify!(v)).is_none());
}
