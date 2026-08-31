pub fn generate_impl_from_token_stream(
    from_type_token_stream: &dyn quote::ToTokens,
    for_type_token_stream: &dyn quote::ToTokens,
    ts: &dyn quote::ToTokens,
) -> crate::proc_macro2_generated_rust_token_stream::ProcMacro2GeneratedRustTokenStream {
    let v_snake_case = naming::domain_types::VSnakeCase;
    quote::quote! {
        impl From<#from_type_token_stream> for #for_type_token_stream {
            fn from(value: #from_type_token_stream) -> Self {
                let #v_snake_case = value;
                #ts
            }
        }
    }
    .into()
}
