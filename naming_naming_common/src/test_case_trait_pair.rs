#[test]
fn test_case_trait_pair_preserves_move_and_raw_parameters() {
    proc_macro_naming_common::case_trait_pair!(
        MoveRawCaseFixture,
        MoveRawTokenCaseFixture,
        std::fmt::Display,
        move |r#type| Ok(r#type.to_string())
    );
    assert_eq!(MoveRawCaseFixture::case(&42u8), 42u8.to_string());
    assert_eq!(
        MoveRawTokenCaseFixture::case_or_panic(&42u8).to_string(),
        42u8.to_string(),
    );
}

#[test]
fn test_naming_case_helpers_accept_empty_input() {
    let input = constants_str::EMPTY;
    let case = crate::convert_case_kind::ConvertCaseKind::from(convert_case::Case::Snake);
    [
        crate::str_case::str_case(input, case),
        crate::case_from_string::case_from_string(input, case),
        crate::display_case_str::display_case_str(&input, case),
        crate::tokenized_case_str::tokenized_case_str(&quote::quote!(), case),
    ]
    .into_iter()
    .fold((), |(), result| {
        assert!(result.is_ok_and(|value| value.as_ref().is_empty()));
    });
}

#[test]
fn test_naming_token_conversion_preserves_empty_and_valid_tokens() {
    let empty = crate::to_token_stream_or_panic::to_token_stream_or_panic(constants_str::EMPTY)
        .into_inner();
    assert!(empty.is_empty());
    let number = crate::to_token_stream_or_panic::to_token_stream_or_panic(&42u8).into_inner();
    assert_eq!(number.to_string(), 42u8.to_string());
}

#[test]
fn test_naming_token_conversion_emits_compile_error_for_unclosed_delimiter() {
    let expected_message = '['
        .to_string()
        .parse::<proc_macro2::TokenStream>()
        .err()
        .map(|error| error.to_string());
    assert!(expected_message.is_some());
    let token_stream = crate::to_token_stream_or_panic::to_token_stream_or_panic(&'[').into_inner();
    let mut tokens = token_stream.into_iter();
    assert!(
        matches!(tokens.next(), Some(proc_macro2::TokenTree::Ident(identifier)) if identifier == stringify!(compile_error))
    );
    assert!(
        matches!(tokens.next(), Some(proc_macro2::TokenTree::Punct(punctuation)) if punctuation.as_char() == '!')
    );
    assert!(
        matches!(tokens.next(), Some(proc_macro2::TokenTree::Group(group)) if group.delimiter() == proc_macro2::Delimiter::Parenthesis && expected_message.as_ref().is_some_and(|message| group.stream().to_string() == quote::quote!(#message).to_string()))
    );
    assert!(
        matches!(tokens.next(), Some(proc_macro2::TokenTree::Punct(punctuation)) if punctuation.as_char() == ';')
    );
    assert!(tokens.next().is_none());
}

#[test]
fn test_naming_trait_families_preserve_exact_overflow_and_error_fallbacks() {
    let maximum = crate::case_string_max_len::CASE_STRING_MAX_LEN;
    let input = constants_str::X.repeat(maximum + 1usize);
    let identifier = proc_macro2::Ident::new(&input, proc_macro2::Span::call_site());
    [
        (crate::domain_types::AsRefStrToUpperCamelCaseStr::try_case(&input), crate::domain_types::AsRefStrToUpperCamelCaseStr::case(&input), crate::domain_types::AsRefStrToUpperCamelCaseTokenStream::case_or_panic(&input)),
        (crate::domain_types::AsRefStrToSnakeCaseStr::try_case(&input), crate::domain_types::AsRefStrToSnakeCaseStr::case(&input), crate::domain_types::AsRefStrToSnakeCaseTokenStream::case_or_panic(&input)),
        (crate::domain_types::AsRefStrToUpperSnakeCaseStr::try_case(&input), crate::domain_types::AsRefStrToUpperSnakeCaseStr::case(&input), crate::domain_types::AsRefStrToUpperSnakeCaseTokenStream::case_or_panic(&input)),
        (crate::domain_types::DisplayToUpperCamelCaseStr::try_case(&input), crate::domain_types::DisplayToUpperCamelCaseStr::case(&input), crate::domain_types::DisplayToUpperCamelCaseTokenStream::case_or_panic(&input)),
        (crate::domain_types::DisplayToSnakeCaseStr::try_case(&input), crate::domain_types::DisplayToSnakeCaseStr::case(&input), crate::domain_types::DisplayToSnakeCaseTokenStream::case_or_panic(&input)),
        (crate::domain_types::DisplayToUpperSnakeCaseStr::try_case(&input), crate::domain_types::DisplayToUpperSnakeCaseStr::case(&input), crate::domain_types::DisplayToUpperSnakeCaseTokenStream::case_or_panic(&input)),
        (crate::domain_types::ToTokensToUpperCamelCaseStr::try_case(&identifier), crate::domain_types::ToTokensToUpperCamelCaseStr::case(&identifier), crate::domain_types::ToTokensToUpperCamelCaseTokenStream::case_or_panic(&identifier)),
        (crate::domain_types::ToTokensToSnakeCaseStr::try_case(&identifier), crate::domain_types::ToTokensToSnakeCaseStr::case(&identifier), crate::domain_types::ToTokensToSnakeCaseTokenStream::case_or_panic(&identifier)),
        (crate::domain_types::ToTokensToUpperSnakeCaseStr::try_case(&identifier), crate::domain_types::ToTokensToUpperSnakeCaseStr::case(&identifier), crate::domain_types::ToTokensToUpperSnakeCaseTokenStream::case_or_panic(&identifier)),
    ]
    .into_iter()
    .fold((), |(), (result, message, tokens)| {
        assert!(matches!(&result, Err(crate::case_string::CaseStringTryFromStringError::TooLong { len, max }) if *len == maximum + 1usize && *max == maximum));
        assert!(result.is_err_and(|error| error.to_string() == message));
        assert_eq!(tokens.to_string(), quote::quote!(compile_error!(#message);).to_string());
    });
}
