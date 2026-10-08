#[test]
fn test_plain_naming_template_reports_oversized_case_conversion() {
    let long_word = constants_str::HELLOWORLD_ALT.repeat(104_858usize);
    let generated = crate::generate_upper_camel_case_and_snake_case_str_and_token_stream(
        quote::quote! {[[#long_word]]},
    );
    assert!(
        generated
            .to_string()
            .contains(constants_str::VALUE_2EDAC0BF)
    );
}

#[test]
fn test_self_naming_template_reports_oversized_case_conversion() {
    let long_word = constants_str::HELLOWORLD_ALT.repeat(104_858usize);
    let self_word = constants_str::SELF_ALT;
    let generated = crate::generate_self_upper_camel_case_and_snake_case_str_and_token_stream(
        quote::quote! {[[#self_word, #long_word]]},
    );
    assert!(
        generated
            .to_string()
            .contains(constants_str::VALUE_2EDAC0BF)
    );
}

#[test]
fn test_naming_templates_reject_invalid_json_shapes() {
    assert!(
        [
            (false, constants_str::DIAGNOSTIC_90E5793B),
            (true, constants_str::DIAGNOSTIC_9D6A20AF),
        ]
        .into_iter()
        .all(|(self_template, diagnostic)| {
            [
                quote::quote! {{}},
                quote::quote! {true},
                quote::quote! {[[1]]},
            ]
            .into_iter()
            .all(|token_stream| {
                let result = std::panic::catch_unwind(|| {
                    if self_template {
                        crate::generate_self_upper_camel_case_and_snake_case_str_and_token_stream(
                            token_stream,
                        )
                    } else {
                        crate::generate_upper_camel_case_and_snake_case_str_and_token_stream(
                            token_stream,
                        )
                    }
                });
                result.is_err_and(|payload| {
                    payload
                        .downcast_ref::<String>()
                        .is_some_and(|string| string.starts_with(diagnostic))
                        || payload
                            .downcast_ref::<&str>()
                            .is_some_and(|str| str.starts_with(diagnostic))
                })
            })
        })
    );
}
