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
