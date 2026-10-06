#[test]
fn test_table_diagnostic_tokens_preserve_borrowed_text_and_escape_control_characters() {
    assert!(
        [
            constants_str::EMPTY.to_owned(),
            constants_str::NON_ASCII_U_E9.to_owned(),
            constants_str::TEST_TEXT_WITH_NUL.to_owned(),
            (u8::MIN..=u8::MAX).map(char::from).collect::<String>(),
        ]
        .iter()
        .all(|text| {
            [
                crate::pg_table_compile_error_message::PgTableCompileErrorMessage::from(text),
                crate::pg_table_compile_error_message::PgTableCompileErrorMessage::from(
                    text.as_str(),
                ),
            ]
            .into_iter()
            .all(|message| {
                assert_eq!(message.as_ref(), text.as_str());
                assert_eq!(message.as_ref().as_ptr(), text.as_ptr());
                let tokens =
                    crate::pg_table_compile_error_tokens::pg_table_compile_error_tokens(message);
                syn::parse2::<syn::ItemMacro>(proc_macro2::TokenStream::from(tokens)).is_ok_and(
                    |item| {
                        item.attrs.is_empty()
                            && item.ident.is_none()
                            && item.mac.path.is_ident(stringify!(compile_error))
                            && matches!(item.mac.delimiter, syn::MacroDelimiter::Paren(_))
                            && item.semi_token.is_some()
                            && syn::parse2::<syn::LitStr>(item.mac.tokens)
                                .is_ok_and(|literal| literal.value() == *text)
                    },
                )
            })
        })
    );
}
