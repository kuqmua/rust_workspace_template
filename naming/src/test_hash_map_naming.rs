#[test]
fn test_hash_map_cases_preserve_display_spelling_and_existing_token_stream() {
    let hash_map_snake_case = crate::hash_map_snake_case::HashMapSnakeCase;
    let hash_map_upper_camel_case = crate::hash_map_upper_camel_case::HashMapUpperCamelCase;
    let mut snake_tokens = quote::quote!(prefix);
    quote::ToTokens::to_tokens(&hash_map_snake_case, &mut snake_tokens);
    let mut upper_tokens = quote::quote!(prefix);
    quote::ToTokens::to_tokens(&hash_map_upper_camel_case, &mut upper_tokens);
    assert!(
        [
            (
                hash_map_snake_case.to_string(),
                snake_tokens,
                constants_str::VALUE_77DD5D04
            ),
            (
                hash_map_upper_camel_case.to_string(),
                upper_tokens,
                constants_str::HASHMAP
            ),
        ]
        .into_iter()
        .all(|(display, token_stream, expected)| {
            let mut tokens = token_stream.into_iter();
            display == expected && tokens.next().is_some_and(|token| {
                matches!(token,
                    proc_macro2::TokenTree::Ident(identifier) if identifier == stringify!(prefix)
                )
            }) && tokens.next().is_some_and(|token| {
                matches!(token,
                    proc_macro2::TokenTree::Ident(identifier) if identifier == expected
                )
            }) && tokens.next().is_none()
        })
    );
}
