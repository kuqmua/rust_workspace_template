#[test]
fn test_token_string_parser_preserves_empty_input_and_declaration_order() {
    let parse_error_id =
        crate::parse_error_id_ref::ParseErrorIdRef::from(constants_str::DIAGNOSTIC_D1846F3A);
    let empty = crate::parse_token_stream_strings_into_generated_vec::parse_token_stream_strings_into_generated_vec(
        crate::parse_token_stream_strings::ParseTokenStreamStrings::from(Vec::new()),
        parse_error_id,
    );
    assert!(quote::quote! { #empty }.is_empty());
    let declarations = [
        quote::quote! { struct TestParsedFirst; },
        quote::quote! { struct TestParsedSecond; },
    ];
    let input = declarations
        .iter()
        .map(ToString::to_string)
        .collect::<Vec<_>>();
    let generated = crate::parse_token_stream_strings::ParseTokenStreamStrings::from(input)
        .into_generated_vec(parse_error_id);
    assert_eq!(
        quote::quote! { #generated }.to_string(),
        quote::quote! { #(#declarations)* }.to_string()
    );
}

#[test]
fn test_token_string_parser_emits_identified_errors_between_valid_items() {
    let first = quote::quote! { struct TestParsedBeforeError; };
    let last = quote::quote! { struct TestParsedAfterError; };
    let malformed = '('.to_string();
    let generated = crate::parse_token_stream_strings_into_generated_vec::parse_token_stream_strings_into_generated_vec(
        crate::parse_token_stream_strings::ParseTokenStreamStrings::from(vec![
            first.to_string(),
            malformed.clone(),
            last.to_string(),
        ]),
        crate::parse_error_id_ref::ParseErrorIdRef::from(constants_str::DIAGNOSTIC_D1846F3A),
    );
    assert!(
        syn::parse2::<syn::File>(quote::quote! { #generated }).is_ok_and(|file| {
            if file.items.len() != 3usize {
                return false;
            }
            let [
                syn::Item::Struct(before),
                syn::Item::Macro(error),
                syn::Item::Struct(after),
            ] = file.items.as_slice()
            else {
                return false;
            };
            before.ident == stringify!(TestParsedBeforeError)
                && after.ident == stringify!(TestParsedAfterError)
                && error.mac.path.is_ident(stringify!(compile_error))
                && error.semi_token.is_some()
                && syn::parse2::<syn::LitStr>(error.mac.tokens.clone()).is_ok_and(|message| {
                    malformed
                        .parse::<proc_macro2::TokenStream>()
                        .err()
                        .is_some_and(|parse_error| {
                            message.value()
                                == format!("{}: {parse_error}", constants_str::DIAGNOSTIC_D1846F3A)
                        })
                })
        })
    );
}
