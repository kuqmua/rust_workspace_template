#[test]
fn test_order_default_helpers_preserve_ascending_variant() {
    assert_eq!(
        crate::order::Order::default(),
        crate::order::Order::Ascending
    );
    assert_eq!(
        <crate::order::Order as crate::default_some_one_element::DefaultSomeOneElement>::default_some_one_element(),
        crate::order::Order::Ascending,
    );
}

#[test]
fn test_order_deserialization_and_parsing_use_full_variant_names() {
    assert!([
        (crate::order::Order::Ascending, stringify!(ascending), stringify!(Asc)),
        (crate::order::Order::Descending, stringify!(descending), stringify!(Desc)),
    ]
    .into_iter()
    .all(|(order, name, display)| {
        matches!(serde_json::from_value::<crate::order::Order>(serde_json::json!(name)), Ok(decoded) if decoded == order)
            && matches!(name.parse::<crate::order::Order>(), Ok(parsed) if parsed == order)
            && order.to_string() == display
    }));
    assert!(
        [
            constants_str::ASC_ALT,
            constants_str::DESC_ALT,
            constants_str::EMPTY
        ]
        .into_iter()
        .all(
            |name| serde_json::from_value::<crate::order::Order>(serde_json::json!(name)).is_err()
                && name.parse::<crate::order::Order>().is_err()
        )
    );
}

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

#[test]
fn test_order_text_preserves_byte_boundaries_and_validation_errors() {
    let maximum_length = crate::pg_crud_string_wrapper_max_len::PG_CRUD_STRING_WRAPPER_MAX_LEN;
    let accepted_result = crate::order_text_string::OrderTextString::try_from(
        constants_str::X.repeat(maximum_length),
    );
    assert!(accepted_result.is_ok());
    let Ok(accepted) = accepted_result else {
        return;
    };
    assert_eq!(accepted.to_string().len(), maximum_length);
    let oversized_length = maximum_length.saturating_add(1usize);
    let rejected_result = crate::order_text_string::OrderTextString::try_from(
        constants_str::X.repeat(oversized_length),
    );
    assert_eq!(
        rejected_result,
        Err(crate::pg_crud_string_wrapper_try_from_string_error::PgCrudStringWrapperTryFromStringError::TooLong {
            len: oversized_length,
            max: maximum_length,
        }),
    );
    let character_count = 524_288usize;
    assert_eq!(
        character_count.saturating_mul('\u{00e9}'.len_utf8()),
        maximum_length
    );
    let unicode_result = crate::order_text_string::OrderTextString::try_from(
        '\u{00e9}'.to_string().repeat(character_count),
    );
    assert!(unicode_result.is_ok());
    let Ok(unicode) = unicode_result else {
        return;
    };
    assert_eq!(unicode.to_string().len(), maximum_length);
    let unicode_overflow = crate::order_text_string::OrderTextString::try_from(
        '\u{00e9}'
            .to_string()
            .repeat(character_count.saturating_add(1usize)),
    );
    assert_eq!(
        unicode_overflow,
        Err(crate::pg_crud_string_wrapper_try_from_string_error::PgCrudStringWrapperTryFromStringError::TooLong {
            len: maximum_length.saturating_add(2usize),
            max: maximum_length,
        }),
    );
}

#[test]
fn test_order_case_text_conversions_preserve_input_and_error_diagnostics() {
    [constants_str::EMPTY, constants_str::X]
        .into_iter()
        .fold((), |(), text| {
            let snake_result =
                crate::order_snake_case_str::OrderSnakeCaseStr::try_from(text.to_owned());
            assert!(snake_result.is_ok());
            let Ok(snake) = snake_result else {
                return;
            };
            assert_eq!(snake.to_string(), text);
            let camel_result = crate::order_upper_camel_case_str::OrderUpperCamelCaseStr::try_from(
                text.to_owned(),
            );
            assert!(camel_result.is_ok());
            let Ok(camel) = camel_result else {
                return;
            };
            assert_eq!(camel.to_string(), text);
        });
    let error = crate::pg_crud_string_wrapper_try_from_string_error::PgCrudStringWrapperTryFromStringError::TooLong {
        len: 2usize,
        max: 1usize,
    };
    let expected = error.to_string();
    assert_eq!(
        crate::order_snake_case_str::OrderSnakeCaseStr::from(error).to_string(),
        expected
    );
    assert_eq!(
        crate::order_upper_camel_case_str::OrderUpperCamelCaseStr::from(error).to_string(),
        expected
    );
    let maximum_length = crate::pg_crud_string_wrapper_max_len::PG_CRUD_STRING_WRAPPER_MAX_LEN;
    let expected_error = crate::pg_crud_string_wrapper_try_from_string_error::PgCrudStringWrapperTryFromStringError::TooLong {
        len: maximum_length.saturating_add(1usize),
        max: maximum_length,
    };
    assert_eq!(
        crate::order_snake_case_str::OrderSnakeCaseStr::try_from(
            constants_str::X.repeat(maximum_length.saturating_add(1usize))
        ),
        Err(expected_error)
    );
    assert_eq!(
        crate::order_upper_camel_case_str::OrderUpperCamelCaseStr::try_from(
            constants_str::X.repeat(maximum_length.saturating_add(1usize))
        ),
        Err(expected_error)
    );
}
