#[test]
fn test_config_parse_assertions_preserve_result_patterns() {
    let input = quote::quote! { u16, input, output };
    assert_eq!(
        crate::assert_parse_ok_matches(input.clone()).to_string(),
        quote::quote! { assert!(matches!(parse_env::<u16>(input), Ok(output))); }.to_string()
    );
    assert_eq!(
        crate::assert_parse_err_matches(input).to_string(),
        quote::quote! { assert!(matches!(parse_env::<u16>(input), Err(output))); }.to_string()
    );
}

#[test]
fn test_config_parse_assertions_preserve_argument_count_diagnostics() {
    assert!(
        [
            quote::quote! {},
            quote::quote! { u16, input, output, extra }
        ]
        .into_iter()
        .all(|input| {
            crate::assert_parse_ok_matches(input.clone())
                .to_string()
                .contains(constants_str::COMPILE_ERROR_CE_040)
                && crate::assert_parse_err_matches(input)
                    .to_string()
                    .contains(constants_str::COMPILE_ERROR_CE_036)
        })
    );
}

#[test]
fn test_config_string_wrapper_generators_validate_argument_count_and_names() {
    assert!(
        [
            (
                quote::quote!(),
                constants_str::COMPILE_ERROR_CE_065,
                constants_str::COMPILE_ERROR_CE_074
            ),
            (
                quote::quote!(Value, Error, Extra),
                constants_str::COMPILE_ERROR_CE_065,
                constants_str::COMPILE_ERROR_CE_074
            ),
            (
                quote::quote!(1, Error),
                constants_str::COMPILE_ERROR_CE_064,
                constants_str::COMPILE_ERROR_CE_073
            ),
            (
                quote::quote!(Value, 1),
                constants_str::COMPILE_ERROR_CE_063,
                constants_str::COMPILE_ERROR_CE_072
            ),
        ]
        .into_iter()
        .all(
            |(input, plain, secret)| crate::impl_try_from_non_empty_string(input.clone())
                .to_string()
                .contains(plain)
                && crate::impl_try_from_secret_url(input)
                    .to_string()
                    .contains(secret)
        )
    );
}

#[test]
fn test_config_parse_wrapper_generators_validate_required_identifiers() {
    assert!(
        [
            (
                quote::quote!(1, Error, Inner, Invalid, source, Source),
                constants_str::COMPILE_ERROR_CE_070
            ),
            (
                quote::quote!(Value, 1, Inner, Invalid, source, Source),
                constants_str::COMPILE_ERROR_CE_067
            ),
            (
                quote::quote!(Value, Error, Inner, 1, source, Source),
                constants_str::COMPILE_ERROR_CE_068
            ),
            (
                quote::quote!(Value, Error, Inner, Invalid, 1, Source),
                constants_str::COMPILE_ERROR_CE_066
            ),
        ]
        .into_iter()
        .all(
            |(input, expected)| crate::impl_try_from_parse(input.clone())
                .to_string()
                .contains(expected)
                && crate::impl_try_from_parse_string_error(input)
                    .to_string()
                    .contains(expected)
        )
    );
    assert!(
        crate::impl_try_from_parse(quote::quote!(Value, Error, Inner, Invalid, source))
            .to_string()
            .contains(constants_str::COMPILE_ERROR_CE_071)
    );
    assert!(
        crate::impl_try_from_parse_string_error(quote::quote!(Value, Error, Inner, Invalid))
            .to_string()
            .contains(constants_str::COMPILE_ERROR_CE_071)
    );
}

#[test]
fn test_config_parse_wrapper_preserves_inner_error_type_and_requested_derives() {
    let text = crate::impl_try_from_parse(quote::quote!(
        Value,
        ParseError,
        Inner,
        Invalid,
        source,
        SourceError,
        Clone,
        Copy
    ))
    .to_string();
    assert!(
        text.contains(
            quote::quote!(
                pub struct Value(Inner);
            )
            .to_string()
            .as_str()
        )
    );
    assert!(
        text.contains(
            quote::quote!(Invalid {
                source: SourceError
            })
            .to_string()
            .as_str()
        )
    );
    assert!(text.contains(quote::quote!(Debug, Clone, Copy,).to_string().as_str()));
    assert!(
        text.contains(
            quote::quote!(|source| Self::Error::Invalid { source })
                .to_string()
                .as_str()
        )
    );
    let fixed = crate::impl_try_from_parse_string_error(quote::quote!(
        Value, ParseError, Inner, Invalid, source
    ))
    .to_string();
    assert!(
        fixed.contains(
            quote::quote!(Invalid { source: String })
                .to_string()
                .as_str()
        )
    );
    assert!(fixed.contains(quote::quote!(Debug, Clone, Copy,).to_string().as_str()));
}

#[test]
fn test_empty_config_parse_assertion_preserves_error_pattern_and_rejects_extra_arguments() {
    assert_eq!(
        crate::assert_empty_parse_err_matches(quote::quote!(Value, ParseError::Invalid { source }))
            .to_string(),
        quote::quote! { assert!(matches!(parse_env::<Value>(""), Err(ParseError::Invalid { source }))); }.to_string()
    );
    assert!(
        [quote::quote!(), quote::quote!(Value, Error, Extra)]
            .into_iter()
            .all(|input| crate::assert_empty_parse_err_matches(input)
                .to_string()
                .contains(constants_str::COMPILE_ERROR_CE_033))
    );
}

#[test]
fn test_nonempty_config_wrapper_emits_bounded_validated_storage() {
    let text =
        crate::impl_try_from_non_empty_string(quote::quote!(Value, ValidationError)).to_string();
    assert!(text.contains(quote::quote! { pub struct Value(bounded_types::bounded_string::BoundedString<1usize, { crate::config_lib_string_wrapper_max_len::CONFIG_LIB_STRING_WRAPPER_MAX_LEN }, false>); }.to_string().as_str()));
    assert!(
        text.contains(
            quote::quote! { impl TryFrom<String> for Value }
                .to_string()
                .as_str()
        )
    );
    assert!(text.contains(quote::quote! { if value.is_empty() { Err(Self::Error::IsEmpty { is_empty: constants_str::CONFIG_ENV_VALUE_IS_EMPTY_MSG, }) } }.to_string().as_str()));
    assert!(text.contains(quote::quote! { bounded_types::bounded_string::BoundedString::try_from(value).map(Self).map_err(|_error| Self::Error::TooLong) }.to_string().as_str()));
}

#[test]
fn test_secret_config_wrapper_validates_before_boxing_bounded_secret() {
    let text =
        crate::impl_try_from_secret_url(quote::quote!(SecretValue, ValidationError)).to_string();
    assert!(text.contains(quote::quote! { pub struct SecretValue(secrecy::SecretBox<crate::std_config_secret_string::StdConfigSecretString>); }.to_string().as_str()));
    assert!(text.contains(quote::quote! { if v.is_empty() { return Err(Self::Error::IsEmpty { is_empty: constants_str::CONFIG_ENV_VALUE_IS_EMPTY_MSG, }); } }.to_string().as_str()));
    assert!(text.contains(quote::quote! { crate::std_config_secret_string::StdConfigSecretString::try_from(String::from(v)).map(|bounded| Self(secrecy::SecretBox::new(Box::new(bounded)))).map_err(|_error| Self::Error::TooLong) }.to_string().as_str()));
}
