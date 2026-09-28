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
