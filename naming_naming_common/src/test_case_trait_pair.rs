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
    let token_stream = crate::to_token_stream_or_panic::to_token_stream_or_panic(&'[').into_inner();
    let mut tokens = token_stream.into_iter();
    assert!(
        matches!(tokens.next(), Some(proc_macro2::TokenTree::Ident(identifier)) if identifier == stringify!(compile_error))
    );
    assert!(
        matches!(tokens.next(), Some(proc_macro2::TokenTree::Punct(punctuation)) if punctuation.as_char() == '!')
    );
    assert!(
        matches!(tokens.next(), Some(proc_macro2::TokenTree::Group(group)) if group.delimiter() == proc_macro2::Delimiter::Parenthesis && !group.stream().is_empty())
    );
    assert!(
        matches!(tokens.next(), Some(proc_macro2::TokenTree::Punct(punctuation)) if punctuation.as_char() == ';')
    );
    assert!(tokens.next().is_none());
}
