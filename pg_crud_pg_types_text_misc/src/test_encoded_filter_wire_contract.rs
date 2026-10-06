#[test]
fn test_encoded_filter_json_preserves_format_names_and_round_trips() {
    [
        (
            where_filters::encode_format::EncodeFormat::Base64,
            stringify!(Base64),
        ),
        (
            where_filters::encode_format::EncodeFormat::Escape,
            stringify!(Escape),
        ),
        (
            where_filters::encode_format::EncodeFormat::Hex,
            stringify!(Hex),
        ),
    ]
    .into_iter()
    .fold((), |(), (encode_format, wire_name)| {
        assert!(serde_json::to_value(encode_format).is_ok_and(|value| {
            value.as_str() == Some(wire_name)
                && serde_json::from_value::<where_filters::encode_format::EncodeFormat>(value)
                    .is_ok_and(|decoded| decoded == encode_format)
        }));
        let encoded_string_representation = encode_format.to_string();
        let filter = where_filters::domain_types::PgTypeWhereEqToEncodedStringRepresentation::new(
            pg_crud_common::operator::Operator::And,
            encode_format,
            encoded_string_representation.clone(),
        );
        assert!(serde_json::to_value(filter).is_ok_and(|value| {
            value
                .get(stringify!(encode_format))
                .and_then(serde_json::Value::as_str)
                == Some(wire_name)
                && value
                    .get(stringify!(encoded_string_representation))
                    .and_then(serde_json::Value::as_str)
                    == Some(encoded_string_representation.as_str())
                && serde_json::from_value::<
                    where_filters::domain_types::PgTypeWhereEqToEncodedStringRepresentation,
                >(value.clone())
                .is_ok_and(|decoded| {
                    serde_json::to_value(decoded).is_ok_and(|round_trip| round_trip == value)
                })
        }));
    });
}
