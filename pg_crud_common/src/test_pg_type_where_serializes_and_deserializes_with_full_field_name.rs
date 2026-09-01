#[test]
fn test_pg_type_where_serializes_and_deserializes_with_full_field_name() {
    let filter = crate::pg_type_where::PgTypeWhere::try_new(
        crate::operator::Operator::default(),
        crate::duplicate_candidates::DuplicateCandidates::from(vec![7u8]),
    )
    .expect(constants_str::DIAGNOSTIC_F465D1AC);
    let serialized = serde_json::to_value(&filter).expect(constants_str::DIAGNOSTIC_529A2FFD);
    assert!(serialized.get(stringify!(values)).is_some());
    assert!(serialized.get(stringify!(v)).is_none());
    assert_eq!(
        serde_json::from_value::<crate::pg_type_where::PgTypeWhere<u8>>(serialized)
            .expect(constants_str::DIAGNOSTIC_28EC5ACA),
        filter
    );
}
