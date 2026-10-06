#[test]
fn test_token_pattern_adapters_preserve_distinct_identifier_and_comma_diagnostics() {
    let cases = [
        (proc_macro2::TokenStream::new(), false),
        (quote::quote! { 1, value }, false),
        (quote::quote! { (Name), value }, false),
        (quote::quote! { Name }, true),
        (quote::quote! { Name value }, true),
        (quote::quote! { Name; value }, true),
    ];
    assert!(cases.into_iter().all(|(input, identifier_present)| {
        let pattern_message = if identifier_present {
            constants_str::COMPILE_ERROR_CE_075
        } else {
            constants_str::COMPILE_ERROR_CE_076
        };
        let function_message = if identifier_present {
            constants_str::COMPILE_ERROR_CE_081
        } else {
            constants_str::COMPILE_ERROR_CE_082
        };
        let expected_diagnostic = |message| {
            workspace_macro_helpers::compile_error_token_stream::compile_error_token_stream(message)
                .into_inner()
                .to_string()
        };
        crate::token_pattern(input.clone()).to_string() == expected_diagnostic(pattern_message)
            && crate::token_stream_path_function(input).to_string()
                == expected_diagnostic(function_message)
    }));
    assert!(
        [
            (
                proc_macro2::TokenStream::new(),
                constants_str::COMPILE_ERROR_CE_078
            ),
            (quote::quote! { Name }, constants_str::COMPILE_ERROR_CE_078),
            (
                quote::quote! { 1, value },
                constants_str::COMPILE_ERROR_CE_077
            ),
            (
                quote::quote! { (Name), value },
                constants_str::COMPILE_ERROR_CE_077
            ),
        ]
        .into_iter()
        .all(|(input, message)| {
            crate::token_pattern_parts(input).to_string()
                == workspace_macro_helpers::compile_error_token_stream::compile_error_token_stream(
                    message,
                )
                .into_inner()
                .to_string()
        }),
    );
}

#[test]
fn test_token_pattern_batch_ignores_other_tokens_and_preserves_output_and_error_order() {
    let first = crate::token_pattern(quote::quote! { Name, value });
    let invalid = workspace_macro_helpers::compile_error_token_stream::compile_error_token_stream(
        constants_str::COMPILE_ERROR_CE_076,
    )
    .into_inner();
    let last = crate::token_pattern(quote::quote! { Other, other_value });
    let expected = [first, invalid, last]
        .into_iter()
        .collect::<proc_macro2::TokenStream>();
    let actual = crate::token_pattern_batch(quote::quote! {
        ignored [Ignored, value] {Ignored, value} 1,
        (Name, value) (,) (Other, other_value)
    });
    assert_eq!(actual.to_string(), expected.to_string());
    assert!(
        crate::token_pattern_batch(quote::quote! {
            ignored [Ignored, value] {Ignored, value} 1,
        })
        .is_empty()
    );
}
