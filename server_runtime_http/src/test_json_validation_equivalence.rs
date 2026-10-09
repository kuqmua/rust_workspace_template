#[test]
fn test_json_formatting_preserves_last_duplicate_key_value_without_mutating_original_text() {
    let first = serde_json::json!({(constants_str::X): 1u64}).to_string();
    let expected = serde_json::json!({(constants_str::X): 2u64}).to_string();
    let input = first
        .chars()
        .take(first.len() - 1usize)
        .chain(std::iter::once(','))
        .chain(expected.chars().skip(1usize))
        .collect::<String>();
    assert!(
        crate::bounded_json_text::BoundedJsonText::try_from(input.clone()).is_ok_and(|original| {
            let compact = original.compact();
            let pretty = original.pretty();
            original.as_ref() == input
                && compact.is_ok_and(|formatted| {
                    formatted.as_ref() == expected
                        && formatted
                            .compact()
                            .is_ok_and(|repeated| repeated == formatted)
                })
                && pretty.is_ok_and(|formatted| {
                    formatted.as_ref().contains('\n')
                        && formatted
                            .compact()
                            .is_ok_and(|compacted| compacted.as_ref() == expected)
                })
        })
    );
}

#[test]
fn test_json_validation_preserves_parser_results_and_diagnostics() {
    let excessive_depth = std::iter::repeat_n('[', 129usize)
        .chain(std::iter::once('0'))
        .chain(std::iter::repeat_n(']', 129usize))
        .collect::<String>();
    let large_number = ['1', 'e', '4', '0', '0'].into_iter().collect::<String>();
    let invalid_surrogate = ['"', '\\', 'u', 'D', '8', '0', '0', '"']
        .into_iter()
        .collect::<String>();
    let cases = [
        String::from(constants_str::TEST_JSON_MAP_WITH_ONE_ENTRY),
        String::from(constants_str::TEST_INVALID_JSON),
        serde_json::json!([
            null,
            true,
            false,
            0i64,
            -1i64,
            u64::MAX,
            1.25f64,
            constants_str::HELLOWORLD
        ])
        .to_string(),
        large_number,
        invalid_surrogate,
        excessive_depth,
    ];
    assert!(cases.into_iter().all(|text| {
        let expected = serde_json::from_str::<serde_json::Value>(&text)
            .map(|_value| ())
            .map_err(|error| error.to_string());
        let actual = crate::bounded_json_text::BoundedJsonText::try_from(text)
            .map(|_value| ())
            .map_err(|error| match error {
                crate::bounded_json_read_error::BoundedJsonReadError::SerdeJson(source) => {
                    source.to_string()
                }
                crate::bounded_json_read_error::BoundedJsonReadError::Read(source) => {
                    source.to_string()
                }
            });
        actual == expected
    }));
}

#[test]
fn test_json_container_boundaries_preserve_success_and_exact_parser_errors() {
    assert!(
        [
            (serde_json::json!([]).to_string(), true),
            (serde_json::json!({}).to_string(), true),
            (
                serde_json::json!({
                    (stringify!(values)): [null, true, -1i64, 1.25f64, [], {}],
                    (stringify!(nested)): { (stringify!(value)): constants_str::HELLOWORLD }
                })
                .to_string(),
                true,
            ),
            (['[', '0', ',', ']'].into_iter().collect::<String>(), false),
            (
                ['[', '0', ',', '{', ']'].into_iter().collect::<String>(),
                false
            ),
            (
                ['{', '"', 'x', '"', ':', '0', ',', '1', ':', '0', '}']
                    .into_iter()
                    .collect::<String>(),
                false
            ),
            (
                ['{', '"', 'x', '"', ':', '0', ',', '"', 'y', '"', ':', '}']
                    .into_iter()
                    .collect::<String>(),
                false
            ),
            (['[', ']', ' ', '0'].into_iter().collect::<String>(), false),
            (
                ['{', '}', ' ', '{', '}'].into_iter().collect::<String>(),
                false
            ),
        ]
        .into_iter()
        .chain(
            [(127usize, true), (128usize, false)]
                .into_iter()
                .map(|(depth, valid)| {
                    (
                        std::iter::repeat_n('[', depth)
                            .chain(std::iter::once('0'))
                            .chain(std::iter::repeat_n(']', depth))
                            .collect::<String>(),
                        valid,
                    )
                })
        )
        .all(|(text, valid)| {
            let expected = serde_json::from_str::<serde_json::Value>(&text)
                .map(|_value| ())
                .map_err(|error| error.to_string());
            let actual = crate::bounded_json_text::BoundedJsonText::try_from(text)
                .map(|_value| ())
                .map_err(|error| match error {
                    crate::bounded_json_read_error::BoundedJsonReadError::SerdeJson(source) => {
                        source.to_string()
                    }
                    crate::bounded_json_read_error::BoundedJsonReadError::Read(source) => {
                        source.to_string()
                    }
                });
            expected.is_ok() == valid && actual == expected
        })
    );
}

#[test]
fn test_json_value_deserialization_rejects_byte_values_with_json_expectation() {
    let deserializer =
        serde::de::value::BorrowedBytesDeserializer::<serde::de::value::Error>::new(&[0u8, 1u8]);
    let result =
        <crate::validated_json_value::ValidatedJsonValue as serde::Deserialize>::deserialize(
            deserializer,
        );
    let expected = format!(
        "{}: {}, {} {}",
        stringify!(invalid type),
        stringify!(byte array),
        stringify!(expected),
        constants_str::JSON,
    );
    assert!(result.is_err_and(|error| error.to_string() == expected));
}
