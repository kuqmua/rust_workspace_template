naming_common_macros::case_trait_pair!(
    AsRefStrToUpperCamelCaseStr,
    AsRefStrToUpperCamelCaseTokenStream,
    AsRef<str>,
    |self_ref| {
        crate::str_case::str_case(
            self_ref.as_ref(),
            crate::convert_case_kind::ConvertCaseKind::from(convert_case::Case::UpperCamel),
        )
        .into_inner()
    }
);
naming_common_macros::case_trait_pair!(
    AsRefStrToSnakeCaseStr,
    AsRefStrToSnakeCaseTokenStream,
    AsRef<str>,
    |self_ref| {
        crate::str_case::str_case(
            self_ref.as_ref(),
            crate::convert_case_kind::ConvertCaseKind::from(convert_case::Case::Snake),
        )
        .into_inner()
    }
);
naming_common_macros::case_trait_pair!(
    AsRefStrToUpperSnakeCaseStr,
    AsRefStrToUpperSnakeCaseTokenStream,
    AsRef<str>,
    |self_ref| crate::str_case::str_case(
        self_ref.as_ref(),
        crate::convert_case_kind::ConvertCaseKind::from(convert_case::Case::UpperSnake)
    )
    .into_inner()
);
naming_common_macros::case_trait_pair!(
    DisplayToUpperCamelCaseStr,
    DisplayToUpperCamelCaseTokenStream,
    std::fmt::Display,
    |self_ref| {
        crate::display_case_str::display_case_str(
            self_ref,
            crate::convert_case_kind::ConvertCaseKind::from(convert_case::Case::UpperCamel),
        )
        .into_inner()
    }
);
naming_common_macros::case_trait_pair!(
    DisplayToSnakeCaseStr,
    DisplayToSnakeCaseTokenStream,
    std::fmt::Display,
    |self_ref| {
        crate::display_case_str::display_case_str(
            self_ref,
            crate::convert_case_kind::ConvertCaseKind::from(convert_case::Case::Snake),
        )
        .into_inner()
    }
);
naming_common_macros::case_trait_pair!(
    DisplayToUpperSnakeCaseStr,
    DisplayToUpperSnakeCaseTokenStream,
    std::fmt::Display,
    |self_ref| crate::display_case_str::display_case_str(
        self_ref,
        crate::convert_case_kind::ConvertCaseKind::from(convert_case::Case::UpperSnake)
    )
    .into_inner()
);
naming_common_macros::case_trait_pair!(
    ToTokensToUpperCamelCaseStr,
    ToTokensToUpperCamelCaseTokenStream,
    quote::ToTokens,
    |self_ref| {
        crate::tokenized_case_str::tokenized_case_str(
            self_ref,
            crate::convert_case_kind::ConvertCaseKind::from(convert_case::Case::UpperCamel),
        )
        .into_inner()
    }
);
naming_common_macros::case_trait_pair!(
    ToTokensToSnakeCaseStr,
    ToTokensToSnakeCaseTokenStream,
    quote::ToTokens,
    |self_ref| {
        crate::tokenized_case_str::tokenized_case_str(
            self_ref,
            crate::convert_case_kind::ConvertCaseKind::from(convert_case::Case::Snake),
        )
        .into_inner()
    }
);
naming_common_macros::case_trait_pair!(
    ToTokensToUpperSnakeCaseStr,
    ToTokensToUpperSnakeCaseTokenStream,
    quote::ToTokens,
    |self_ref| crate::tokenized_case_str::tokenized_case_str(
        self_ref,
        crate::convert_case_kind::ConvertCaseKind::from(convert_case::Case::UpperSnake)
    )
    .into_inner()
);
#[cfg(test)]
mod tests {
    fn assert_case_triplet<S>(to_upper_camel_case: S, to_snake_case: S, to_upper_snake_case: S)
    where
        S: AsRef<str>,
    {
        assert_eq!(to_upper_camel_case.as_ref(), "HelloWorld");
        assert_eq!(to_snake_case.as_ref(), "hello_world");
        assert_eq!(to_upper_snake_case.as_ref(), "HELLO_WORLD");
    }
    #[test]
    fn test_as_ref_case_conversions_are_expected() {
        assert_case_triplet(
            super::AsRefStrToUpperCamelCaseStr::case(&constants_str::HELLO_WORLD_ALT),
            super::AsRefStrToSnakeCaseStr::case(&constants_str::HELLOWORLD),
            super::AsRefStrToUpperSnakeCaseStr::case(&constants_str::HELLOWORLD_ALT),
        );
    }
    #[test]
    fn test_ts_case_conversions_are_expected() {
        assert_case_triplet(
            super::AsRefStrToUpperCamelCaseTokenStream::case_or_panic(
                &constants_str::HELLO_WORLD_ALT,
            )
            .to_string(),
            super::AsRefStrToSnakeCaseTokenStream::case_or_panic(&constants_str::HELLOWORLD)
                .to_string(),
            super::AsRefStrToUpperSnakeCaseTokenStream::case_or_panic(
                &constants_str::HELLOWORLD_ALT,
            )
            .to_string(),
        );
    }
    #[test]
    fn test_display_and_tokens_conversion_are_expected() {
        assert_case_triplet(
            super::DisplayToUpperCamelCaseStr::case(&constants_str::HELLO_WORLD_ALT),
            super::DisplayToSnakeCaseStr::case(&constants_str::HELLOWORLD),
            super::DisplayToUpperSnakeCaseStr::case(&constants_str::HELLOWORLD_ALT),
        );
        assert_case_triplet(
            super::ToTokensToUpperCamelCaseStr::case(&quote::quote! {hello_world}),
            super::ToTokensToSnakeCaseStr::case(&quote::quote! {HelloWorld}),
            super::ToTokensToUpperSnakeCaseStr::case(&quote::quote! {helloWorld}),
        );
    }
    #[test]
    fn test_display_and_tokens_token_stream_conversion_are_expected() {
        assert_case_triplet(
            super::DisplayToUpperCamelCaseTokenStream::case_or_panic(
                &constants_str::HELLO_WORLD_ALT,
            )
            .to_string(),
            super::DisplayToSnakeCaseTokenStream::case_or_panic(&constants_str::HELLOWORLD)
                .to_string(),
            super::DisplayToUpperSnakeCaseTokenStream::case_or_panic(
                &constants_str::HELLOWORLD_ALT,
            )
            .to_string(),
        );
        assert_case_triplet(
            super::ToTokensToUpperCamelCaseTokenStream::case_or_panic(&quote::quote! {hello_world})
                .to_string(),
            super::ToTokensToSnakeCaseTokenStream::case_or_panic(&quote::quote! {HelloWorld})
                .to_string(),
            super::ToTokensToUpperSnakeCaseTokenStream::case_or_panic(&quote::quote! {helloWorld})
                .to_string(),
        );
    }
}
