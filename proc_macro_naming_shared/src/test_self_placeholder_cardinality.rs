#[test]
fn test_self_naming_requires_exactly_one_placeholder() {
    assert!(
        [constants_usize::ZERO, constants_usize::TWO, 3usize]
            .into_iter()
            .all(|count| {
                let result = std::panic::catch_unwind(|| {
                    let words = std::iter::repeat_n(constants_str::SELF_ALT, count);
                    crate::generate_self_upper_camel_case_and_snake_case_str_and_token_stream(
                        quote::quote! {[[#(#words),*]]},
                    )
                });
                match result {
                    Ok(_tokens) => false,
                    Err(payload) => {
                        payload.downcast_ref::<String>().is_some_and(|string| {
                            string.starts_with(constants_str::DIAGNOSTIC_5680DD63)
                        }) || payload
                            .downcast_ref::<&str>()
                            .is_some_and(|str| str.starts_with(constants_str::DIAGNOSTIC_5680DD63))
                    }
                }
            })
    );
    assert!(
        [
            [constants_str::SELF_ALT, constants_str::CURSOR_TEST_PAYLOAD],
            [constants_str::CURSOR_TEST_PAYLOAD, constants_str::SELF_ALT],
        ]
        .into_iter()
        .all(|words| {
            syn::parse2::<syn::File>(
                crate::generate_self_upper_camel_case_and_snake_case_str_and_token_stream(
                    quote::quote! {[[#(#words),*]]},
                ),
            )
            .is_ok_and(|file| !file.items.is_empty())
        })
    );
}
