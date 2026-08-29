pub fn generate_impl_to_err_string_token_stream(
    impl_generics_token_stream: &dyn quote::ToTokens,
    ident_token_stream: &dyn quote::ToTokens,
    ident_generics_token_stream: &dyn quote::ToTokens,
    ts: &dyn quote::ToTokens,
) -> crate::proc_macro2_generated_rust_token_stream::ProcMacro2GeneratedRustTokenStream {
    let self_snake_case = naming::domain_types::SelfSnakeCase;
    let to_err_string_snake_case = naming::domain_types::ToErrStringSnakeCase;
    let to_err_string_upper_camel_case = naming::domain_types::ToErrStringUpperCamelCase;
    quote::quote! {
        impl #impl_generics_token_stream to_err_string::to_err_string::#to_err_string_upper_camel_case for #ident_token_stream #ident_generics_token_stream {
            fn #to_err_string_snake_case(&#self_snake_case) -> to_err_string::error_text::ErrorText {
                to_err_string::error_text::ErrorText::try_from(#ts).unwrap_or_else(to_err_string::error_text::ErrorText::from)
            }
        }
    }
    .into()
}
