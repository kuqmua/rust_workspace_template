pub fn generate_impl_to_err_string_token_stream(
    impl_generics_token_stream: &dyn quote::ToTokens,
    ident_token_stream: &dyn quote::ToTokens,
    ident_generics_token_stream: &dyn quote::ToTokens,
    ts: &dyn quote::ToTokens,
) -> crate::proc_macro2_tokens::ProcMacro2GeneratedRustTokenStream {
    let self_snake_case = naming::SelfSnakeCase;
    let to_err_string_snake_case = naming::ToErrStringSnakeCase;
    let to_err_string_upper_camel_case = naming::ToErrStringUpperCamelCase;
    quote::quote! {
        impl #impl_generics_token_stream to_err_string::#to_err_string_upper_camel_case for #ident_token_stream #ident_generics_token_stream {
            fn #to_err_string_snake_case(&#self_snake_case) -> to_err_string::ToErrStringValue {
                to_err_string::ToErrStringValue::try_from(#ts).unwrap_or_else(to_err_string::ToErrStringValue::from)
            }
        }
    }
    .into()
}
