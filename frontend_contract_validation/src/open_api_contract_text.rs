#[derive(
    proc_macro_optimal_memory_layout::OptimalMemoryLayout,
    Clone,
    Debug,
    Eq,
    Ord,
    PartialEq,
    PartialOrd,
    proc_macro_newtype_as_ref_str::AsRefStr,
    proc_macro_newtype_bounded_string_wrapper::BoundedStringWrapper,
)]
#[bounded_string(max = constants_usize::VALUE_1_048_576)]
pub struct OpenApiContractText(
    bounded_types::bounded_string::BoundedString<
        0usize,
        { constants_usize::VALUE_1_048_576 },
        false,
    >,
);

#[cfg(test)]
mod tests {
    #[test]
    fn test_openapi_contract_text_preserves_exact_byte_limits_and_owned_content() {
        let maximum = constants_usize::VALUE_1_048_576;
        [
            constants_str::EMPTY.to_owned(),
            constants_str::X.repeat(maximum),
            '\u{10ffff}'.to_string().repeat(maximum.div_euclid(4usize)),
        ]
        .into_iter()
        .fold((), |(), value| {
            let length = value.len();
            let pointer = value.as_ptr();
            let expected_character = value.chars().next();
            assert!(
                crate::open_api_contract_text::OpenApiContractText::try_from(value).is_ok_and(
                    |text| {
                        text.as_ref().len() == length
                            && text.as_ref().as_ptr() == pointer
                            && text
                                .as_ref()
                                .chars()
                                .all(|character| Some(character) == expected_character)
                    }
                )
            );
        });
        [
            constants_str::X.repeat(maximum + 1usize),
            '\u{10ffff}'
                .to_string()
                .repeat(maximum.div_euclid(4usize) + 1usize),
        ]
        .into_iter()
        .fold((), |(), value| {
            let length = value.len();
            assert!(matches!(
                crate::open_api_contract_text::OpenApiContractText::try_from(value),
                Err(crate::open_api_contract_text::OpenApiContractTextTryFromStringError::TooLong { len, max })
                    if len == length && max == maximum
            ));
        });
    }
}
