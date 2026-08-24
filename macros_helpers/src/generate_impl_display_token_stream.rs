pub fn generate_impl_display_token_stream(
    impl_generics_token_stream: &dyn quote::ToTokens,
    ident_token_stream: &dyn quote::ToTokens,
    ident_generics_token_stream: &dyn quote::ToTokens,
    ts: &dyn quote::ToTokens,
) -> crate::proc_macro2_tokens::ProcMacro2GeneratedRustTokenStream {
    let self_snake_case = naming::domain_types::SelfSnakeCase;
    quote::quote! {
        impl #impl_generics_token_stream std::fmt::Display for #ident_token_stream #ident_generics_token_stream {
            fn fmt(&#self_snake_case, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                #ts
            }
        }
    }
    .into()
}
