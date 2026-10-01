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
