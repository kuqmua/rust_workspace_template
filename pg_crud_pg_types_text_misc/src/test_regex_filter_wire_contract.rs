#[test]
fn test_regex_filter_json_preserves_case_names_and_round_trips() {
    [
        (where_filters::regex_case::RegexCase::Insensitive, stringify!(Insensitive)),
        (where_filters::regex_case::RegexCase::Sensitive, stringify!(Sensitive)),
    ].into_iter().fold((), |(), (regex_case, wire_name)| {
        assert!(serde_json::to_value(regex_case).is_ok_and(|value| {
            value.as_str() == Some(wire_name)
                && serde_json::from_value::<where_filters::regex_case::RegexCase>(value)
                    .is_ok_and(|decoded| decoded == regex_case)
        }));
        assert!(where_filters::regex_regex::RegexRegex::try_from(wire_name.to_owned()).is_ok_and(|regex_regex| {
            let filter = where_filters::domain_types::PgTypeWhereRegex::new(
                pg_crud_common::operator::Operator::And, regex_case, regex_regex,
            );
            serde_json::to_value(filter).is_ok_and(|value| {
                value.get(stringify!(regex_case)).and_then(serde_json::Value::as_str) == Some(wire_name)
                    && serde_json::from_value::<where_filters::domain_types::PgTypeWhereRegex>(value.clone())
                        .is_ok_and(|decoded| serde_json::to_value(decoded).is_ok_and(|round_trip| round_trip == value))
            })
        }));
    });
}
